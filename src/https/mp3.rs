
use actix_web::cookie::time::Instant;
use tokio::time::{timeout, Duration};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};
use reqwest::Client;
use std::path::Path;
use futures::future::{try_join_all};
use async_recursion::async_recursion;
use std::i16;
use std::process::Stdio;

use ffmpeg_cli::{FfmpegBuilder, File, Parameter};

use std::fs;


fn get_name(s:&str)->Result<String,String>{
    let name=Path::new(s).components().last().and_then(|c| c.as_os_str().to_str());
    match name{
        Some(a)=>Ok(a.to_owned()),
        None=>Err("error".to_owned())
    }
}

fn get_url(s:&str)->Result<&Path,String>{
    let parent_path = Path::new(s).parent();
    match parent_path{
        Some(a)=>Ok(a),
        None=>Err("Path not generated".to_owned())
    }
}

#[async_recursion]
pub async fn download(url:String)->Result<String,Box<dyn std::error::Error>>{
    let realurl=htmlescape::decode_html(url.as_str()).unwrap_or("error".to_owned()); 
    let name=get_name(&realurl)?;
    let client = Client::new();
    // Send the GET request
    let mut response = timeout(Duration::from_secs(10),client.get(&realurl).send()).await??;

    // Check if the request was successful
    if response.status().is_success() {
        if realurl.contains("m3u8"){
            let text=response.text().await.unwrap();
            return Ok(text);
        }else{
                // Open a file for writing
            let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("temp/{}",name))
            .await?;

        // Create a buffered writer for better performance
            let mut writer = BufWriter::new(file);

            // Download and write the response body asynchronously
            while let Some(chunk) = response.chunk().await?{
                writer.write_all(&chunk).await?;
            }

            // Flush the writer to ensure all data is written to the file
            writer.flush().await?;
            println!("{} downloaded and saved successfully!",realurl.as_str());
            Ok(convert(&format!("temp/{}",name)).await?)
        }
    } else {
        println!("Failed to download MP3 file. Status: {}", response.status());
        return Ok("Error in download".to_owned());
    }

    
}


async fn convert(input: &String)->Result<String,Box<dyn std::error::Error>> {
    let target_char = '.';

    if let Some(index) = input.rfind(target_char) {
        let new_filename = format!("{}.wav", &input[..index]);
        println!("{}",new_filename);
        
        let builder = FfmpegBuilder::new()
        .stderr(Stdio::piped())
        .option(Parameter::Single("nostdin"))
        .option(Parameter::Single("y"))
        .input(File::new(input))
        .output(
            File::new(new_filename.as_str())
        );

        let ffmpeg = builder.run().await.unwrap();

        ffmpeg
            .progress;

        let output = ffmpeg.process.wait_with_output().unwrap();

        println!(
            "{}\nstderr:\n{}",
            output.status,
            std::str::from_utf8(&output.stderr).unwrap()
        );
        fs::remove_file(input)?;  
        Ok(new_filename.to_owned())
    } else {
        println!("Character '{}' not found in the string", target_char);
        Ok("err".to_owned())
    }


}



fn concatenate_files(inputs: &[String], output_file: &str) -> Result<(), hound::Error>  {
    println!("{:?}",inputs);
    let first_input_reader = hound::WavReader::open(&inputs[0])?;

    let spec = first_input_reader.spec();
    let path: &Path = output_file.as_ref();
    let mut writer = match path.is_file() {
        true => hound::WavWriter::append(path).unwrap(),
        false => hound::WavWriter::create(path, spec).unwrap(),
    };

    for input_file in inputs {
        let reader = hound::WavReader::open(input_file)?;

        for sample in reader.into_samples::<i16>() {
            writer.write_sample(sample?)?;
        }
        fs::remove_file(input_file)?;
    }

    writer.finalize()?;
    Ok(())
}

async fn m3u8_helper(i:usize,url:&String)->Result<String, Box<dyn std::error::Error>>{
    println!("{}", url);
        let parent = get_url(&url)?;
        let text = download(url.to_owned()).await?;
        let list: Vec<&str> = text.split('\n').filter(|&s| !s.contains('#')).collect();
        println!("{:?}", list);
        let mut new = Vec::new();
        for i in list {
            if i.len() > 0 {
                let t = format!("{}/{}", parent.to_str().unwrap(), i);
                println!("{}", t);
                new.push(download(t));
            }
        }
        let resutls = try_join_all(new).await?;
       concatenate_files(&resutls, &format!("{}.mp3", &i))?;
        return Ok(format!("{}.mp3", &i));
}


async fn handlem3u8(vec: Vec<String>, output: String) -> Result<String, Box<dyn std::error::Error>> {
    let chunk=50;
    let mut longest=Instant::now()-Instant::now();
    let mut results=Vec::new();
    let mut i=0;
    for urls in vec.chunks(chunk){
        let start=Instant::now();
        let mut mps = Vec::new();
        for (u, url) in urls.iter().enumerate() {
            let num=u+chunk*i;
            println!("{}",num);
            mps.push(m3u8_helper(num, url));

        }
        results.push(try_join_all(mps).await?);
        let end=Instant::now()-start;
        if longest<end{longest=end;}

        i+=1;
    }
    let res: Vec<String>=results.into_iter()
    .flatten()
    .collect();


    println!("{:?}",longest);

    println!("res:{:?} vec:{:?}, true:{:?}",&res.len(),&vec.len(), res.len()==vec.len());
    if res.len()==vec.len(){
        println!("{}",output);
        concatenate_files(&res, &output)?;
        return Ok("done".to_owned());
    }else{
        println!("error in downl9oading");
        for i in res{
            fs::remove_file(i)?;
        }
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "error".to_string())));
    }

    //Ok("test".to_string())
}
/*
async fn handlem3u8(vec: Vec<String>, output: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut mps: Vec<String> = Vec::new();
    for (i, url) in vec.iter().enumerate() {
        println!("{}", url);
        let parent = get_url(&url)?;
        let text = download(url.to_owned()).await?;
        let list: Vec<&str> = text.split('\n').filter(|&s| !s.contains('#')).collect();
        println!("{:?}", list);
        let mut new = Vec::new();
        for i in list {
            if i.len() > 0 {
                let t = format!("{}/{}", parent.to_str().unwrap(), i);
                println!("{}", t);
                new.push(download(t));
            }
        }
        let resutls = try_join_all(new).await?;
        concatenate_files(&resutls, &format!("{}.ts", &i))?;
        mps.push(format!("{}.ts", &i));
    }
    concatenate_files(&mps, &output)?;
    Ok("ok".to_owned())
}

*/
async fn handlemp3(vec:Vec<String>,output:String)->Result<String, Box<dyn std::error::Error>>{
    let mut new=Vec::new();
    for i in vec{
        if i.len()>0{
            new.push(download(i));
        }
    }
    let results=try_join_all(new).await?;


    concatenate_files(&results, &output.as_str())?;
    Ok("ok".to_owned())
    
}

pub async fn handleaudio(vec:Vec<String>,name:String)->Result<String, Box<dyn std::error::Error>>{
    if vec.len()>0 && vec[0].contains("m3u8"){
        handlem3u8(vec,name).await?;
    }else{
        handlemp3(vec,name).await?;
    }
    Ok("done".to_owned())
}

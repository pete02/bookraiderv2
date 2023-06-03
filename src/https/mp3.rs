
use tokio::time::{timeout, Duration};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};
use reqwest::Client;
use std::path::Path;
use futures::future::try_join_all;
use async_recursion::async_recursion;
use std::process::Stdio;
use std::future::Future;
use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use std::io;
use std::fs;

use std::error::Error;
use std::fmt;


#[derive(Debug,Clone)]
pub struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}


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
pub async fn download(url:String)->Result<String,Box<MyError>>{
    let realurl=htmlescape::decode_html(url.as_str()).unwrap_or("error".to_owned()); 
    let name;
    match get_name(&realurl) {
        Ok(s)=>name=s,
        Err(_)=>return Err(Box::new(MyError("Oops in download".into())))
    };
    let client = Client::new();
    // Send the GET request
    let mut response;

    match timeout(Duration::from_secs(10),client.get(&realurl).send()).await {
        Ok(s)=> match s {
            Ok(a)=>response=a,
            Err(_)=>return Err(Box::new(MyError("Oops in download".into())))
        }
        Err(_)=>return Err(Box::new(MyError("Oops in download".into())))
    }

    // Check if the request was successful
    if response.status().is_success() {
        if realurl.contains("m3u8"){
            let text=response.text().await.unwrap();
            return Ok(text);
        }else{
                // Open a file for writing
            let mut writer;
            match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("temp/{}",name))
            .await{
                Ok(file)=>writer=BufWriter::new(file),
                Err(_)=>return Err(Box::new(MyError("Oops in download".into())))
            }

        // Create a buffered writer for better performance

            // Download and write the response body asynchronously
            while let Ok(Some(chunk)) = response.chunk().await{
                match writer.write_all(&chunk).await{
                    Ok(_)=>(),
                    Err(_)=>return Err(Box::new(MyError("Oops in download".into())))
                }
            }

            // Flush the writer to ensure all data is written to the file
            match writer.flush().await{
                Ok(_)=>(),
                Err(_)=>return Err(Box::new(MyError("Oops in download".into())))
            }
            Ok(format!("temp/{}",name))

        }
    } else {
        println!("Failed to download MP3 file. Status: {}", response.status());
        return Ok("Error in download".to_owned());
    }

    
}


async fn convert(input: &String)->Result<String,Box<dyn std::error::Error>> {
    let target_char = '.';

    if let Some(index) = input.rfind(target_char) {
        let new_filename = format!("{}.mp3", &input[..index]);
        
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

        ffmpeg.process.wait_with_output()?;
        
        fs::remove_file(input)?;  
        Ok(new_filename.to_owned())
    } else {
        println!("Character '{}' not found in the string", target_char);
        Ok("err".to_owned())
    }


}




fn concatenate_files(inputs: &[String], output_file: &str) -> Result<(), hound::Error>  {
    let mut output = fs::File::create(output_file)?;
    for i in inputs {
        let mut input = fs::File::open(i)?;
        io::copy(&mut input, &mut output)?;
        fs::remove_file(i)?;
    }
    Ok(())
}

async fn m3u8_helper(i: usize, url: String) -> Result<String, Box<dyn std::error::Error>> {
    let parent = get_url(&url)?;
    let text = download(url.to_owned()).await?;
    let list: Vec<&str> = text.split('\n').filter(|&s| !s.contains('#')).collect();
    let mut new = Vec::new();
    println!("batch {} started",i);
    for i in list {
        if i.len() > 0 {
            let t = format!("{}/{}", parent.to_str().unwrap(), i);
            new.push(tokio::spawn(async move {download(t).await}));
        }
    }
    let results = try_join_all(new).await?;
    println!("batch {} downloaded",i);
    let mut true_r:Vec<String>=Vec::new();
    for r in results{
        let a=r?;

        match timeout(Duration::from_secs(10),convert(&format!("{}",a))).await {
            Ok(s)=> match s {
                Ok(a)=>{
                    println!("batch {} done",i);
                    true_r.push(a)
                },
                Err(e)=>return Err(Box::new(MyError(format!("Oops in helper timeout:{:?}",e).into())))
            }
            Err(_)=>return Err(Box::new(MyError("Oops in helper timeout #2".into())))
        }
    }


    concatenate_files(&true_r, &format!("temp/{}.mp3", &i))?;
    Ok(format!("temp/{}.mp3", &i))
}

async fn process_futures<F>(futures: Vec<F>, i: usize) -> Result<String, Box<dyn Error>>
where
    F: Future<Output = Result<String, Box<dyn Error>>> + Send + 'static,

{
    let a = try_join_all(futures).await?;

    concatenate_files(&a, &format!("temp/done{}.mp3", i))?;
    Ok(format!("temp/done{}.mp3", i))
}


async fn handlem3u8(vec: Vec<String>, output: String) -> Result<String, Box<dyn std::error::Error>> {
    let chunk=5;
    let mut results=Vec::new();
    let mut i=0;
    for urls in vec.chunks(chunk){
        let mut mps = Vec::new();
        for (u, url) in urls.iter().enumerate() {
            let num=u+chunk*i;
            mps.push(m3u8_helper(num, url.to_owned()));

        }
        results.push(process_futures(mps,i));


        i+=1;
    }


    let res=try_join_all(results).await?;

    println!("res:{:?} vec:{:?}, true:{:?}",&res.len(),&vec.len(), res.len()==vec.len());
    println!("{}",output);
    concatenate_files(&res, &output)?;
    return Ok("don".to_owned());


    //Ok("test".to_string())
}

async fn handlemp3(vec:Vec<String>,output:String)->Result<String, Box<dyn std::error::Error>>{
    let mut new=Vec::new();
    for i in vec{
        if i.len()>0{
            new.push(download(i));
        }
    }
    let results=try_join_all(new).await?;

    println!("{:?}",results);
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

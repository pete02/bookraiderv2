use std::time::{Instant};
use select::document::Document;
use futures::future::{join_all};

use crate::https::bookdata::get_google_book;
use crate::https::bookdata::download_thumbnail;
use crate::utils::structs::Site;
use crate::https::http::make_request;
use crate::https::http::get_response;

use serde_json::json;

use crate::utils::checks;
use checks::check_audio_format;

use crate::https::httpdoc;
use httpdoc::find_url;
use httpdoc::find_url_include;
use httpdoc::get_node;
use crate::https::mp3;
use crate::utils::text;
use text::find_audio_links;



pub async fn find_audio_book(book: &str, site: &Site) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let s = Instant::now();
    let request = make_request(format!("{}{}{}", site.url, site.search, book).as_str()).await?;

    let response = get_response(request).await?;
    let doc = Document::from(response.as_str());
    let node = get_node(&doc, &site.container, &site.classname)?;
    let urls = find_url(&node, &site.page,  &site.title);

    let result = json!({
        "Site": site.head,
        "res": urls,
    });

    println!("Request to {} took {:?}", site.url, Instant::now() - s);
    Ok(result)
}


pub async fn get_audiobook_page(url:String)->Result<Vec<String>,Box<dyn std::error::Error>>{
    let request=make_request(url.as_str()).await?;
    let response=get_response(request).await?;
    let extension=check_audio_format(&response)?;
    let urls=find_audio_links(&response,extension);
    Ok(urls)
}
//gets links for audiobook
pub async fn get_audiobook(url:&str,site:&Site,name:&String)->Result<String,Box<dyn std::error::Error>>{
    let request=make_request(&url).await?;
    let response=get_response(request).await?;
    let doc=Document::from(response.as_str());
    let node=get_node(&doc, "body", "")?;
    let urls=find_url_include(&node, &site.page);
    let mut vec=Vec::new();
    
    vec.push(get_audiobook_page(url.to_owned()));
    if urls.len()>0 {
        for url in urls{
            vec.push(get_audiobook_page(url))
        }
    }
    let results=join_all(vec).await;
    let mut res:Vec<String>=Vec::new();
    for result in results{
        let vec=result?;
        for text in vec{
            res.push(text);
        }
    }
    let bookdata=get_google_book(name).await?;

    let path_string = format!("done/{}/{}", bookdata.0, name);
    let path = std::path::Path::new(&path_string);

    if path.exists() {
        println!("Path exists!");
    } else {
        std::fs::create_dir(format!("done/{}",bookdata.0).as_str())?;
        std::fs::create_dir(format!("done/{}/{}",bookdata.0,name).as_str())?;
    }


    download_thumbnail(&bookdata.1, format!("done/{}/{}/cover.jpg",bookdata.0,name).as_str()).await?;
    mp3::handleaudio(res, format!("done/{}/{}/{}.mp3",bookdata.0,name,name)).await?;
    Ok("done".to_owned())
}

pub async fn search_audio_books(book: &str, sites: Vec<Site>) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>>{
    let mut tasks = vec![];

    for i in 0..sites.len() {
        let task = find_audio_book(&book, &sites[i]);
        tasks.push(task);
    }

    let results = futures::future::try_join_all(tasks).await?;
    Ok(results)
}

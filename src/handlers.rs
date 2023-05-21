use std::time::{Instant};
use select::document::Document;
use futures::future::{join_all};


use crate::utils::structs::Site;
use crate::http::make_request;
use crate::http::get_response;


use crate::utils::checks;
use checks::check_audio_format;

use crate::httpdoc;
use httpdoc::find_url;
use httpdoc::find_url_include;
use httpdoc::get_node;

use crate::utils::text;
use text::find_audio_links;


pub async fn find_audio_book(book:&str,site:&Site)-> Result<Vec<String>, Box<dyn std::error::Error>>{
    let s=Instant::now();
    let request=make_request(format!("{}{}{}",site.url,site.search,book).as_str()).await?;
    
    let response=get_response(request).await?;
    let doc=Document::from(response.as_str());
    let node=get_node(&doc, &site.container, &site.classname)?;
    let urls=find_url(&node, &site.page,&site.filters);
    println!("resuest to {} took {:?}",site.url,Instant::now()-s);
    Ok(urls)
}


pub async fn get_audiobook_page(url:String)->Result<Vec<String>,Box<dyn std::error::Error>>{
    let request=make_request(url.as_str()).await?;
    let response=get_response(request).await?;
    let extension=check_audio_format(&response)?;
    let urls=find_audio_links(&response,extension);
    Ok(urls)
}
//gets links for audiobook
pub async fn get_audiobook(url:&str,site:&Site)->Result<Vec<String>,Box<dyn std::error::Error>>{
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
    Ok(res)
}

pub async fn search_audio_books(book: &str, sites: Vec<Site>) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>>{
    let mut tasks = vec![];

    for i in 0..sites.len() {
        let task = find_audio_book(&book, &sites[i]);
        tasks.push(task);
    }

    let results = futures::future::try_join_all(tasks).await;
    results
}

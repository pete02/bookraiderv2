
use actix_web::web::Path;
use actix_web::{HttpServer, App, middleware::Logger,get,web::Json,post};
use httpdoc::find_sites;
mod utils{pub mod text; pub mod checks; pub mod structs;}
use std::time::{Instant};

use utils::structs::BookPayload;
mod httpdoc;
use utils::text::create_json;
use utils::text::readfile;
use utils::structs::Sites;
use utils::structs::UrlPayload;

mod http;
mod handlers;
use handlers::search_audio_books;
use handlers::get_audiobook;


fn get_sites()->Result<Sites,String>{
    let file=readfile("sites.json");
    match file {
        Ok(file)=>{
            let sites=create_json(file);
            match sites{
                Ok(sites)=>return Ok(sites),
                Err(_)=>return Err("error in creating json".to_owned())
            }
        },
        Err(_)=> return Err("error in reading file".to_owned())
    }
}

#[get("/search/{book}")]
async fn get_task(book:Path<BookPayload>)->Json<Vec<Vec<String>>>{
    let start=Instant::now();
    let sites=get_sites();
    match sites {
        Ok(sites)=>{
            let end=Instant::now()-start;
            println!("until request{:?}",end);

            let s=Instant::now();
            let res=search_audio_books(book.into_inner().book.as_str(), sites.sites).await;
            println!("request: {:?}",Instant::now()-s);
            match res{
                Ok(r)=>return Json(r),
                Err(_)=>return Json([["error in search".to_owned()].to_vec()].to_vec())
            }
        }
        Err(e)=>return Json([[e].to_vec()].to_vec())

    }
}

#[post("/get")]
async fn get_book(url:Json<UrlPayload>)->Json<Vec<String>>{
    let sites=get_sites();
    match sites{
        Ok(sites)=>{
            let site=find_sites(&url.url, &sites.sites);
            match site {
                Ok(s)=>{
                    let vec=get_audiobook(&url.url, s).await;
                    match vec {
                        Ok(vec)=>Json(vec),
                        Err(_)=>Json(["not found books".to_owned()].to_vec())
                    }
                }
                Err(_)=>Json(["error in finding the site".to_owned()].to_vec())
            }
        },
        Err(e)=>Json([e].to_vec())
    }
}

//main
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(get_task)
            .service(get_book)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

#[cfg(test)]
mod test;
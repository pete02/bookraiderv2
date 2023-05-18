
use actix_web::web::Path;
use actix_web::{HttpServer, App, middleware::Logger,get,web::Json};
mod utils{pub mod text; pub mod checks;}
mod structs;
use structs::BookPayload;
use std::time::{Instant};



mod httpdoc;


use utils::text::create_json;
use utils::text::readfile;


mod http;


mod handlers;
use handlers::search_audio_books;

#[get("/search/{book}")]
async fn get_task(book:Path<BookPayload>)->Json<Vec<Vec<String>>>{
    let start=Instant::now();
    let file=readfile("sites.json");

    match file{
        Ok(file)=>{
            let sites=create_json(file);
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
                Err(_)=>return Json([["error in json creation".to_owned()].to_vec()].to_vec())
            }
        }
        Err(_)=>return Json([["error in file reading".to_owned()].to_vec()].to_vec())
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

    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

#[cfg(test)]
mod test;
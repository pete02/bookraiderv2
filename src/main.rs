
use actix_web::web::Path;
use actix_web::{HttpServer, App, middleware::Logger,get,web::Json,post,Responder,HttpResponse};
mod https{pub mod http; pub mod handlers;pub mod httpdoc; pub mod mp3;}
mod utils{pub mod text; pub mod checks; pub mod structs;}
use std::time::{Instant};


use utils::structs::BookPayload;
use utils::text::create_json;
use utils::text::readfile;
use utils::structs::Sites;
use utils::structs::UrlPayload;
use https::httpdoc::find_sites;

use utils::structs::Site;

use https::handlers::search_audio_books;
use https::handlers::get_audiobook;




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
async fn get_task(book:Path<BookPayload>)->Json<Vec<Vec<(String,String)>>>{
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
                Err(_)=>return Json([[("error in search".to_owned(),"".to_string())].to_vec()].to_vec())
            }
        }
        Err(e)=>return Json([[(e,"".to_string())].to_vec()].to_vec())

    }
}


#[post("/get")]
async fn get_book(url:Json<UrlPayload>)->Json<String>{
    let sites=get_sites();
    match sites{
        Ok(sites)=>{
            let site=find_sites(&url.url, &sites.sites);
            match site {
                Ok(s)=>{
                    let vec=get_audiobook(&url.url, s).await;
                    match vec {
                        Ok(_)=>Json("done".to_owned()),
                        Err(e)=>Json(e.to_string())
                    }
                }
                Err(e)=>Json(e)
            }
        },
        Err(e)=>Json(e)
    }
}

#[post("/add")]
async fn add_site(site: Json<Site>) -> impl Responder {
    // Load sites from sites.json
    let file=utils::text::readfile("sites.json");
    let sitesres;
    match file{
        Ok(s)=>sitesres=create_json(s),
        Err(_)=>return HttpResponse::InternalServerError().body("Failed to read file")
    }
    let mut sites;
    match sitesres {
        Ok(s)=>sites=s,
        Err(_)=> return HttpResponse::InternalServerError().body("Failed to create sites"),//http error: create sites
    }

    // Add the received site to the vector
    sites.sites.push(site.into_inner());

    // Serialize the updated vector to JSON
    let json_data = serde_json::to_string_pretty(&sites).unwrap();

    // Write the JSON data back to sites.json
    match utils::text::write_sites(json_data.as_str(), "sites.json"){
        Ok(_)=>return HttpResponse::Ok().body("Site added successfully!"),
        Err(_)=> HttpResponse::InternalServerError().body("Failed to write file")
    }

    
}


//main
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let timer=Instant::now();
    println!("{:?}",Instant::now()-timer);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(get_task)
            .service(get_book)
            .service(add_site)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

#[cfg(test)]
mod test;
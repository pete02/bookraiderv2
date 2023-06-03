
use actix_web::web::Path;
use actix_web::{HttpServer, App, middleware::Logger,get,web::Json,post,Responder,HttpResponse};
mod https{pub mod http; pub mod handlers;pub mod httpdoc; pub mod mp3; pub mod bookdata;}
mod utils{pub mod text; pub mod checks; pub mod structs;}
use std::time::{Instant};
use serde_json::json;
use utils::structs::BookPayload;
use utils::text::create_json;
use utils::text::readfile;
use utils::structs::Sites;
use utils::structs::UrlPayload;
use https::httpdoc::find_sites;
use utils::structs::Site;
use utils::structs::GetResponse;

use std::fs;
use std::io::{Write, Read};
use std::fs::File;

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
async fn get_task(book:Path<BookPayload>)->Json<Vec<serde_json::Value>>{
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
                Err(_)=>return Json([json!([["error in search"]])].to_vec())
            }
        }
        Err(e)=>return Json([json!([[e]])].to_vec())

    }
}


#[post("/get")]
async fn get_book(url:Json<UrlPayload>)-> impl Responder{
    let sites=get_sites();
    println!("{:?}",url);
    match sites{
        Ok(sites)=>{
            let site=find_sites(&url.url, &sites.sites);
            match site {
                Ok(s)=>{
                    let vec=get_audiobook(&url.url, s, &url.name).await;
                    match vec {
                        Ok(_)=>{
                            let res=GetResponse{
                                name:url.name.to_string(),
                                url:url.url.to_string(),
                                response:200,
                            };

                           match save(res){
                            Ok(_)=>println!("done"),
                            Err(_)=>println!("err in writing")
                           }
                            return HttpResponse::Ok().json(Json("done".to_owned()))
                        }
                        Err(e)=>{
                            let res=GetResponse{
                                name:url.name.to_string(),
                                url:url.url.to_string(),
                                response:500,
                            };

                           match save(res){
                            Ok(_)=>println!("done"),
                            Err(_)=>println!("err in writing")
                           }
                           delete_temp_files();
                        return HttpResponse::InternalServerError().json(Json(e.to_string()))
                        }
                    }
                }
                Err(e)=>{
                    let res=GetResponse{
                        name:url.name.to_string(),
                        url:url.url.to_string(),
                        response:500,
                    };

                   match save(res){
                    Ok(_)=>println!("done"),
                    Err(_)=>println!("err in writing")
                   }
                   delete_temp_files();
                return HttpResponse::InternalServerError().json(Json(e.to_string()))
                }
            }
        },
        Err(e)=>{
            let res=GetResponse{
                name:url.name.to_string(),
                url:url.url.to_string(),
                response:500,
            };

           match save(res){
            Ok(_)=>println!("done"),
            Err(_)=>println!("err in writing")
           }
           delete_temp_files();
        return HttpResponse::InternalServerError().json(Json(e.to_string()))
        }
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


#[get("/res")]
async fn sen()-> impl Responder{
    match load() {
        Ok(a)=>return HttpResponse::Ok().json(a),
        Err(a)=>return HttpResponse::InternalServerError().body(a.to_string())
    }
}

fn save(res:GetResponse)->Result<(), Box<dyn std::error::Error>>{
    let mut file = File::create("save.json")?;
    file.write_all(serde_json::to_string(&res)?.as_bytes())?;
    Ok(())
}

fn delete_temp_files() -> Result<(), std::io::Error> {
    for entry in fs::read_dir("temp")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn load()->Result<GetResponse,Box<dyn std::error::Error>>{
    let mut f=File::open("save.json")?;
    let mut json=String::new();
    f.read_to_string(&mut json)?;
    let res:GetResponse=serde_json::from_str(&json)?;
    Ok(res)
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
            .service(sen)
            .service(actix_files::Files::new("/", "./frontend/build").index_file("index.html"))
            
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}


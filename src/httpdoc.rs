use select::document::Document;
use select::node::Node;
use select::predicate::Name;

use crate::structs::Site;
use crate::utils::checks::check_text_contains_filter;

pub fn find_url<'a, 'b>(node: &'a Node<'b>, excluded_class: &str,filters:&Vec<String>) -> Vec<String> {
    let urls:Vec<&str>;

    if excluded_class.len()>0 {
        urls=node.find(Name("a"))
        .filter(|n| n.attr("href").is_some() && (n.attr("class").is_none() || !n.attr("class").unwrap().contains(excluded_class)))
        .map(|n| n.attr("href").unwrap())
        .collect();

    }else{
        urls=node.find(Name("a"))
        .filter(|n| n.attr("href").is_some())
        .map(|n| n.attr("href").unwrap())
        .collect();
    }

    let mut vec:Vec<String>=Vec::new();
    for text in urls{
        if !check_text_contains_filter(text,filters){
            if !vec.contains(&text.to_string()){
                vec.push(text.to_owned())
            }
        }
        
    }
    vec
}



pub fn find_url_include<'a, 'b>(node: &'a Node<'b>, included_class: &str) -> Vec<String> {
    let urls:Vec<&str>;
    if included_class.len()>0 {
        urls=node.find(Name("a"))
        .filter(|n| n.attr("href").is_some() && (!n.attr("class").is_none() && n.attr("class").unwrap().contains(included_class)))
        .map(|n| n.attr("href").unwrap())
        .collect();

    }else{
        urls=node.find(Name("a"))
        .filter(|n| n.attr("href").is_some())
        .map(|n| n.attr("href").unwrap())
        .collect();
    }
    let mut vec:Vec<String>=Vec::new();
    for text in urls{
        if !vec.contains(&text.to_string()){
            vec.push(text.to_owned())
            
        }
        
    }
    vec
}

pub fn find_sites<'a>(url:&str,sites:&'a Vec<Site>)->Result<&'a Site,String>{
    for site in sites{
        if url.contains(&site.url){
            return Ok(site);
        }
    }
    Err("no site found".to_string())
}


pub fn get_node<'a>(doc:&'a Document,name:&str,classname:&str)->Result<Node<'a>,String>{
    let mut list=doc.find(Name(name));
    let found:Option<Node>;

    if classname.len()>0{
        found=list.filter(|n|!n.attr("class").is_none() && n.attr("class").unwrap().contains(classname)).next();
    }else{
        found=list.next();
    }

    match found{
        Some(node)=>Ok(node),
        None=>Err("error".to_string())
    }
}
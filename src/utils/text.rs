use std::fs;
use crate::structs::Sites;
use regex::Regex;

pub fn readfile(file:&str)->Result<String, std::io::Error>{
    fs::read_to_string(file)
}

pub fn create_json(json:String)-> Result<Sites,serde_json::Error>{
    serde_json::from_str(&json.as_str())
}

pub fn find_audio_links(html: &str,audio_format:&str) -> Vec<String> {
    let regex_pattern = format!("http(.*?){}", audio_format);
    let regex = Regex::new(&regex_pattern).unwrap();
    
    let links: Vec<Vec<&str>> = regex
        .captures_iter(&html)
        .map(|captures| captures.iter().map(|m| m.unwrap().as_str()).collect())
        .collect();
    
    let mut unique_links: Vec<String> = Vec::new();
    for link_group in links {
        unique_links.push(link_group[0].to_string());
    }
    
    unique_links.dedup();
    unique_links
}
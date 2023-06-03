use serde::{Deserialize,Serialize};

use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub url: String,
    pub search: String,
    pub filters: Vec<String>,
    pub container: String,
    pub classname: String,
    pub page: String,
    pub title:String,
    pub head:String,
}

impl PartialEq for Site {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
            && self.search == other.search
            && self.filters == other.filters
            && self.container == other.container
            && self.classname == other.classname
            && self.page == other.page
    }
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Sites {
    pub sites: Vec<Site>,
}

impl PartialEq for Sites {
    fn eq(&self, other: &Self) -> bool {
        self.sites == other.sites
    }
}


#[derive(Debug, Deserialize)]
pub struct BookPayload {
    pub book: String,
}


#[derive(Debug, Deserialize)]
pub struct UrlPayload {
    pub url: String,
    pub name:String,
    pub writer:Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct Link {
    pub url: String,
    pub name: String,
}

#[derive(Serialize,Deserialize)]
pub struct GetResponse{
    pub name:String,
    pub url:String,
    pub response: i32
}
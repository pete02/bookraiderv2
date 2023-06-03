use reqwest::Client;
use scraper::{Html, Selector};

async fn get_book_data(book: &str) -> Option<serde_json::Value> {
    let url = format!(
        "https://bookscouter.com/search?query={}",
        book.replace("–", "").replace("’s", "").trim().replace(" ", "+")
    );
    println!("{}", url);

    let client = Client::new();
    let response = client.get(&url).send().await.ok()?;
    let body = response.text().await.ok()?;
    let document = Html::parse_document(&body);

    let mut book_list = Vec::new();
    let selector = Selector::parse(".BookContainer_b1a7u5jm").unwrap();
    for element in document.select(&selector) {
        let name = element
            .select(&Selector::parse(".BookTitle_b1xw0hok").unwrap())
            .next()
            .map(|e| e.inner_html());
        let author = element
            .select(&Selector::parse(".BookText_b1ofiyxa").unwrap())
            .nth(0)
            .map(|e| e.inner_html());
        let isbn = element
            .select(&Selector::parse(".BookText_b1ofiyxa").unwrap())
            .nth(2)
            .map(|e| e.inner_html());

        if let (Some(name), Some(author), Some(isbn)) = (name, author, isbn) {
            let book_data = serde_json::json!({
                "name": name,
                "Author": author,
                "ISBN": isbn
            });
            book_list.push(book_data);
        }
    }

    book_list.into_iter().next()
}

pub async fn get_google_book(book: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    if let Some(book_data) = get_book_data(book).await {
        let key = "AIzaSyAXQi3VffK7zX5vcF1l-_POt3t8Zk0V288";

        let isbn = book_data["ISBN"].as_str().unwrap_or("");
        let name = book_data["name"].as_str().unwrap_or("");
        let author = book_data["Author"].as_str().unwrap_or("");

        let url = format!(
            "https://www.googleapis.com/books/v1/volumes?q=isbn:{}&key={}&langRestrict=en",
            isbn, key
        );

        let response = reqwest::get(&url).await?;
        let json = response.text().await?;

        let total_items = serde_json::from_str::<serde_json::Value>(&json)
            .ok()
            .and_then(|data| data["totalItems"].as_i64())
            .unwrap_or(0);

        if total_items == 0 {
            let url = format!(
                "https://www.googleapis.com/books/v1/volumes?q=intitle:{}&inauthor={}&key={}&langRestrict=en",
                name, author, key
            );
            let response = reqwest::get(&url).await?;
            let json = response.text().await?;

            let data: serde_json::Value = serde_json::from_str(&json)?;
            let items = data["items"].as_array();

            if let Some(item) = items.and_then(|arr| arr.first()) {
                let author = item["volumeInfo"]["authors"][0]
                    .as_str()
                    .map(|s| s.to_owned())
                    .unwrap_or_default();
                let thumbnail = item["volumeInfo"]["imageLinks"]["thumbnail"]
                    .as_str()
                    .map(|s| s.to_owned())
                    .unwrap_or_default();
                Ok((author, thumbnail))
            } else {
                Err("Book information not found.".into())
            }
        } else {
            let data: serde_json::Value = serde_json::from_str(&json)?;
            let items = data["items"].as_array();

            if let Some(item) = items.and_then(|arr| arr.first()) {
                let author = item["volumeInfo"]["authors"][0]
                    .as_str()
                    .map(|s| s.to_owned())
                    .unwrap_or_default();
                let thumbnail = item["volumeInfo"]["imageLinks"]["thumbnail"]
                    .as_str()
                    .map(|s| s.to_owned())
                    .unwrap_or_default();
                Ok((author, thumbnail))
            } else {
                Err("Book information not found.".into())
            }
        }
    } else {
        Err("Book information not found.".into())
    }
}

use std::io::Write;

pub async fn download_thumbnail(url: &str, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let mut file = std::fs::File::create(filename)?;
    file.write_all(&bytes)?;

    Ok("ok".to_owned())
}
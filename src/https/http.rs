

pub async fn make_request(link:&str)->Result<reqwest::Response, reqwest::Error>{
    reqwest::get(link).await
}


pub async fn get_response(response: reqwest::Response) -> Result<String, String> {
    if response.status().is_success() {
        let tex=response.text().await;
        match tex{
            Ok(t)=>return Ok(t),
            Err(_)=>Err("text error".to_string())
        }
    } else {Err(response.status().to_string())}
}
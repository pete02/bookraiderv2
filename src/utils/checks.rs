

pub fn check_audio_format(text: &str) -> Result<&str, &'static str> {
    let v = [".m4a", ".flac", ".mp3", ".mp4", ".wav", ".wma", ".aac", ".m3u8"];
    for i in v {if text.contains(i) {return Ok(i);}}
    Err("not found")
}

pub fn check_text_contains_filter(f:&str,filters:&Vec<String>)->bool{
    for i in filters{if f.contains(i.as_str()){return true}}   
    false
}




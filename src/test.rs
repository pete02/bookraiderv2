use tokio;

use select::document::Document;

use select::predicate::Name;

use crate::utils::structs::Site;
use crate::utils::structs::Sites;
use crate::https::http::make_request;
use crate::https::http::get_response;


use crate::utils::checks;
use checks::check_audio_format;

use crate::https::httpdoc;
use httpdoc::find_sites;
use httpdoc::find_url_include;
use httpdoc::get_node;

use crate::utils::text;
use text::find_audio_links;
use checks::check_text_contains_filter;

use super::*;



#[test]
fn test_site_equal(){
    let a:Site=Site{
        url:"https://audiobooksfreelisten.com/".to_string(),
        search:"?s=".to_string(),
        filters:["category".to_string(),"tag".to_string(),"#".to_string()].to_vec(),
        container:"div".to_string(),
        classname:"generate-columns-container".to_string(),
        page:"No".to_string(),
        title:"bookmark".to_string()
    };
    let b:Site=Site{
        url:"https://audiobooksfreelisten.com/".to_string(),
        search:"?s=".to_string(),
        filters:["category".to_string(),"tag".to_string(),"#".to_string()].to_vec(),
        container:"div".to_string(),
        classname:"generate-columns-container".to_string(),
        page:"No".to_string(),
        title:"bookmark".to_string()
    };
    assert_eq!(a,b);

    let a:Site=Site{
        url:"https://audiobooksfreelisten.com/".to_string(),
        search:"?s=".to_string(),
        filters:["category".to_string(),"tag".to_string(),"#".to_string()].to_vec(),
        container:"div".to_string(),
        classname:"generate-columns-container".to_string(),
        page:"No".to_string(),
        title:"bookmark".to_string()
    };
    let b:Site=Site{
        url:"https://audiobooksfreelisten.com/".to_string(),
        search:"?s=".to_string(),
        filters:["category".to_string(),"tag".to_string(),"#".to_string()].to_vec(),
        container:"div".to_string(),
        classname:"generate-columns-container".to_string(),
        page:"ye".to_string(),
        title:"bookmark".to_string()
    };
    assert_ne!(a,b)
}


#[test]
fn test_readfile(){
    assert!(readfile("sites.json").is_ok())
}
#[test]
fn test_createjson(){
    let file=readfile("tesst.json").unwrap();
    assert!(create_json(file).is_ok());

    let a:Site=Site{
        url:"https://audiobooksfreelisten.com/".to_string(),
        search:"?s=".to_string(),
        filters:["category".to_string(),"tag".to_string(),"#".to_string()].to_vec(),
        container:"div".to_string(),
        classname:"generate-columns-container".to_string(),
        page:"No".to_string(),
        title:"bookmark".to_string()
    };
    let s=Sites{
        sites:[a].to_vec(),
    };

    let file=readfile("tesst.json").unwrap();
    assert_eq!(s.sites,create_json(file).unwrap().sites)
}


#[test]
fn test_checkaudio() {
    let html1 = "This is an HTML document containing .mp3";
    assert_eq!(check_audio_format(html1), Ok(".mp3"));

    let html2 = "This HTML document does not contain any audio file extensions";
    assert_eq!(check_audio_format(html2), Err("not found"));
}
#[test]
fn test_check() {
    // Test case 1: Matching filter
    let f1 = "This is a sample string";
    let filters1 = vec!["sample".to_string(), "test".to_string()];
    assert_eq!(check_text_contains_filter(f1, &filters1), true);

    // Test case 2: No matching filter
    let f2 = "This is another string";
    let filters2 = vec!["sample".to_string(), "test".to_string()];
    assert_eq!(check_text_contains_filter(f2, &filters2), false);
}



#[tokio::test]
async fn test_call(){
    assert!(make_request("https://www.google.com").await.is_ok())
}

#[tokio::test]
async fn test_response(){
    let r=make_request("http://www.example.com").await.unwrap();
    assert!(get_response(r).await.is_ok());
    let l=make_request("http://www.example.com/nothing").await.unwrap();
    assert!(get_response(l).await.is_err());
}

#[test]
fn test_find_audio_links() {
    let html = r#"
        <html>
            <body>
                <a href="http://example.com/audio1.mp3">Audio 1</a>
                <a href="http://example.com/audio1.wav">Audio 1</a>
                <a href="http://example.com/audio3.mp3">Audio 3</a>
            </body>
        </html>
    "#;

    let expected_links = vec![
        "http://example.com/audio1.mp3".to_string(),
        "http://example.com/audio3.mp3".to_string(),
    ];

    let actual_links = find_audio_links(html,"mp3");
    assert_eq!(actual_links, expected_links);

    let html = r#"
        <html>
            <body>
                <a href="http://example.com/audio1.wav">Audio 1</a>
                <a href="http://example.com/audio3.wav">Audio 3</a>
            </body>
        </html>
    "#;

    let expected_links:Vec<String>=vec![];
    let actual_links = find_audio_links(html,"mp3");
    assert_eq!(actual_links, expected_links);
}


#[test]
fn test_find_url_include() {
    let html = r#"
        <html>
            <body>
                <a href="http://example.com/page1" class="excluded">Link 1</a>
                <a href="http://example.com/page2" class="excluded">Link 2</a>
                <a href="http://example.com/page3">Link 3</a>
            </body>
        </html>
    "#;

    let document=Document::from(html);

    let body_node = document.find(Name("body")).next().unwrap();
    let excluded_class = "excluded";

    let result = find_url_include(&body_node, excluded_class);

    assert_eq!(
        result,
        vec![
            "http://example.com/page1",
            "http://example.com/page2",
        ]
    ); 
}




#[test]
fn test_get_node_success() {
    let html = r#"<html><body><div class="content">Hello, world!</div></body></html>"#;
    let doc = Document::from(html);

    // Test case with matching name and class
    let result = get_node(&doc, "div", "content");
    assert!(result.is_ok());
    // Test case with matching name but no class
    let result = get_node(&doc, "div", "");
    assert!(result.is_ok());
}

#[test]
fn test_get_node_error() {
    let html = r#"<html><body></body></html>"#;
    let doc = Document::from(html);

    // Test case with non-matching name
    let result = get_node(&doc, "span", "");
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(err, "error");
    } else {
        panic!("Expected Err, but got a success result.");
    }

    // Test case with matching name but no matching class
    let result = get_node(&doc, "div", "content");
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(err, "error");
    } else {
        panic!("Expected Err, but got a success result.");
    }
}

#[test]
fn test_findsite(){
    let a:Site=Site{
        url:"https://audiobooksfreelisten.com/".to_string(),
        search:"?s=".to_string(),
        filters:["category".to_string(),"tag".to_string(),"#".to_string()].to_vec(),
        container:"div".to_string(),
        classname:"generate-columns-container".to_string(),
        page:"No".to_string(),
        title:"bookmark".to_string()
    };
    let b:Site=Site{
        url:"https://google.com/".to_string(),
        search:"?s=".to_string(),
        filters:["category".to_string(),"tag".to_string(),"#".to_string()].to_vec(),
        container:"div".to_string(),
        classname:"generate-columns-container".to_string(),
        page:"No".to_string(),
        title:"bookmark".to_string()
    };
    let sites=Sites{
        sites:[a,b].to_vec()
    };
    let txt=find_sites("https://audiobooksfreelisten.com/the-sword-of-summer-audiobook-01/",&sites.sites);
    assert!(&txt.is_ok());
    assert_eq!(txt.unwrap().url.as_str(),"https://audiobooksfreelisten.com/");

    let txt=find_sites("https://bla",&sites.sites);
    assert!(&txt.is_err());

}
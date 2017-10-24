extern crate select;
extern crate reqwest;
use select::document::Document;
use select::predicate::Class;
use std::process::Command;
use std::io::Read;

pub fn extract_facebook_text(url: String) -> Option<String> {
    let mut request = reqwest::get(&url).ok()?;
    let mut body = String::new();
    request.read_to_string(&mut body).ok()?;

    // Remove BigPipe's commented code
    body = str::replace(&body, "<!-- <", "<");
    body = str::replace(&body, "> -->", ">");

    let document = Document::from(&*body);
    for node in document.find(Class("userContent")).take(1) {
        return Some(node.text());
    }
    None
}

pub fn extract_text(url: String) -> Option<String> {
    if url.contains("facebook.com/") {
        return extract_facebook_text(url);
    }

    let output = Command::new("node")
        .arg("src/extractor/unfluff.js")
        .arg(url)
        .output()
        .ok()?;

    let encoded = String::from_utf8_lossy(&output.stdout.as_slice());
    let response = format!("{}", encoded);

    if response.is_empty() {
        return None;
    }
    Some(response)
}

#[test]
fn it_extracts_text_from_url() {
    let text = extract_text(String::from("https://goo.gl/d9WM3W")).unwrap_or(String::from(""));

    println!("Found text: {}", text);
    assert!(text.contains(
        "Era de se esperar que a Globo se juntasse aos artistas",
    ));
}

#[test]
fn it_extracts_text_from_facebook_posts() {
    let url = "https://www.facebook.com/VerdadeSemManipulacao/videos/479313152193503/";
    let text = extract_text(String::from(url)).unwrap_or(String::from(""));

    println!("Found text: {}", text);
    assert!(text.contains(
        "Feliciano,admite que estaria com um grupo, blindando e salvando a pele de Eduardo Cunha",
    ));
}

use reqwest::{self, StatusCode};

fn get_url() -> String {
    let port = std::env::var("PORT").expect("No port specified to run test");
    format!("http://localhost:{}", port)
}

fn fetch_url(path: &str) -> reqwest::Result<reqwest::blocking::Response> {
    let url = format!("{}/{}", get_url(), path);
    reqwest::blocking::get(&url)
}

fn submit_url(url: &str) -> reqwest::Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::new();

    client
        .post(&get_url())
        .form(&[("url", url)])
        .send()
}

#[test]
fn test_inexistent_url() {
    let res = fetch_url("aijsd").unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND)
}

#[test]
fn test_submit_get_url() {
    let url = "https://www.google.com/";

    let response = submit_url(url).unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let shortened_url = response.text().unwrap();

    let response = fetch_url(&shortened_url).unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.url().to_string(), url);
}

#[test]
fn test_submit_multiple() {
    let url = "https://www.youtube.com";

    let res = submit_url(url).unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res1 = res.text().unwrap();
    let res2 = submit_url(url).unwrap().text().unwrap();

    assert_eq!(res1, res2);
}

#[test]
fn test_submit_invalid_url() {
    // Random text
    let url = "askjdhaskldsaldka';sad";

    let res = submit_url(url).unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(res.text().unwrap(), "Not a valid URL");
}

#![allow(non_snake_case)]

use std::error;
use std::env;

use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    data: Translation
}

#[derive(Deserialize)]
struct Translation {
    translations: Vec<TranslatedData>
}

#[derive(Deserialize)]
struct TranslatedData {
    translatedText: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let url = "https://google-translate1.p.rapidapi.com/language/translate/v2";

    let client = reqwest::Client::new();

    let host = env::var("RapidAPI_Host").expect("Failed environmaent variable!");
    let key = env::var("RapidAPI_Key").expect("Failed environmaent variable!");

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("application/gzip"));

    let params = [
        ("source", "en"),
        ("target", "ko"),
        ("q", "Hello")
    ];

    let res = client
        .post(url)
        .header("X-RapidAPI-Host", &host)
        .header("X-RapidAPI-Key", &key)
        .form(&params)
        .send()
        .await?;

    if res.status().is_success() {
        let data = res.text().await?;
        let res_data: Response = serde_json::from_str(&data)?;
        println!("translated: {}", res_data.data.translations[0].translatedText);
    } else {
        println!("Failed: {}", res.status());
    }
    Ok(())
}

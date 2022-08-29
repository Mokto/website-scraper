use core::time;
use std::collections::HashMap;
use std::thread;

use fantoccini::Client;
use lingua::Language::{English, French, Italian, Spanish};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use pyo3::PyResult;
use select::document::Document;
use select::predicate::Name;

pub struct ScrapingProcess<'a> {
    htmls: HashMap<String, String>,

    domain: String,
    client: &'a Client,
    prefer_browser: bool,
    urls: Vec<String>,
}

impl<'a> ScrapingProcess<'a> {
    pub fn new(domain: String, client: &'a Client) -> Self {
        let url = ["https://", domain.as_str()].join("");
        println!("Scraping {}", url);

        Self {
            domain,
            client,
            prefer_browser: false,
            urls: vec!["/".to_string()],
            htmls: HashMap::new(),
        }
    }

    pub async fn run(&self) -> PyResult<String> {
        let url = ["https://", self.domain.as_str()].join("");
        let mut body = ScrapingProcess::direct_crawl(url.clone())
            .await
            .expect("error direct crawling");
        let mut valuable_content = ScrapingProcess::get_valuable_content(body.as_str());
        let mut body = "".to_string();
        let mut valuable_content = "".to_string();

        if valuable_content.is_empty() {
            println!("Not working with direct call. Crawling...");
            body = self.crawl_from_browser(url).await?;
            valuable_content = ScrapingProcess::get_valuable_content(body.as_str());
        }

        println!("{}", valuable_content);

        Document::from(body.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| println!("{}", x));

        println!("{}", ScrapingProcess::get_language(body.as_str()));

        Ok("COUCOU".to_string())
    }

    async fn crawl_from_browser(&self, url: String) -> PyResult<String> {
        // let client = self.client.as_ref().unwrap();
        // c.set_ua("Googlebot").await?;
        self.client.goto(url.as_str()).await.expect("Goto failed");
        let s = self.client.source().await.expect("Getting source failed");

        thread::sleep(time::Duration::from_millis(10000));

        Ok(s)
    }

    async fn direct_crawl(url: String) -> Result<String, Box<dyn std::error::Error>> {
        let proxy = reqwest::Proxy::all("p.webshare.io:9999")?;
        let client = reqwest::Client::builder().proxy(proxy).build()?;
        let body = client.get(url.as_str()).send().await?.text().await?;
        // println!("{body}");

        Ok(body)
    }

    fn get_valuable_content(content: &str) -> String {
        let doc = boilerpipe::parse_document(content);
        let val = doc.content();

        val.to_string()
    }

    fn get_language(content: &str) -> String {
        let languages = vec![English, French, Italian, Spanish];
        let detector: LanguageDetector =
            LanguageDetectorBuilder::from_languages(&languages).build();
        let detected_language: Option<Language> = detector.detect_language_of(content);
        return detected_language.unwrap().to_string();
    }
}

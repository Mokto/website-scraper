use std::collections::HashMap;
use std::sync::Arc;

use fantoccini::{Client, ClientBuilder};
use lingua::Language::{English, French, Italian, Spanish};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use pyo3::prelude::*;
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::PyAny;
use select::document::Document;
use select::predicate::Name;
use tokio::sync::Mutex;

struct ScrapingState {
    prefer_browser: bool,
    urls: Vec<String>,
    htmls: HashMap<String, String>,
}

struct Scraper {
    client: Option<Client>,
}

impl Scraper {
    pub async fn start_browser(&self) -> Client {
        let mut caps = serde_json::map::Map::new();
        // { "args": ["--headless"] }
        let opts = serde_json::json!({
            "prefs": {
                "permissions.default.image": 2,
                "webdriver.load.strategy": "eager",
                "webdriver_accept_untrusted_certs": true
            },
        });
        caps.insert("moz:firefoxOptions".to_string(), opts.clone());
        caps.insert(
            "proxy".to_string(),
            serde_json::json!({
                "proxyType": "manual",
                "httpProxy": "p.webshare.io:9999",
                "sslProxy": "p.webshare.io:9999",
            }),
        );

        ClientBuilder::native()
            .capabilities(caps)
            .connect("http://localhost:4444")
            .await
            .expect("failed to connect to WebDriver")
    }

    pub async fn close_browser(&self) -> PyResult<()> {
        // self.client.unwrap().close().await?;
        Ok(())
    }

    pub async fn scrape(&self, domain: String) -> PyResult<String> {
        let url = ["https://", domain.as_str()].join("");
        println!("Scraping {}", url);

        let scraping_state = ScrapingState {
            urls: vec!["/".to_string()],
            htmls: HashMap::new(),
            prefer_browser: false,
        };

        let mut body = Scraper::direct_crawl(url.clone())
            .await
            .expect("error direct crawling");
        let mut valuable_content = Scraper::get_valuable_content(body.as_str());
        // // let mut body = "".to_string();
        // // let mut valuable_content = "".to_string();

        if valuable_content.is_empty() {
            println!("Not working with direct call. Crawling...");
            body = self.crawl_from_browser(url).await?;
            valuable_content = Scraper::get_valuable_content(body.as_str());
        }

        println!("{}", valuable_content);

        Document::from(body.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| println!("{}", x));

        println!("{}", Scraper::get_language(body.as_str()));
        Ok("coucou".to_string())
    }

    async fn crawl_from_browser(&self, url: String) -> PyResult<String> {
        // c.set_ua("Googlebot").await?;
        // self.client.unwrap().goto(url.as_str()).await?;
        // let s = self.client.unwrap().source().await?;

        // Ok(s.to_string())
        Ok("".to_string())
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

#[pyclass(name = "Scraper")]
struct PyScraper(Arc<Mutex<Scraper>>);

#[pymethods]
impl PyScraper {
    #[new]
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Scraper { client: None })))
    }

    pub fn start_browser<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let inner = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let c = inner.lock().await.start_browser().await;
            inner.lock().await.client = Some(c);
            Ok(())
        })
    }

    pub fn stop_browser<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let inner = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            inner.lock().await.close_browser().await?;
            inner.lock().await.client = None;
            Ok(())
        })
    }

    pub fn scrape<'a>(&self, py: Python<'a>, domain: String) -> PyResult<&'a PyAny> {
        let inner = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            inner.lock().await.scrape(domain).await?;
            Ok(())
        })
    }
}

#[pymodule]
fn website_scraper(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyScraper>()?;
    Ok(())
}

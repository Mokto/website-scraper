use std::sync::Arc;

use fantoccini::{Client, ClientBuilder};
use pyo3::prelude::*;
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::PyAny;
use tokio::sync::Mutex;

pub mod scrape_process;

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

    pub async fn scrape(&self, domain: String) -> PyResult<String> {
        let process = scrape_process::ScrapingProcess::new(domain, self.client.as_ref().unwrap());
        process.run().await?;

        Ok("coucou".to_string())
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

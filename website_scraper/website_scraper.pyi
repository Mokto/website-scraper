

from typing import TypedDict


class ScrapeResult(TypedDict):
    page_contents: dict[str, str]


class Scraper:
    async def scrape(self, domain: str) -> ScrapeResult:
        ...

    async def start_browser(self) -> str:
        ...

    async def close_browser(self) -> str:
        ...

import asyncio
from website_scraper import Scraper


async def test():
    print("COUCOU")
    scraper = Scraper()
    
    await scraper.start_browser()
    print("BROWSER STARTED")
    await scraper.scrape("home.dk")
    print("SCRAPED")

    await asyncio.sleep(10)
    # await start_browser()
    # await scrape()

asyncio.run(test())

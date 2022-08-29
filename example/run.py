import asyncio
from website_scraper import Scraper


async def test():
    print("COUCOU")
    scraper = Scraper()

    await scraper.start_browser()
    print("BROWSER STARTED")
    await scraper.scrape("ocean.io")
    print("SCRAPED")

    # await start_browser()
    # await scrape()

    print("STOPPING BROWSER")
    await scraper.stop_browser()
    print("STOPPED BROWSER")
    await asyncio.sleep(10)

asyncio.run(test())

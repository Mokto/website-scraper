import asyncio
from website_scraper import Scraper


async def tab1(scraper):
    await asyncio.sleep(1)
    await scraper.scrape("ocean.io")


async def tab2(scraper):
    await asyncio.sleep(1)
    await scraper.scrape("home.dk")


async def test():
    print("COUCOU")
    scraper = Scraper()

    await scraper.start_browser()
    print("BROWSER STARTED")

    await asyncio.gather(tab1(scraper), tab2(scraper))
    # await scraper.scrape("ocean.io")
    # print("SCRAPED")

    # # await start_browser()
    # # await scrape()

    # print("STOPPING BROWSER")
    await scraper.stop_browser()
    # print("STOPPED BROWSER")
    # await asyncio.sleep(10)

asyncio.run(test())

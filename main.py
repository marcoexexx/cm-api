import asyncio
import aiohttp
import bs4


CM_BASE_URL = "https://channelmyanmar.org/austin-powers-in-goldmember-2002"


async def main() -> None:
    movies = []

    async with aiohttp.ClientSession() as session:
        async with session.get(CM_BASE_URL) as response:
            html_content = await response.text()
            soup = bs4.BeautifulSoup(html_content, "html.parser")
            li_tags = soup.find_all("li", attrs={"class", "elemento"})

            for li_tag in li_tags:
                a_tag = li_tag.find("a")
                if not a_tag:
                    continue

                link = a_tag["href"]
                protocol_tag = a_tag.find("span", attrs={"class", "b"})
                size_tag = a_tag.find("span", attrs={"class", "c"})
                quality_tag = a_tag.find("span", attrs={"class", "d"})

                protocol = protocol_tag.get_text(strip=True)
                size = size_tag.get_text(strip=True)
                quality = quality_tag.get_text(strip=True)

                movie = {
                    "protocol": protocol,
                    "link": link,
                    "size": size,
                    "quality": quality,
                }

                movies.append(movie)

    print([m["link"] for m in movies if m["quality"] == "1080p" and m["protocol"] == "yoteshinportal.cc"])


if __name__ == "__main__":
    asyncio.run(main())

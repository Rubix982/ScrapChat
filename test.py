import requests
from bs4 import BeautifulSoup
import os
from pydantic import BaseModel, HttpUrl, ValidationError
import concurrent.futures
import uuid

# Define Pydantic model to validate the URL input
class URLInput(BaseModel):
    url: HttpUrl

def fetch_title(url: HttpUrl):
    # Make the request to the provided URL
    try:
        response = requests.get(url, timeout=10)  # Added timeout to prevent hanging indefinitely

        # Check if the request was successful
        if response.status_code == 200:
            # Use lxml parser (should be installed via pip install lxml)
            soup = BeautifulSoup(response.content, 'lxml')

            # Find the <title> tag inside the <head> tag
            title_tag = soup.find('title')

            if title_tag:
                title = title_tag.text.removeprefix("ChatGPT - ")

                # Create the directory if it doesn't exist
                os.makedirs('misc', exist_ok=True)

                # Save the title content to 'misc/output_title.txt'
                with open(f"misc/{title}.txt", 'w', encoding='utf-8') as f:
                    f.write(str(soup.prettify()))

                print(f"Title content saved to 'misc/{title}.txt'")
            else:
                print(f"No <title> tag found in the HTML for {url}.")
        else:
            print(f"Failed to fetch the URL {url}. Status code: {response.status_code}")
    except Exception as e:
        print(f"Error fetching {url}: {e}")

def main():
    # Generate a set of random URLs upfront
    seen_urls = set()
    urls = set()

    while len(urls) < 100000:
        tmp_url = str(uuid.uuid4())
        if tmp_url not in seen_urls:
            seen_urls.add(tmp_url)
            urls.add(f"https://chatgpt.com/share/{tmp_url}")

    with concurrent.futures.ThreadPoolExecutor(max_workers=30) as executor:
        futures = []
        for i, url in enumerate(urls):
            print(f"Request# {i}: Route: {url}")

            try:
                # Validate the input using Pydantic
                validated_input = URLInput(url=url)
                futures.append(executor.submit(fetch_title, validated_input.url))
            except ValidationError as e:
                print(f"Invalid URL: {e}")

        # Wait for all futures to complete
        concurrent.futures.wait(futures)

if __name__ == "__main__":
    main()
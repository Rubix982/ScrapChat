import requests
from bs4 import BeautifulSoup
import bs4 as BS4
import os
from pydantic import BaseModel, HttpUrl, ValidationError
import datetime
import json

def log_time(msg: str):
    print(f"[{datetime.datetime.now()}] {msg}")

# Define Pydantic model to validate the URL input
class URLInput(BaseModel):
    url: HttpUrl

def fetch_title(url: HttpUrl):
    # Make the request to the provided URL
    response = requests.get(url)

    # Check if the request was successful
    if response.status_code == 200:
        # Parse the page content using BeautifulSoup
        soup = BeautifulSoup(response.content, 'html.parser')

        # Find the <title> tag inside the <head> tag
        chat_script: BS4.element.Tag = soup.find_all('script')[2]
        chat_script_contents: str = chat_script.contents[0]
        title: str = soup.find('title').text.removeprefix("ChatGPT - ")
        position = chat_script_contents.find(";__remixContext.p = function")
        if position != -1:
            chat_script_contents = chat_script_contents[:position]
        prefix = "window.__remixContext = "
        position = chat_script_contents.find(prefix)
        if position != -1 or position is not None:
            chat_script_contents = chat_script_contents[position + len(prefix):]

        # Create the directory if it doesn't exist
        os.makedirs(f'chat/{title}', exist_ok=True)

        # Save the title content to 'misc/output_title.txt'
        with open(f"chat/{title}/main.html", 'w', encoding='utf-8') as f:
            f.write(str(soup.prettify()))

        try:
            # Parse the text as JSON
            parsed_data = json.loads(chat_script_contents)
            
            # Save the parsed data to a file
            with open(f"chat/{title}/script.json", "w") as json_file:
                json.dump(parsed_data, json_file, indent=4)  # Save with pretty formatting (indentation)
            
            print("JSON saved to output.json")
        except json.JSONDecodeError as e:
            print(f"Failed to parse JSON: {e}")

        print(f"Content saved to 'misc/{title}'")
    else:
        print(f"Failed to fetch the URL. Status code: {response.status_code}")

def main():
    # Accept the URL input from the user
    url_input = input("Enter the URL: ")
    url_constant: str = "https://chatgpt.com/share/"
    if not url_input.startswith(url_constant):
        log_time(f"Provided URL does not start with '{url_constant}'. Exiting.")
        return

    try:
        # Validate the input using Pydantic
        validated_input = URLInput(url=url_input)
        fetch_title(validated_input.url)
    except ValidationError as e:
        print(f"Invalid URL: {e}")

if __name__ == "__main__":
    main()
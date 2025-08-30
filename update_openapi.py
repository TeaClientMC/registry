# TODO: Remove once API is Up and Online..
import requests

url = "http://localhost:3000/openapi.json"

response = requests.get(url)

if response.status_code == 200:
    with open("openapi.json", "w") as f:
        f.write(response.text)
    print("OpenAPI JSON saved to openapi.json")
else:
    print(f"Failed to fetch OpenAPI JSON. Status code: {response.status_code}")

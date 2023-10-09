import requests
import urllib3

from urllib3.exceptions import InsecureRequestWarning


def disable_certificate_verification():
    urllib3.disable_warnings(InsecureRequestWarning)


api_url = "https://localhost:3443"

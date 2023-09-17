import requests
import random

from tests import disable_certificate_verification
from tests import api_url

username = str(random.randint(1000, 9999))
password = str(random.randint(1000, 9999))


def test_user_registration():
    address = api_url + "/user/auth/register"
    disable_certificate_verification()

    body = {"username": username, "password": password}
    response = requests.post(address, json=body, verify=False)
    assert response.status_code == 200

    body = {"username": username, "password": password}
    response = requests.post(address, json=body, verify=False)
    assert response.status_code == 409


def test_user_authorization():
    address = api_url + "/user/auth/login"
    disable_certificate_verification()

    not_found_username = str(random.randint(1000, 9999))
    not_found_password = str(random.randint(1000, 9999))

    body = {"username": username, "password": password}
    response = requests.post(address, json=body, verify=False)
    assert response.status_code == 200
    # assert response.json()["message"] == "OK"

    body = {"username": not_found_username, "password": password}
    response = requests.post(address, json=body, verify=False)
    assert response.status_code == 404
    # assert response.json()["error"] == "Not Found"

    body = {"username": username, "password": not_found_password}
    response = requests.post(address, json=body, verify=False)
    assert response.status_code == 401
    # assert response.json()["error"] == "Unauthorized"

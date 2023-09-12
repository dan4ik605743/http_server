import requests

from tests import disable_certificate_verification
from tests import api_url


def test_registration():
    disable_certificate_verification()

    response = requests.get(api_url + "/register", verify=False)
    assert response.status_code == 200


def test_authorization():
    disable_certificate_verification()

    response = requests.get(api_url + "/login", verify=False)
    assert response.status_code == 200

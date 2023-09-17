import requests

from tests import disable_certificate_verification
from tests import api_url


def test_user_registration():
    disable_certificate_verification()

    response = requests.get(api_url + "/user/auth/register", verify=False)
    assert response.status_code == 200


def test_user_authorization():
    disable_certificate_verification()

    response = requests.get(api_url + "/user/auth/login", verify=False)
    assert response.status_code == 200


# import sys


# def test_cookie():
#     disable_certificate_verification()

#     session = requests.Session()
#     response = session.get(api_url + "/get_cookie", verify=False)
#     cookies = session.cookies
#     sys.stdout.write("Cookies received: {}\n".format(cookies))

#     print(session.cookies)

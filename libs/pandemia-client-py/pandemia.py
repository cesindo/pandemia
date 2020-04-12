#!/usr/bin/env python3

import requests
import json
from enum import Enum

PUBLIC_API_BASE_URL = "http://localhost:8080/api"
PRIVATE_API_BASE_URL = "http://localhost:9090/api"

HEADERS = {'Content-Type': 'application/json'}

class Scope(Enum):
  PUBLIC = 0
  PRIVATE = 1


def api_url(path, scope=Scope.PUBLIC):
    global PUBLIC_API_BASE_URL, PRIVATE_API_BASE_URL
    if scope == Scope.PUBLIC:
      return "%s/%s" % (PUBLIC_API_BASE_URL, path)
    elif scope == Scope.PRIVATE:
      return "%s/%s" % (PUBLIC_API_BASE_URL, path)
    else:
      raise Exception("Unknown scope:", scope)

def api_post(path, data, scope=Scope.PUBLIC):
    global HEADERS
    return requests.post(api_url(path, scope), data=json.dumps(data), headers=HEADERS)

def api_get(path, scope=Scope.PUBLIC):
    global HEADERS
    return requests.get(api_url(path, scope), headers=HEADERS)

def register_user(full_name, email, phone_num):
    rv = api_post("user/v1/user/register", {
        "full_name": full_name,
        "email": email,
        "phone_num": phone_num
    })
    # print(rv)
    rv = json.loads(rv.text)
    if rv.get("status") and rv['status'] == "error":
        print("ERROR: %d (%s)" % (rv['code'], rv["description"]))
        return {}
    return rv["result"]

def activate_user(token, password):
    rv = api_post("user/v1/user/activate", {
        "token": token,
        "password": password
    })
    return rv

def authorize(email, phone, passhash):
    global HEADERS
    rv = api_post("auth/v1/authorize", {
        "email": email,
        "phone": phone,
        "passhash": passhash
    })
    d = rv.json()
    if d.get("code") == 0:
        HEADERS["X-Access-Token"] = d["result"]["token"]
    return rv

def get_key():
    rv = api_get("auth/v1/get_key")
    return rv

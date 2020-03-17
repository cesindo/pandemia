#!/usr/bin/env python

import sys
import os

sys.path.append(os.path.join(os.path.dirname(__file__), "..", "..", "libs", "pandemia-client-py"))

import pandemia

def main():

    target_users = [
        ["Zufar", "zufar@mail.com", "+628123123"],
        ["Akmal", "akmal@mail.com", "+628123124"],
        ["Anto", "anto@mail.com", "+628123125"],
        ["Hanky", "hanky@mail.com", "+628123126"],
        ["Andrie", "andrie@mail.com", "+628123127"],
        ["Ubai", "ubai@mail.com", "+628123128"],
    ]

    tokens = []
    for acc in target_users:
        tokens.append(pandemia.register_user(acc[0], acc[1], acc[2]))

    print(tokens)

    for token in tokens:
        if type(token) == unicode:
            if pandemia.activate_user(token, "123").status_code != 200:
                print("cannot activate user with token %s", token)
        

if __name__ == "__main__":
    main()


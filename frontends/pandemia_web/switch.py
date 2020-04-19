#!/usr/bin/env bash


import re
import sys

ROUTER_MATCH = re.compile("import router from '\\./router.+$")
BASE_URL_MATCH = re.compile("\\? '/.*?'")


def main():
    router_name = sys.argv[1]
    base_url = sys.argv[2]
    save = []
    with open("src/main.js") as f:
        for line in f.readlines():
            if ROUTER_MATCH.search(line):
                save.append("import router from './routers/%s'\n" % router_name)
            else:
                save.append(line)
    
    with open("src/main.js", "w") as f:
        f.writelines(save)

    save = []
    with open("vue.config.js") as f:
        for line in f.readlines():
            if BASE_URL_MATCH.search(line):
                save.append("    ? '%s'\n" % base_url)
            else:
                save.append(line)
    
    with open("vue.config.js", "w") as f:
        f.writelines(save)



if __name__ == "__main__":
    main()


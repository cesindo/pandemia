#!/usr/bin/env bash

export CURDIR=`dirname $0`
. $CURDIR/includes.sh

export ANSIBLE_CONFIG=ansible.cfg

GIT_REV=$(git rev-parse HEAD)

echo -n "Deploy Pandemia $VERSION to remote server? [y/n] "
read confirm

if [ "$confirm" == "y" ]; then
    echo $GIT_REV > $PROJDIR/etc/ansible/GIT_REV
    ansible-playbook -v -i etc/ansible/hosts -e "server=api" etc/ansible/api.yml
fi

echo -n "Deploy web control center? [y/n] "
read confirm

if [ "$confirm" == "y" ]; then
    make build-web-frontend
    ansible-playbook -v -i etc/ansible/hosts -e "server=control_center_web" etc/ansible/control_center_web.yml
fi


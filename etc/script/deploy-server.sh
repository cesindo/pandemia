#!/usr/bin/env bash

export CURDIR=`dirname $0`
. $CURDIR/includes.sh

export ANSIBLE_CONFIG=ansible.cfg

GIT_REV=$(git rev-parse HEAD)

echo -n "Deploy Pandemia $VERSION to remote server? [y/n] "
read confirm

if [ "$confirm" == "y" ]; then
    echo $GIT_REV > $PROJDIR/etc/ansible/GIT_REV
    ansible-playbook -v -i etc/ansible/hosts -e "server=bc2" etc/ansible/playbook.yml
fi

#!/usr/bin/env bash


set -e

export PROJDIR=$(pwd -P)
export SCRIPTDIR=$PROJDIR/etc/script

source $PROJDIR/.env

export VERSION=`cat $PROJDIR/VERSION`

if [ -z "$PG_USER" ]; then
    PG_USER=Robin
fi

function run_on_remote {
    ssh root@$HOST $@
}


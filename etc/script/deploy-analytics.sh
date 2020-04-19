#!/usr/bin/env bash

export CURDIR=`dirname $0`
. $CURDIR/includes.sh

TARGET=$1

pushd frontends/pandemia_web

python switch.py wonosobokab /
yarn run build

# turn back
python switch.py router /cc

rsync -avzrhcP -e "ssh -i $SSH_KEY" ./dist/ $PANDEMIA_DEPLOY_USER@$PANDEMIA_DEPLOY_SERVER:$PANDEMIA_DEST_SERVER_PATH/pandemia_wonosobokab

popd


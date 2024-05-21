#!/bin/bash

LOGFILE=/tmp/dave_install.log
DAVE_HOME="/opt/dave"

echo '************************************************'
echo '* Welcome to the guided installation for Dave! *'
echo '************************************************'

echo 'Where would you like to install Dave to? [default: /opt/dave]: '
read install_dir

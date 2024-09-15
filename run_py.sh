#!/bin/bash

./scripts/is_python_installed.sh
if [ $? -ne 0 ]; then
    exit 1
fi

./scripts/is_pip_installed.sh
if [ $? -ne 0 ]; then
    exit 1
fi

python ./py/init.py "$@"

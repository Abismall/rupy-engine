#!/bin/bash

if command -v python &>/dev/null; then
    exit 0
else
    echo "Requirement not satisfied: Python is not installed or not accessible in the PATH."
    exit 1
fi

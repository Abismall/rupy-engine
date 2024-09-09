#!/bin/bash

if command -v pip &>/dev/null; then
    exit 0
else
    echo "Requirement not satisfied: Pip is not installed or not accessible in the PATH."
    exit 1
fi

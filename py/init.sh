#!/bin/bash

REQUIRED_PYTHON_VERSION="3.8"

check_python_version() {
    PYTHON_VERSION=$(python3 -c 'import sys; print(f"{sys.version_info.major}.{sys.version_info.minor}")')
    if [[ "$(printf '%s\n' "$REQUIRED_PYTHON_VERSION" "$PYTHON_VERSION" | sort -V | head -n1)" == "$REQUIRED_PYTHON_VERSION" ]]; then 
        echo "Python version $PYTHON_VERSION is compatible."
    else
        echo "Error: Python $REQUIRED_PYTHON_VERSION or higher is required. Found version $PYTHON_VERSION."
        exit 1
    fi
}

if ! command -v python3 &> /dev/null; then
    echo "Python is not installed. Please install Python $REQUIRED_PYTHON_VERSION or higher."
    exit 1
fi

check_python_version

if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv venv
fi

source venv/bin/activate

pip install --upgrade pip

if [ -f "requirements.txt" ]; then
    echo "Installing Python dependencies..."
    pip install -r requirements.txt
else
    echo "Error: requirements.txt not found. Please provide a requirements file."
    deactivate
    exit 1
fi


deactivate

echo "Setup completed successfully."

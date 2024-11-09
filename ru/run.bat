@echo off

REM Set environment variables
set "RUPY_ENGINE_STATIC_DIR=target\release\static"
set "RUPY_ENGINE_IMAGES_DIR=target\release\static\images"
set "RUPY_ENGINE_SHADERS_DIR=target\release\static\shaders"

REM Run the Rust executable
target\release\rupy.exe

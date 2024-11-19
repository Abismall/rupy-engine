@echo off

REM 
set "RUPY_ENGINE_STATIC_DIR=target\release\static"
set "RUPY_ENGINE_TEXTURES_DIR=target\release\static\textures"
set "RUPY_ENGINE_SHADERS_DIR=target\release\static\shaders"

REM 
target\release\rupy.exe

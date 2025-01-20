#!/bin/bash

if [ -f .env ]; then
    source .env
else
    echo ".env file not found!"
    exit 1
fi

if [ -z "$PATH_TO_STUBS" ]; then
    echo "PATH_TO_STUBS is not set in the .env file!"
    exit 1
fi

file="src/main.rs"

sed -i "s|stub_dir: Dir = include_dir!(\"$PATH_TO_STUBS\");|stub_dir: Dir = include_dir!(\"stubs\");|" "$file"

echo "Line replaced in $file with path: $PATH_TO_STUBS"

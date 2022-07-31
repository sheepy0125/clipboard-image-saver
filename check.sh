#!/bin/bash
# A script that runs `cargo clippy` for the backend and frontend
# Assumes current working directory is project root

display() {
    if type "figlet" &> /dev/null; then
        figlet $1
    else
        echo "========="
        echo $1
        echo "========="
    fi
}

echo "CWD: $(pwd)"

display "Frontend"
cd frontend
cargo clippy
cd ..

display "Tauri"
cd src-tauri
cargo clippy
cd ..

echo "Check done"
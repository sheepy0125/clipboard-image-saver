#!/bin/bash
# A script to build an AppImage and to put it in the project root
# Assumes the CWD is the project root

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

display "Building"
cargo tauri build

display "Moving"
mv --verbose src-tauri/target/release/bundle/appimage/*.AppImage .

echo "Build done"
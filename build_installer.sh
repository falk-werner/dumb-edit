#!/bin/sh

cargo build -r
if [[ "$?" != "0" ]] ; then
    echo "error: build failed"
    exit 1
fi

cargo test -r --verbose
if [[ "$?" != "0" ]] ; then
    echo "error: unit test execution failed"
    exit 1
fi

rcedit-x64.exe target/release/dumb-edit.exe --set-icon src/img/32x32.ico
if [[ "$?" != "0" ]] ; then
    echo "error: failed to set icon"
    exit 1
fi

dotnet build
if [[ "$?" != "0" ]] ; then
    echo "error: failed to create installer"
    exit 1
fi

#!/usr/bin/env bash

wasm-pack build --target web --features wasm
rm pkg/.gitignore
rm -rf ../Frontend/wasm-bindings
mv pkg ../Frontend/wasm-bindings

#performace command
#cat ../../../TestDocuments/moby-dick.txt | sudo perf record -- ./prosetta
wasm-pack build --target web --features wasm && `
& {Remove-Item pkg/.gitignore -ErrorAction SilentlyContinue ; Remove-Item ../Frontend/wasm-bindings -Recurse -ErrorAction SilentlyContinue ; Move-Item pkg ../Frontend/wasm-bindings}

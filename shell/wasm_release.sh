
#!/bin/sh

cd $(pwd)/examples/filters/

# 允许内存动态增涨：-s ALLOW_MEMORY_GROWTH=1
# 使用 ES 版本：-s FULL_ES2=1
# failed to asynchronously prepare wasm: Error: Table import env:table provided an 'initial' that is too small
# -s ASSERTIONS=1 
cargo rustc  --target wasm32-unknown-emscripten --release -- -Clink-args="-s FULL_ES2=1 -s ALLOW_MEMORY_GROWTH=1 -s ASSERTIONS=1"
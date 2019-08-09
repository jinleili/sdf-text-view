#!/bin/sh

# vscode 的 task 通过 bash 来执行此 shell 脚本，需要主动指定一下执行目录
# https://my.oschina.net/leejun2005/blog/150662
cd $(pwd)/idroid

cargo fmt

cargo run --example noise
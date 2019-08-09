# idroid
Write rust code, run on MacOS iOS Android and Web.

# Rust 中文社区
[https://www.rust-lang.org/zh-CN/](https://www.rust-lang.org/zh-CN/)

# Install
```
// install rust
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
// install target
$ rustup target add wasm32-unknown-emscripten --toolchain stable
$ cargo install cargo-web
```

## Build 
```
// As web app
$ cargo web start --example basic --target=wasm32-unknown-emscripten
// release
$ cargo web start --target=wasm32-unknown-emscripten --example basic --release

// As desktop app (native-opengl)
$ cargo run --example basic --release
```

# 用 VisualStudio Code 编译运行
使用快捷键 'CMD + Shift + B' 呼出任务面板, 选择 'example basic' 运行.

# 一些说明
gleam 是 Mozilla Servo 项目的 OpenGL 接口封装库，仅支持 OpenGL 与 Android 上的 OpenGL ES 2.0, Copy 这个项目是为了改造它来支持 iOS 系统与 ES 3.0。扩展了 ES30 这个 trait， 定义的都是 ES 3.0 里才有的接口。

uni-app，webgl 是从 unrust 项目里移植来的，原项目里把 gl 各对象封装成了 WebGL 对象，使用起来比较别扭

wasm 版本需要浏览器支持 WebGL2

## 常见问题
> 交叉编译时报：error: can't find crate for 'core':
方案 1：参考 https://os.phil-opp.com/cross-compile-libcore/;
方案 2：如果是多次执行了 rust 的安装脚本，则尝试 $sudo sh /usr/local/lib/rustlib/uninstall.sh

> 生成 standalone ndk 后编译报：aarch64-linux-gnu-gcc: Command not found
方案：直接在 .bash_profile 里指定：export PATH=$PATH:/Users/grenlight/Rust/NDK_standalone/arm64/bin/

> 允许 wasm 动态增加运行内存：
cargo rustc  --target wasm32-unknown-emscripten -- -Clink-args="-s ALLOW_MEMORY_GROWTH=1"

> examples 里配置成 workspace 的原因：
默认是可以直接写 rust 文件（如：basic.rs），然后在构建时通过 --example basic 来运行的，但是通过 cargo web 命令来测试 wasm 会导致无法正确加载 wasm 文件。

> 在 ES 版本的着色器里，int 与 float 是不能直接  * 的, 需要先强制转换成同一个数据类型。

> $cargo web start 使用的是 rust 内创建的 canvas, 而 emscripten 默认使用的是 html 页面里写的且绑定到 Module['canvas'] 的 canvas

> 除了 Safari 之外，离屏渲染的纹理源与纹理目标不能是同一个（GL ERROR :GL_INVALID_OPERATION : glDrawElements: Source and destination textures of the draw are the same）；

> ES 2.0 里，GLSL 不支持常量数组;

> 单精度浮点数的锅，**sin(0) * 1000 = 89!!!!!!!** fuck!!!;


## wasm 文件大小

> 只写一行 println! 时 debug: 196KB, release: 93KB;

> 引入没有使用的库，不影响实际 wasm 大小；

> 实际项目代码时，debug: 1.2MB, release: 283KB;

## 踩过的坑
> mio, reqwest 等网络库不支持编译到 wasm;


## kotlin 生成 jni 桥接 header
参考：https://github.com/fuzzdota/kotlin-jni-example

### java 生成 jni c 头文件
// cd 到 .java 源文件目录编译 java 文件
javac IdroidJNILib.java
// 然后 cd 到 src 里的 java 目录生成 jni c 头文件
javah -jni com.lijinlei.rustapp.IdroidJNILib


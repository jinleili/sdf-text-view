# SDFTextView
View component that based wgpu to calculate SDF(signed distance field) at real time and rendering.

Also implemented a cpu-based SDF version on ```sdf-generator/```, it accept a input image and output a sdf gray image.

## Real time performance of  SDF compute shader
Test 10 times, the resolution of the input image used is 368*130.

Device | SDF time cost
--------- | -------------
MBP 2018 (15-inch, Intel i7) |  0.420992ms
iPhone 6 plus  | 3.211291ms
iPhone Xs Max   | 0.688541ms

## Screenshot
<div style="width:100%; margin:0 auto; overflow:auto; _display:inline-block;"> 
    <div style="width:49%; float:right; "><image src="screenshot/iPhone.gif" alt="iPhone gif" /></div> 
    <div style=" margin-right:51%; width:49%;"><image src="screenshot/macOS_gif.gif" alt="macOS gif" /></div> 
</div> 

## What is a SDF rendering 
SDF rendering is a technique that allows you to render bitmap fonts without jagged edges even at high magnifications. 

Original SIGGRAPH 2007 paper: [Improved Alpha-Tested Magnification for Vector Textures and Special Effects](https://steamcdn-a.akamaihd.net/apps/valve/2007/SIGGRAPH2007_AlphaTestedMagnification.pdf)

Improved algorithm: [Felzenszwalb/Huttenlocher distance transform](http://cs.brown.edu/people/pfelzens/papers/dt-final.pdf)

## Running Example
### On macOS
```
# macOS
cargo run --example demo
```
### On iOS device
```
# build for iOS
# debug 
RUST_BACKTRACE=1 cargo build --target aarch64-apple-ios
# release
cargo build --target aarch64-apple-ios --release
```
Then,use Xcode to open iOS/demo.xcworkspace and run.

### cpu-based version
```
cd sdf-generator
cargo run input.png output.png
```

## To do list
- [ ] Use Android ```SurfaceView``` to Run on Android device
- [ ] Improve screen rotate on iOS device
- [ ] Scrollable when scaled model projection  over the viewport boundaries

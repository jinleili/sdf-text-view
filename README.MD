# sdf-text-view
Real time SDF(signed distance field) calculate and rendering on gpu.

## SDF rendering 
SDF rendering is a technique that allows you to render bitmap fonts without jagged edges even at high magnifications. 

Original SIGGRAPH 2007 paper: [Improved Alpha-Tested Magnification for Vector Textures and Special Effects](https://steamcdn-a.akamaihd.net/apps/valve/2007/SIGGRAPH2007_AlphaTestedMagnification.pdf)

Improved algorithm: [Felzenszwalb/Huttenlocher distance transform](http://cs.brown.edu/people/pfelzens/papers/dt-final.pdf)

## Running Example
### On PC
```
// macOS
cargo run --example demo
// Windows
cargo run --example demo --features dx12
```
### On iOS device
use Xcode to open iOS/demo.xcworkspace
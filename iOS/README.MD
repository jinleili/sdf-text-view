### Run on iOS

first, build rust proj for iOS
```shell
$ cd ../
$ RUST_BACKTRACE=1 cargo build --target aarch64-apple-ios
```
second, install dependencies of this iOS proj.
```shell
$ cd iOS
$ pod install
```
third, use Xcode open ```demo.xcworkspace``` file


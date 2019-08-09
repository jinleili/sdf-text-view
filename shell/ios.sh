#!/bin/sh

# vscode 的 task 通过 bash 来执行此 shell 脚本，需要主动指定一下执行目录
# https://my.oschina.net/leejun2005/blog/150662
#
cd $(pwd)

cargo fmt

# cargo lipo --release
TARGET='x86_64-apple-ios' #x86_64-apple-ios, i386-apple-ios
# TARGET='aarch64-apple-ios'
# TARGET='armv7s-apple-ios'
cargo build --target $TARGET --release

# copy 编译后的文件到 iOS 工程里
cp target/$TARGET/release/libidroid.a app/ios/RustApp/libidroid.a

cp idroid.h app/ios/RustApp/idroid.h

exit 

# 启动模拟器
cd ./app/ios/

# xcrun instruments -w "iPhone 6s (10.3.1) [2D38A96A-85FB-41A7-ACF4-51E89180F7A1] (Simulator)"
# execute this to using simctl control simulator

# 编译项目 https://developer.apple.com/library/content/technotes/tn2339/_index.html
# xcodebuild 命令的参数：https://developer.apple.com/legacy/library/documentation/Darwin/Reference/ManPages/man1/xcodebuild.1.html

# xcodebuild test -scheme RustApp -destination 'platform=iOS Simulator,name=iPhone 6s (10.3.1),os=10.3.1'
# xcodebuild build -scheme RustApp -destination 'platform=iOS Simulator,OS=10.3.1,name=iPhone 6s'

# 用模拟器运行：ios-sim: https://github.com/phonegap/ios-sim
# https://gist.github.com/odemolliens/c77645c5e42e5de3233d8b1948f9a3a4
PROJECTNAME='RustApp'
CONFIGURATION='Debug'
LOGFILE='error.log'

touch -cm www
# 这个写法会编译所有的 cpu 架构的版本，但在调度阶段，只需编译指定的版本就可以了
xcodebuild -configuration $CONFIGURATION -sdk iphonesimulator -project $PROJECTNAME.xcodeproj 
# 编译指定的 cpu 架构版本
xcodebuild build -scheme RustApp -destination 'platform=iOS Simulator,OS=10.3.1,name=iPhone 6s' -derivedDataPath build
ios-sim launch build/Build/Products/$CONFIGURATION-iphonesimulator/$PROJECTNAME.app --devicetypeid iPhone-6s --stderr $LOGFILE --exit
osascript -e "tell application \"Simulator\" to activate"
tail -f $LOGFILE


#!/usr/bin/env bash
set -euo pipefail

echo "=============================================="
echo "  Building Relay Rover for iOS"
echo "=============================================="

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

IOS_OUT="$ROOT_DIR/packages/rover/ios"
mkdir -p "$IOS_OUT"

# Verify Rust code compiles for iOS targets
echo "==> Verifying iOS compilation (aarch64-apple-ios)..."
cargo check --target aarch64-apple-ios --release

echo "==> Verifying iOS Simulator compilation (aarch64-apple-ios-sim)..."
cargo check --target aarch64-apple-ios-sim --release

if command -v xcrun &> /dev/null && xcrun --sdk iphoneos --show-sdk-path &> /dev/null; then
  echo "==> Xcode toolchain detected. Compiling iOS static libraries..."
  cargo build --target aarch64-apple-ios --release
  cargo build --target aarch64-apple-ios-sim --release

  cp "$ROOT_DIR/target/aarch64-apple-ios/release/librelay_rover.a" "$IOS_OUT/librelay_rover.a"

  if command -v lipo &> /dev/null && command -v xcodebuild &> /dev/null; then
    echo "==> Creating XCFramework with Xcode..."
    mkdir -p "$IOS_OUT/sim"
    cp "$ROOT_DIR/target/aarch64-apple-ios-sim/release/librelay_rover.a" "$IOS_OUT/sim/librelay_rover.a"

    rm -rf "$IOS_OUT/RelayRover.xcframework"
    xcodebuild -create-xcframework \
      -library "$ROOT_DIR/target/aarch64-apple-ios/release/librelay_rover.a" \
      -headers "$IOS_OUT" \
      -library "$IOS_OUT/sim/librelay_rover.a" \
      -headers "$IOS_OUT" \
      -output "$IOS_OUT/RelayRover.xcframework"
    echo "✓ Created RelayRover.xcframework"
  fi
else
  echo "ℹ Notice: iOS binary linking requires macOS with Xcode and iPhoneOS.sdk."
  echo "  Rust iOS target compilation has been verified via 'cargo check --target aarch64-apple-ios'."
  echo "  When building iOS apps on macOS, CocoaPods / Xcode will compile the Rust staticlib via the podspec."
fi

echo "=============================================="
echo "✓ iOS verification complete!"
echo "=============================================="

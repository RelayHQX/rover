#!/usr/bin/env bash
set -euo pipefail

echo "=============================================="
echo "  Generating UniFFI Bindings (Kotlin & Swift)"
echo "=============================================="

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# 1. Build uniffi-bindgen CLI tool
echo "==> Building uniffi-bindgen..."
cargo build --features cli --bin uniffi-bindgen

# 2. Target directories
ANDROID_OUT="$ROOT_DIR/packages/rover/android/src/main/java"
IOS_OUT="$ROOT_DIR/packages/rover/ios"
mkdir -p "$ANDROID_OUT" "$IOS_OUT"

# 3. Generate Kotlin and Swift bindings
echo "==> Generating bindings from src/rover.udl..."
./target/debug/uniffi-bindgen generate src/rover.udl --language kotlin --no-format --out-dir "$ANDROID_OUT"
./target/debug/uniffi-bindgen generate src/rover.udl --language swift --no-format --out-dir "$IOS_OUT"

echo "✓ UniFFI bindings generated successfully:"
echo "  - Kotlin: $ANDROID_OUT/uniffi/relay_rover/relay_rover.kt"
echo "  - Swift:  $IOS_OUT/relay_rover.swift"
echo "  - C FFI:  $IOS_OUT/relay_roverFFI.h"

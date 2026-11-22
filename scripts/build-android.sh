#!/usr/bin/env bash
set -euo pipefail

echo "=============================================="
echo "  Building Relay Rover for Android (NDK/Rust)"
echo "=============================================="

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ANDROID_HOME="${ANDROID_HOME:-$HOME/Android/Sdk}"
if [[ ! -d "$ANDROID_HOME" ]]; then
  echo "Error: ANDROID_HOME not found at $ANDROID_HOME. Set ANDROID_HOME before running." >&2
  exit 1
fi

# Detect installed NDK version
NDK_DIR=$(find "$ANDROID_HOME/ndk" -mindepth 1 -maxdepth 1 -type d | sort -V | tail -n 1)
if [[ -z "$NDK_DIR" || ! -d "$NDK_DIR" ]]; then
  echo "Error: Android NDK not found in $ANDROID_HOME/ndk." >&2
  exit 1
fi

echo "==> Using NDK at: $NDK_DIR"

TOOLCHAIN="$NDK_DIR/toolchains/llvm/prebuilt/linux-x86_64"
if [[ ! -d "$TOOLCHAIN" ]]; then
  # Try darwin or windows paths if on other platforms
  TOOLCHAIN=$(find "$NDK_DIR/toolchains/llvm/prebuilt" -mindepth 1 -maxdepth 1 -type d | head -n 1)
fi

API_LEVEL=21
JNILIBS_DIR="$ROOT_DIR/packages/rover/android/src/main/jniLibs"

# Array of targets: Rust Target | Android ABI | Clang Prefix
TARGETS=(
  "aarch64-linux-android|arm64-v8a|aarch64-linux-android${API_LEVEL}"
  "x86_64-linux-android|x86_64|x86_64-linux-android${API_LEVEL}"
  "armv7-linux-androideabi|armeabi-v7a|armv7a-linux-androideabi${API_LEVEL}"
)

for ENTRY in "${TARGETS[@]}"; do
  IFS="|" read -r RUST_TARGET ABI CLANG_PREFIX <<< "$ENTRY"
  echo "==> Compiling for $ABI ($RUST_TARGET)..."

  export CC="$TOOLCHAIN/bin/${CLANG_PREFIX}-clang"
  export CXX="$TOOLCHAIN/bin/${CLANG_PREFIX}-clang++"
  export AR="$TOOLCHAIN/bin/llvm-ar"

  VAR_SUFFIX=$(echo "$RUST_TARGET" | tr '-' '_' | tr '[:lower:]' '[:upper:]')
  export "CARGO_TARGET_${VAR_SUFFIX}_LINKER"="$CC"
  export "CC_${RUST_TARGET//-/_}"="$CC"
  export "CXX_${RUST_TARGET//-/_}"="$CXX"
  export "AR_${RUST_TARGET//-/_}"="$AR"

  cargo build --target "$RUST_TARGET" --release

  OUT_DIR="$JNILIBS_DIR/$ABI"
  mkdir -p "$OUT_DIR"
  cp "$ROOT_DIR/target/$RUST_TARGET/release/librelay_rover.so" "$OUT_DIR/librelay_rover.so"
  echo "✓ Output: $OUT_DIR/librelay_rover.so"
done

echo "=============================================="
echo "✓ All Android native libraries compiled!"
echo "=============================================="

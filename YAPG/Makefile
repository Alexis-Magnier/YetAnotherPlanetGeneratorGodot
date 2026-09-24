# File originated from https://github.com/shatadev/dear-imgui-godot/blob/main/imgui/Makefile

CRATE_NAME := yapg_gd
TARGET_DIR := ../godot/addons/YAPG/target

# Pinned toolchain — single source of truth. Bump deliberately, not via `rustup update`.
# Latest stable for: EH default (panic=abort path), the --export=__heap_base requirement,
# and -Zbuild-std on emscripten.
NIGHTLY := nightly-2026-06-19

# Shared flags. panic=abort (in Cargo.toml) handles EH; no -wasm-use-legacy-eh needed.
RUSTFLAGS_COMMON := -C relocation-model=pic \
                    -C link-args=-sSIDE_MODULE=2 \
                    -C llvm-args=-enable-emscripten-cxx-exceptions=0 \
                    -Z default-visibility=hidden \
                    -Z link-native-libraries=no

# Threaded: + atomics, pthread, and --export=__heap_base (required on this nightly).
RUSTFLAGS_THREADS := $(RUSTFLAGS_COMMON) \
                     -C link-args=-pthread \
                     -C target-feature=+atomics \
                     -C link-arg=--export=__heap_base

RUSTFLAGS_NOTHREADS := $(RUSTFLAGS_COMMON)

CARGO := cargo +$(NIGHTLY)
# Platform-specific cdylib filename produced by Cargo.
# Windows: yapg_gd.dll
# macOS:   libyapg_gd.dylib
# Linux:   libyapg_gd.so
ifeq ($(OS),Windows_NT)
    LIBRARY := $(CRATE_NAME).dll
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Darwin)
        LIBRARY := lib$(CRATE_NAME).dylib
    else
        LIBRARY := lib$(CRATE_NAME).so
    endif
endif

.PHONY: all ensure-toolchain native native-editor native-release native-debug clean

all: native

ensure-toolchain:
	rustup toolchain install $(NIGHTLY) --profile minimal --component rust-src

native: native-editor native-release

# Optimized (inherits release) with the in-editor API docs. 
native-editor: | ensure-toolchain
	$(CARGO) build --profile editor --features editor-docs
	mkdir -p $(TARGET_DIR)/debug/
	cp target/editor/$(LIBRARY) $(TARGET_DIR)/debug/
	
native-release: | ensure-toolchain
	$(CARGO) build --release
	mkdir -p $(TARGET_DIR)/release/
	cp target/release/$(LIBRARY) $(TARGET_DIR)/release/

native-debug: | ensure-toolchain
	$(CARGO) build --features
	mkdir -p $(TARGET_DIR)/debug/
	cp target/debug/$(LIBRARY) $(TARGET_DIR)/debug/

clean:
	@rm -r target
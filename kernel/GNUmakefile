# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

override DEFAULT_ARCH := x86_64
$(eval $(call DEFAULT_VAR,ARCH,$(DEFAULT_ARCH)))

ifeq ($(ARCH), x86_64)
    override TARGET := x86_64-unknown-none
else ifeq ($(ARCH), aarch64)
    override TARGET := aarch64-unknown-none
else ifeq ($(ARCH), riscv64)
    override TARGET := riscv64imac-unknown-none-elf
endif

# Default target.
.PHONY: all
all:
	echo "fn main() { \
	    println!("cargo:rustc-link-arg=-Tlinker-aarch64.ld"); \
	    println!("cargo:rerun-if-changed=linker-aarch64.ld"); \
	}" > build.rs
	cargo build --target $(TARGET) --profile $(RUST_PROFILE)
	cp target/$(TARGET)/$(RUST_PROFILE_SUBDIR)/ccos kernel

# Remove object files and the final executable.
.PHONY: clean
clean:
	cargo clean
	rm -rf kernel

.PHONY: distclean
distclean: clean
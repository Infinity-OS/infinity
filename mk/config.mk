# Automatic variables
ROOT=$(PWD)

# Userspace variables
export INITFS_FOLDER=$(ROOT)/initfs
RUSTC=$(PWD)/utils/rustc.sh
CARGO=RUSTC="$(RUSTC)" CARGO_INCREMENRAL=1 xargo

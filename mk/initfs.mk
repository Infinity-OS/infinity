# Build the InitFS
initfs.tag: initfs/bin/init

# Compile the programs
initfs/bin/%: programs/%/Cargo.toml
	mkdir -p initfs/bin
	@xargo rustc --manifest-path $< --target $(target) -- -o $@

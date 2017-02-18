from SCons.Script import *
import os

# Builder to pre-process a linker script
ld_script_builder = Builder(action = Action(
    '$CC $_CCCOMCOM $ASFLAGS -E -x c $SOURCE | grep -v "^\#" > $TARGET',
    '$GENCOMSTR'))

# Builder to run the "cargo rustc" command
cargo_builder = Builder(action = Action('cargo rustc --manifest-path $SOURCE.abspath $CARGOFLAGS --lib --target x86_64-unknown-linux-gnu -- --emit=obj=$TARGET', '$CARGOCOMSTR'),
                        source_factory = SCons.Node.FS.Entry,
                        source_scanner = SCons.Defaults.DirScanner,
                        multi = 1)

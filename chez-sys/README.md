# chez-sys

Low-level bindings to Chez Scheme

## Building

Get the git submodules, recursively since ChezScheme contains submodules.

The build script `build.rs` queries the platform and builds a native lib
suitable for static linking. If the platform isn't supported it falls back
to the portable bytecode build.

An option for dynamic linking is a TODO. Same goes for using the Chez Scheme
provided by the system (a special problem here is that all the distros call it
something different). It would also be desirable to have environment variables
for the location of binaries to use.

Zlib and LZ4 are also linked statically. Enabling use of system libraries is a
TODO.

## TODO
- Feature to use system libs
- Dynamic linking
- Feature for portable bytecode variant
- More tests
- More examples

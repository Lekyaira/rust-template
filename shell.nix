let
  pkgs = import <nixpkgs> { };
  # Unstable Nix
  # To use this, just prepend your package name with `unstable.`
  unstable = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") {};
  # Rust toolchain
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
  rust_toolchain = fenix.combine [
    fenix.stable.toolchain # Standard Rust
    # fenix.complete.toolchain # Nightly
    # fenix.targets.wasm32-unknown-unknown.latest.rust-std # Web Assembly Target
  ];
  # Get project directory.
  pd = builtins.toString ./.;
in
pkgs.mkShell {
  # C/C++ libraries go here.
  nativeBuildInputs = with pkgs; [
    rust_toolchain
		lld clang
		pkg-config
#alsa-lib udev pkg-config
  ];

  # Other dependencies, cli tools, etc go here.
  buildInputs = with pkgs; [
  ];

  # Cargo
  TMPDIR = "${pd}/.cargo/target";
  CARGO_HOME = "${pd}/.cargo";
  CARGO_TARGET_DIR = "${pd}/.cargo/target";
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "clang";
  # Libraries
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ 
	];

  shellHook = ''
#### Cargo ####
# Make sure our Cargo directories exist.
    if [ ! -d $TMPDIR ]; then 
      mkdir -p $TMPDIR
    fi
  '';
}

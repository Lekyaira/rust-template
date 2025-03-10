let
  pkgs = import <nixpkgs> { };
  # Unstable Nix
  unstable = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") {};
  # Rust toolchain
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
  rust_toolchain = fenix.complete.toolchain;
  # rust_toolchain = fenix.combine [
  #   fenix.complete.toolchain
  #   fenix.targets.wasm32-unknown-unknown.latest.rust-std
  # ];
  # Get project directory.
  pd = builtins.toString ./.;
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rust_toolchain
    pkg-config
  ];

  buildInputs = with pkgs; [
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
  ];

  # Cargo
  TMPDIR = "${pd}/.cargo/target";
  CARGO_HOME = "${pd}/.cargo";
  CARGO_TARGET_DIR = "${pd}/.cargo/target";
  # Libraries
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl pkgs.sqlite ];

  shellHook = ''
    #### Cargo ####
    if [ ! -d $TMPDIR ]; then 
      mkdir -p $TMPDIR
    fi
  '';
}

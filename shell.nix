{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "dioxus-dev-shell";

  buildInputs = with pkgs; [
    rustc
    cargo
    wasm-pack
    binaryen       # for wasm-opt (optional optimization)
    dioxus-cli     # Dioxus tooling for build/serve
    trunk          # optional: alternative to dioxus-cli
    openssl
    pkg-config
  ];

  shellHook = ''
    echo "Dioxus development environment loaded!"
    echo "Use: dioxus serve - for hot reload dev server"
  '';
}


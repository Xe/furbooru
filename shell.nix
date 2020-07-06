let pkgs = import <nixpkgs> { };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    # rust
    cargo
    cargo-watch
    rls
    rustc
    rustfmt

    # system
    openssl
    pkg-config

    # lua
    lua5_3
  ];
}

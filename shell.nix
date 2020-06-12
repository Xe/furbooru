let
  pkgs = import <nixpkgs> {};
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    cargo-watch
    openssl
    pkg-config
    rls
    rustc
    rustfmt
  ];
}

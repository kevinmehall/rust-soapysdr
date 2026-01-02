{ pkgs ? import <nixpkgs> {} }:
with pkgs; mkShell {
  buildInputs = [
    soapysdr-with-plugins
    pkg-config
    llvm
    cargo
    rustc
    rustPlatform.bindgenHook
  ];
}

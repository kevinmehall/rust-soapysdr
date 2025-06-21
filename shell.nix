with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "env";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    soapysdr-with-plugins
    pkg-config
    llvm
    cargo
    rustPlatform.bindgenHook
  ];
}

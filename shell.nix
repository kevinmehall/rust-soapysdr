with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "env";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    soapysdr-with-plugins
    pkgconfig
    llvm
  ];
  LIBCLANG_PATH="${llvmPackages.libclang}/lib";
}

{
  fenix,
  makeRustPlatform,
  pkg-config,
  openssl,
}:
(makeRustPlatform {
  inherit (fenix) cargo rustc;
}).buildRustPackage rec {
  pname = "mcmods";
  version = "0.1.0";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [pkg-config];
  buildInputs = [openssl];
}

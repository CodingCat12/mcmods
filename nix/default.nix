{pkgs}: let
  toolchain = pkgs.fenix.minimal.toolchain;
in
  (pkgs.makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  })
  .buildRustPackage {
    pname = "mcmods";
    version = "0.1.0";

    src = ../.;

    cargoLock.lockFile = ../Cargo.lock;

    nativeBuildInputs = [pkgs.pkg-config pkgs.openssl];
    buildInputs = [pkgs.openssl pkgs.pkg-config];
  }

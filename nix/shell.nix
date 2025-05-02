{
  lib,
  mkShell,
  lldb,
  fenix,
  callPackage,
  pkg-config,
  openssl,
}:
mkShell rec {
  packages = [
    (fenix.complete.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
      "rust-analyzer"
    ])
    lldb
  ];

  nativeBuildInputs = [pkg-config];
  buildInputs = [openssl];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}

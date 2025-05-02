{pkgs}:
pkgs.mkShell {
  packages = with pkgs; [
    (fenix.complete.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
      "rust-analyzer"
    ])
    lldb

    alejandra
    nil
  ];

  nativeBuildInputs = [pkgs.pkg-config];
  buildInputs = [pkgs.openssl];
}

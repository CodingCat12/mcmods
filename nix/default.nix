{
  lib,
  fenix,
  makeRustPlatform,
  fetchFromGitHub,
  pkg-config,
  openssl,
}:
(makeRustPlatform {
  inherit (fenix) cargo rustc;
}).buildRustPackage rec {
  pname = "mcmods";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "CodingCat12";
    repo = "mcmods";
    rev = "3448aba";
    hash = "sha256-LYJrrcmNNsta7xxgtWkifHdiYfw2Y1Nxbtz+Sw5mauc=";
  };

  cargoHash = "sha256-GcSxrqF33aj7rstp9b5BC8DHjESFbJULR6gxFjBL6xI=";

  nativeBuildInputs = [pkg-config];
  buildInputs = [openssl];

  meta = with lib; {
    description = "A simple Minecraft mod manager";
    license = licenses.mit;
  };
}

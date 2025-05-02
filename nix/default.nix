{
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
    rev = "c355c23";
    hash = "sha256-wsIY/DrGTBQjWzbcwmJT0iZGLEmPgdZUULJ78kRbKiI=";
  };

  cargoHash = "sha256-+nkJBtK8gDLjmW/bJM0We59bY0r15GvIeDZ1sxGgwOA=";

  nativeBuildInputs = [pkg-config];
  buildInputs = [openssl];
}

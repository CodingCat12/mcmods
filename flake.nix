{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        pkgs,
        config,
        system,
        ...
      }: let
        fenix = inputs.fenix.packages.${system};
      in {
        packages.default = pkgs.callPackage ./nix/default.nix {};
        devShells.default = pkgs.callPackage ./nix/shell.nix {inherit fenix;};
        formatter = pkgs.alejandra;
      };
    };
}

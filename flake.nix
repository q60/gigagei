{
  description = "gigagei";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem
    (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      packages = {
        default = pkgs.rustPlatform.buildRustPackage {
          name = "gigagei";

          nativeBuildInputs = [pkgs.clippy];

          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
      };

      apps.default = utils.lib.mkApp {drv = self.packages.${system}.default;};
    });
}

{
  description = "A beginning of an awesome project bootstrapped with github:bleur-org/templates";

  inputs = {
    # Stable for keeping thins clean
    # nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";

    # Fresh and new for testing
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    # The flake-utils library
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      # Nix script formatter
      formatter = pkgs.alejandra;

      # Development environment
      devShells.default = import ./shell.nix {inherit pkgs;};

      # Output package
      packages.default = pkgs.callPackage ./. {inherit pkgs;};
    });
}

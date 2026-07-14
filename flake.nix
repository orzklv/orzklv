{
  description = "Just a profile readme toy thing";

  inputs = {
    # Stable for keeping thins clean
    # nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";

    # Fresh and new for testing
    nixpkgs.url = "https://git.oss.uzinfocom.uz/xinux/nixpkgs/archive/nixos-unstable.tar.gz";

    # The flake-utils library
    flake-parts.url = "https://git.oss.uzinfocom.uz/mirrors/flake-parts/archive/main.tar.gz";
  };

  outputs =
    inputs@{ self, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      { ... }:
      {
        systems = [ "x86_64-linux" ];
        perSystem =
          { pkgs, ... }:
          {
            # Nix script formatter
            formatter = pkgs.nixfmt-rs;

            # Development environment
            devShells.default = import ./shell.nix self { inherit pkgs; };

            # Output package
            packages.default = pkgs.callPackage ./. { inherit pkgs; };
          };
      }
    );
}

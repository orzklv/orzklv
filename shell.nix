# Either have nixpkgs and fenix in your channels
# Or build it using flakes, flake way is more recommended!
flake:
{
  pkgs ?
    let
      lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
      nixpkgs = fetchTarball {
        url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
        sha256 = lock.narHash;
      };
    in
    import nixpkgs { overlays = [ ]; },
  ...
}:
let
  # Hostplatform system
  system = pkgs.hostPlatform.system;

  # Production package
  base = flake.packages.${system}.default;
in
pkgs.mkShell {
  inputsFrom = [ base ];

  packages = with pkgs; [
    nixd
    statix
    deadnix
    nixfmt-rs

    rustfmt
    clippy
    rust-analyzer
    cargo-watch

    # Other packages here
    prettier
  ];
}

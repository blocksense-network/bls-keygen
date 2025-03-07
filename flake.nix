{
  description = "Blocksense Network Monorepo";

  nixConfig = {
    extra-substituters = [
      "https://blocksense.cachix.org"
      "https://mcl-blockchain-packages.cachix.org"
      "https://mcl-public-cache.cachix.org"
    ];
    extra-trusted-public-keys = [
      "blocksense.cachix.org-1:BGg+LtKwTRIBw3BxCWEV//IO7v6+5CiJVSGzBOQUY/4="
      "mcl-blockchain-packages.cachix.org-1:qoEiUyBgNXmgJTThjbjO//XA9/6tCmx/OohHHt9hWVY="
      "mcl-public-cache.cachix.org-1:OcUzMeoSAwNEd3YCaEbNjLV5/Gd+U5VFxdN2WGHfpCI="
    ];
  };

  inputs = {
    mcl-blockchain.url = "github:metacraft-labs/nix-blockchain-development";
    nixpkgs.follows = "mcl-blockchain/nixpkgs";
    crane.follows = "mcl-blockchain/crane";
    mcl-nixos-modules.follows = "mcl-blockchain/nixos-modules";
    flake-parts.follows = "mcl-blockchain/flake-parts";
    fenix.follows = "mcl-blockchain/fenix";
  };

  outputs =
    inputs@{ flake-parts, crane, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        { pkgs, inputs', ... }:
        let
          rustToolchain =
            with inputs'.fenix.packages;
            with latest;
            combine [
              cargo
              clippy
              rust-analyzer
              rust-src
              rustc
              rustfmt
              targets.wasm32-wasip1.latest.rust-std
            ];

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        in
        {
          devShells.default = pkgs.mkShell {
            packages = [ rustToolchain ];
          };

          packages.default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;
          };
        };
    };
}

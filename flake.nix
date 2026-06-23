{
  description = "Blocksense Network Monorepo";

  nixConfig = {
    extra-substituters = [
      "https://cache.metacraft-labs.com/blocksense-public"
      "https://cache.metacraft-labs.com/metacraft-public"
    ];
    extra-trusted-public-keys = [
      "blocksense-public:OOgTc0ye1FONCiVHMrbpScc/HP+lX3uoU0EfwzX6ypE="
      "metacraft-public:UtS6PK+p0uZaJK3i/jD2DQOjTpddhQUQmNQDQih5N4Q="
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
            packages = [
              rustToolchain
              inputs'.mcl-nixos-modules.checks.foundry
            ];
          };

          packages.default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;
          };
        };
    };
}

{
  description = "vrf-rust";

  nixConfig = {
    extra-substituters = [
      "https://crane.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = inputs@{self, flake-parts, treefmt-nix, ...}:
    flake-parts.lib.mkFlake { inherit inputs; }{
      systems = [ "aarch64-darwin" "x86_64-linux" ];
      imports = [
        treefmt-nix.flakeModule
      ];
      perSystem = {config, self', inputs', system, pkgs, lib, ...}:
        let
          toolchain = inputs'.fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "";
          };
          rustToolchain = inputs'.fenix.packages.complete.toolchain;
          craneLib = inputs.crane.lib.${system}.overrideToolchain rustToolchain;

          attributes = {
            src = lib.cleanSourceWith {
              src = craneLib.path "./";
              filter = path: type: craneLib.filterCargoSources path type;
            };
            nativeBuildInputs = with pkgs; [ pkg-config  ];
            buildInputs = with pkgs; [ openssl.dev sqlite ] ++ (lib.optionals (system == "aarch64-darwin") [pkgs.libiconv pkgs.darwin.Security pkgs.darwin.apple_sdk.frameworks.SystemConfiguration]);
          };
        in
          {
            devShells.default = pkgs.mkShell {
              # Rust Analyzer needs to be able to find the path to default crate
              RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
              nativeBuildInputs = [
                rustToolchain
              ]  ++ attributes.nativeBuildInputs ++ attributes.buildInputs;
          };

          packages = {
            vrf-rust-deps = craneLib.buildDepsOnly (attributes // {
              pname = "vrf-rust-deps";
            });
            vrf-rust =
                let vrf-rust' =
                    craneLib.buildPackage (attributes // {
                    cargoArtifacts = self'.packages.vrf-rust-deps;
                    meta.mainProgram = "vrf-rust";
                    });
                in pkgs.runCommand "vrf-rust-wrapped" {
                    buildInputs = [ pkgs.makeWrapper ];
                    meta.mainProgram = "vrf-rust";
                }
                ''
                    mkdir -p $out/bin
                    makeWrapper ${vrf-rust'}/bin/vrf-rust $out/bin/vrf-rust \
                        --set PATH ${pkgs.lib.makeBinPath [ inputs'.noir.packages.nargo ]} \
                '';

            default = self'.packages.vrf-rust;

            vrf-rust-docs = craneLib.cargoDoc (attributes // {
              cargoArtifacts = self'.packages.vrf-rust-deps;
            });
        };
    };
  };
}
{
  description = "webbrot flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        toolchain = fenix.packages.${system}.latest.toolchain;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            toolchain

            pkg-config
            openssl

            git
            just
            bacon
            trunk

            rust-analyzer
          ];

          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}

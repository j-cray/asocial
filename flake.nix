{
  description = "Asocial: Deployment and Dev Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        
        # macOS specific frameworks
        darwinFrameworks = with pkgs.darwin.apple_sdk.frameworks; [
          ApplicationServices
          CoreFoundation
          CoreGraphics
          CoreText
          Foundation
          IOKit
          Metal
          QuartzCore
          Security
          SystemConfiguration
          pkgs.libiconv
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust Toolchain
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy

            # Database & Infra
            sqlx-cli
            docker-compose

            # Libraries
            openssl
            pkg-config
          ] ++ (if pkgs.stdenv.isDarwin then darwinFrameworks else []);

          shellHook = ''
            export RUST_BACKTRACE=1
          '';
        };
      }
    );
}

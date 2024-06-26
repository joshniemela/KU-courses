

{
  description = "A basic Rust devshell for NixOS users developing Leptos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              docker
              docker-compose
              #git
              #nodejs
              #leiningen
              #pipenv
              #python311

              ## browsers for testing
              #firefox
              #chromium
              #pkg-config
              #rust-bin.stable.latest.default 
            ];

            DATABASE_URL="postgres://postgres:password123@localhost:5432/disku";

          };
        }
    );
}

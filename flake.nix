{
  description = "astrid.tech site";

  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, flake-utils, nixpkgs, ... }@inputs:
    {
      overlay = (final: prev: { });
    } // (flake-utils.lib.eachSystem [
      "x86_64-linux"
      "x86_64-darwin"
      "aarch64-linux"
    ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ self.overlay ];
        };
        lib = pkgs.lib;
      in rec {
        devShells.default = with pkgs;
          mkShell {
            nativeBuildInputs = [
              #autoreconfHook
              #backblaze-b2
              #cargo
              #curl
              #docker
              #docker-compose
              #git
              #nodejs
              pipenv
              #python310
              #rustc
              #yarn

              #nodePackages.prettier
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath [ stdenv.cc.cc 
            libpng glibc mlib zlib ];
          };

        devShells.content = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ code ];
        };
      }));
}

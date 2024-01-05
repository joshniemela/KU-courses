{
  description = "Application packaged using poetry2nix";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.poetry2nix = {
    url = "github:nix-community/poetry2nix/";
    inputs = {
      nixpkgs.follows = "nixpkgs";
      flake-utils.follows = "flake-utils";
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , poetry2nix
    ,
    }: (flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
        overlays = [ poetry2nix.overlays.default ];
      };

      customOverrides = self: super: {
        scikit-build = super.scikit-build.overridePythonAttrs (
          old: {
            buildInputs = [ self.wheel ] ++ (old.buildInputs or [ ]);
          }
        );
        plaidml = super.plaidml.overridePythonAttrs (old: {
          pipInstallFlags = "--no-deps";
        });

        plaidml-keras = super.plaidml-keras.overridePythonAttrs (old: {
          pipInstallFlags = "--no-deps";
        });

        pyacvd = super.pyacvd.overridePythonAttrs (old: {
          buildInputs = [ self.cython ] ++ (old.buildInputs or [ ]);
        });

        nvidia-cusparse-cu12 = super.nvidia-cusparse-cu12.overridePythonAttrs(old: {
          autoPatchelfIgnoreMissingDeps = true;
          buildInputs = [
            self.nvidia-nvjitlink-cu12
          ] ++ (old.buildInputs or [ ]);
        });

      nvidia-cudnn-cu12 = super.nvidia-cudnn-cu12.overridePythonAttrs (attrs: {
        autoPatchelfIgnoreMissingDeps = true;
        # (Bytecode collision happens with nvidia-cuda-nvrtc-cu12.)
        postFixup = ''
          rm -r $out/${self.python.sitePackages}/nvidia/{__pycache__,__init__.py}
        '';
        propagatedBuildInputs = attrs.propagatedBuildInputs or [ ] ++ [
          self.nvidia-cublas-cu12
        ];
      });

      nvidia-cuda-nvrtc-cu12 = super.nvidia-cuda-nvrtc-cu12.overridePythonAttrs (_: {
        # (Bytecode collision happens with nvidia-cudnn-cu12.)
        postFixup = ''
          rm -r $out/${self.python.sitePackages}/nvidia/{__pycache__,__init__.py}
        '';
      });

      nvidia-cusolver-cu12 = super.nvidia-cusolver-cu12.overridePythonAttrs (attrs: {
        autoPatchelfIgnoreMissingDeps = true;
        # (Bytecode collision happens with nvidia-cusolver-cu12.)
        postFixup = ''
          rm -r $out/${self.python.sitePackages}/nvidia/{__pycache__,__init__.py}
        '';
        propagatedBuildInputs = attrs.propagatedBuildInputs or [ ] ++ [
          self.nvidia-cublas-cu12
        ];
      });


        pybind11 = pkgs.python311Packages.pybind11;

        torch = super.torch.overridePythonAttrs
          (old: {
            buildInputs = [
              self.nvidia-cublas-cu12
              self.nvidia-cuda-cupti-cu12
              self.nvidia-cuda-nvrtc-cu12
              self.nvidia-cuda-runtime-cu12
              self.nvidia-cudnn-cu12
              self.nvidia-cufft-cu12
              self.nvidia-curand-cu12
              self.nvidia-cusolver-cu12
              self.nvidia-cusparse-cu12
              self.nvidia-nccl-cu12
              self.nvidia-nvtx-cu12
              self.triton
            ] ++ (old.buildInputs or [ ]);
          });
          };

        gpu_libs = with pkgs; [
          cudaPackages_11.cudatoolkit
          cudaPackages_11.cudatoolkit.lib
          cudaPackages_11.cudnn
          cudaPackages_11.libcufft
          cudaPackages_11.libcublas
          cudaPackages_11.libcurand
          ocl-icd
        ];

        my_env = (pkgs.poetry2nix.mkPoetryEnv
          {
            projectDir = ./.;
            preferWheels = true;
            overrides = [ customOverrides pkgs.poetry2nix.defaultPoetryOverrides ];
            python = pkgs.python311;
          }).override { ignoreCollisions = true; };
        in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            poetry
            my_env
            gtk3
            glib
            gsettings-desktop-schemas
            clinfo
            zlib
            cmake
            pkg-config
          ];
          # ++ gpu_libs;
          LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";
          #LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          #  "/run/opengl-driver"
          #  "${my_env}/${my_env.python.sitePackages}/nvidia/cudnn"
          #  "${my_env}/${my_env.python.sitePackages}/nvidia/cublas"
          #  "${my_env}/${my_env.python.sitePackages}/nvidia/nvjitlink"
          #  "${my_env}/${my_env.python.sitePackages}/nvidia/cusparse"
          #];
        };
        defaultPackage = my_env;
      }));
      }

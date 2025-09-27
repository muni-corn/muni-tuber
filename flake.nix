{
  description = "munituber";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-flake = {
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      imports = [
        inputs.git-hooks-nix.flakeModule
        inputs.devenv.flakeModule
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        {
          self',
          config,
          lib,
          pkgs,
          system,
          ...
        }:
        let
          pname = "munituber";

          buildInputs = with pkgs; [
            alsa-lib
            pkg-config
            udev
            vulkan-loader

            # x11
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr

            # wayland
            libxkbcommon
            wayland

            # gl
            libGL
            libGLU
          ];
          nativeBuildInputs = [ ];
        in
        {
          # rust setup
          devenv.shells.default = {
            env = {
              RUST_LOG = "info,${pname}=debug";
              LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${lib.makeLibraryPath buildInputs}";
            };

            languages.rust = {
              enable = true;
              channel = "nightly";
              mold.enable = true;
            };

            packages = [
              config.treefmt.build.wrapper
              pkgs.cargo-outdated
            ]
            ++ buildInputs
            ++ nativeBuildInputs
            ++ (builtins.attrValues config.treefmt.build.programs);

            # git hooks
            git-hooks.hooks = {
              # commit linting
              commitlint-rs =
                let
                  config = pkgs.writers.writeYAML "commitlintrc.yml" {
                    rules = {
                      description-empty.level = "error";
                      description-format = {
                        level = "error";
                        format = "^[a-z].*$";
                      };
                      description-max-length = {
                        level = "error";
                        length = 72;
                      };
                      scope-max-length = {
                        level = "warning";
                        length = 10;
                      };
                      scope-empty.level = "warning";
                      type = {
                        level = "error";
                        options = [
                          "build"
                          "chore"
                          "ci"
                          "docs"
                          "feat"
                          "fix"
                          "perf"
                          "refactor"
                          "style"
                          "test"
                        ];
                      };
                    };
                  };

                in
                {
                  enable = true;
                  name = "commitlint-rs";
                  package = pkgs.commitlint-rs;
                  description = "Validate commit messages with commitlint-rs";
                  entry = "${pkgs.lib.getExe pkgs.commitlint-rs} -g ${config} -e";
                  always_run = true;
                  stages = [ "commit-msg" ];
                };

              # format on commit
              treefmt = {
                enable = true;
                packageOverrides.treefmt = config.treefmt.build.wrapper;
              };
            };
          };

          # rust build settings
          rust-project = {
            # use the same rust toolchain from the dev shell for consistency
            toolchain = config.devenv.shells.default.languages.rust.toolchainPackage;

            # specify dependencies
            defaults.perCrate.crane.args = {
              inherit nativeBuildInputs buildInputs;
            };
          };

          # formatting
          treefmt.programs = {
            nixfmt.enable = true;
            rustfmt.enable = true;
            taplo.enable = true;
          };

          # package definitions
          packages.default = config.rust-project.crates.${pname}.crane.outputs.packages.${pname};
        };
    };
}

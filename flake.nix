{
  description = "muni-tuber";

  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, fenix, naersk }:
    let
      appName = "muni-tuber";

      out = utils.lib.eachDefaultSystem
        (system:
          let
            pkgs = import nixpkgs { inherit system; };

            rust = fenix.packages.${system}.complete;
            naersk-lib = naersk.lib."${system}".override {
              inherit (rust) cargo rustc;
            };

            nativeBuildInputs = with pkgs; [
              rust.toolchain
              pkg-config
            ];
            buildInputs = with pkgs; [
              alsa-lib
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
            ];
          in
          {
            # `nix build`
            defaultPackage = naersk-lib.buildPackage {
              pname = appName;
              root = builtins.path {
                path = ./.;
                name = "${appName}-src";
              };
              inherit nativeBuildInputs buildInputs;
            };

            # `nix run`
            defaultApp = utils.lib.mkApp {
              name = appName;
              drv = self.defaultPackage."${system}";
              exePath = "/bin/${appName}";
            };

            # `nix develop`
            devShell = with pkgs; mkShell {
              inherit nativeBuildInputs buildInputs;
              LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
            };
          });
    in
    out // {
      overlay = final: prev: {
        ${appName} = self.defaultPackage.${prev.system};
      };
    };
}

{
  description = "muni-tuber";

  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    let
      appName = "muni-tuber";

      overlays = [ (import rust-overlay) ];
      out = utils.lib.eachDefaultSystem
        (system:
          let
            pkgs = import nixpkgs { inherit system overlays; };

            rust = pkgs.rust-bin.nightly.latest.default;
            naersk-lib = naersk.lib."${system}".override {
              cargo = rust;
              rustc = rust;
            };

            nativeBuildInputs = with pkgs; [
              rust
              pkg-config
              llvmPackages.bintools
            ];
            buildInputs = with pkgs; [
              alsa-lib
              clang
              libxkbcommon
              udev
              vulkan-loader
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              xorg.libX11
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
              packages = nativeBuildInputs ++ buildInputs ++ [
                cargo-watch
                clippy
                rust-analyzer
                rustfmt
              ];
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

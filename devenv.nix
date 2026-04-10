{
  config,
  lib,
  pkgs,
  ...
}:
let
  pname = "muni-tuber";
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

    # gl
    libGL
    libGLU
  ];
  nativeBuildInputs = with pkgs; [
    autoPatchelfHook
    pkg-config
  ];
  libraryPath = lib.makeLibraryPath buildInputs;
in
{
  # needed for dynamic linking at runtime
  env.RUSTFLAGS = lib.mkForce "-C link-args=-Wl,-fuse-ld=mold,-rpath,${libraryPath}";

  languages.rust = {
    enable = true;
    channel = "nightly";
    mold.enable = true;
  };

  packages = buildInputs ++ nativeBuildInputs;

  outputs.default =
    let
      args = {
        crateOverrides = pkgs.defaultCrateOverrides // {
          ${pname} = attrs: {
            inherit buildInputs nativeBuildInputs;
            runtimeDependencies = buildInputs;
          };
        };
      };
    in
    config.languages.rust.import ./. args;
}

{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    clang
    openssl
    pkg-config
    # llvmPackages.bintools
    rustup
    # cargo-cross
    # alsa-lib
    # lldb
    # glibc.dev
  ];
  RUSTC_VERSION = "stable";
  # https://github.com/rust-lang/rust-bindgen#environment-variables
  LIBCLANG_PATH =
    pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
  RUST_BACKTRACE = 1;

  # LD_LIBRARY_PATH = with pkgs;
  #   lib.makeLibraryPath [
  #     libGL
  #     libxkbcommon
  #     wayland
  #     xorg.libX11
  #     xorg.libXcursor
  #     xorg.libXi
  #     xorg.libXrandr
  #     alsa-lib
  #     # vulkan-loader
  #   ];

}
let
  pkgs =
    import <nixpkgs> {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [pkgs.rustc pkgs.cargo pkgs.rustPlatform.rustcSrc];
  };
in with pkgs;
mkShell {
  name = "scriptr";
  buildInputs = [
    rust-toolchain
    linuxKernel.packages.linux_latest_libre.perf
    hotspot
    hyperfine
    cargo-flamegraph
  ];
  RUST_BACKTRACE = 1;
}
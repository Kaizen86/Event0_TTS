{ pkgs ? import <nixpkgs> {} }:
  {
    devShell = with pkgs; mkShell {
      buildInputs = [
        cargo
        rustc
        rust-analyzer
        #alsa-oss
        alsa-lib
      ];
      nativeBuildInputs = [
        pkg-config
      ];
      
      RUST_LOG = "debug";
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  }

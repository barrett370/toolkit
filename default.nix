{ pkgs ? import <nixpkgs> {} }:
pkgs.stdenv.mkDerivation {

  name = "toolkit";
  version = "0.1.0";
  src = ./.;
  buildInputs = with pkgs;[
    cargo
    libiconv
    nix
    rustc
  ];

  installPhase = ''
      cargo build --release -Z unstable-options --out-dir $out/bin
  '';

  postInstall = ''
        rm -rf $out
  '';
}

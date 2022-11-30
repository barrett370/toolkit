{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    crate2nix
    nixpkgs-fmt
  ];
}

{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    gtk3
    gtk-layer-shell
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
}
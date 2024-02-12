{ pkgs ? (import ../tools/pinned-nixpkgs.nix) {} }:

pkgs.mkShellNoCC {
  buildInputs = [
    pkgs.nodePackages.pnpm
    (pkgs.callPackage ./javy.nix { })
    pkgs.binaryen
    pkgs.wasmtime
    pkgs.gnumake
  ];
}

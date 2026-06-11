{
  pkgs ? (import ../../build-support/pinned-nixpkgs.nix) { },
}:

pkgs.mkShellNoCC {
  packages = [
    pkgs.pnpm
    pkgs.nodejs-slim
    (pkgs.callPackage ./javy.nix { })
    pkgs.binaryen
    pkgs.wasmtime
    pkgs.gnumake
  ];
}

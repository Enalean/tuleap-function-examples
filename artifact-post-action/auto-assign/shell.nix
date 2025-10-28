{
  pkgs ? (import ../../build-support/pinned-nixpkgs.nix) { },
}:

pkgs.mkShellNoCC {
  buildInputs = [
    pkgs.nodePackages.pnpm
    pkgs.nodejs-slim
    (pkgs.callPackage ./javy.nix { })
    pkgs.binaryen
    pkgs.wasmtime
    pkgs.gnumake
  ];
}

{
  pkgs ? (import ../../build-support/pinned-nixpkgs.nix) { },
}:

pkgs.mkShellNoCC {
  packages = [
    (pkgs.rust-bin.stable.latest.default.override {
      targets = [ "wasm32-wasip1" ];
      extensions = [
        "cargo"
        "rustc"
        "rust-src"
      ];
    })
    pkgs.gnumake
  ];
}

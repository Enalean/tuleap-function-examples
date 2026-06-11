{
  pkgs ? (import ./build-support/pinned-nixpkgs.nix) { },
  treefmt-nix ? (import ./build-support/pinned-treefmt-nix.nix { }),
}:

pkgs.mkShellNoCC {
  packages = [
    (treefmt-nix.mkWrapper pkgs {
      programs.gofmt.enable = true;
      programs.rustfmt.enable = true;
      programs.nixfmt = {
        enable = true;
        package = pkgs.nixfmt;
      };
      programs.actionlint.enable = true;
      programs.zizmor.enable = true;
    })
  ];
}

# TODO:
# * build from sources
# * upstream to nixpkgs
{ stdenv
, lib
, fetchurl
, autoPatchelfHook
, gzip
}:

stdenv.mkDerivation rec {
  pname = "javy";
  version = "1.4.0";

  src = fetchurl {
    url = "https://github.com/bytecodealliance/javy/releases/download/v${version}/javy-x86_64-linux-v${version}.gz";
    hash = "sha256-NZIzT8BdtgKiE3RbePWEY1E5TWe9mr2LSRhhmxzWzd8=";
  };

  nativeBuildInputs = [
    autoPatchelfHook
    gzip
  ];

  buildInputs = [
    stdenv.cc.cc.lib
  ];
  
  unpackPhase = ''
    runHook preUnpack
    gzip -cd "${src}" > javy
    runHook postUnpack
  '';

  installPhase = ''
    runHook preInstall
    install -m755 -D javy $out/bin/javy
    runHook postInstall
  '';

  meta = with lib; {
    homepage = "https://github.com/bytecodealliance/javy";
    description = "JS to WebAssembly toolchain";
    license = licenses.asl20;
  };
}

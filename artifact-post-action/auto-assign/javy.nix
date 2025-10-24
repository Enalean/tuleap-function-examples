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
  version = "7.0.1";

  src = fetchurl {
    url = "https://github.com/bytecodealliance/javy/releases/download/v${version}/javy-x86_64-linux-v${version}.gz";
    hash = "sha256-NhzCN1B3OAzabgAlRc+oMfK0cwK+zsC66/VCVIMfe54=";
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

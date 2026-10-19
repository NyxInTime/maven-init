{
  lib,
  fetchFromGitHub,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "maven-init";
  version = "1.2.0";

  src = fetchFromGitHub {
    owner = "NyxInTime";
    repo = "maven-init";
    tag = finalAttrs.version;
    hash = "sha256-4/ESv775o1QchpwpuVdWesaE4dyqIQ6K5HaQCHiX4DI=";
  };

  cargoHash = "sha256-2a/7cMj8vKKmj99dIlcajxpMpSBiTAxf/XKJQQyyUcI=";

  meta = {
    description = "CLI tool to make my life, and only my life, easier";
    homepage = "https://github.com/NyxInTime/maven-init";
    liscense = lib.licenses.mit;
    maintainers = [ ];
  };

})

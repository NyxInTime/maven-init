{
  lib,
  fetchFromGitHub,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "maven-init";
  version = "1.0.0";

  src = fetchFromGitHub {
    owner = "NyxInTime";
    repo = "maven-init";
    tag = finalAttrs.version;
    hash = "sha256-46YMCL8kr4YGVJK/TZJwzDI5uhQLh0k8IU6ovr5HCKw=";
  };

  cargoHash = "sha256-2a/7cMj8vKKmj99dIlcajxpMpSBiTAxf/XKJQQyyUcI=";

  meta = {
    description = "CLI tool to make my life, and only my life, easier";
    homepage = "https://github.com/NyxInTime/sizeof";
    liscense = lib.licenses.mit;
    maintainers = [ ];
  };

})

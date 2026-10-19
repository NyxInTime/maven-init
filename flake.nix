{
  description = "sizeof flake";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      lib = nixpkgs.lib;

      systems = [
        "x86_64-linux"
      ];

      forEachSystem =
        perSystem: nixpkgs.lib.genAttrs systems (system: perSystem nixpkgs.legacyPackages.${system});
    in
    {
      nixosModules.default =
        {
          config,
          lib,
          pkgs,
          ...
        }:
        {
          options.programs.maven-init.enable = lib.mkEnableOption "sizeof";
          config = lib.mkIf config.programs.maven-init.enable {
            environment.systemPackages = [
              (pkgs.callPackage ./default.nix { })
            ];
          };
        };
      packages = forEachSystem (pkgs: {
        default = pkgs.callPackage ./default.nix { };
      });
    };
}

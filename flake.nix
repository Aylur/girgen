{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    forAllSystems = nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "x86_64-darwin"
      "aarch64-linux"
      "aarch64-darwin"
    ];
  in {
    devShells = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      default = with pkgs;
        mkShell {
          packages = [
            cargo
            rustup
            gcc

            gobject-introspection
            glib
            libadwaita
            gtk3
            gtk4
            libsoup_3
          ];
        };
    });

    packages = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.package.name;
        version = cargoToml.package.version;
        src = ./.;
        cargoHash = "sha256-VPsIQ6Cj0i1vZNlkwlvevvjcRNSvBOzDzknhOH4Sx6A=";
      };
    });
  };
}

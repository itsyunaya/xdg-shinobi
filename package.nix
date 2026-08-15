{ pkg-config, rustPlatform }: let
    manifest = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
    inherit (manifest.package) name version;
in rustPlatform.buildRustPackage {
    pname = name;
    inherit version;

   src = ./.;

   cargoHash = "sha256-hKSd/i81utyyYCQvHvE1zsH5mvqMXQ0Oahc7XQT2u3g=";
   nativeBuildInputs = [ pkg-config ];
}

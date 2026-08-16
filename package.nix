{ rustPlatform }: let
	manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
	inherit (manifest.package) name version;
in
	rustPlatform.buildRustPackage {
		pname = name;
		inherit version;

		# there are no tests to run
		doCheck = false;

		src = ./.;
		cargoHash = "sha256-RWD++yORDrNOyxrS4ZceZHPtnGEEkHDgMFvNdwFRMmE=";
	}

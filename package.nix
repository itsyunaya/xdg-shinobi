{ lib, rustPlatform }: let
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

		meta = {
		    description = "Program which checks your $HOME for unwanted files and directories, and adaptation of xdg-ninja to Rust";
		    license = lib.licenses.gpl3Plus;
		    mainProgram = "xdg-shinobi";
		};
	}

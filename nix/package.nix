{ lib, rustPlatform }: let
	manifest = builtins.fromTOML (builtins.readFile ../Cargo.toml);
	inherit (manifest.package) name version;
in
	rustPlatform.buildRustPackage {
		pname = name;
		inherit version;

		src = ../.;
		cargoLock.lockFile = ../Cargo.lock;

		meta = {
		    description = "Program which checks your $HOME for unwanted files and directories, and adaptation of xdg-ninja";
			homepage = "https://github.com/itsyunaya/xdg-shinobi";
		    license = lib.licenses.gpl3Plus;
			maintainers = [ lib.maintainers.itsyunaya ];
		    mainProgram = "xdg-shinobi";
		};
	}

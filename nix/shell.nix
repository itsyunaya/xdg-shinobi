{ mkShell, pkgs, rustPlatform }:
mkShell {
	packages = builtins.attrValues {
		inherit
			(pkgs)
			cargo
			rustc
			clippy
			rust-analyzer
			rustfmt
			;
	};

	env = {
		RUST_SRC_PATH = rustPlatform.rustLibSrc;
	};
}

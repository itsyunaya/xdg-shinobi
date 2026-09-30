{
	description = "A program which checks your $HOME for unwanted files and directories.";

	inputs.nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.zst";

	outputs = { nixpkgs, ... }: let
		systems = [
			"x86_64-linux"
			"aarch64-linux"
			"aarch64-darwin"
		];

		forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
	in {
		packages = forAllSystems (pkgs: let
			xdg-shinobi = pkgs.callPackage ./nix/package.nix {};
		in {
			inherit xdg-shinobi;
			default = xdg-shinobi;
		});

		devShells = forAllSystems (pkgs: {
			default = pkgs.callPackage ./nix/shell.nix {};
		});
	};
}

{
    description = "A program which checks your $HOME for unwanted files and directories.";

    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    outputs = { self, nixpkgs }: let
        systems = [
            "x86_64-linux"
            "aarch64-linux"
            "aarch64-darwin"
        ];

        forAllSystems = f: nixpkgs.lib.genAttrs systems f;
    in {
        packages = forAllSystems (system: let
            pkgs = import nixpkgs { inherit system; };
            xdg-shinobi = pkgs.callPackage ./package.nix {};
        in {
            inherit xdg-shinobi;
            default = xdg-shinobi;
        });
    };
}

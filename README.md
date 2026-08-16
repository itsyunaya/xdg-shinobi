<div align="center">
    <h1>xdg-shinobi</h1>
</div>

## Overview

`xdg-shinobi` is a project I made out of curiosity and a reimplementation of [xdg-ninja](https://github.com/b3nj5m1n/xdg-ninja), which is a shell script to check your $HOME for unwanted files and directories.

It behaves roughly the same as the original, but with a slightly different user experience.

## Installation
### From Source
Clone the repository and build the project with cargo.
```bash
git clone https://github.com/itsyunaya/xdg-shinobi && cd xdg-shinobi
cargo build --release
```

Optionally, use `cargo install`:
```bash
cargo install --path .
```

### Nix
#### Nix run
```bash
nix run github:itsyunaya/xdg-shinobi
```

#### Installation via Flakes
1. Add the project to your Flake inputs
```nix
inputs = {
    xdg-shinobi = {
        url = "github:itsyunaya/xdg-shinobi";
        inputs.nixpkgs.follows = "nixpkgs";
    };
};
```

2. Add it to your package list
```nix
environment.systemPackages = with pkgs; [
    inputs.xdg-shinobi.packages.${pkgs.stdenv.hostPlatform.system}.default
]; 
```

3. Rebuild

## Usage

When ran without any arguments or environment variables, `xdg-shinobi` will simply scan your $HOME directory for potentially unwanted data.

For more information, you can run `xdg-shinobi -h`.

It is possible to override the list of programs `xdg-shinobi` checks against, by providing the `XN_PROGRAMS_DIR` environment variable with a valid path to a directory of JSON files following the correct [schema](https://github.com/b3nj5m1n/xdg-ninja/blob/main/json-schema/program.json).

If it is desired to simply append new programs to the scan list instead of overriding the old one entirely, the `XN_APPEND_PROGRAMS` environment variable can be set (to anything). 

## Comparison to xdg-ninja

Pros:
- Considerably faster
- No external dependencies needed

Cons:
- Possibly bug-prone due to being a newer project
- Not packaged for any platform besides Nix
- Larger binary size

## Why remake xdg-ninja?

Although `xdg-ninja` is an amazing program, I feel like the fact that it is implemented entirely as a shell script holds it back in a way. It's a bit slow, requires external dependencies to some extent, and just doesn't offer the flexibility I'd want from it.

Also noticeable might be the language I chose to implement it in, which is Rust. Although there is a movement centered around rewriting existing programs and tools in Rust just for the sake of it, I personally chose it because it's the language I'm most comfortable with.

## Additional info

The data that's used to source information about XDG-noncompliant programs has been taken from upstream `xdg-ninja` verbatim, and is the only directly forked part of it.

In case of any issues or complaints about this program, please refrain from contacting upstream, and rather open an issue in this repository.
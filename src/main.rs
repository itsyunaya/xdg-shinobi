#![allow(clippy::needless_return)]

use clap::Parser;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};
use termimad::crossterm::style::Stylize;

enum XdgEnvVars {
    DataHome,
    ConfigHome,
    StateHome,
    CacheHome,
    RuntimeDir,
}

impl XdgEnvVars {
    fn as_str_with_rec(&self) -> (&'static str, &'static str) {
        match self {
            XdgEnvVars::DataHome => ("XDG_DATA_HOME", "$HOME/.local/share"),
            XdgEnvVars::ConfigHome => ("XDG_CONFIG_HOME", "$HOME/.config"),
            XdgEnvVars::StateHome => ("XDG_STATE_HOME", "$HOME/.local/state"),
            XdgEnvVars::CacheHome => ("XDG_CACHE_HOME", "$HOME/.cache"),
            XdgEnvVars::RuntimeDir => ("XDG_RUNTIME_DIR", "/run/user/$UID"),
        }
    }

    // this is subpar, since if i ever wanna add another variable i'll have to do it in two places.
    // the fact that drift could occur can be partially mitigated by a compiletime check via
    // std::mem::variant_count<XdgEnvVars>(), but that function is not yet available in stable rust
    const ALL: [XdgEnvVars; 5] = [
        XdgEnvVars::DataHome,
        XdgEnvVars::ConfigHome,
        XdgEnvVars::StateHome,
        XdgEnvVars::CacheHome,
        XdgEnvVars::RuntimeDir,
    ];

    fn iter() -> impl Iterator<Item = XdgEnvVars> {
        Self::ALL.into_iter()
    }

    pub fn check_vars() {
        XdgEnvVars::iter().for_each(|xdg| {
            let (x, r) = xdg.as_str_with_rec();
            if env::var(x).is_err() {
                let msg_warn = format!("The ${x} environment variable is not set, make sure to add it to your shell's configuration before setting any of the other environment variables!");
                let msg_rec = format!("     ⤷ The recommended value is: {r}");

                println!("{}", msg_warn.cyan());
                println!("{}", msg_rec.cyan());
            }
        });

        println!();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Spec {
    pub name: String,
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileEntry {
    pub path: String,
    pub movable: bool,
    pub help: String,
}

#[derive(RustEmbed)]
#[folder = "programs/"]
struct Programs;

fn get_progs_inbuilt(p: &mut Vec<Spec>) {
    Programs::iter().for_each(|entry| {
        let content = Programs::get(&entry).unwrap();
        p.push(serde_json::from_slice(&content.data).unwrap());
    });
}

fn get_progs_user(p: &mut Vec<Spec>, x: &String) -> Result<(), std::io::Error> {
    // we cannot use unwrap here because we don't know if the user passed program directory
    // actually contains valid files. in that case, we instead exit with an error
    Path::new(&x).read_dir()?.flatten().for_each(|entry| {
        let path = entry.path();

        if !(path.ends_with(".json") || path.ends_with(".jsonc")) {
            return;
        }

        let content = fs::read_to_string(&path).unwrap_or_else(|_| {
            eprintln!("Couldn't read files in provided directory '{x}'.");
            std::process::exit(1);
        });
        p.push(serde_json::from_str(&content).unwrap_or_else(|_| {
            eprintln!(
                "Couldn't parse file {} in provided directory '{}'. Make sure it is valid JSON.",
                path.display(),
                x
            );
            std::process::exit(1);
        }))
    });

    return Ok(());
}

fn check_programs(vars: Vars, args: Args) -> Result<(), std::io::Error> {
    let progs: Vec<Spec> = {
        let mut p: Vec<Spec> = Vec::new();

        if let Some(x) = vars.xn_prog_dir {
            get_progs_user(&mut p, &x)?;
            #[rustfmt::skip]
            if vars.xn_append_progs.is_some() { get_progs_inbuilt(&mut p) };
        } else {
            get_progs_inbuilt(&mut p);
        }

        p
    };

    fs::read_dir(&vars.home_dir)?.flatten().for_each(|entry| {
        let path = entry.path().to_string_lossy().to_string();
        let subpath = path
            .strip_prefix(format!("{}/", vars.home_dir).as_str())
            .unwrap();

        let spec = progs.iter().find(|&q| {
            q.files.iter().any(|f| {
                f.path == format!("$HOME/{}", subpath)
                    || f.path == format!("$HOME/{}", subpath.to_lowercase())
            })
        });

        if let Some(spec) = spec {
            let file: &FileEntry = spec
                .files
                .iter()
                .find(|&f| f.path == format!("$HOME/{}", subpath))
                .unwrap();

            if args.skip_unsupported && !file.movable {
                return;
            }

            let spec_name = spec.name.clone().to_string();

            #[rustfmt::skip]
            println!(
                "[{}]: {}",
                if file.movable { spec_name.red() } else { spec_name.yellow() },
                termimad::inline(&format!("**{}**", file.path))
            );

            println!();

            termimad::print_text(&file.help);

            println!();
        }
    });

    return Ok(());
}

struct Vars {
    home_dir: String,
    xn_prog_dir: Option<String>,
    xn_append_progs: Option<String>,
}

impl Vars {
    pub fn new(
        home_dir: String,
        xn_prog_dir: Option<String>,
        xn_append_progs: Option<String>,
    ) -> Self {
        Vars {
            home_dir,
            xn_prog_dir,
            xn_append_progs,
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// Don't display anything for files that do not have fixes available
    #[clap(long)]
    skip_unsupported: bool,
}

fn main() {
    let args = Args::parse();

    XdgEnvVars::check_vars();

    println!("Starting to check your {}.", "$HOME".cyan());
    println!();

    let vars = Vars::new(
        env::var("HOME").expect("Should've been able to read $HOME environment variable"),
        env::var("XN_PROGRAMS_DIR").ok(),
        env::var("XN_APPEND_PROGRAMS").ok(),
    );

    if let Err(e) = check_programs(vars, args) {
        eprintln!("{e}");
    }

    println!("Finished checking your {}.", "$HOME".cyan());
    println!();
    println!(
        "If you have files in your {} that shouldn't be there, but weren't recognised by xdg-shinobi, please consider creating a configuration file for it and opening a pull request on xdg-ninja's GitHub.",
        "$HOME".dark_cyan()
    );
}

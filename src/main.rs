use clap::{Parser, Subcommand};
use colored::Colorize;
use girgen::generator::{Error, Event, typescript};
use girgen::{default_dirs, girgen};
use std::{ffi, path, process, sync};

static VERBOSE: sync::OnceLock<bool> = sync::OnceLock::new();

fn stem(path: &path::Path) -> &str {
    path.file_stem()
        .and_then(ffi::OsStr::to_str)
        .expect("valid utf8 file name")
}

fn on_event(event: Event) {
    if !VERBOSE.get().unwrap_or(&true) {
        return;
    }

    match event {
        Event::Parsed { file_path } => {
            eprintln!(
                "{}: {} {}",
                "   parsed".green(),
                stem(file_path),
                file_path.display().to_string().black()
            );
        }
        Event::ParseFailed { file_path, err } => {
            eprintln!(
                "{}: could not parse {} {} {}",
                "   failed".red(),
                stem(file_path),
                file_path.display().to_string().black(),
                err,
            );
        }
        Event::Ignored { file_path, cause } => {
            eprintln!(
                "{}: {}{} {}",
                "  ignored".yellow(),
                cause,
                stem(file_path),
                file_path.display().to_string().black(),
            );
        }
        Event::Failed { repo, err } => match repo {
            Some(repo) => {
                eprintln!("{}: failed to render {} {}", "error".red(), repo, err);
            }
            None => {
                eprintln!("{}: {}", "error".red(), err);
            }
        },
        Event::Generated { repo, out_path } => {
            eprintln!("{}: {} {}", "generated".green(), repo, out_path.black());
        }
        Event::CacheHit { repo, out_path } => {
            eprintln!("{}: {} {}", "cache hit".green(), repo, out_path.black());
        }
        Event::Warning { warning } => {
            eprintln!("{}: {}", "warning".yellow(), warning);
        }
    }
}

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Avoid logging debug statements
    #[arg(short, long, default_value_t = false)]
    silent: bool,

    /// Extra directories for .gir lookup
    #[arg(short, long, value_name = "PATHS", default_values_os_t = default_dirs())]
    dirs: Vec<path::PathBuf>,

    /// Skip rendering by name and version, e.g "Gtk-4.0"
    #[arg(short, long, value_name = "NAMESPACE")]
    ignore: Vec<String>,

    #[command(subcommand)]
    command: Language,
}

#[derive(Subcommand)]
enum Language {
    /// Generate annotations for TypeScript
    Typescript {
        /// Target directory to generate to
        #[arg(short, long, value_name = "PATH", default_value = "./.types/gi")]
        outdir: String,

        /// Generate non versioned import aliases
        #[arg(short, long)]
        alias: bool,
    },
}

fn main() -> process::ExitCode {
    let cli = Cli::parse();
    let mut dirs = [cli.dirs, default_dirs()].concat();
    dirs.dedup();

    if cli.silent {
        VERBOSE.set(false).unwrap();
    }

    let res = match cli.command {
        Language::Typescript { outdir, alias } => girgen(girgen::Args {
            dirs,
            ignore: cli.ignore,
            outdir,
            on_event,
            generator: typescript::TypeScript { alias },
        }),
    };

    match res {
        Ok(_) => process::ExitCode::SUCCESS,
        Err(Error::Empty) => {
            eprintln!("nothing to generate");
            process::ExitCode::FAILURE
        }
        Err(Error::FsError(err)) => {
            eprintln!("{}", err);
            process::ExitCode::FAILURE
        }
    }
}

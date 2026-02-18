mod parser;

pub mod generator;
pub use parser::{element, error, parse_gir};

use generator::{Error, Event, Generator, Gir};
use rayon::prelude::*;
use std::{env, fs, path};

pub fn default_dirs() -> String {
    let data_dirs = match env::var("XDG_DATA_DIRS") {
        Ok(dirs) => dirs,
        Err(_) => return "".to_string(),
    };

    let mut dirs = data_dirs
        .split(":")
        .filter_map(|path| {
            // ignore nix path as this is a side effect
            if path == "/run/current-system/sw/share" {
                return None;
            }
            let name = format!("{}/gir-1.0", &path);
            let gir_path = path::Path::new(&name);
            match gir_path.exists() && gir_path.is_dir() {
                true => Some(name),
                false => None,
            }
        })
        .collect::<Vec<_>>();

    dirs.sort();
    dirs.dedup();
    dirs.join(":")
}

pub struct Args<'a, T> {
    pub dirs: &'a [&'a path::Path],
    pub outdir: &'a str,
    pub ignore: &'a [&'a str],
    pub event: fn(Event),
    pub generator: Generator<T>,
}

pub fn girgen<T: Sync>(opts: T, args: &Args<T>) -> Result<(), Error> {
    let mut gir_paths = args
        .dirs
        .iter()
        .filter_map(|path| {
            if path.exists() && path.is_dir() {
                path.read_dir().ok()
            } else {
                None
            }
        })
        .flat_map(|dir| {
            dir.filter_map(Result::ok)
                .map(|file| file.path())
                .filter(|path| matches!(path.extension().and_then(|ext| ext.to_str()), Some("gir")))
        })
        .collect::<Vec<_>>();

    gir_paths.sort();
    gir_paths.dedup();

    gir_paths.retain({
        let mut uniq = std::collections::HashSet::new();

        move |path| {
            path.file_stem().is_some_and(|name| {
                let is_new = uniq.insert(name.to_owned());
                let ignore = args.ignore.iter().any(|ignore| **ignore == *name);
                let keep = is_new && !ignore;
                if !keep {
                    (args.event)(Event::Ignored {
                        file_path: path,
                        cause: (if !is_new { "duplicate " } else { "" }),
                    })
                }
                keep
            })
        }
    });

    let girs: Vec<Gir> = gir_paths
        .par_iter()
        .filter_map(|path| {
            let contents = match fs::read_to_string(path) {
                Ok(ok) => ok,
                Err(err) => {
                    (args.event)(Event::ParseFailed {
                        file_path: path,
                        err: err.to_string().as_str(),
                    });
                    return None;
                }
            };

            match parse_gir::parse(&contents) {
                Ok(repo) => {
                    (args.event)(Event::Parsed { file_path: path });
                    Some(Gir {
                        name: path.file_stem().and_then(|f| f.to_str()).unwrap(),
                        repo,
                        contents,
                    })
                }
                Err(err) => {
                    (args.event)(Event::ParseFailed {
                        file_path: path,
                        err: err.to_string().as_str(),
                    });
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    (args.generator)(opts, &girs, args.outdir, args.event)
}

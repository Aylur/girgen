pub(in crate::generator) mod cache;
pub mod typescript;

use std::{io, path};

pub enum Event<'a> {
    Parsed {
        file_path: &'a path::Path,
    },
    ParseFailed {
        file_path: &'a path::Path,
        err: &'a str,
    },
    Warning {
        warning: &'a str,
    },
    Failed {
        repo: Option<&'a str>,
        err: &'a str,
    },
    Generated {
        repo: &'a str,
        out_path: &'a str,
    },
    CacheHit {
        repo: &'a str,
        out_path: &'a str,
    },
}

pub enum Error {
    Empty,
    FsError(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::FsError(value)
    }
}

pub trait Generator {
    const NAME: &'static str;

    fn generate(gir_paths: &[&path::Path], outdir: &str, event: fn(Event)) -> Result<(), Error>;
}

use super::{super::cache, gjs_lib};
use crate::generator::{Error, Event, Gir};
use rayon::prelude::*;
use std::{collections, fs};

pub struct Opts {
    pub short_paths: bool,
}

fn unique_girs<'a>(items: &'a [Gir]) -> Vec<(&'a str, &'a str)> {
    let names = items
        .iter()
        .filter_map(|gir| gir.name.rsplit_once('-'))
        .collect::<Vec<_>>();

    let mut counts: collections::HashMap<&str, usize> = collections::HashMap::new();
    for (name, _) in names.iter() {
        *counts.entry(name).or_insert(0) += 1;
    }

    let mut out = Vec::new();
    for (name, version) in names {
        if matches!(counts.get(name), Some(1)) {
            out.push((name, version));
        }
    }

    out
}

pub fn generate(opts: &Opts, girs: &[Gir], outdir: &str, event: fn(Event)) -> Result<(), Error> {
    if girs.is_empty() {
        return Err(Error::Empty);
    }

    fs::create_dir_all(outdir)?;

    let repos = girs.iter().map(|gir| &gir.repo).collect::<Vec<_>>();

    let imports = girs
        .par_iter()
        .filter_map(|gir| {
            let hash = cache::hash("ts_", gir.name, &gir.contents);
            let cache_path = cache::lookup_cache(&hash);
            let out_path = format!("{}/{}.d.ts", outdir, gir.name);

            if let Some(path) = cache_path {
                match fs::read_to_string(path) {
                    Err(err) => event(Event::Warning {
                        warning: err.to_string().as_str(),
                    }),
                    Ok(result) => match fs::write(&out_path, result) {
                        Err(err) => event(Event::Warning {
                            warning: err.to_string().as_str(),
                        }),
                        Ok(_) => {
                            event(Event::CacheHit {
                                repo: gir.name,
                                out_path: &out_path,
                            });
                            return Some(gir.name);
                        }
                    },
                }
            }

            let result = match gir.repo.generate_dts(&repos, event) {
                Ok(result) => result,
                Err(err) => {
                    event(Event::Failed {
                        repo: Some(gir.name),
                        err: err.as_str(),
                    });
                    return None;
                }
            };

            if let Err(err) = cache::cache(&hash, &result) {
                event(Event::Warning {
                    warning: err.to_string().as_str(),
                })
            }

            match fs::write(&out_path, &result) {
                Err(err) => {
                    event(Event::Failed {
                        repo: Some(gir.name),
                        err: err.to_string().as_str(),
                    });
                    None
                }
                Ok(_) => {
                    event(Event::Generated {
                        repo: gir.name,
                        out_path: &out_path,
                    });
                    Some(gir.name)
                }
            }
        })
        .chain(gjs_lib::GJS_LIBS.par_iter().filter_map(|lib| {
            let path = format!("{}/{}.d.ts", outdir, lib.name);
            if let Err(err) = fs::write(&path, lib.content) {
                event(Event::Failed {
                    repo: Some(lib.name),
                    err: err.to_string().as_str(),
                });
                None
            } else {
                event(Event::CacheHit {
                    repo: lib.name,
                    out_path: &path,
                });
                Some(lib.name)
            }
        }))
        .collect::<Vec<_>>();

    let aliases = if opts.short_paths {
        unique_girs(girs)
            .iter()
            .map(|(name, version)| {
                minijinja::context! {
                    name,
                    version,
                }
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let index = minijinja::Environment::new()
        .render_str(
            include_str!("./templates/index.jinja"),
            minijinja::context! { imports, aliases },
        )
        .unwrap();

    let index_path = format!("{}/index.d.ts", outdir);
    fs::write(&index_path, index)?;
    event(Event::Generated {
        repo: "index",
        out_path: index_path.as_str(),
    });

    let package_path = format!("{}/package.json", outdir);
    fs::write(&package_path, include_str!("./gjs_lib/package.json"))?;
    event(Event::CacheHit {
        repo: "package",
        out_path: package_path.as_str(),
    });

    Ok(())
}

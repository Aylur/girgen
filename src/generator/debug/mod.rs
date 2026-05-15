use crate::element::Repository;
use crate::generator::{Error, Event, Generator, Gir};

pub struct Debug;

impl Generator for Debug {
    fn generate(&self, girs: &[Gir], _: fn(Event)) -> Result<(), Error> {
        if girs.is_empty() {
            return Err(Error::Empty);
        }

        let repos: Vec<&Repository> = girs.iter().map(|gir| &gir.repo).collect();
        println!("{:#?}", repos);

        Ok(())
    }
}

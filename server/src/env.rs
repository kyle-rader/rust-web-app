use std::{
    env::{self, VarError},
    fs::File,
    io::{self, Read},
};

use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse line: '{0}'")]
    Parse(String),

    #[error(transparent)]
    Io(#[from] io::Error),
}

pub fn load_dot_env() -> Result<(), Error> {
    let cwd = env::current_dir()?;

    let dot_env = cwd.join(".env");

    if dot_env.exists() && dot_env.is_file() {
        let mut file = File::open(&dot_env)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        content
            .lines()
            .map(parse_env_line)
            .for_each(|line_result| match line_result {
                Ok((k, v)) => {
                    if let Err(VarError::NotPresent) = env::var(&k) {
                        info!("Loading {k}={v} from dot env file");
                        env::set_var(k, v);
                    }
                }
                Err(e) => warn!("{e}"),
            });
    }

    Ok(())
}

fn parse_env_line(line: impl AsRef<str>) -> Result<(String, String), Error> {
    let line = line.as_ref();
    if let Some((k, v)) = line.split_once('=') {
        Ok((k.to_owned(), v.to_owned()))
    } else {
        Err(Error::Parse(line.to_owned()))
    }
}

#[cfg(test)]
mod test {
    use super::parse_env_line;

    static DOT_ENV: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));

    #[test]
    fn parse_db() {
        let line = DOT_ENV
            .lines()
            .filter(|l| l.starts_with("DATABASE"))
            .take(1)
            .next()
            .expect("There should be a line starting with DATABASE in the .env file");

        let subject = parse_env_line(line).expect("should parse");
        assert_eq!(
            subject,
            (
                "DATABASE_URL".to_owned(),
                "postgres://pgdev:pgdev@localhost:5432/dev?sslmode=disable".to_owned()
            )
        );
    }
}

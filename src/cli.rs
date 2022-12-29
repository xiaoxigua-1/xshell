use clap::Parser;
use std::{error::Error, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "xshell")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// set environment variable
    #[arg(short = 'e', long = "env", value_parser = parse_key_val::<String, String>)]
    envs: Vec<(String, String)>,

    /// set config file path
    #[arg(long = "config")]
    config: Option<PathBuf>,

    /// run script
    #[arg(last = true)]
    path: Vec<PathBuf>,

    /// run the given commands and then exit
    #[arg(short = 'c', long)]
    command: Option<String>,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

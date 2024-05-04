use clap::{Args, Parser, Subcommand, ValueEnum};
use semver::Version;
use serde::Serialize;
use serde_json::json;
use std::{collections::HashMap, process::Stdio};
use toml::{self, map::Map};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Bump(BumpArgs),
    Parse(ParseArgs),
}

#[derive(Args)]
struct BumpArgs {
    segment: Segments,
    tag: Option<String>,
    #[arg(short, long)]
    strict: bool,
}

#[derive(Args)]
struct ParseArgs {
    tag: Option<String>,
    #[arg(long, short, default_value_t=Formats::Json)]
    format: Formats,
    #[arg(short, long)]
    strict: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Segments {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Formats {
    Json,
    Toml,
}

impl ToString for Formats {
    fn to_string(&self) -> String {
        match self {
            Self::Json => "json".to_owned(),
            Self::Toml => "toml".to_owned(),
        }
    }
}

trait Bump {
    fn bump_patch(&mut self);
    fn bump_minor(&mut self);
    fn bump_major(&mut self);
}

impl Bump for Version {
    fn bump_patch(&mut self) {
        self.patch += 1;
    }

    fn bump_minor(&mut self) {
        self.patch = 0;
        self.minor += 1;
    }

    fn bump_major(&mut self) {
        self.patch = 0;
        self.minor = 0;
        self.major += 1;
    }
}

fn parse(mut tag: String, strict: bool) -> (Result<semver::Version, semver::Error>, bool) {
    let mut had_prefix = false;
    if !strict {
        if tag.chars().next().unwrap() == 'v' {
            tag = tag[1..].to_owned();
            had_prefix = true;
        }
    }
    (Version::parse(&tag), had_prefix)
}

fn vers2json(vers: Version) -> String {
    json!({"major": vers.major, "minor": vers.minor, "patch": vers.patch, "pre": vers.pre.to_string(), "build": vers.build.to_string()})
        .to_string()
}

fn vers2toml(vers: Version) -> String {
    let mut dict = Map::new();
    dict.insert("major".to_owned(), toml::Value::Integer(vers.major as i64));
    dict.insert("minor".to_owned(), toml::Value::Integer(vers.minor as i64));
    dict.insert("patch".to_owned(), toml::Value::Integer(vers.patch as i64));
    dict.insert("pre".to_owned(), toml::Value::String(vers.pre.to_string()));
    dict.insert(
        "build".to_owned(),
        toml::Value::String(vers.build.to_string()),
    );
    dict.to_string()
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Bump(BumpArgs {
            segment,
            tag,
            strict,
        }) => {
            let tag = tag.unwrap();
            let (sv, had_prefix) = parse(tag, strict);
            let mut sv = sv.unwrap();
            match segment {
                Segments::Major => sv.bump_major(),
                Segments::Minor => sv.bump_minor(),
                Segments::Patch => sv.bump_patch(),
            }
            if had_prefix {
                println!("v{}", sv);
            } else {
                println!("{}", sv);
            }
        }
        Commands::Parse(ParseArgs {
            tag,
            format,
            strict,
        }) => {
            let sv = parse(tag.unwrap(), strict).0.unwrap();
            let s = match format {
                Formats::Json => vers2json(sv),
                Formats::Toml => vers2toml(sv),
            };
            println!("{}", s);
        }
    }
}

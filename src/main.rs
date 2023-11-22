extern crate clap;
extern crate rand;

use clap::Parser;
use rand::seq::IteratorRandom;
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    time::Duration,
};

const SLUG_CHARS: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

type Result<T> = core::result::Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t=String::from("127.0.0.1") )]
    domain: String,

    #[arg(short, long, default_value_t = 4)]
    slug_size: u8,

    #[arg(short, long, default_value_t = 30000)]
    buffer_size: usize,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long)]
    port: u16,
}

fn create_slug(size: u8) -> String {
    let mut rand_gen = rand::thread_rng();
    SLUG_CHARS
        .chars()
        .choose_multiple(&mut rand_gen, size.into())
        .iter()
        .collect()
}

fn save_content(dir: PathBuf, content: &[u8]) -> Result<()> {
    create_dir_all(&dir)?;

    let mut file = File::create(dir.join("index.txt"))?;
    file.write_all(content)?;

    Ok(())
}

fn handle_tcp(
    stream: &mut TcpStream,
    buf_limit: usize,
    slug_size: u8,
    paste_path: &PathBuf,
    domain: &String,
) -> Result<()> {
    stream.set_read_timeout(Some(Duration::new(5, 0)))?;
    let mut buffer: Vec<u8> = Vec::new();

    if stream.read_to_end(&mut buffer).unwrap_or(0) > buf_limit {
        return Ok(());
    }

    let slug = create_slug(slug_size);
    save_content(paste_path.join(&slug), buffer.as_slice())?;
    stream.write(format!("http://{domain}/{slug}\n").as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args.port.to_string())?;

    for stream in listener.incoming() {
        handle_tcp(
            &mut stream?,
            args.buffer_size,
            args.slug_size,
            &args.output,
            &args.domain,
        )?;
    }
    Ok(())
}

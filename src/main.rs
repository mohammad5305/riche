extern crate clap;
extern crate log;
extern crate rand;
extern crate rouille;
extern crate std_logger;

use clap::Parser;
use log::info;
use rand::seq::IteratorRandom;
use rouille::{router, Response};
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    path::PathBuf,
    thread,
    time::Duration,
};

const SLUG_CHARS: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

type Result<T> = core::result::Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    domain: Option<String>,

    #[arg(short, long, default_value_t = 4, value_name = "NUM")]
    slug_size: u8,

    #[arg(short, long, default_value_t = 30000, value_name = "NUM")]
    buffer_size: usize,

    #[arg(short, long, help = "Pastes directory", value_name = "PATH")]
    output: PathBuf,

    #[arg(short, long)]
    port: u16,

    #[arg(
        short,
        long,
        help = "Runs a simple HTTP server for showing pastes on this port (not recommended)",
        value_name = "PORT"
    )]
    webserver: Option<u16>,
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
    info!("Incoming connection from: {}", stream.peer_addr()?.ip());
    stream.set_read_timeout(Some(Duration::new(5, 0)))?;
    let mut buffer: Vec<u8> = Vec::new();

    if stream.read_to_end(&mut buffer).unwrap_or(buffer.len()) > buf_limit {
        return Ok(());
    }

    let slug = create_slug(slug_size);
    save_content(paste_path.join(&slug), buffer.as_slice())?;
    stream.write(format!("http://{domain}/{slug}\n").as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    std_logger::Config::logfmt()
        .with_call_location(false)
        .init();

    let args = Cli::parse();
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), args.port))?;
    info!("Server started listening on: 0.0.0.0:{}", args.port);

    if let Some(port) = args.webserver {
        let output_dir = args.output.clone();
        thread::spawn({
            move || {
                rouille::start_server(
                    SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port),
                    move |request| {
                        router!(request,
                        (GET) (/{slug: String}) => {
                            if let Ok(file) = File::open(output_dir.join(slug + "/index.txt")) {
                                Response::from_file("text/plain", file)
                            }
                            else {
                                Response::empty_404()
                            }
                        },
                        _ => Response::empty_404(),
                        )
                    },
                );
            }
        });
    }
    let domain = &args
        .domain
        .unwrap_or(format!("127.0.0.1:{}", &args.webserver.unwrap_or(8080)));

    for stream in listener.incoming() {
        handle_tcp(
            &mut stream?,
            args.buffer_size,
            args.slug_size,
            &args.output,
            domain,
        )?;
    }
    Ok(())
}

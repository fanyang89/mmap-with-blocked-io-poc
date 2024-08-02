use core::time;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::{fs::File, path::PathBuf, thread::sleep, time::Instant};

use clap::{Parser, Subcommand};
use memmap::MmapOptions;
use tracing::info;
use tracing_subscriber::{fmt::time::ChronoLocal, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[arg(short, long, default_value = "1000ms")]
    interval: humantime::Duration,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Allocate memory use mmap(2)
    Alloc,

    /// Read path
    Read {
        #[arg(short, long)]
        file_path: PathBuf,
    },
}

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_timer(ChronoLocal::rfc_3339()))
        .init();
    let Cli { interval, command } = Cli::parse();
    match command {
        Commands::Alloc => loop {
            let now = Instant::now();
            let mut ptr = MmapOptions::new().len(4096).map_anon().unwrap();
            (&mut ptr[..]).write(b"Hello, world!").unwrap();
            ptr.flush().unwrap();
            let elapsed = Instant::now() - now;
            if elapsed > time::Duration::from_millis(1) {
                info!("mmap(2) elapsed: {:?}", elapsed);
            }
            sleep(*interval);
        },

        Commands::Read { file_path } => {
            loop {
                let file = File::open(file_path.clone()).unwrap();

                #[cfg(target_os = "linux")]
                unsafe {
                    let rc =
                        libc::posix_fadvise(file.as_raw_fd(), 0, 4096, libc::POSIX_FADV_DONTNEED);
                    assert_eq!(rc, 0);
                }

                let now = Instant::now();
                let mut acc = 0u64;
                {
                    let m = unsafe { memmap::Mmap::map(&file).unwrap() };
                    for b in m.iter() {
                        acc += *b as u64;
                    }
                    // unmap on drop()
                }
                let elapsed = Instant::now() - now;
                info!("mmap(2) io read elapsed: {:?}, acc: {}", elapsed, acc);
                sleep(*interval);
            }
        }
    }
}

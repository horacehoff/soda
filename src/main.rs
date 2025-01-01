use std::fs::File;
use std::io::Write;
use std::ops::RangeInclusive;
use std::path::Path;
use std::process::{Command, Stdio};
use clap::{Parser, Subcommand};
use colored::Colorize;

const OPTIM_RANGE: RangeInclusive<usize> = 0..=2;

#[derive(Parser, Debug)]
#[command(name = "Soda")]
#[command(version = "B1")]
#[command(about = None)]
#[command(long_about = "A utility that makes Rust feel like an interpreted language.")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(default_value_t = String::from("project"), help = "Filename to execute. Leave empty when running a project (not recommended).")]
    filename: String,

    #[arg(short, long, default_value_t = 0, value_parser=clap::value_parser!(u8).range(0..2), help = "Set optimization level:\n- 0 is not optimized, by default\n- 1 is lightly optimized\n- 2 is heavily optimized")]
    optimized: u8,

    #[arg(long, default_value_t = false, help = "Include debug info in the program")]
    debug: bool,

    #[arg(long, name="rust-update", default_value_t = false, help = "Update Rust and its components")]
    rust_update: bool,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        #[arg(default_value = "new.rs", help = "Filename for the new file")]
        filename: String,
    },
}

macro_rules! std_cfg {
    ($args:expr, $stdout_config:ident, $stderr_config:ident) => {
        let $stdout_config = if $args.verbose {
            // Output in real-time
            Stdio::inherit()
        } else {
            // No output
            Stdio::null()
        };
        let $stderr_config = if $args.verbose {
            // Output in real-time
            Stdio::inherit()
        } else {
            // No output
            Stdio::null()
        };
    };
}

fn main() {
    let args = Args::parse();
    println!("[LOG] PARSED:\n{args:?}");
    if args.rust_update {
        std_cfg!(args, stdout_config, stderr_config);
        let status = Command::new("rustup")
            .arg("update")
            .stdout(stdout_config)
            .stderr(stderr_config)
            .status()
            .expect(&"[SODA] Failed to update Rust".red());
        if status.success() {
            println!("{}", "[SODA] Rust updated successfully!".green());
        } else {
            eprintln!("{} {}", "[SODA] Rust update failed with exit code: {}".red(), status);
        }
    }

    match args.command {
        Some(Commands::New { filename }) => {
            let mut file = File::create(&filename).unwrap();
            file.write_all("fn main() {\nprintln!(\"Hello World!\");\n}".as_ref()).unwrap();
            println!("{} {filename}", "[SODA] Successfully created".green());
        }
        None => {
            // RUN PROJECT
            if args.filename == "project" {

            } else {
                std_cfg!(args, stdout_config, stderr_config);
                let status = Command::new("rustc")
                    .arg(args.filename.to_string())
                    .stdout(stdout_config)
                    .stderr(stderr_config)
                    .status()
                    .expect(&format!("{} {}", "[SODA] Failed to compile".red(), args.filename));
                if !status.success() {
                    eprintln!("{} {}{}{}", "[SODA] Failed to compile".red(), args.filename,".\nExit code: ".red(), status);
                    std::process::exit(1);
                }
                let output_name = "./".to_owned() +Path::new(&args.filename).file_stem().unwrap().to_str().unwrap() + std::env::consts::EXE_EXTENSION;
                Command::new(output_name)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status()
                    .expect(&"[SODA] Failed to execute program".red());
            }
        }
    }
}

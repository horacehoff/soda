use std::fs;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::fs::File;
use std::io::Write;
use std::ops::RangeInclusive;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "Soda")]
#[command(version = "1.0.0")]
#[command(about = None)]
#[command(long_about = "A utility that makes Rust feel like an interpreted language.")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(default_value_t = String::from("project"), help = "Filename to execute. Leave empty when running a cargo project.")]
    filename: String,

    #[arg(short, long, default_value_t = 0, value_parser=clap::value_parser!(u8).range(0..=2), help = "Set optimization level:\n- 0 is not optimized, by default\n- 1 is lightly optimized\n- 2 is heavily optimized\nOn projects, 1 and 2 are the same.")]
    optimized: u8,

    #[arg(
        long,
        default_value_t = false,
        help = "Include debug info in the program. Not supported with projects."
    )]
    debug: bool,

    #[arg(
        long,
        name = "rust-update",
        default_value_t = false,
        help = "Update Rust and its components"
    )]
    rust_update: bool,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Remove all executables from the .soda folder."
    )]
    clean: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(subcommand_help_heading = "Creates a new file with the specified filename (default: new.rs).")]
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
            eprintln!(
                "{} {}",
                "[SODA] Rust update failed with exit code: {}".red(),
                status
            );
        }
    }
    if args.clean {
        fs::remove_dir_all("./.soda").expect(&"[SODA] Failed to remove .soda folder".red());
        println!("{}", "[SODA] Removed .soda folder successfully".green());
    }

    match args.command {
        Some(Commands::New { filename }) => {
            let mut file = File::create(&filename).unwrap();
            file.write_all("fn main() {\nprintln!(\"Hello World!\");\n}".as_ref())
                .unwrap();
            println!("{} {filename}", "[SODA] Successfully created".green());
        }
        None => {
            // RUN PROJECT
            if args.filename == "project" {
                let mut cmd = Command::new("cargo");
                cmd.arg("run")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit());

                let mut dynamic_args:Vec<String> = Vec::new();
                if !args.verbose {
                    dynamic_args.push("-q".parse().unwrap())
                }
                if args.debug {
                    dynamic_args.push("--profile".parse().unwrap());
                    dynamic_args.push("debug".parse().unwrap());
                } else if args.optimized > 0 {
                    dynamic_args.push("--profile".parse().unwrap());
                    dynamic_args.push("release".parse().unwrap());
                }
                cmd.args(dynamic_args);

                let status = cmd.status().expect(&format!(
                    "{} {}",
                    "[SODA] Failed to compile".red(),
                    args.filename
                ));
                if !status.success() {
                    eprintln!(
                        "{} {}{}{}",
                        "[SODA] Failed to compile".red(),
                        args.filename,
                        ".\nExit code: ".red(),
                        status
                    );
                    std::process::exit(1);
                }
            } else {
                std_cfg!(args, stdout_config, stderr_config);
                let mut cmd = Command::new("rustc");
                cmd.arg(args.filename.to_string())
                    .arg("--out-dir")
                    .arg("./.soda")
                    .stdout(stdout_config)
                    .stderr(stderr_config);

                let mut dynamic_args:Vec<String> = Vec::new();
                if args.debug {
                    dynamic_args.push("-Cdebuginfo=2".parse().unwrap())
                }
                if args.optimized == 1 {
                    dynamic_args.push("-Copt-level=1".parse().unwrap())
                } else if args.optimized == 2 {
                    dynamic_args.push("-Copt-level=3".parse().unwrap());
                    dynamic_args.push("-Clto".parse().unwrap())
                }
                cmd.args(dynamic_args);

                let status = cmd.status().expect(&format!(
                    "{} {}",
                    "[SODA] Failed to compile".red(),
                    args.filename
                ));
                if !status.success() {
                    eprintln!(
                        "{} {}{}{}",
                        "[SODA] Failed to compile".red(),
                        args.filename,
                        ".\nExit code: ".red(),
                        status
                    );
                    std::process::exit(1);
                }
                let output_name = "./.soda/".to_owned()
                    + Path::new(&args.filename)
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                    + std::env::consts::EXE_EXTENSION;
                Command::new(output_name)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status()
                    .expect(&"[SODA] Failed to execute program".red());
            }
        }
    }
}

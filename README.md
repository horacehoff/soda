# Soda
> A utility that makes Rust feel like an interpreted language

Soda allows you to use Rust like an interpreted language, e.g. for quick prototyping.
 - To create a new file run `soda new`.
 - To run any rust file, run `soda myfile.rs`. It compiles it using rustc using the specified flags and immediately runs it, providing a Python-like experience. Executables are stored in a local .soda folder. 
 - Run `soda --clean` to remove the .soda folder

```
Soda
Usage: soda [OPTIONS] [FILENAME] [COMMAND]

Commands:
  new          Create a new file with the specified filename (default: new.rs).
  rust-update  Update Rust and its components.
  clean        Delete the cache (.soda) folder.
  help         Print this message or the help of the given subcommand(s)

Arguments:
  [FILENAME]  Filename to execute. Leave empty when running a cargo project. [default: project]

Options:
  -o, --optimized <OPTIMIZED>  Set optimization level:
                               - 0 is not optimized, by default
                               - 1 is lightly optimized
                               - 2 is heavily optimized
                               On projects, 1 and 2 are the same. [default: 0]
      --debug                  Include debug info in the program. Not supported with projects.
  -v, --verbose                
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```
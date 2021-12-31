mod filesystem;
use crate::filesystem::file::FileAttributes;
use crate::filesystem::lib::traverse_directory;
use std::env;
use std::path::Path;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		eprintln!("No path specified");

		process::exit(1);
	}

	let path: &Path = Path::new(&args[1]);

	let mut files: Vec<FileAttributes> = Vec::new();
	let result: Option<()> = traverse_directory(path, &mut files);

	match result {
		Some(_) => {
			for i in &files {
				println!("{}", i.file_path);
			}

		},
		None => {
			// the error was printed in traverse_directory - just exit here
			process::exit(1)
		},
	}
}

mod filesystem;
use crate::filesystem::file::FileAttributes;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

pub fn traverse_directory(root: &Path, files: &mut Vec<FileAttributes>) -> Option<()> {
	if !root.exists() {
		eprintln!("No such path as {}", root.display().to_string());

		return None;
	}

	if !root.is_dir() {
		eprintln!("{} is not a directory", root.display().to_string());

		return None;
	}

	match fs::read_dir(&root) {
		Ok(directory) => {
			for entry in directory {
				// eh, if we've got this far, entry should be fine
				let path = entry.unwrap().path();
				// assume that we can get the metadata and that the file won't just disappear out from under us
				let attributes = fs::metadata(&path).unwrap();

				if attributes.is_dir() {
					traverse_directory(&path, files);
				} else if attributes.is_file() {
					// the path might not convert nicely to a string, but we'll assume that it does
					files.push(FileAttributes {
						file_path: path.to_str().unwrap().to_string(),
						// TODO: get just the file name here
						file_name: "".to_string(),
						size: attributes.len()
					});
				}
			}
		},
		Err(err) => {
			eprintln!("Error in fs::read_dir for {} {}", root.display(), err);

			return None;
		}
	}

	return Some(())
}

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

mod filesystem;
use crate::filesystem::file::FileAttributes;
use crate::filesystem::lib::find_duplicates;
use crate::filesystem::lib::get_files_in_path;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut list_flag: bool = true;
	let mut find_duplicates_flag: bool = false;

	if args.len() < 2 {
		eprintln!("No path specified");

		process::exit(1);
	}

	if args.len() > 2 {
		if args[2].eq("-l") {
			list_flag = true;
		}
		if args[2].eq("-d") {
			list_flag = false;
			find_duplicates_flag = true;
		}
	}

	let path: &Path = Path::new(&args[1]);

	let mut files: Vec<FileAttributes> = Vec::new();
	let result: Option<()> = get_files_in_path(path, &mut files);

	match result {
		Some(_) => {
			if list_flag {
				for file in &files {
					println!("{}", file.file_path);
				}
			}
			if find_duplicates_flag {
				let duplicates: HashMap::<String, Vec<String>> = find_duplicates(&mut files);

				for (duplicated_file, duplicates) in duplicates.iter() {
					println!("{}", duplicated_file);

					for duplicate_file in duplicates {
						println!("{} duplicates {}", duplicate_file, duplicated_file);
					}
				}
			}
		},
		None => {
			// the error was printed by get_files_in_path - just exit here
			process::exit(1)
		},
	}
}

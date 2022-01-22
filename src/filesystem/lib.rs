use crate::filesystem::file::FileAttributes;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use sha2::Digest;
use sha2::Sha256;

pub fn find_duplicates(files: &mut Vec<FileAttributes>) -> HashMap::<String, Vec<String>> {
	let mut duplicates = HashMap::<String, Vec<String>>::new();
	
	for file_index in 0..files.len() {
		for possible_duplicate_index in 0..files.len() {
			let file: &FileAttributes = &files[file_index];
			let possible_duplicate: &FileAttributes = &files[possible_duplicate_index];

			if file.is_duplicated {
				// this file is already a duplicate of another file
				continue;
			}
			if file.file_path.eq(&possible_duplicate.file_path) {
				// if it's the same file, skip it
				continue;
			}
			if file.size == possible_duplicate.size && file.hash.eq(&possible_duplicate.hash) {
				if duplicates.contains_key(&file.file_path) {
					duplicates.get_mut(&file.file_path).unwrap().push(possible_duplicate.file_path.clone());
				}
				else {
					let mut duplicate_names: Vec<String> = Vec::<String>::new();

					duplicate_names.push(possible_duplicate.file_path.clone());
					duplicates.insert(file.file_path.clone(), duplicate_names);
				}
				
				files[possible_duplicate_index].is_duplicated = true;
			}
		}
	}

	return duplicates;
}

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
				let path = entry.ok()?.path();
				let attributes = fs::metadata(&path).ok()?;

				if attributes.is_dir() {
					traverse_directory(&path, files);
				} else if attributes.is_file() {
					files.push(FileAttributes {
						// the path might not convert nicely to a string, but we'll assume that it does
						file_path: path.to_str().unwrap().to_string(),
						// TODO: get just the file name here
						file_name: "".to_string(),
						size: attributes.len(),
						hash: sha256_file(&path),
						is_duplicated: false
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

fn sha256_file(file_path: &Path) -> String {
	let mut file = File::open(file_path).unwrap();
	let mut sha256 = Sha256::new();

	io::copy(&mut file, &mut sha256).unwrap();

	// convert it to a hex string - finalize returns not a string
	return format!("{:x}", sha256.finalize());
}

#[cfg(test)]
mod tests {
	use super::*;

	// Windows only because of OS-specific path separators
	#[cfg(target_os = "windows")]
	#[test]
	fn test_find_duplicates() {
		let path: &Path = Path::new("./test_data");

		let mut files: Vec<FileAttributes> = Vec::new();

		let result: Option<()> = traverse_directory(path, &mut files);

		assert_eq!(true, result.is_some());
		assert_eq!(6, files.len());

		let duplicates: HashMap::<String, Vec<String>> = find_duplicates(&mut files);

		assert_eq!(2, duplicates.keys().len());

		for (duplicate_name, duplicates) in duplicates.iter() {
			println!("{:?}: {:?}", duplicate_name, duplicates.len());
		}

		let asdf_copy_txt_dupes: &Vec<String> = duplicates.get("./test_data\\dir1\\asdf - Copy.txt").unwrap();
		assert_eq!(2, asdf_copy_txt_dupes.len());
		assert_eq!("./test_data\\dir1\\asdf.txt", asdf_copy_txt_dupes[0]);
		assert_eq!("./test_data\\dir2\\asdf.txt", asdf_copy_txt_dupes[1]);
		let hello_txt_dupes: &Vec<String> = duplicates.get("./test_data\\dir2\\dir1\\hello.txt").unwrap();
		assert_eq!(1, hello_txt_dupes.len());
		assert_eq!("./test_data\\dir2\\hello.txt", hello_txt_dupes[0]);
	}

	// Windows only because of OS-specific path separators
	#[cfg(target_os = "windows")]
	#[test]
	fn test_traverse_directory_existing_directory() {
		let path: &Path = Path::new("./src");

		let mut files: Vec<FileAttributes> = Vec::new();

		let result: Option<()> = traverse_directory(path, &mut files);

		assert_eq!(true, result.is_some());
		assert_eq!(4, files.len());

		files.sort_by_key(|file| file.file_path.clone());

		assert_eq!("./src\\filesystem\\file.rs", files[0].file_path);
		assert_eq!("17ad1db5a2818a0fabbf1f20678207fd53ca6cbd5378a06b934e4562d0cc3be3", files[0].hash);
		assert_eq!("./src\\filesystem\\lib.rs", files[1].file_path);
		assert_eq!("./src\\filesystem\\mod.rs", files[2].file_path);
		assert_eq!("b8b70d63bf52a78dd7ce5912bc96e1cf3ab319d437339be339111c789e09ba13", files[2].hash);
		assert_eq!("./src\\main.rs", files[3].file_path);
	}

	#[test]
	fn test_traverse_directory_no_such_directory() {
		let path: &Path = Path::new("./nosuchdir");

		let mut files: Vec<FileAttributes> = Vec::new();

		let result: Option<()> = traverse_directory(path, &mut files);

		assert_eq!(false, result.is_some());
		assert_eq!(0, files.len());
	}

	#[test]
	fn test_traverse_directory_existing_file() {
		let path: &Path = Path::new("./src/main.rs");

		let mut files: Vec<FileAttributes> = Vec::new();

		let result: Option<()> = traverse_directory(path, &mut files);

		assert_eq!(false, result.is_some());
		assert_eq!(0, files.len());
	}
}
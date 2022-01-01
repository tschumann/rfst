use crate::filesystem::file::FileAttributes;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use sha2::Digest;
use sha2::Sha256;

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
						hash: sha256_file(&path)
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
	fn test_traverse_directory_existing_directory() {
		let path: &Path = Path::new("./src");

		let mut files: Vec<FileAttributes> = Vec::new();

		let result: Option<()> = traverse_directory(path, &mut files);

		assert_eq!(true, result.is_some());
		assert_eq!(4, files.len());

		files.sort_by_key(|file| file.file_path.clone());

		assert_eq!("./src\\filesystem\\file.rs", files[0].file_path);
		assert_eq!("6116b2252b2ca3b9a2cfffc8bc8de23bc2063b3f511064cf70994f0ffdeb3262", files[0].hash);
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
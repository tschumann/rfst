use crate::filesystem::file::FileAttributes;
use std::fs;
use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse_directory() {
		let path: &Path = Path::new("./src");

		let mut files: Vec<FileAttributes> = Vec::new();

		let result: Option<()> = traverse_directory(path, &mut files);

		// TODO: filenames will differ here based on the operating system in use - normalise the paths somehow?
		assert_eq!(true, result.is_some());
        assert_eq!(4, files.len());
    }
}
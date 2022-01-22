#[derive(Clone)]
pub struct FileAttributes {
	pub file_path: String,
	pub file_name: String,
	pub size: u64,
	pub hash: String,
	pub is_duplicated: bool,
}

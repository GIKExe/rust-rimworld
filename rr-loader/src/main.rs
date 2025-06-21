use libloading::{Library, Symbol};
use std::fs;

mod FT {
	pub const DIR: &str = "Dir";
	pub const FILE: &str = "File";
	pub const OTHER: &str = "Other";
}

fn main() {
	// загрузчик ищет модули для загрузки
	let Ok(entries ) = fs::read_dir(".") else {
		println!("Указанный путь не является директорией или нет прав чтения"); return;
	};

	for entry in entries {
		println!();
		let Ok(entry) = entry else {continue};
		let Ok(file_type) = entry.file_type() else {continue};
		let path: &str = &entry.path().to_string_lossy().to_string();
		let file_type = {
			if file_type.is_dir() {FT::DIR}
			else if file_type.is_file() {FT::FILE}
			else {FT::OTHER}
		};
		print!("{}: {:?} ", file_type, path);
		if file_type != FT::FILE {continue};
		// windows_only

		if !(path.ends_with(".dll")) {continue};

		let lib: Library;
		let init: Symbol<fn(&str)>;
		unsafe {
			lib = match Library::new(path) {
				Ok(v) => v,
				Err(e) => {print!("{e}"); continue}
			};

			init = match lib.get(b"init") {
				Ok(v) => v,
				Err(e) => {print!("{e}"); continue}
			};
		};

		print!("Loaded");
		init(path);
	}
}

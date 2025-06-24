use libloading::{Library, Symbol};
use serde::Deserialize;
use std::fs::{self};

#[derive(Debug)]
enum LoaderError {
	PathIsNotDir
}

// Определяем структуры данных
#[derive(Deserialize)]
struct Config {
	name: String,
	version: String,
	author: Option<String>,
	load_after: Option<Vec<String>>,
	load_before: Option<Vec<String>>,
	incompatible: Option<Vec<String>>,
	dependencies: Option<Vec<String>>,
}


// в этой функции получаем директории с модами из основной директории
fn get_mods(mods_path: &str) -> Result<Vec<(Config, Library)>, LoaderError> {
	let mut mods = Vec::new();

	let Ok(entries ) = fs::read_dir(mods_path) else {
		return Err(LoaderError::PathIsNotDir);
	};

	// просматриваем директории в указанном месте
	for entry in entries {
		let Ok(entry) = entry else {continue};
		let Ok(file_type) = entry.file_type() else {continue};
		if !file_type.is_dir() {continue};
		let mod_path = entry.path().to_string_lossy().to_string();
		let Ok(dir) = fs::read_dir(mod_path) else {continue};

		let mut config_path = None;
		let mut library_path = None;

		// просматриваем файлы в директории мода
		for entry in dir {
			let Ok(entry) = entry else {continue};
			let Ok(file_type) = entry.file_type() else {continue};
			if !file_type.is_file() {continue};

			let path = entry.path().to_string_lossy().to_string();
			let filename = entry.file_name().to_string_lossy().to_string();
			if filename == "config.toml" { config_path = Some(path); continue };
			if filename == "main.rr" { library_path = Some(path); continue };
		};

		// проверяем пути
		if config_path.is_none() | library_path.is_none() {continue};
		let Some(config_path) = config_path.clone() else {continue};
		let Some(library_path) = library_path.clone() else {continue};

		// читаем конфиг
		let Ok(content) = fs::read_to_string(config_path) else {continue};
		let Ok(config) = toml::from_str::<Config>(&content) else {continue};

		// загружаем либу
		let Ok(lib) = (unsafe { Library::new(library_path) }) else {continue};

		mods.push((config, lib));
	};

	Ok(mods)
}

fn main() {
	println!("Начинаю поиск...");
	let mods = match get_mods("./mods") {
		Ok(v) => v,
		Err(e) => {println!("Ошибка: {e:?}"); return}
	};
	if mods.is_empty() {println!("Модов нет."); return}
	println!("Моды найдены, вызываю init()...");
	for (mut config, lib) in mods {
		println!("Имя: {}", config.name);
		if config.author.is_none() {
			config.author = Some("unknown".to_string())
		}
		println!("Автор: {}", config.author.unwrap());

		let Ok(init) = (unsafe {lib.get::<fn(&str)>(b"init")}) else {continue};
		init(&config.name)
	}
}

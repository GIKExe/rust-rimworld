
#[unsafe(no_mangle)]
pub extern "Rust" fn init(path: &str) {
	println!("Hello From {path}!")
}
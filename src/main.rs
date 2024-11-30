use std::env;
use std::fs;
use std::path::Path;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = Path::new(&args[1]);
    let target_location_path = Path::new(&args[2]);
    let filename = file_path.file_name().unwrap().to_str().unwrap().to_string();

    if target_location_path.is_file() {
        panic!("Is a file")

    } else {
        let copied_file = target_location_path.join(filename);

        fs::copy(file_path, copied_file)?;
        Ok(())
    }
}
use glob::glob;
use std::fs;
use std::io::Write;

fn main() {
    // Get all the files in the src/bin folder and its subfolders
    let files = glob("src/bin/**/*.rs").expect("Failed to read glob pattern");

    // Create a new Cargo.toml file with the bin section
    let mut cargo_toml = fs::File::create("Cargo.toml").expect("Failed to create Cargo.toml");
    cargo_toml.write_all(b"[package]
name = \"learning-rust\"
version = \"0.1.0\"
edition = \"2021\"
build = \"build.rs\"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
glob = \"0.3.1\"

[build-dependencies]
glob = \"0.3.1\"
").expect("Failed to write to Cargo.toml");

    // Loop through the files and append them to the bin section
    for entry in files {
        match entry {
            Ok(path) => {
                // Get the file name without the extension
                let name = path.file_stem().unwrap().to_str().unwrap();
                // Get the file path as a string
                let path = path.to_str().unwrap().replace("\\", "/");
                // Write the name and path to the bin section
                cargo_toml
                    .write_all(
                        format!("\n[[bin]]\nname = \"{}\"\npath = \"{}\"\n\n", name, path).as_bytes(),
                    )
                    .expect("Failed to write to Cargo.toml");
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

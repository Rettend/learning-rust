use glob::glob;
use std::fs;
use std::io::Write;

fn main() {
    // Get all the files in the src/bin folder and its subfolders
    let files = glob("src/bin/**/*.rs").expect("Failed to read glob pattern");

    // Create a new Cargo.toml file with the bin section
    let mut cargo_toml = fs::File::create("Cargo.toml").expect("Failed to create Cargo.toml");
    cargo_toml.write_all(b"
    [package]\n
    name = \"learning-rust\"\n
    version = \"0.1.0\"\n
    edition = \"2021\"\n
    build = \"build.rs\"\n
    \n
    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n
    \n
    [[bin]]\n
    name = \"chapter1\"\n
    path = \"src/bin/1. Getting Started/2_hello_world.rs\"\n
    \n
    [dependencies]\n
    glob = \"0.3.1\"\n
    \n
    [build-dependencies]\n
    glob = \"0.3.1\"\n
    ").expect("Failed to write to Cargo.toml");

    // Loop through the files and append them to the bin section
    for entry in files {
        match entry {
            Ok(path) => {
                // Get the file name without the extension
                let name = path.file_stem().unwrap().to_str().unwrap();
                // Get the file path as a string
                let path = path.to_str().unwrap();
                // Write the name and path to the bin section
                cargo_toml
                    .write_all(
                        format!("[[bin]]\nname = \"{}\"\npath = \"{}\"\n\n", name, path).as_bytes(),
                    )
                    .expect("Failed to write to Cargo.toml");
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

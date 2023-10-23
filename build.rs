use glob::glob;
use std::fs;
use std::io::Write;

fn main() {
    // Get all the files in the src/bin folder and its direct subfolders
    let files = glob("src/bin/*/*.rs").expect("Failed to read glob pattern");

    // Read the contents of the original Cargo.toml file
    let original = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let mut original = original.split("\n").collect::<Vec<&str>>();

    // Create a new Cargo.toml file with the same contents as the original, but without the bin section
    let mut cargo_toml = fs::File::create("Cargo.toml").expect("Failed to create Cargo.toml");
    for line in original.iter_mut() {
        if line == &"[[bin]]" {
            break;
        } else {
            cargo_toml
                .write_all(format!("{}\n", line).as_bytes())
                .expect("Failed to write to Cargo.toml");
        }
    }

    // Loop through the files and append them to the bin section
    for entry in files {
        match entry {
            Ok(path) => {
                // Get the file name without the extension
                let name = path.file_stem().unwrap().to_str().unwrap();

                // Get the file path as a string
                let path = path.to_str().unwrap().replace("\\", "/");

                // Get just the number from the folder name
                let number = path
                    .split("/")
                    .nth(2)
                    .unwrap()
                    .split(".")
                    .nth(0)
                    .unwrap()
                    .to_string();

                // Write the name and path to the bin section
                cargo_toml
                    .write_all(
                        format!(
                            "[[bin]]\nname = \"{}_{}\"\npath = \"{}\"\n\n",
                            number, name, path
                        )
                        .as_bytes(),
                    )
                    .expect("Failed to write to Cargo.toml");
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

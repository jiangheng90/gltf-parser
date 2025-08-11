use std::{fs, io};

use std::boxed::Box;
use std::error::Error as StdError;

fn run(path: &str) -> Result<(), Box<dyn StdError>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf_parser::Gltf::from_reader_without_validation(reader)?;
    println!("{:#?}", gltf);
    Ok(())
}

// cargo run --package gltf-parser --example gltf-display --features extensions --features KHR_materials_unlit > out.txt
fn main() {
    run("tests/extracted_model.glb").expect("runtime error");
}

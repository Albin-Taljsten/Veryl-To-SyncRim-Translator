use std::path::Path;

mod read_veryl_files;

mod code_gen;

fn main() {

    // 1. Define the location of your target Veryl hardware file
    let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_dir = cargo_dir.parent().unwrap().join("veryl/src/");
    let output_dir = cargo_dir.join("src/output");
    // Ignores Result value since the Result value is only used when testing
    let result = read_veryl_files::read_veryl_files(&source_dir, &output_dir);
    println!("Result: {:?}", result);
}

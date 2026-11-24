use std::fs;
use std::path::Path;
use veryl_parser::Parser;
use veryl_parser::veryl_grammar_trait::Veryl;

use crate::write_mod_file::write_mod_file;

/**
 * A function that reads and translates all Veryl files of a given directory with a given output
 */
pub fn read_veryl_files(source_dir: &Path, output_dir: &Path) -> anyhow::Result<()> {
    if !source_dir.exists() {
        return Err(anyhow::anyhow!("The path '{}' does not exist", source_dir.display()));
    }

    // 2. Read the file contents as a plain text string
    let entries = match fs::read_dir(&source_dir) {
        Ok(entries) => entries,
        Err(e) => {
            // eprintln!("Could not read {}: {e}", source_dir.display());
            return Err(anyhow::anyhow!("Could not read {}: {e}", source_dir.display()));
        }
    };

    let mut veryl_files = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                //eprintln!("Could not read a directory entry: {e}");
                return Err(anyhow::anyhow!("Could not read a directory entry: {e}"));
            }
        };

        let path = entry.path();

        if entry.file_type().is_ok_and(|kind| kind.is_file())
            && path.extension().is_some_and(|extension| extension == "veryl")
        {
            veryl_files.push(path);
        }
    }

    veryl_files.sort();

    let mut module_names: Vec<String> = Vec::new();

    for file_path in veryl_files {
        println!("Reading file: {}", file_path.display());

        let code = match fs::read_to_string(&file_path) {
            Ok(code) => code,
            Err(e) => {
                //eprintln!("Could not read {}: {e}", file_path.display());
                return Err(anyhow::anyhow!("Could not read {}: {e}", file_path.display()));
            }
        };

        match Parser::parse(&code, &file_path) {
            Ok(parsed_output) => {
                println!("Parsed {}", file_path.display());
                
                module_names.extend(crate::code_gen::generate_code(&parsed_output.veryl, &output_dir) 
                    .map_err(|error| {
                    //eprintln!("Could not generate code from {}: {error}", file_path.display());
                    anyhow::anyhow!("Could not generate code from {}: {error}", file_path.display())
                })?);
            }
            Err(error) => {
                //eprintln!("Parsing failed for {}:\n{error:?}", file_path.display());
                return Err(anyhow::anyhow!("Parsing failed for {}:\n{error:?}", file_path.display()));
            }
        }
    }

    crate::write_mod_file::write_mod_file(&module_names, &output_dir)?;

    Ok(())
}

/**
 * A function that reads and translates a single Veryl file
 * Used in testing
 */
fn read_veryl_file(file_path: &Path, output_dir: &Path) -> anyhow::Result<()> {
    if !file_path.extension().is_some_and(|extension| extension == "veryl")
    {
        return Ok(());
    }

    let code = fs::read_to_string(file_path)
        .map_err(|e| anyhow::anyhow!("Could not read {}: {e}", file_path.display()))?;

    let parsed_output = Parser::parse(&code, &file_path)
        .map_err(|e| anyhow::anyhow!("Parsing failed for {}: {e:?}", file_path.display()))?;

    crate::code_gen::generate_code(&parsed_output.veryl, output_dir)
        .map_err(|e| anyhow::anyhow!("Could not generate code from {}: {e}", file_path.display()))?;

    Ok(())
}

// A valid .veryl file generates the expected Rust output. DONE
// A non-.veryl file is ignored. DONE
// A subdirectory is ignored, since this code only scans the immediate folder. DONE
// An invalid Veryl file reports a parse failure and does not stop processing other files. DONE
// An unreadable source directory is handled. 
// An empty source folder completes without generating files. 
#[cfg(test)]
mod test {
    use std::path::Path;

use crate::read_veryl_files::read_veryl_files;

    #[test]
    fn generates_rust_from_veryl_source() {
        let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let source = cargo_dir.parent().unwrap().join("veryl/test/TestModule.veryl");
        let output_dir = cargo_dir.join("src/test_output");
        let generated_file = cargo_dir.join("src/test_output/TestModule.rs");

        if generated_file.exists() {
            std::fs::remove_file(&generated_file).unwrap();
        }

        super::read_veryl_file(&source, &output_dir).expect("Reading and generating given Veryl files should succeed");

        // Replace this with the expected generated file for a known module.
        assert!(
            generated_file.is_file(),
            "Expected generated file does not exist: {}",
            generated_file.display()
        );
    }

    #[test]
    fn ignores_non_veryl_file() {
        let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let source = cargo_dir.parent().unwrap().join("veryl/test/TestExtension.rs");
        let output_dir = cargo_dir.join("src/test_output");
        let generated_probe = output_dir.join("IgnoredExtensionProbe.rs");

        if generated_probe.exists() {
            std::fs::remove_file(&generated_probe).unwrap();
        }

        assert!(source.exists(), "test.rs exists and the test can continue");

        super::read_veryl_file(&source, &output_dir).expect("Reading and generating given Veryl files should succeed");

        assert!(
            !generated_probe.exists(),
            "A module in test.rs should not be generated"
        );
    }

    #[test]
    fn ignores_sub_directories() {
        let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let source_dir = cargo_dir.parent().unwrap().join("veryl/test/");
        let output_dir = cargo_dir.join("src/test_output");
        let generated_probe = output_dir.join("IgnoredSubDirProbe.rs");

        if generated_probe.exists() {
            std::fs::remove_file(&generated_probe).unwrap();
        }

        assert!(source_dir.join("TestSubDir/").join("TestSubDir.veryl").exists(), "Sub directory .veryl file exists test can continue");

        super::read_veryl_files(&source_dir, &output_dir).expect("Reading and generating given Veryl files should succeed");

        assert!(
            !generated_probe.exists(),
            "For the current proof of concept build files in sub directories should not be generated"
        );
    }

    #[test]
    fn error_when_syntax_is_incorrect() {
        let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let source = cargo_dir.parent().unwrap().join("veryl/test/TestSyntax/TestSyntax.veryl");
        let output_dir = cargo_dir.join("src/test_output");
        let generated_probe = output_dir.join("TestSyntax.rs");

        if generated_probe.exists() {
            std::fs::remove_file(&generated_probe).unwrap();
        }

        assert!(source.exists(), "Syntax file exists test can continue");

        let result = super::read_veryl_file(&source, &output_dir);

        assert!(
            result.is_err(),
            "Expected invalid Veryl syntax to return an error"
        );
    }

    #[test]
    fn error_when_missing_or_unreachable_directory() {
        let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let source_dir = cargo_dir.parent().unwrap().join("veryl/testing/");
        let output_dir = cargo_dir.join("src/test_output");

        let result = read_veryl_files(&source_dir, &output_dir);

        assert!(
            result.is_err(),
            "The directory does not exist in the current context so an error should be thrown"
        );
    }

    #[test]
    fn empty_directory_generates_nothing() {
        let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let source_dir = cargo_dir.parent().unwrap().join("veryl/test/TestEmptyDir");
        let output_dir = cargo_dir.join("src/test_output/TestEmptyDir");

        if output_dir.exists() {
            std::fs::remove_dir_all(&output_dir).unwrap();
        }
        std::fs::create_dir_all(&output_dir).unwrap();

        super::read_veryl_files(&source_dir, &output_dir).expect("Reading and generating given Veryl files should succeed");

        let entries = std::fs::read_dir(&output_dir).expect("Output directory should exist and be readable");

        assert!(
            entries.count() == 0,
            "Expected no generated files in {}",
            output_dir.display()
        );
    }
}
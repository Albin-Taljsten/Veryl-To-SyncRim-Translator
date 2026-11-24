use std::{fs::File, io::{BufWriter, Write}, path::{Path}};

pub fn write_mod_file(module_names: &Vec<String>, output_dir: impl AsRef<Path>) -> std::io::Result<()> {
    let path = output_dir.as_ref().join("mod.rs");
    let mut writer = BufWriter::new(File::create(path)?);

    for name in module_names {
        writeln!(writer, "mod {name};")?;
    }

    writeln!(writer)?;

    for name in module_names {
        writeln!(writer, "pub use {name}::*;")?;
    }

    writer.flush()?;
    Ok(())
}
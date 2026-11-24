use std::io::{self, Write};

pub(super) fn struct_gen <W: Write>(writer: &mut W, module_name: &str, generated_ports: &Vec<(String, &'static str)>) -> io::Result<()> {
    writeln!(writer, "#[derive(Serialize, Deserialize, Clone)]")?;
    writeln!(writer, "pub struct {module_name} {{")?;
    writeln!(writer, "    pub(crate) id: common::Id,")?;
    writeln!(writer, "    pub(crate) pos: (f32, f32),")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            writeln!(writer, "    pub(crate) {port_name}: {field_type},")?;
        }
    }
    writeln!(writer, "}}")?;
    Ok(())
}
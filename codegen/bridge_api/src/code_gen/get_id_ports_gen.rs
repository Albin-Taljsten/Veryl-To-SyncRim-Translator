use std::io::{self, Write};

use veryl_analyzer::ir::{Declaration, Module};

pub(super) fn get_id_ports_gen <W: Write>(writer: &mut W, module: &Module, generated_ports: &Vec<(String, &'static str)>) -> io::Result<()> {
    writeln!(writer, "    fn get_id_ports(&self) -> (Id, Ports) {{")?;
    writeln!(writer, "        (")?;
    writeln!(writer, "            self.id.clone(),")?;
    writeln!(writer, "            Ports::new(")?;
    writeln!(writer, "                vec![")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            let constant_name = format!("{}_ID", port_name.to_uppercase());

            writeln!(
                writer,
                "                    &InputPort {{ port_id: {constant_name}.to_string(), input: self.{port_name}.clone() }},"
            )?;
        }
    }
    writeln!(writer, "                ],")?;
    let is_sequential = module.declarations.iter().any(|declaration| {
        matches!(declaration, Declaration::Ff(_))
    });
    let output_type = if is_sequential {
        "OutputType::Sequential"
    } else {
        "OutputType::Combinatorial"
    };
    writeln!(writer, "                {output_type},")?;
    writeln!(writer, "                vec![")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Output" {
            let constant_name = format!("{}_ID", port_name.to_uppercase());
            writeln!(writer, "                    {constant_name}")?;
        }
    }
    writeln!(writer, "                ],")?;
    writeln!(writer, "            ),")?;
    writeln!(writer, "        )")?;
    writeln!(writer, "    }}")?;
    Ok(())
}
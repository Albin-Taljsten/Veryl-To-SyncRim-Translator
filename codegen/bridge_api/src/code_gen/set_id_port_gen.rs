use std::io::{self, Write};

pub(super) fn set_id_port_gen <W: Write>(writer: &mut W, generated_ports: &Vec<(String, &'static str)>) -> io::Result<()> {
    writeln!(writer, "    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {{")?;
    writeln!(writer, "        match target_port_id.as_str() {{")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            let constant_name = format!("{}_ID", port_name.to_uppercase());
            writeln!(writer,"            {constant_name} => self.{port_name} = new_input,")?;
        } 
    }
    writeln!(writer, "            _ => (),")?;
    writeln!(writer, "        }}")?;
    writeln!(writer, "    }}")?;
    Ok(())
}
use std::io::{self, Write};

pub(super) fn impl_name_gen <W: Write>(writer: &mut W, module_name: &str, generated_ports: &Vec<(String, &'static str)>) -> io::Result<()> {
    writeln!(writer, "impl {module_name} {{")?;
    writeln!(writer, "    pub fn new(id: &str, pos: (f32, f32),")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            writeln!(writer, "        {port_name}: Input,")?;
        }
    }
    writeln!(writer, "    ) -> Self {{")?;
    writeln!(writer, "        {module_name} {{")?;
    writeln!(writer, "            id: id.to_string(),")?;
    writeln!(writer, "            pos,")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            writeln!(writer, "            {port_name},")?;
        }
    }
    writeln!(writer, "            scale: 1.0,")?;
    writeln!(writer, "        }}")?;
    writeln!(writer, "    }}")?;

    writeln!(writer, "    pub fn rc_new(id: &str, pos: (f32, f32),")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            writeln!(writer, "        {port_name}: Input,")?;
        }
    }
    writeln!(writer, "    ) -> Rc<Self> {{")?;
    writeln!(writer, "        Rc::new({module_name}::new(id, pos,")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            writeln!(writer, "            {port_name},")?;
        }
    }
    writeln!(writer, "        ))")?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "    pub fn rc_new_with_scale(")?;
    writeln!(writer, "        id: &str,")?;
    writeln!(writer, "        pos: (f32, f32),")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            writeln!(writer, "        {port_name}: Input,")?;
        }
    }
    writeln!(writer, "        scale: f32,")?;
    writeln!(writer, "    ) -> Rc<Self> {{")?;
    writeln!(writer, "        Rc::new({module_name} {{")?;
    writeln!(writer, "            id: id.to_string(),")?;
    writeln!(writer, "            pos,")?;
    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            writeln!(writer, "            {port_name},")?;
        }
    }
    writeln!(writer, "            scale,")?;
    writeln!(writer, "        }})")?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "}}")?;
    Ok(())
}
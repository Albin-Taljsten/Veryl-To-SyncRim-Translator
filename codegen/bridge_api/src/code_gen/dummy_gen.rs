use std::io::{self, Write};

pub(super) fn dummy_gen <W: Write>(writer: &mut W, module_name: &str, generated_ports: &Vec<(String, &'static str)>) -> io::Result<()> {
    let gui_text = "gui-egui".to_string();
    writeln!(writer, "    #[cfg(feature = {:?})]", gui_text)?;
    writeln!(writer, "    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {{")?;
    let dummy = "dummy".to_string();
    let out = "out".to_string();
    writeln!(writer, "        let dummy_input = Input::new({:?}, {:?});", dummy, out)?;
    writeln!(writer, "        Box::new(Rc::new({module_name} {{")?;
    writeln!(writer, "            id: id.to_string(),")?;
    writeln!(writer, "            pos: (pos.0, pos.1),")?;

    for (port_name, field_type) in generated_ports {
        if *field_type == "Input" {
            writeln!(writer, "            {port_name}: dummy_input.clone(),")?;
        }
    }
    writeln!(writer, "            scale: 1.0,")?;
    writeln!(writer, "        }}))")?;
    writeln!(writer, "    }}")?;
    Ok(())
}
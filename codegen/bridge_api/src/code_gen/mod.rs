use veryl_parser::{veryl_grammar_trait::{self}};
use std::{fs::{self, File}, io::{self, BufWriter, Write}, path::{Path, PathBuf}};

use veryl_analyzer::{ir::{Component, Ir, Module, VarKind::{self}}};

mod get_ir;
pub(crate) use get_ir::get_ir;

mod struct_gen;
mod dummy_gen;
mod get_id_ports_gen;
mod set_id_port_gen;
mod impl_name_gen;

struct CodeGenerator {
    ir: Ir,
    output_path: PathBuf
}

impl CodeGenerator {
    fn new(ir: Ir, output_path: impl AsRef<Path>) -> Self {
        Self {
            ir,
            output_path: output_path.as_ref().to_path_buf(),
        }
    }

    fn generate(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.output_path)?;

        for component in &self.ir.components {
            match component {
                Component::Module(module) => {
                    let name = module.name.to_string();
                    let path = self.output_path.join(format!("{name}.rs"));
                    let mut writer = BufWriter::new(File::create(path)?);

                    write_module(&mut writer, module)?;
                    writer.flush()?;
                }
                Component::Interface(interface) => {
                    let name = interface.name.to_string();
                    let path = self.output_path.join(format!("{name}.rs"));
                    let mut writer = BufWriter::new(File::create(path)?);

                    // Write interface
                    writer.flush()?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

pub fn generate_code(root_node: &veryl_grammar_trait::Veryl, output_path: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    let ir = match get_ir::get_ir(root_node) {
        Ok(ir) => ir,
        Err(e) => {
            eprintln!("❌ Error generating IR: {e}");
            panic!("Failed to generate IR");
        }
    };

    let module_names = ir.components
        .iter()
        .filter_map(|component| match component {
            Component::Module(module) => Some(module.name.to_string()),
            _ => None,
        })
        .collect();

    CodeGenerator::new(ir, output_path).generate()?;
    Ok(module_names)
}

/**
 * Writes module code into file<br>
 * Important:<br>
 * Since SyncRim's simualator already has a built in clock, "clock" variables and ports are essentially useless to translate
 * 
 */

fn write_module<W: Write>(writer: &mut W, module: &Module) -> io::Result<()> {
    let name = module.name.to_string();

    writer.write_all(
    b"#[cfg(feature = \"gui-egui\")]\n\
      use crate::common::EguiComponent;\n\
      use crate::common::*;\n\
      use log::*;\n\
      use num_enum::{IntoPrimitive, TryFromPrimitive};\n\
      use serde::{Deserialize, Serialize};\n\
      use std::{any::Any, cell::RefCell, collections::BTreeMap, convert::{Into, TryFrom}, ops::{Deref, Range}, rc::Rc};\n\n",
    )?;

    let mut generated_ports: Vec<(String, &'static str)> = Vec::new();

    for (path, var_id) in &module.ports {
        let Some(variable) = module.variables.get(var_id) else {
            continue;
        };

        if variable.r#type.is_clock() || variable.r#type.is_reset() {
            continue;
        }

        let field_type = match variable.kind {
            VarKind::Input => "Input",
            VarKind::Output => "Output",
            _ => continue,
        };

        let port_name = path.to_string();
        let constant_name = format!("{}_ID", port_name.to_uppercase());

        writeln!(
            writer, 
            "pub const {constant_name}: &str = {:?};", 
            port_name
        )?;

        generated_ports.push((port_name, field_type));
    }
    writeln!(writer, "")?;

    // Skeleton: emit the component declaration and implementation here.
    struct_gen::struct_gen(writer, &name, &generated_ports)?;
    writeln!(writer, "")?;
    writeln!(writer, "#[typetag::serde]")?;
    writeln!(writer, "impl Component for {name} {{")?;
    writeln!(writer, "    fn to_(&self) {{")?;
    writeln!(writer, "        trace!({:?});", name)?;
    writeln!(writer, "    }}")?;

    dummy_gen::dummy_gen(writer, &name, &generated_ports)?;
    get_id_ports_gen::get_id_ports_gen(writer, module, &generated_ports)?;

    writeln!(writer, "    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {{")?;
    writeln!(writer, "        Ok(())")?;
    writeln!(writer, "    }}")?;

    set_id_port_gen::set_id_port_gen(writer, &generated_ports)?;

    writeln!(writer, "    fn as_any(&self) -> &dyn Any {{")?;
    writeln!(writer, "        self")?;
    writeln!(writer, "    }}")?;

    // Keep this is the end of the "impl Component for {name}" part
    writeln!(writer, "}}")?;
    writeln!(writer, "")?;

    impl_name_gen::impl_name_gen(writer, &name, &generated_ports)?;

    Ok(())
}
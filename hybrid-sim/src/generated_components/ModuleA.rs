#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::*;
use log::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use std::{any::Any, cell::RefCell, collections::BTreeMap, convert::{Into, TryFrom}, ops::{Deref, Range}, rc::Rc};


#[derive(Serialize, Deserialize, Clone)]
pub struct ModuleA {
    pub(crate) id: common::Id,
    pub(crate) pos: (f32, f32),
}

#[typetag::serde]
impl Component for ModuleA {
    fn to_(&self) {
        trace!("ModuleA");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(ModuleA {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            scale: 1.0,
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                ],
                OutputType::Combinatorial,
                vec![
                ],
            ),
        )
    }
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        Ok(())
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            _ => (),
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ModuleA {
    pub fn new(id: &str, pos: (f32, f32),
    ) -> Self {
        ModuleA {
            id: id.to_string(),
            pos,
            scale: 1.0,
        }
    }
    pub fn rc_new(id: &str, pos: (f32, f32),
    ) -> Rc<Self> {
        Rc::new(ModuleA::new(id, pos,
        ))
    }
    pub fn rc_new_with_scale(
        id: &str,
        pos: (f32, f32),
        scale: f32,
    ) -> Rc<Self> {
        Rc::new(ModuleA {
            id: id.to_string(),
            pos,
            scale,
        })
    }
}

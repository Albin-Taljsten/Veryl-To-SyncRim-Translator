#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::*;
use log::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use std::{any::Any, cell::RefCell, collections::BTreeMap, convert::{Into, TryFrom}, ops::{Deref, Range}, rc::Rc};

pub const I_SEL_ID: &str = "i_sel";
pub const O_DATA_ID: &str = "o_data";
pub const I_DATA_ID: &str = "i_data";
pub const I_SEL2_ID: &str = "i_sel2";

#[derive(Serialize, Deserialize, Clone)]
pub struct TestModule {
    pub(crate) id: common::Id,
    pub(crate) pos: (f32, f32),
    pub(crate) i_sel: Input,
    pub(crate) i_data: Input,
    pub(crate) i_sel2: Input,
}

#[typetag::serde]
impl Component for TestModule {
    fn to_(&self) {
        trace!("TestModule");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(TestModule {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            i_sel: dummy_input.clone(),
            i_data: dummy_input.clone(),
            i_sel2: dummy_input.clone(),
            scale: 1.0,
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort { port_id: I_SEL_ID.to_string(), input: self.i_sel.clone() },
                    &InputPort { port_id: I_DATA_ID.to_string(), input: self.i_data.clone() },
                    &InputPort { port_id: I_SEL2_ID.to_string(), input: self.i_sel2.clone() },
                ],
                OutputType::Sequential,
                vec![
                    O_DATA_ID
                ],
            ),
        )
    }
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        Ok(())
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            I_SEL_ID => self.i_sel = new_input,
            I_DATA_ID => self.i_data = new_input,
            I_SEL2_ID => self.i_sel2 = new_input,
            _ => (),
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TestModule {
    pub fn new(id: &str, pos: (f32, f32),
        i_sel: Input,
        i_data: Input,
        i_sel2: Input,
    ) -> Self {
        TestModule {
            id: id.to_string(),
            pos,
            i_sel,
            i_data,
            i_sel2,
            scale: 1.0,
        }
    }
    pub fn rc_new(id: &str, pos: (f32, f32),
        i_sel: Input,
        i_data: Input,
        i_sel2: Input,
    ) -> Rc<Self> {
        Rc::new(TestModule::new(id, pos,
            i_sel,
            i_data,
            i_sel2,
        ))
    }
    pub fn rc_new_with_scale(
        id: &str,
        pos: (f32, f32),
        i_sel: Input,
        i_data: Input,
        i_sel2: Input,
        scale: f32,
    ) -> Rc<Self> {
        Rc::new(TestModule {
            id: id.to_string(),
            pos,
            i_sel,
            i_data,
            i_sel2,
            scale,
        })
    }
}

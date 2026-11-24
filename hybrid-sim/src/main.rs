use std::rc::Rc;

use hybrid_sim::load_veryl_bridge;

fn main() {
    let veryl_bridge = load_veryl_bridge("../../codegen/veryl/src/test.veryl", "test").unwrap();

    let test = Rc::new(test::new())
}
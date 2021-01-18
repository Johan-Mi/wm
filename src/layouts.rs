use super::defs::{STACK_NUM_MASTER, STACK_RATIO};
use penrose::core::layout::{side_stack, Layout, LayoutConf};

pub fn get_layouts() -> Vec<Layout> {
    let stack_conf = LayoutConf {
        follow_focus: false,
        ..LayoutConf::default()
    };

    vec![Layout::new(
        "[side]",
        stack_conf,
        side_stack,
        STACK_NUM_MASTER,
        STACK_RATIO,
    )]
}

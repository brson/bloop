use crate::Result;

pub trait Walk {
    type Node;
    type FrameState;
    type FrameResult;

    fn enter_frame(node: Self::Node) -> Result<Self::FrameState>;

    fn visit_child(frm: Self::FrameState, ch: Self::FrameResult) -> Result<Self::FrameState>;

    fn leave_frame(frm: Self::FrameState) -> Result<Self::FrameResult>;
    
    fn run(node: Self::Node) -> Result<Self::FrameResult> {
        panic!()
    }

    fn register_subframe(node: Self::Node) {
        panic!()
    }
}

use crate::global_defs::Usp;
use std::iter::IntoIterator;

use crate::Result;

pub trait Walk {
    type Node;
    type FrameState;
    type FrameResult;

    fn enter_frame(node: Self::Node) -> Result<Self::FrameState>;

    fn visit_child(frm: Self::FrameState, ch: Self::FrameResult) -> Result<Self::FrameState>;

    fn leave_frame(frm: Self::FrameState) -> Result<Self::FrameResult>;
    
    fn run<I>(nodes: I) -> Result<Self::FrameResult>
    where I: IntoIterator<Item = Self::Node>
    {
        let nodes = nodes.into_iter();

        let mut node_stack = vec![];

        push_next_children(&mut node_stack, nodes, 0);

        panic!()
    }

    fn register_child(node: Self::Node) {
        panic!()
    }
}

enum Phase<TNode> {
    Enter(Usp, TNode),
}

fn push_next_children<I, N>(node_stack: &mut Vec<Phase<N>>, mut next_nodes: I, lvl: Usp)
where I: IntoIterator<Item = N>
{
    // collect next nodes in forward order
    let mut next_nodes_accum = vec![];
    for next_node in next_nodes {
        next_nodes_accum.push(Phase::Enter(lvl, next_node))
    }

    next_nodes_accum.reverse();
    node_stack.append(&mut next_nodes_accum);
}

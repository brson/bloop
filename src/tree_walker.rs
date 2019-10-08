use crate::global_defs::Usp;
use std::iter::IntoIterator;

use crate::Result;

pub trait Walk {
    type Node;
    type FrameState;
    type FrameResult;

    // FIXME: this only returns Option because I can't figure out how to make
    // pest not visit EOI
    fn enter_frame(node: Self::Node, push_child: impl FnMut(Self::Node)) -> Result<Option<Self::FrameState>>;

    fn handle_child_result(frm: Self::FrameState, ch: Self::FrameResult) -> Result<Self::FrameState>;

    fn leave_frame(frm: Self::FrameState) -> Result<Self::FrameResult>;
    
    fn walk<I>(nodes: I) -> Result<Vec<Self::FrameResult>>
    where I: IntoIterator<Item = Self::Node>
    {
        let nodes = nodes.into_iter();

        let mut state_stack: Vec<Phase<Self::Node, Self::FrameState>> = vec![];
        let mut result_stack: Vec<Vec<Self::FrameResult>> = vec![];

        push_next_children(&mut state_stack, nodes, 0);
        result_stack.push(vec![]);

        while let Some(this_phase) = state_stack.pop() {
            match this_phase {
                Phase::Enter(lvl, node) => {
                    let mut children = vec![];
                    {
                        let push_child = |child: Self::Node| {
                            children.push(child);
                        };
                        let next_state = Self::enter_frame(node, push_child)?;
                        if let Some(next_state) = next_state {
                            state_stack.push(Phase::Leave(lvl, next_state));
                        } else {
                            continue;
                        }
                    }
                    {
                        let next_lvl = lvl.checked_add(1).expect("level exceeds capacity");
                        push_next_children(&mut state_stack, children, next_lvl);
                    }
                    {
                        // Push a vector to collect the results of visiting children
                        result_stack.push(vec![]);
                    }
                }
                Phase::Leave(lvl, mut frame_state) => {
                    let child_results = result_stack.pop().expect("no results for child nodes");

                    for result in child_results {
                        frame_state = Self::handle_child_result(frame_state, result)?;
                    }

                    let new_result = Self::leave_frame(frame_state)?;
                    let peer_results = result_stack.last_mut().expect("no vec for node result");
                    peer_results.push(new_result);
                }
            }
        }

        assert!(state_stack.is_empty());
        let result = result_stack.pop().expect("result stack empty");
        assert!(state_stack.is_empty());

        Ok(result)
    }
}

enum Phase<TNode, TFrameState> {
    Enter(Usp, TNode),
    Leave(Usp, TFrameState),
}

fn push_next_children<I, N, S>(state_stack: &mut Vec<Phase<N, S>>, mut next_nodes: I, lvl: Usp)
where I: IntoIterator<Item = N>
{
    // Collect next nodes in forward order
    let mut next_states = vec![];
    for next_node in next_nodes {
        next_states.push(Phase::Enter(lvl, next_node))
    }

    // Push them on the stack in reverse order so they
    // can be popped in forward order later
    next_states.reverse();
    state_stack.append(&mut next_states);
}

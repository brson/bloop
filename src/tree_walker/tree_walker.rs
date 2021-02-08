use std::iter::IntoIterator;
use b_deps::anyhow::Result;
use std::fmt::Debug;
use std::sync::mpsc::{self, Sender, Receiver};

pub trait Walk {
    type Node: Debug;
    type FrameState: Debug;
    type FrameResult: Debug;

    // FIXME: this returns Option only because I can't figure out how to make
    // pest not visit EOI
    fn enter_frame(node: Self::Node, push_child: impl FnMut(Self::Node)) -> Result<Option<Self::FrameState>>;

    fn handle_child_result(frm: Self::FrameState, ch: Self::FrameResult) -> Result<Self::FrameState>;

    fn leave_frame(frm: Self::FrameState) -> Result<Self::FrameResult>;
    
    fn walk<I>(nodes: I) -> Result<Vec<Self::FrameResult>>
    where I: IntoIterator<Item = Self::Node>
    {
        let nodes = nodes.into_iter();

        #[derive(Debug)]
        struct Enter<Node, FrameResult> {
            node: Node,
            child_results_rx: Receiver<FrameResult>,
            child_results_tx: Sender<FrameResult>,
            parent_results_tx: Sender<FrameResult>,
        }

        struct Leave<FrameState, FrameResult> {
            frame_state: FrameState,
            child_results_rx: Receiver<FrameResult>,
            parent_results_tx: Sender<FrameResult>,
        }

        let (root_results_tx, root_results_rx) = mpsc::channel();

        let mut current_enter_list: Vec<Enter<_, _>> = nodes.map(|node| {
            let (child_results_tx, child_results_rx) = mpsc::channel();
            Enter {
                node,
                child_results_rx,
                child_results_tx,
                parent_results_tx: root_results_tx.clone(),
            }
        }).collect();
        let mut leave_lists_stack: Vec<Vec<Leave<_, _>>> = vec![];

        while !current_enter_list.is_empty() {
            let mut new_enter_lists: Vec<Vec<Enter<_, _>>> = vec![];
            let mut new_leave_lists: Vec<Leave<_, _>>  = vec![];

            for enter in current_enter_list {
                let mut children = vec![];
                let push_child = |child: Self::Node| children.push(child);
                let frame_state = Self::enter_frame(enter.node, push_child)?;
                let frame_state = if let Some(frame_state) = frame_state {
                    frame_state
                } else {
                    continue;
                };
                let parent_results_tx = enter.child_results_tx;
                let new_enters: Vec<Enter<_, _>> = children.into_iter().map(|node| {
                    let (child_results_tx, child_results_rx) = mpsc::channel();
                    Enter {
                        node,
                        child_results_rx,
                        child_results_tx,
                        parent_results_tx: parent_results_tx.clone(),
                    }
                }).collect();
                new_enter_lists.push(new_enters);
                new_leave_lists.push(Leave {
                    frame_state,
                    child_results_rx: enter.child_results_rx,
                    parent_results_tx: enter.parent_results_tx,
                });
            }

            current_enter_list = new_enter_lists.into_iter().flatten().collect();
            leave_lists_stack.push(new_leave_lists);
        }

        while let Some(leave_list) = leave_lists_stack.pop() {
            for leave in leave_list {
                let mut frame_state = leave.frame_state;

                for result in leave.child_results_rx.into_iter() {
                    frame_state = Self::handle_child_result(frame_state, result)?;
                }

                let new_result = Self::leave_frame(frame_state)?;

                leave.parent_results_tx.send(new_result).expect("send");
            }
        }

        drop(root_results_tx);
        let root_results: Vec<Self::FrameResult> = root_results_rx.into_iter().collect();
        Ok(root_results)
    }

    fn walk2<I>(nodes: I) -> Result<Vec<Self::FrameResult>>
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
                Phase::Leave(_lvl, mut frame_state) => {
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
        assert!(result_stack.is_empty());

        Ok(result)
    }
}

enum Phase<TNode, TFrameState> {
    Enter(u32, TNode),
    Leave(u32, TFrameState),
}

fn push_next_children<I, N, S>(state_stack: &mut Vec<Phase<N, S>>, next_nodes: I, lvl: u32)
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

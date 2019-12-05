#![allow(unused)]

use std::collections::VecDeque;
use log::debug;
use b_base_ast::{BaseAst, Module};
use b_base_parser_traits::BaseParse;
use b_error::{BResult, BError};
use b_token_tree::TokenTree;
use b_tree_walker::Walk;
use b_base_ast::{
    ArgList,
    Declaration,
    Body,
};
use b_base_parser_lalrpop::{
    parse_module,
    parse_arg_list,
    parse_body,
};
use b_base_partial_ast::{
    ArgListTree,
    PartialModule,
    PartialDeclaration,
    PartialFunction,
    PartialArgList,
    PartialBody,
};

pub struct BaseParser;

impl BaseParse for BaseParser {
    fn parse(&self, tt: &TokenTree) -> BResult<BaseAst> {
        // FIXME bad clone
        let node = Node(CurrentTarget::Module, tt.clone());
        let mut parsed = Node::walk(Some(node))?;

        let ast = parsed.pop().expect("parse results");
        assert!(parsed.is_empty());

        match ast.0 {
            AstNode::Module(m) => {
                return Ok(BaseAst(m));
            }
            _ => {
                return Err(BError::new("wrong parse result"));
            }
        }

        Ok(BaseAst(Module { decls: vec![] }))
    }
}

#[derive(Debug)]
pub enum AstNode {
    Module(Module),
    ArgList(ArgList),
    Body(Body),
}

struct Node(CurrentTarget, TokenTree);

enum CurrentTarget {
    Module,
    ArgList,
    Body,
}

#[derive(Debug)]
enum FrameState {
    Module(PartialModule, ChildParts),
    ArgList(PartialArgList),
    Body(PartialBody),
}

#[derive(Debug)]
struct FrameResult(AstNode);

#[derive(Debug, Default)]
struct ChildParts {
    arg_lists: VecDeque<ArgList>,
    bodies: VecDeque<Body>,
}

// This is going to traverse the token tree, parsing each flat vector of
// ThingOrTree's into a vector of AST items. It decends in parallel into
// sub-trees.

impl Walk for Node {
    type Node = Node;
    type FrameState = FrameState;
    type FrameResult = FrameResult;

    // FIXME bad clones
    fn enter_frame(node: Self::Node, mut push_child: impl FnMut(Self::Node)) -> BResult<Option<Self::FrameState>> {
        match node.0 {
            CurrentTarget::Module => {
                let ast = parse_module(&node.1)?;

                for decl in &ast.decls {
                    match decl {
                        PartialDeclaration::Function(
                            PartialFunction {
                                ref args,
                                ref body,
                                ..
                            }
                        ) => {
                            let args_tt = (args.0).0.clone();
                            let body_tt = (body.0).0.clone();
                            push_child(Node(CurrentTarget::ArgList, args_tt));
                            push_child(Node(CurrentTarget::Body, body_tt));
                        }
                    }
                }

                Ok(Some(FrameState::Module(ast, ChildParts::default())))
            },
            CurrentTarget::ArgList => {
                let ast = parse_arg_list(&node.1)?;
                Ok(Some(FrameState::ArgList(ast)))
            }
            CurrentTarget::Body => {
                let ast = parse_body(&node.1)?;
                Ok(Some(FrameState::Body(ast)))
            }
        }
    }

    fn handle_child_result(frm: Self::FrameState, ch: Self::FrameResult) -> BResult<Self::FrameState> {
        debug!("handle_child_result");
        debug!("frm: {:#?}", frm);
        debug!("ch: {:#?}", ch);
        match frm {
            FrameState::Module(m, mut child_parts) => {
                match ch.0 {
                    AstNode::ArgList(arg_list) => {
                        child_parts.arg_lists.push_back(arg_list);
                        Ok(FrameState::Module(m, child_parts))
                    }
                    AstNode::Body(body) => {
                        child_parts.bodies.push_back(body);
                        Ok(FrameState::Module(m, child_parts))
                    }
                    _ => panic!()
                }
            }
            _ => panic!()
        }
    }

    fn leave_frame(frm: Self::FrameState) -> BResult<Self::FrameResult> {
        match frm {
            FrameState::Module(..) => panic!(),
            FrameState::ArgList(arg_list) => {
                Ok(FrameResult(AstNode::ArgList(ArgList(arg_list.0))))
            }
            FrameState::Body(body) => {
                Ok(FrameResult(AstNode::Body(Body {
                    stmts: body.stmts,
                })))
            }
        }
    }

}


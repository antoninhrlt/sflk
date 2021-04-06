use crate::object::Obj;
use crate::program::{Block, Chain};
use std::collections::HashMap;

enum BlockOrChain {
	Block(Block),
	Chain(Chain),
	Empty, // TODO: Remove this variant, maybe
}

struct ExecNode {
	block_or_chain: BlockOrChain,
	ip: usize,
	sub_nodes: Vec<ExecNode>,
	variables: HashMap<String, Obj>,
}

impl ExecNode {
	fn new() -> ExecNode {
		ExecNode {
			block_or_chain: BlockOrChain::Empty,
			ip: 0,
			sub_nodes: Vec::new(),
			variables: HashMap::new(),
		}
	}
}

pub struct Mem {
	exec_tree: ExecNode,
}

impl Mem {
	pub fn new() -> Mem {
		Mem {
			exec_tree: ExecNode::new(),
		}
	}
}

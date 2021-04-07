use crate::object::Obj;
use crate::program::{Block, Chain};
use std::collections::HashMap;

enum BlockOrChain {
	Block(Block),
	Chain(Chain),
	Empty, // TODO: Remove this variant, maybe
}

type BranchId = usize;

type PathUp = Vec<BranchId>;
type PathUpSlice = [BranchId];

struct ExecNode {
	block_or_chain: BlockOrChain,
	ip: usize,
	sub_nodes: HashMap<BranchId, ExecNode>,
	variables: HashMap<String, Obj>,
}

impl ExecNode {
	fn new() -> ExecNode {
		ExecNode {
			block_or_chain: BlockOrChain::Empty,
			ip: 0,
			sub_nodes: HashMap::new(),
			variables: HashMap::new(),
		}
	}
}

impl ExecNode {
	fn follow_mut(&mut self, path_up: &PathUpSlice) -> &mut ExecNode {
		if let Some((first_id, path)) = path_up.split_first() {
			self.sub_nodes
				.get_mut(first_id)
				.expect("TODO")
				.follow_mut(path)
		} else {
			self
		}
	}

	fn follow(&self, path_up: &PathUpSlice) -> &ExecNode {
		if let Some((first_id, path)) = path_up.split_first() {
			self.sub_nodes.get(first_id).expect("TODO").follow(path)
		} else {
			self
		}
	}
}

impl ExecNode {
	fn step(&mut self) {}
}

pub struct Machine {
	exec_tree: ExecNode,
	current_exec_path: PathUp,
}

impl Machine {
	pub fn new() -> Machine {
		Machine {
			exec_tree: ExecNode::new(),
			current_exec_path: Vec::new(),
		}
	}
}

impl Machine {
	pub fn step(&mut self) {
		let current_exec_node = self.current_exec_node_mut();
		current_exec_node.step();
	}

	fn current_exec_node_mut(&mut self) -> &mut ExecNode {
		self.exec_tree.follow_mut(&self.current_exec_path)
	}
}

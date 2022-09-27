
use rs_lex::rlex::tree::btree::*;


fn main() -> std::io::Result<()> {

    let mut tree : Tree<i32> = Tree::new();
    let mut node : TreeNode<i32> = TreeNode::new(25);
    node.left = Some(Box::new(TreeNode::new(15)));
    let node_left = Some(Box::new(TreeNode::new(100)));
    let node_right = Some(Box::new(TreeNode::new(200)));
    node.right = Some(Box::new(TreeNode::new_node(15, node_left, node_right)));
    tree.root = Some(Box::new(node));

    TreeNode::<i32>::print_nodes(&tree.root);

    Ok(())

}
pub mod btree {

    #[derive(PartialEq, Debug)]
    pub struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub data: T,
    }

    pub type Node<T> = Option<Box<TreeNode<T>>>;

    impl<T: std::fmt::Debug> TreeNode<T> {
        pub fn new(dat: T) -> Self {
            Self {
                left: None,
                right: None,
                data: dat,
            }
       }

       pub fn new_node(dat: T, l: Node<T>, r: Node<T>) -> Self {
            Self {
                left: l,
                right: r,
                data: dat,
            }
       }

       pub fn print_nodes(node: &Node<T>) {
            if let Some(n) = node {
                println!("{:?}", n.data);
                Self::print_nodes(&n.left);
                Self::print_nodes(&n.right);
            }
       }

       pub fn set_left_node(&mut self, data: T) {
            self.left = Some(Box::new(TreeNode::new(data)));
       }

       pub fn set_right_node(&mut self, data: T) {
            self.right = Some(Box::new(TreeNode::new(data)));
       }
    }

    pub struct Tree<T> {
        pub root: Node<T>,
    }

    impl<T: std::fmt::Debug> Tree<T> {

        pub fn new() -> Self {
            Self {
                root: None,
            }
        }       
        
        pub fn new_node(node: Node<T>) -> Self {
            Self {
                root: node,
            }
        }

        pub fn print_nodes(&self) {
            TreeNode::<T>::print_nodes(&self.root);
        }
    }

}
// pub struct BinaryTree<T>(Option<Box<Node<T>>>);

use std::fmt::Debug;

// pub struct BinaryTree<T> {
//     root: Option<Box<Node<T>>>,
// }
// impl<T: Ord> BinaryTree<T> {
//     pub fn void() -> Self {
//         BinaryTree { root: None }
//     }

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Debug> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            value: val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, val: T) {
        if val < self.value {
            match &mut self.left {
                Some(left_child) => left_child.insert(val),
                None => self.left = Some(Box::new(Node::new(val))),
            }
        } else {
            match &mut self.right {
                Some(right_child) => right_child.insert(val),
                None => self.right = Some(Box::new(Node::new(val))),
            }
        }
    }

    pub fn inorder_traversal(&self) {
        if let Some(left_child) = &self.left {
            left_child.inorder_traversal()
        }

        println!("{:#?}", self.value);

        if let Some(right_child) = &self.right {
            right_child.inorder_traversal()
        }
    }

    pub fn BFS(&self) {}

    pub fn print_tree(&self, prefix: String, is_left: bool) {
        println!(
            "{}{} {:#?}",
            prefix,
            if is_left { "├──" } else { "└──" },
            self.value
        );

        if let Some(ref left) = self.left {
            left.print_tree(
                format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
                true,
            );
        }

        if let Some(ref right) = self.right {
            right.print_tree(
                format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
                false,
            );
        }
    }

    // pub fn zigzag_pattern(&self, is_even: bool) {
    //     if is_even {
    //         println!("{:#?}", &self.right.unwrap().value);
    //         println!("{:#?}", &self.left.unwrap().value);
    //         is_even = false
    //     } else {
    //         is_even = true
    //     }
    //
    //     self.zigzag_pattern(is_even, no)
    // }
}

#[test]
fn testing() {
    let mut tree = Node::new(30);

    tree.insert(23);
    tree.insert(3);
    tree.insert(2);
    tree.insert(11);
    tree.insert(64);
    tree.insert(99);
    tree.insert(1);
    tree.insert(1);
    tree.inorder_traversal();
    println!();
    tree.print_tree(String::from(""), false);
    assert_eq!(1, 1);
}

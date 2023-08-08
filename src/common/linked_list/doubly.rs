use std::fmt::Debug;

type List<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: List<T>,
    prev: List<T>,
}

pub struct DoublyLinkedList<T> {
    head: List<T>,
    tail: Option<*mut Node<T>>,
    length: usize,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            prev: None,
        }
    }
    fn new_next(data: T, next: List<T>) -> Self {
        Node {
            data,
            next,
            prev: None,
        }
    }
    fn new_next_prev(data: T, next: List<T>, prev: List<T>) -> Self {
        Node { data, next, prev }
    }
}

impl<T: Debug + Copy> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn prepend(&mut self, data: T) {
        let new_node = Box::new(Node::new_next(data, self.head.take()));

        // here we are converting new_node to a raw pointer which is kinda similar to references (&)
        let raw_ptr = Box::into_raw(new_node);
        if self.head.is_none() {
            self.tail = Some(raw_ptr);
        }
        // extracting the value out of a raw pointer is an unsafe operation
        self.head = unsafe { Some(Box::from_raw(raw_ptr)) };
        self.length += 1;
    }

    pub fn append(&mut self, data: T) {
        if self.head.is_none() {
            self.prepend(data);
        }

        unsafe {
            let mut new_node = Box::new(Node::new(data));
            new_node.prev = Some(Box::from_raw(self.tail.unwrap()));
            new_node.next = None;

            // here we have to create a temporary raw pointer for new node because we have to pass
            // the reference to the new_node in self.tail.next and the n move the new_node as new self.tail
            let temp_raw_ptr = Box::into_raw(new_node);
            (*self.tail.unwrap()).next = Some(Box::from_raw(temp_raw_ptr));
            self.tail = Some(temp_raw_ptr);
        }
    }
}

// pub fn new() -> Self {
//     SinglyLinkedList {
//         head: None,
//         length: 0,
//     }
// }
//
// pub fn infront(&mut self, data: T) {
//     let new_node = Box::new(Node {
//         data,
//         next: self.head.take(),
//     });
//     // let raw_ptr = Box::into_raw(new_node);
//
//     // if self.head.is_none() {
//     //     self.tail = Some(raw_ptr);
//     // }
//     self.head = Some(new_node);
//
//     self.length += 1;
// }
//  1 2 2 2  2    [7]     2 2 2 2 2
//
// pub fn infront(&mut self, data: T) {
//     if let Some(tail_pointer_exists) = self.tail {
//         let new_node = Box::new(Node { data, next: None });
//         let raw_ptr = Box::into_raw(new_node);
//
//         unsafe {
//             (*tail_pointer_exists).next = Some(Box::from_raw(raw_ptr));
//             self.tail = Some(raw_ptr);
//         }
//     } else {
//         self.infront(data);
//     }
//
//     self.length += 1;
// }
//
// pub fn traverse(&self) {
//     let mut current = &self.head;
//     while let Some(node) = current {
//         print!("{:#?} -> ", node.data);
//         current = &node.next;
//     }
//     println!("None");
// }

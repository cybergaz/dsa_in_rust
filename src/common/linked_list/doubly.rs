use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;
type NodeWeakRef<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: NodeRef<T>,
    prev: NodeRef<T>,
}

pub struct DoublyLinkedList<T> {
    head: NodeRef<T>,
    tail: NodeRef<T>,
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
}

impl<T: Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn prepend(&mut self, data: T) {
        let mut new_node = Rc::new(RefCell::new(Node::new(data)));

        if let Some(node) = &self.head {
            // node.as_ref().borrow_mut().next = self.head.clone();
            new_node.borrow_mut().next.clone_from(&self.head);
            node.borrow_mut().prev = Some(new_node.clone());
            self.head = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }

        self.length += 1;
    }

    // pub fn append(&mut self, data: T) {
    //     if self.head.is_none() {project
    //         self.prepend(data);
    //     }
    //
    //     unsafe {
    //         let mut new_node = Box::new(Node::new(data));
    //         new_node.prev = Some(Box::from_raw(self.tail.unwrap()));
    //         new_node.next = None;
    //
    //         // here we have to create a temporary raw pointer for new node because we have to pass
    //         // the reference to the new_node in self.tail.next and the n move the new_node as new self.tail
    //         let temp_raw_ptr = Box::into_raw(new_node);
    //         (*self.tail.unwrap()).next = Some(Box::from_raw(temp_raw_ptr));
    //         self.tail = Some(temp_raw_ptr);
    //     }
    // }
    //
    // pub fn insert_at(&mut self, data: T, index: usize) {
    //     if self.length < index {
    //         // panic!("can not insert : list is not big enough");
    //         println!("can not insert : list is not big enough");
    //         return;
    //     }
    //
    //     if self.head.is_none() {
    //         self.prepend(data);
    //         return;
    //     }
    //
    //     let mut current = &mut self.head;
    //
    //     for _x in 0..index {
    //         current = &mut current.as_mut().unwrap().next;
    //     }
    //
    //     let mut new_node = Box::new(Node::new(data));
    //     new_node.next = current.as_mut().unwrap().next.take();
    //     // to convert &mut Option<> to Option you can use .take() or mem::replace()
    //     new_node.prev = current.take();
    //     *current = Some(new_node);
    //
    //     self.length += 1;
    // }

    pub fn view(&mut self) {
        println!("-----------------------------------------------------------------------------");
        // println!("{:#?}", self.head);
        //
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{:#?} <-> ", node.borrow_mut().data);
            current = node.as_ref().borrow_mut().next.clone();
        }
        println!("None");
        // println!("the length up until now is : {}", self.length / 2);
        println!("-----------------------------------------------------------------------------");
    }
}

#[test]
fn testing() {
    //
    let mut list = DoublyLinkedList::new();
    list.prepend(23);
    list.prepend(9);
    list.prepend(232);

    list.view();
    //
    // list.insert_at(37, 3);
    // // list.insert_at(38, 0);
    // // list.insert_at(39, 9);
    // list.view();
    //
    // list.append(100);
    // list.view();
}

// an unsafe implementation without actually considering 'prev' (it was not possible without Rc)

// use std::{fmt::Debug, os::linux::net::SocketAddrExt};
//
// type List<T> = Option<Box<Node<T>>>;
//
// #[derive(Debug)]
// struct Node<T> {
//     data: T,
//     next: List<T>,
//     prev: List<T>,
// }
//
// pub struct DoublyLinkedList<T> {
//     head: List<T>,
//     tail: Option<*mut Node<T>>,
//     length: usize,
// }
//
// impl<T> Node<T> {
//     fn new(data: T) -> Self {
//         Node {
//             data,
//             next: None,
//             prev: None,
//         }
//     }
//     fn new_next(data: T, next: List<T>) -> Self {
//         Node {
//             data,
//             next,
//             prev: None,
//         }
//     }
//     fn new_next_prev(data: T, next: List<T>, prev: List<T>) -> Self {
//         Node { data, next, prev }
//     }
// }
//
// impl<T: Debug + Copy> DoublyLinkedList<T> {
//     pub fn new() -> Self {
//         DoublyLinkedList {
//             head: None,
//             tail: None,
//             length: 0,
//         }
//     }
//
//     pub fn prepend(&mut self, data: T) {
//         let new_node = Box::new(Node::new_next(data, self.head.take()));
//
//         // here we are converting new_node to a raw pointer which is kinda similar to references (&)
//         let raw_ptr = Box::into_raw(new_node);
//         if self.head.is_none() {
//             self.tail = Some(raw_ptr);
//         }
//         // extracting the value out of a raw pointer is an unsafe operation
//         self.head = unsafe { Some(Box::from_raw(raw_ptr)) };
//         self.length += 1;
//     }
//
//     pub fn append(&mut self, data: T) {
//         if self.head.is_none() {
//             self.prepend(data);
//         }
//
//         unsafe {
//             let mut new_node = Box::new(Node::new(data));
//             new_node.prev = Some(Box::from_raw(self.tail.unwrap()));
//             new_node.next = None;
//
//             // here we have to create a temporary raw pointer for new node because we have to pass
//             // the reference to the new_node in self.tail.next and the n move the new_node as new self.tail
//             let temp_raw_ptr = Box::into_raw(new_node);
//             (*self.tail.unwrap()).next = Some(Box::from_raw(temp_raw_ptr));
//             self.tail = Some(temp_raw_ptr);
//         }
//     }
//
//     pub fn insert_at(&mut self, data: T, index: usize) {
//         if self.length < index {
//             // panic!("can not insert : list is not big enough");
//             println!("can not insert : list is not big enough");
//             return;
//         }
//
//         if self.head.is_none() {
//             self.prepend(data);
//             return;
//         }
//
//         let mut current = &mut self.head;
//
//         for _x in 0..index {
//             current = &mut current.as_mut().unwrap().next;
//         }
//
//         let mut new_node = Box::new(Node::new(data));
//         new_node.next = current.as_mut().unwrap().next.take();
//         // to convert &mut Option<> to Option you can use .take() or mem::replace()
//         new_node.prev = current.take();
//         *current = Some(new_node);
//
//         self.length += 1;
//     }
//
//     pub fn view(&mut self) {
//         // while let Some()
//         println!("-----------------------------------------------------------------------------");
//         println!("{:#?}", self.head);
//
//         // let mut current = &mut self.head;
//         // while let Some(node) = current {
//         //     print!("{:#?} -> ", node);
//         //     current = &mut node.next;
//         // }
//         // println!("None");
//         // println!("the length up until now is : {}", self.length / 2);
//         println!("-----------------------------------------------------------------------------");
//     }
// }

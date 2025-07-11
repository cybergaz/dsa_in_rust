use std::{cell::RefCell, fmt::Debug, rc::Rc};

// ---------------------------------------------------------------------------------------------------------
// a safe implementation using Rc and RefCell
// ---------------------------------------------------------------------------------------------------------
type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;

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

// Node constructor
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
    // DoublyLinkedList constructor
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // 1. prepend : add a node at the beginning of the list
    pub fn prepend(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

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

    // 2. append : add a node at the end of the list
    pub fn append(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if self.is_empty() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            new_node.borrow_mut().prev = self.tail.clone();
            self.tail.as_ref().unwrap().borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }

        self.length += 1;
    }

    // 3. insert_at : add a node at a given index
    pub fn insert_at(&mut self, data: T, index: usize) {
        if index == 0 {
            self.prepend(data);
        } else if index > self.length {
            println!("hehe lol : list is not big enough");
            return;
        } else if index == self.length {
            self.append(data);
        } else {
            let mut current = self.head.clone();

            for _x in 0..index - 1 {
                current = current.unwrap().borrow().next.clone();
            }
            let new_node = Rc::new(RefCell::new(Node::new(data)));

            new_node.borrow_mut().next = current.as_ref().unwrap().borrow().next.clone();
            new_node.borrow_mut().prev = current.clone();
            current.as_ref().unwrap().borrow_mut().next = Some(new_node.clone());
            new_node
                .borrow_mut()
                .next
                .as_ref()
                .unwrap()
                .borrow_mut()
                .prev = Some(new_node.clone());

            self.length += 1;
        }
    }

    // 4. pop_front : remove a node from the beginning of the list
    pub fn pop_front(&mut self) {
        if self.is_empty() {
            panic!("List is empty");
        } else if self.length == 1 {
            self.head = None;
            self.tail = None;
        } else {
            // get the reference of next node of head
            let next = self.head.as_ref().unwrap().borrow().next.clone();
            // next node prev will point to None
            next.as_ref().unwrap().borrow_mut().prev = None;
            // head will point to next node, so first node will be removed
            self.head = next;
        }

        self.length -= 1;
    }

    // 5. pop_at : remove a node from a given index
    pub fn pop_at(&mut self, index: usize) {
        if self.is_empty() {
            panic!("List is empty");
        } else if index == 0 {
            self.pop_front();
        } else if index == self.length {
            self.pop_back();
        } else if index > self.length {
            panic!("Index out of bound");
        } else {
            let mut current = self.head.clone();

            for _ in 0..index {
                current = current.unwrap().borrow().next.clone();
            }

            // get the reference of next node of current node
            let next = current.as_ref().unwrap().borrow().next.clone();
            // next node prev will point to current node
            next.as_ref()
                .unwrap()
                .borrow_mut()
                .prev
                .clone_from(&current.as_ref().unwrap().borrow().prev);
            // prev node next will point to next node
            current.as_ref().unwrap().borrow_mut().next = next.clone();

            self.length -= 1;
        }
    }

    // 6. pop_back : remove a node from the end of the list
    pub fn pop_back(&mut self) {
        if self.is_empty() {
            panic!("List is empty");
        } else if self.length == 1 {
            self.head = None;
            self.tail = None;
        } else {
            // get the reference of prev node of tail
            let prev = self.tail.as_ref().unwrap().borrow().prev.clone();
            // prev node next will point to None
            prev.as_ref().unwrap().borrow_mut().next = None;
            // tail will point to prev node, so last node will be removed
            self.tail = prev;
        }

        self.length -= 1;
    }

    // 7. view : print the list
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
        // println!("the length up until now is : {}", self.lengthgth / 2);
        println!("-----------------------------------------------------------------------------");
    }
}

// #[test]
fn _test_prepend() {
    let mut list = DoublyLinkedList::new();
    assert_eq!(list.length, 0);
    assert!(list.is_empty());

    list.prepend(1);
    assert_eq!(list.length, 1);
    assert!(!list.is_empty());

    list.prepend(2);
    assert_eq!(list.length, 2);
    assert!(!list.is_empty());
}

// #[test]
fn _test_insert_at() {
    let mut list = DoublyLinkedList::new();
    assert_eq!(list.length, 0);
    assert!(list.is_empty());

    list.insert_at(1, 0);
    assert_eq!(list.length, 1);
    assert!(!list.is_empty());

    list.insert_at(2, 1);
    assert_eq!(list.length, 2);
    assert!(!list.is_empty());

    list.insert_at(3, 1);
    assert_eq!(list.length, 3);
    assert!(!list.is_empty());
}

// #[test]
fn _test_append() {
    let mut list = DoublyLinkedList::new();
    assert_eq!(list.length, 0);
    assert!(list.is_empty());

    list.append(1);
    assert_eq!(list.length, 1);
    assert!(!list.is_empty());

    list.append(2);
    assert_eq!(list.length, 2);
    assert!(!list.is_empty());
}

// #[test]
fn _test_pop_front() {
    let mut list = DoublyLinkedList::new();
    assert_eq!(list.length, 0);
    assert!(list.is_empty());

    list.append(1);
    list.append(2);
    list.append(3);
    assert_eq!(list.length, 3);
    assert!(!list.is_empty());

    list.pop_front();
    assert_eq!(list.length, 2);
    assert!(!list.is_empty());

    list.pop_front();
    assert_eq!(list.length, 1);
    assert!(!list.is_empty());

    list.pop_front();
    assert_eq!(list.length, 0);
    assert!(list.is_empty());
}

// #[test]
fn _test_pop_at() {
    let mut list = DoublyLinkedList::new();
    assert_eq!(list.length, 0);
    assert!(list.is_empty());

    list.prepend(1);
    assert_eq!(list.length, 1);
    list.append(2);
    list.append(3);
    assert_eq!(list.length, 3);
    assert!(!list.is_empty());

    list.pop_at(1);
    assert_eq!(list.length, 2);
    assert!(!list.is_empty());

    list.pop_at(1);
    assert_eq!(list.length, 1);
    assert!(!list.is_empty());

    list.pop_at(0);
    assert_eq!(list.length, 0);
    assert!(list.is_empty());
}

// #[test]
fn _test_pop_back() {
    let mut list = DoublyLinkedList::new();
    assert_eq!(list.length, 0);
    assert!(list.is_empty());

    list.append(1);
    list.append(2);
    list.append(3);
    assert_eq!(list.length, 3);
    assert!(!list.is_empty());

    list.pop_back();
    assert_eq!(list.length, 2);
    assert!(!list.is_empty());

    list.pop_back();
    assert_eq!(list.length, 1);
    assert!(!list.is_empty());

    list.pop_back();
    assert_eq!(list.length, 0);
    assert!(list.is_empty());
}

// ---------------------------------------------------------------------------------------------------------
// an unsafe implementation without actually considering 'prev' (it was not possible without Rc)
// ---------------------------------------------------------------------------------------------------------

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
//         self.lengthgth += 1;
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
//         if self.lengthgth < index {
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
//         self.lengthgth += 1;
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
//         // println!("the length up until now is : {}", self.lengthgth / 2);
//         println!("-----------------------------------------------------------------------------");
//     }
// }

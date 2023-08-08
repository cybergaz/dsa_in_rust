use std::{borrow::BorrowMut, usize};

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;
// ------------------------------------------------------------------------------
// defining our Node and List structs
// ------------------------------------------------------------------------------
type Joker<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T: Copy> {
    data: T,
    next: Joker<T>,
}

pub struct SinglyLinkedList<T: Copy> {
    head: Joker<T>,
    length: usize,
}

// ------------------------------------------------------------------------------
// constructors for Node
// ------------------------------------------------------------------------------
impl<T: Copy> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
    fn new_next(data: T, next: Joker<T>) -> Self {
        Node { data, next }
    }
}

// ------------------------------------------------------------------------------
// methods for out singly linked lists
// ------------------------------------------------------------------------------
impl<T: std::fmt::Debug + Copy> SinglyLinkedList<T> {
    // --------------------------------------------------------------------------
    // constructor for Linked List
    // --------------------------------------------------------------------------
    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            length: 0,
        }
    }

    // --------------------------------------------------------------------------
    // pushing element at front
    // --------------------------------------------------------------------------
    pub fn prepend(&mut self, data: T) {
        let new_node = Box::new(Node::new_next(data, self.head.take()));

        self.head = Some(new_node);

        self.length += 1;
    }

    // --------------------------------------------------------------------------
    // pushing element at end
    // --------------------------------------------------------------------------
    pub fn append(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let mut current = self.head.borrow_mut();

        while let Some(ref mut ghost) = current {
            // you can use this also :
            //
            // if ghost.next.is_none() {
            //     ghost.next = Some(new_node);
            //     return;
            // }
            current = &mut ghost.next;
        }

        *current = Some(new_node);

        self.length += 1;
    }

    // --------------------------------------------------------------------------
    // pushing element at a given index
    // --------------------------------------------------------------------------
    pub fn insert_at(&mut self, data: T, index: usize) {
        if self.length < index {
            println!("list is not big enough ");
            return;
        }

        if self.head.is_none() {
            self.prepend(data);
            return;
        }

        let mut current = &mut self.head;

        // loop will take you to 1 node previous to the insertion point
        for _x in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }

        let new_node = Box::new(Node::new_next(data, current.take()));

        // we have to deref the last settled value of current , cause we don't have to find it's
        // next anymore , it is the right place for insertion
        *current = Some(new_node);

        self.length += 1;
    }

    // --------------------------------------------------------------------------
    // finding middle of the linked list using two pointers
    // --------------------------------------------------------------------------
    pub fn find_mid(&mut self) {
        let mut slow_ptr = &self.head;
        let mut fast_ptr = &self.head;

        let mut temp_len = 0;

        while fast_ptr.is_some() && fast_ptr.as_ref().unwrap().next.is_some() {
            fast_ptr = &fast_ptr.as_ref().unwrap().next.as_ref().unwrap().next;
            slow_ptr = &slow_ptr.as_ref().unwrap().next;
            temp_len += 1;
        }

        println!(
            "the middle element is at index {} with value {:#?}",
            temp_len,
            slow_ptr.as_ref().unwrap().data
        );
    }

    // --------------------------------------------------------------------------
    // pushing element at the middle of the list
    // --------------------------------------------------------------------------
    pub fn insert_mid(&mut self, data: T) {
        let mut midlength = self.length / 2;

        if midlength % 2 != 0 {
            midlength += 1;
        }

        let mut current = &mut self.head;

        for _x in 0..midlength {
            current = &mut current.as_mut().unwrap().next;
        }

        let new_node = Box::new(Node::new_next(data, current.take()));

        *current = Some(new_node);
        self.length += 1;
    }

    // --------------------------------------------------------------------------
    // removing head node
    // --------------------------------------------------------------------------
    pub fn pop_front(&mut self) -> Option<T> {
        self.length -= 1;
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // --------------------------------------------------------------------------
    // removing tail node
    // --------------------------------------------------------------------------
    pub fn pop_end(&mut self) -> Option<T> {
        self.length -= 1;
        let mut current = &mut self.head;
        while current.as_mut().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }

        current.take().map(|node| node.data)
    }

    // --------------------------------------------------------------------------
    // removing element at particular index
    // --------------------------------------------------------------------------
    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        self.length -= 1;
        let mut current = &mut self.head;

        for _x in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }
        // current.as_mut().unwrap().next =
        //     current.as_mut().unwrap().next.as_mut().unwrap().next.take();
        let mut node = current.take();
        *current = node.as_mut().unwrap().next.take();

        Some(node.unwrap().data)
    }

    // --------------------------------------------------------------------------
    // visualising our linked list
    // --------------------------------------------------------------------------
    pub fn view(&mut self) {
        // while let Some()
        println!("-----------------------------------------------------------------------------");
        // println!("{:#?}", self.head);

        let mut current = &mut self.head;
        while let Some(node) = current {
            print!("{:#?} -> ", node.data);
            current = &mut node.next;
        }
        println!("None");
        // println!("the length up until now is : {}", self.length / 2);
        println!("-----------------------------------------------------------------------------");
    }
}

#[test]
fn testing() {
    //
    let mut list = SinglyLinkedList::new();
    list.prepend(23);
    list.prepend(9);
    list.prepend(232);

    list.view();

    list.insert_at(37, 3);
    // list.insert_at(38, 0);
    // list.insert_at(39, 9);
    list.view();

    list.append(100);
    list.view();

    list.find_mid();
    list.insert_mid(879);
    list.view();

    println!("hehe -- popped => {:#?}", list.pop_at(3).unwrap());
    list.view();

    // let mut list = SinglyLinkedList::new();
    // list.prepend("heeh".to_string());
    // list.prepend("hello world".to_string());
    // list.prepend("lets figureit out".to_string());
    // list.prepend("ah.. shit , here we go again".to_string());
    // list.view();

    // println!("{:#?}", list.pop_end().unwrap());
    // println!("{:#?}", list.pop_end().unwrap());
    // println!("{:#?}", list.pop_end().unwrap());
    // list.view();

    // list.find_mid();

    // let current_mem = PEAK_ALLOC.current_usage_as_mb();
    // println!("This program currently uses {} MB of RAM.", current_mem);
    // // list.traverse();
    //
    // let current_mem = PEAK_ALLOC.current_usage_as_kb();
    // println!("This program currently uses {} KB of RAM.", current_mem);
    let peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("\n\n---------------------------------------------------------------------");
    println!("\t  The max amount of memory used : {} KB", peak_mem);
    println!("---------------------------------------------------------------------");
}

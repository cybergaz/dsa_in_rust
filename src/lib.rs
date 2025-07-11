pub mod searching {
    #[derive(Debug, PartialEq)]
    pub enum Status {
        Found(i64),
        NotFound,
    }

    pub mod binary_search;
    pub mod exponential_search;
    pub mod jump_search;
    pub mod ternary_search;
}

pub mod sorting {
    pub mod bubble;
    pub mod quick_sort;
}

pub mod common {
    pub mod linked_list {
        pub mod doubly;
        pub mod singly;
    }
    pub mod queue;
    pub mod stack;

    pub mod tree {
        pub mod binary;
        pub mod n_ary;
    }
}

// "pub use" is used to re-export the modules and functions to the parent module
// ( means shorten the path when importing the module or function or struct or whatever )
pub use common::linked_list::doubly::DoublyLinkedList;
// this means from where i stand in the module tree (src/lib.rs) i can access the DoublyLinkedList
// struct directly from here when importing the structs in other files

fn _testing_func() {
    // hehe
}

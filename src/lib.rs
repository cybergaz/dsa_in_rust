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
}

pub mod common {
    pub mod linked_list {
        pub mod doubly;
        pub mod singly;
        pub mod temp;
    }
    pub mod queue;
    pub mod stack;
}

fn _testing_func() {
    // hehe
}

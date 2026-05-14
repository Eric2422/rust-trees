use std::mem;

pub struct LinkedListNode<T> {
    pub data: Option<Box<T>>,
    pub next: Option<Box<LinkedListNode<T>>>,
}

enum Link<T> {
    Empty,
    More(Box<LinkedListNode<T>>),
}

pub struct LinkedList<T> {
    pub head: LinkedListNode<T>,
    pub tail: LinkedListNode<T>,
    pub size: i32,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        let linked_list: LinkedList<T> = LinkedList {
            size: 0,
            head: LinkedListNode {
                data: None,
                next: None,
            },
            tail: LinkedListNode {
                data: None,
                next: None,
            },
        };

        linked_list
    }

    pub fn add_to_head(&mut self, data: T) -> i32 {
        self.size += 1;

        self.size
    }

    pub fn remove_index(index: i32) -> i32 {
        0
    }
}

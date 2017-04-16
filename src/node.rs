use std::cmp::PartialEq;

pub struct Node<T> {
    id: u32,
    data: T
}

impl<T: Default> Default for Node<T> {

    fn default() -> Node<T> {
        Node::create(T::default())
    }
}

impl<T> PartialEq for Node<T> {

    fn eq(&self, other: &Node<T>) -> bool {
        self.id == other.id
    }
}

fn get_id() -> u32 {
    static mut NEXT_ID: u32 = 0;

    unsafe {
        NEXT_ID += 1;
        NEXT_ID - 1
    }
}

impl<T> Node<T> {

    pub fn create(data: T) -> Node<T> {
        Node {
            id: get_id(),
            data: data
        }
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }

}

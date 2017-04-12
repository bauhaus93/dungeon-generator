
use connection::Connection;

pub struct Node<'a> {
    connection: Vec<Connection<'a>>
}

impl<'a> Default for Node<'a> {

    fn default() -> Node<'a> {
        Node::create()
    }
}

impl<'a> Node<'a> {

    pub fn create() -> Node<'a> {
        Node {
            connection: Vec::new()
        }
    }


}

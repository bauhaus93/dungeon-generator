use node::Node;

pub struct Connection<'a> {
    node: [&'a Node<'a>; 2],
    active: bool
}


impl<'a> Connection<'a> {

    pub fn create(node_a: &'a Node<'a>, node_b: &'a Node<'a>) -> Connection<'a> {

    }
}

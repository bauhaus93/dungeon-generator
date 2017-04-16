

pub struct NodeInfo {
    pos: (i32, i32)
}

impl Default for NodeInfo {

    fn default() -> NodeInfo {
        NodeInfo {
            pos: (0, 0)
        }
    }
}

impl NodeInfo {

    pub fn new(x: i32, y: i32) -> NodeInfo {
        NodeInfo {
            pos: (x, y)
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.pos
    }
}

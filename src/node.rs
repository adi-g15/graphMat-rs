use generational_arena::Index as IndexInArena;

// Node is an internal type, caller should not directly read it
#[derive(Debug)]
pub(crate) struct Node<T> {
    data: Option<T>,    // when this is just a node to get to some other node, then it is None
    pub coord: (i32,i32,i32),

    // Make it Mutex, for multi-thread use
    // Way 2: Message passing, node sends index, GraphMat replies with node, but that is slow since blocks
    pub(crate) north: Option<IndexInArena>,
    pub(crate) east: Option<IndexInArena>,
    pub(crate) sky: Option<IndexInArena>,
}

impl<T> Node<T> {
    pub fn new(data: Option<T>, coord: (i32,i32,i32)) -> Self {
        Node {
            data,
            coord,

            north: None,
            east: None,
            sky: None,
        }
    }

    pub fn get<'a>(&'a self) -> Option<&'a T> {
        self.data.as_ref()
    }

    pub fn get_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        self.data.as_mut()
    }

    pub fn set(&mut self, data: T) {
        self.data.replace(data);
    }

}

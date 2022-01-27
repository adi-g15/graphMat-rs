use crate::node::Node;
use std::{cell::RefCell, collections::HashMap};
use generational_arena::{Arena, Index};

pub struct GraphMat<T> {
    arena: Arena<Node<T>>,
    map: HashMap<(i32, i32, i32), Index>,
}

impl<T> GraphMat<T> {
    pub fn new() -> Self {
        GraphMat {
            arena: Arena::new(),
            map: HashMap::new(),
        }
    }

    // Note: This function does NOT modify `self.map`, do it in the other functions
    fn allocate_one_node(&mut self, data: T) -> Index {
        self.arena.insert( Node::new( data ) )
    }

    pub fn get<'a>(&'a self, coord: &(i32, i32, i32)) -> Option<&'a T> {
        match self.map.get(&coord) {
            None => None,
            Some(node_idx) => Some(self.arena.get(*node_idx).unwrap().get()),
        }
    }

    pub fn get_mut<'a>(&'a mut self, coord: &(i32, i32, i32)) -> Option<&'a mut T> {
        match self.map.get(&coord) {
            None => None,
            Some(node_idx) => Some(self.arena.get_mut(*node_idx).unwrap().get_mut()),
        }
    }

    pub fn set(&mut self, coord: (i32, i32, i32), data: T) {
        match self.map.get(&coord) {
            None => {
                let node = self.allocate_one_node(data);

                self.map.insert(coord, node);
            },
            Some(idx) => {
                // SAFETY: .unwrap() is safe, since if in this branch that means, map contains a value
                // at (coord)
                self.arena.get_mut(*idx).unwrap().set(data)
            }
        }
    }
}

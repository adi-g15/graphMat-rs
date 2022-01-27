use std::cell::Ref;
use std::{rc::Rc, cell::RefCell, collections::HashMap};
use std::sync::mpsc;
use crate::node::{Node, ContactUnit};

pub struct GraphMat<T> {
    arena: Vec<Rc<RefCell<Node<T>>>>,
    map: HashMap<(i32,i32,i32), Rc<RefCell<Node<T>>>>,
    freed: Vec<usize>
}

impl<T> GraphMat<T> {
    pub fn new() -> Self {
        GraphMat {
            arena: Vec::new(),
            map: HashMap::new(),
            freed: Vec::new()
        }
    }

    pub fn get<'a>(&'a self, coord: &(i32,i32,i32)) -> Option<&'a Rc<RefCell<Node<T>>>> {
        match self.map.get(&coord) {
            None => None,
            Some(node) => Some(&node)
        }
    }

    pub fn get_mut<'a>(&'a mut self, coord: &(i32,i32,i32)) -> Option<&'a Rc<RefCell<Node<T>>>> {
        self.get(coord)
    }

    /* Why not working?
    pub fn get<'a>(&'a self, coord: (i32,i32,i32)) -> Option<&'a T> {
        match self.map.get(&coord) {
            None => None,
            Some(node) => {
                let borrow = &node.borrow();
                Some(&borrow.data)
            }
        }
    }
    */

    pub fn set(&mut self, coord: (i32,i32,i32), data: T) {
        if self.map.contains_key(&coord) == false {
            let (mut tx, rx) = mpsc::channel();
            let node = Rc::new( RefCell::new( Node::new(data, tx.clone()) ) );

            tx.send(ContactUnit::NewBox((1,2,3)));

            self.arena.push(node.clone());
            self.map.insert(coord, node.clone());
        } else {
            // SAFETY: .unwrap() is safe, since if in this branch that means, map contains a value
            // at (coord)
            self.map.get_mut(&coord).unwrap().borrow_mut().set(data);
        }
    }
}


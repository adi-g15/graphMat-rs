use std::sync::mpsc::Sender;

use crate::direction::Direction;
use crate::graphmat::{GraphMat, self};

pub enum ContactUnit {
    NewBox((u32,u32,u32))
}

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,

    // WARNING: When using this, remove yourself from being a mutable borrow
    mpsc_tx: Sender<ContactUnit>,

    north: usize,
    east: usize,
    west: usize,
    south: usize,
    sky: usize,
    earth: usize,

    // now directions according to our texts
    ishanya: usize,
    agneya: usize,
    nairutya: usize,
    vayavya: usize,
    urdhwa: usize
}

impl<'a, T> Node<T> {
    pub fn new(data: T, mpsc_tx: Sender<ContactUnit> ) -> Self {
        Node {
            data,
            mpsc_tx,

            north: 0,
            east: 0,
            west: 0,
            south: 0,
            sky: 0,
            earth: 0,

            ishanya: 0,
            agneya: 0,
            nairutya: 0,
            vayavya: 0,
            urdhwa: 0
        }
    }

    pub fn set(&mut self, data: T) {
        self.data = data;
    }
    /*
    fn get_neighbour_index(dir: Direction) {
        match dir {
            Direction::uttar => {},
            Direction::purva => {},
            Direction:: => {},
            Direction:: => {},
            Direction:: => {},
            Direction:: => {},
            Direction:: => {},
            Direction:: => {},
            Direction:: => {},
            Direction:: => {},
        }
    }
    */
}


use generational_arena::Index as IndexInArena;

use crate::direction::Direction;

pub enum ContactUnit {
    NewBox((u32, u32, u32)),
}

// Node is an internal type, caller should not directly read it
#[derive(Debug)]
pub(crate) struct Node<T> {
    data: T,
    pub coord: (i32,i32,i32),

    // Make it Mutex, for multi-thread use
    // Way 2: Message passing, node sends index, GraphMat replies with node, but that is slow since blocks
    north: Option<IndexInArena>,
    east: Option<IndexInArena>,
    west: Option<IndexInArena>,
    south: Option<IndexInArena>,
    sky: Option<IndexInArena>,
    earth: Option<IndexInArena>,
}

impl<T> Node<T> {
    pub fn new(data: T, coord: (i32,i32,i32)) -> Self {
        Node {
            data,
            coord,

            north: None,
            east: None,
            west: None,
            south: None,
            sky: None,
            earth: None,
        }
    }

    pub fn get<'a>(&'a self) -> &'a T {
        &self.data
    }

    pub fn get_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.data
    }

    pub fn set(&mut self, data: T) {
        self.data = data;
    }

    /*pub(in crate::graphmat)*/
    pub(crate) fn set_neighbour(&mut self, dir: Direction, node_idx: IndexInArena) {
        match dir {
            Direction::uttar => self.north,
            Direction::purva => self.east,
            Direction::paschim => self.west,
            Direction::dakshin => self.south,

            Direction::urdhwa => self.sky,
            Direction::adharastha => self.earth,

            _ => panic!("Can only assign direct neighbours"),
        }.replace(node_idx);
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

use generational_arena::Index as IndexInArena;

use crate::direction::Direction;
use crate::graphmat::GraphMat;

fn inc_coord(coord: (i32, i32, i32), increment: (i32, i32, i32)) -> (i32, i32, i32) {
    (
        coord.0 + increment.0,
        coord.1 + increment.1,
        coord.2 + increment.2,
    )
}

pub struct GraphMatFreeIterator<'a, T> {
    pub(crate) graphmat: &'a mut GraphMat<T>,
    pub curr_pos: (i32, i32, i32),
    pub curr_node_idx: Option<IndexInArena>,
    pub curr_dir: Direction,
}

impl<'a, T> GraphMatFreeIterator<'a, T> {
    /**
     * @returns Previous 'direction of iteration'
     */
    pub fn set_direction(&mut self, dir: Direction) -> Direction {
        let old_dir = self.curr_dir.clone();

        self.curr_dir = dir;

        old_dir
    }
}

impl<'a, T> GraphMatFreeIterator<'a, T> {
    pub fn next<'b>(&'b mut self) -> Option<((i32,i32,i32), &'b T)> {
        let prev_pos = self.curr_pos.clone();
        let prev_node_idx = match self.curr_node_idx {
            None => { return None; },
            Some(idx) => idx
        };

        let prev_node = self.graphmat.arena.get(prev_node_idx).unwrap();

        (self.curr_pos, self.curr_node_idx) = match self.curr_dir {
            Direction::uttar => {
                (inc_coord(self.curr_pos, (0, 1, 0)), prev_node.north)
            },
            Direction::purva => {
               (inc_coord(self.curr_pos, (1, 0, 0)), prev_node.east)
            },
            Direction::paschim => {
                let curr_pos = inc_coord(self.curr_pos, (-1, 0, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },
            Direction::dakshin => {
                let curr_pos = inc_coord(self.curr_pos, (0, -1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },

            Direction::vayavya => {
                let curr_pos = inc_coord(self.curr_pos, (-1, 1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },
            Direction::ishanya => {
                let curr_pos = inc_coord(self.curr_pos, (1, 1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },
            Direction::nairutya => {
                let curr_pos = inc_coord(self.curr_pos, (-1, -1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },
            Direction::agneya => {
                let curr_pos = inc_coord(self.curr_pos, (1, -1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },

            Direction::urdhwa => {
                (inc_coord(self.curr_pos, (1, 0, 0)), prev_node.sky)
            },
            Direction::adharastha => {
                let curr_pos = inc_coord(self.curr_pos, (0, 0, -1));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            }
        };

        match prev_node.get() {
            None => None,
            Some(prev_node) => Some((prev_pos, prev_node))
        }
    }
}

pub struct GraphMatIterator<'a, T, const DIR: Direction> {
    pub(crate) graphmat: &'a mut GraphMat<T>,
    pub curr_node_idx: Option<IndexInArena>,
    pub curr_pos: (i32, i32, i32),
}

// Implementing `Iterator`, causes lifetime mismatch, it expects the function signatures to match,
// and for that the lifetime of the returns Self::Item, be the anonymous/annotated lifetime of the impl block,
// but the actual reference I am returning has the lifetime of the `fn next`,
// which it says "this lifetime may not always outlive that" some message like it
// 
// Can just try to see that error
impl<'a, T, const DIR: Direction> GraphMatIterator<'a, T, DIR> {
    pub fn next<'b>(&'b mut self) -> Option<((i32,i32,i32), &'b T)> {
        // TODO: There is good scope of improvement here, instead of using graphmat.get() everytime, try using the node to get neighbours
        let prev_pos = self.curr_pos.clone();
        let prev_node_idx = match self.curr_node_idx {
            None => { return None; },
            Some(idx) => idx
        };

        let prev_node = self.graphmat.arena.get(prev_node_idx).unwrap();

        (self.curr_pos, self.curr_node_idx) = match DIR {
            Direction::uttar => {
                (inc_coord(self.curr_pos, (0, 1, 0)), prev_node.north)
            },
            Direction::purva => {
               (inc_coord(self.curr_pos, (1, 0, 0)), prev_node.east)
            },
            Direction::paschim => {
                let curr_pos = inc_coord(self.curr_pos, (-1, 0, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },
            Direction::dakshin => {
                let curr_pos = inc_coord(self.curr_pos, (0, -1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },

            Direction::vayavya => {
                let curr_pos = inc_coord(self.curr_pos, (-1, 1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },
            Direction::ishanya => {
                let curr_pos = inc_coord(self.curr_pos, (1, 1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },
            Direction::nairutya => {
                let curr_pos = inc_coord(self.curr_pos, (-1, -1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },
            Direction::agneya => {
                let curr_pos = inc_coord(self.curr_pos, (1, -1, 0));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            },

            Direction::urdhwa => {
                (inc_coord(self.curr_pos, (1, 0, 0)), prev_node.sky)
            },
            Direction::adharastha => {
                let curr_pos = inc_coord(self.curr_pos, (0, 0, -1));
                let node_idx = self.graphmat.get_node_index(curr_pos);

                (curr_pos, node_idx)
            }
        };

        match prev_node.get() {
            None => None,
            Some(prev_node) => Some((prev_pos, prev_node))
        }
    }
}

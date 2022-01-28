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
        let prev_element = self.graphmat.get(self.curr_pos);
        let prev_pos = self.curr_pos.clone();

        self.curr_pos = match self.curr_dir {
            Direction::uttar =>
                inc_coord(self.curr_pos, (0, 1, 0)),
            Direction::purva =>
                inc_coord(self.curr_pos, (1, 0, 0)),
            Direction::paschim =>
                inc_coord(self.curr_pos, (-1, 0, 0)),
            Direction::dakshin =>
                inc_coord(self.curr_pos, (0, -1, 0)),

            Direction::vayavya =>
                inc_coord(self.curr_pos, (-1, 1, 0)),
            Direction::ishanya =>
                inc_coord(self.curr_pos, (1, 1, 0)),
            Direction::nairutya =>
                inc_coord(self.curr_pos, (-1, -1, 0)),
            Direction::agneya =>
                inc_coord(self.curr_pos, (1, -1, 0)),

            Direction::urdhwa =>
                inc_coord(self.curr_pos, (0, 0, 1)),
            Direction::adharastha =>
                inc_coord(self.curr_pos, (0, 0, -1))
        };

        match prev_element {
            Some(prev_element) => Some((prev_pos, prev_element)),
            None => None
        }
    }
}

pub struct GraphMatIterator<'a, T, const DIR: Direction> {
    pub(crate) graphmat: &'a mut GraphMat<T>,
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
        // TODO: There is HUGE scope of improvement here, instead of using graphmat.get() for coords, instead use the node to get neighbours
        let prev_pos = self.curr_pos.clone();
        let prev_element = self.graphmat.get(prev_pos);

        self.curr_pos = match DIR {
            Direction::uttar =>
                inc_coord(self.curr_pos, (0, 1, 0)),
            Direction::purva =>
                inc_coord(self.curr_pos, (1, 0, 0)),
            Direction::paschim =>
                inc_coord(self.curr_pos, (-1, 0, 0)),
            Direction::dakshin =>
                inc_coord(self.curr_pos, (0, -1, 0)),

            Direction::vayavya =>
                inc_coord(self.curr_pos, (-1, 1, 0)),
            Direction::ishanya =>
                inc_coord(self.curr_pos, (1, 1, 0)),
            Direction::nairutya =>
                inc_coord(self.curr_pos, (-1, -1, 0)),
            Direction::agneya =>
                inc_coord(self.curr_pos, (1, -1, 0)),

            Direction::urdhwa =>
                inc_coord(self.curr_pos, (0, 0, 1)),
            Direction::adharastha =>
                inc_coord(self.curr_pos, (0, 0, -1))
        };

        match prev_element {
            Some(prev_element) => Some((prev_pos, prev_element)),
            None => None
        }
    }
}

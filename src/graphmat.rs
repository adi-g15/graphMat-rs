use crate::iterators::{GraphMatFreeIterator, GraphMatIterator};
use crate::{direction::Direction, node::Node};
use generational_arena::{Arena, Index};
use std::collections::HashMap;

pub struct GraphMat<T> {
    pub(crate) arena: Arena<Node<T>>,
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
    fn allocate_one_node(&mut self, data: T, coord: (i32, i32, i32)) -> Index {
        self.arena.insert(Node::new(Some(data), coord))
    }

    // Note: This function does NOT modify `self.map`, do it in the other functions
    fn default_allocate_one_node(&mut self, coord: (i32, i32, i32)) -> Index {
        self.arena.insert(Node::new(None, coord))
    }

    pub fn get<'a>(&'a self, coord: (i32, i32, i32)) -> Option<&'a T> {
        // let leader_coord = ((coord.0 / 2) * 2, (coord.1 / 2) * 2, (coord.2 / 2) * 2);
        let leader_coord = (
            if coord.0 % 2 == 0 { coord.0 } else { coord.0 - 1 },
            if coord.1 % 2 == 0 { coord.1 } else { coord.1 - 1 },
            if coord.2 % 2 == 0 { coord.2 } else { coord.2 - 1 }
        );

        if coord == leader_coord {
            return match self.map.get(&coord) {
                None => None,
                Some(node_idx) => {
                    // SAFETY: If self.map says this index refers to a node, it must be allocated in self.arena
                    self.arena.get(*node_idx).unwrap().get()
                }
            };
        }

        let leader = match self.map.get(&leader_coord) {
            None => {
                return None;
            }
            Some(idx) => {
                // SAFETY: If self.map says this index refers to a node, it must be allocated in self.arena
                self.arena.get(*idx).unwrap()
            }
        };

        let diff = (
            coord.0 - leader_coord.0,
            coord.1 - leader_coord.1,
            coord.2 - leader_coord.2,
        );
        match diff {
            (1, 0, 0) => {
                // east
                match leader.east {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        self.arena.get(idx).unwrap().get()
                    }
                }
            }
            (0, 1, 0) => {
                // north
                match leader.north {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        self.arena.get(idx).unwrap().get()
                    }
                }
            }
            (0, 0, 1) => {
                // sky
                match leader.sky {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        self.arena.get(idx).unwrap().get()
                    }
                }
            }
            (0, 1, 1) => {
                // north-sky
                match leader.north {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        match self.arena.get(idx).unwrap().sky {
                            None => {
                                return None;
                            }
                            Some(idx) => {
                                // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                self.arena.get(idx).unwrap().get()
                            }
                        }
                    }
                }
            }
            (1, 0, 1) => {
                // east-sky
                match leader.east {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        match self.arena.get(idx).unwrap().sky {
                            None => {
                                return None;
                            }
                            Some(idx) => {
                                // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                self.arena.get(idx).unwrap().get()
                            }
                        }
                    }
                }

            }
            (1, 1, 0) => {
                // north-east
                match leader.north {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        match self.arena.get(idx).unwrap().east {
                            None => {
                                return None;
                            }
                            Some(idx) => {
                                // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                self.arena.get(idx).unwrap().get()
                            }
                        }
                    }
                }

            }
            (1, 1, 1) => {
                // north-east-sky
                match leader.north {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        match self.arena.get(idx).unwrap().east {
                            None => {
                                return None;
                            }
                            Some(idx) => {
                                // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                match self.arena.get(idx).unwrap().sky {
                                    None => {
                                        return None;
                                    }
                                    Some(idx) => {
                                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                        self.arena.get(idx).unwrap().get()
                                    }
                                }
                            }
                        }
                    }
                }
            }

            _ => {
                panic!(
                    "Invalid direction, coord {:?} is probably not a neighbour of {:?}",
                    coord, leader_coord
                );
            }
        }
    }

    pub fn get_mut<'a>(&'a mut self, coord: (i32, i32, i32)) -> Option<&'a mut T> {
        let leader_coord = (
            if coord.0 % 2 == 0 { coord.0 } else { coord.0 - 1 },
            if coord.1 % 2 == 0 { coord.1 } else { coord.1 - 1 },
            if coord.2 % 2 == 0 { coord.2 } else { coord.2 - 1 }
        );

        // let leader_coord = ((coord.0 / 2) * 2, (coord.1 / 2) * 2, (coord.2 / 2) * 2);

        if coord == leader_coord {
            return match self.map.get(&coord) {
                None => None,
                Some(node_idx) => {
                    // SAFETY: If self.map says this index refers to a node, it must be allocated in self.arena
                    self.arena.get_mut(*node_idx).unwrap().get_mut()
                }
            };
        }

        let leader = match self.map.get(&leader_coord) {
            None => {
                return None;
            }
            Some(idx) => {
                // SAFETY: If self.map says this index refers to a node, it must be allocated in self.arena
                self.arena.get(*idx).unwrap()
            }
        };

        let diff = (
            coord.0 - leader_coord.0,
            coord.1 - leader_coord.1,
            coord.2 - leader_coord.2,
        );
        match diff {
            (1, 0, 0) => {
                // east
                match leader.east {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        self.arena.get_mut(idx).unwrap().get_mut()
                    }
                }
            }
            (0, 1, 0) => {
                // north
                match leader.north {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        self.arena.get_mut(idx).unwrap().get_mut()
                    }
                }
            }
            (0, 0, 1) => {
                // sky
                match leader.sky {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        self.arena.get_mut(idx).unwrap().get_mut()
                    }
                }
            }
            (0, 1, 1) => {
                // north-sky
                match leader.north {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        match self.arena.get(idx).unwrap().sky {
                            None => {
                                return None;
                            }
                            Some(idx) => {
                                // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                self.arena.get_mut(idx).unwrap().get_mut()
                            }
                        }
                    }
                }
            }
            (1, 0, 1) => {
                // east-sky
                match leader.east {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        match self.arena.get(idx).unwrap().sky {
                            None => {
                                return None;
                            }
                            Some(idx) => {
                                // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                self.arena.get_mut(idx).unwrap().get_mut()
                            }
                        }
                    }
                }

            }
            (1, 1, 0) => {
                // north-east
                match leader.north {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        match self.arena.get(idx).unwrap().east {
                            None => {
                                return None;
                            }
                            Some(idx) => {
                                // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                self.arena.get_mut(idx).unwrap().get_mut()
                            }
                        }
                    }
                }

            }
            (1, 1, 1) => {
                // north-east-sky
                match leader.north {
                    None => {
                        return None;
                    }
                    Some(idx) => {
                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                        match self.arena.get(idx).unwrap().east {
                            None => {
                                return None;
                            }
                            Some(idx) => {
                                // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                match self.arena.get(idx).unwrap().sky {
                                    None => {
                                        return None;
                                    }
                                    Some(idx) => {
                                        // SAFETY: If a node says this index refers to a neighbour node, it must be allocated in self.arena
                                        self.arena.get_mut(idx).unwrap().get_mut()
                                    }
                                }
                            }
                        }
                    }
                }
            }

            _ => {
                panic!(
                    "Invalid direction, coord {:?} is probably not a neighbour of {:?}",
                    coord, leader_coord
                );
            }
        }
    }

    pub fn set(&mut self, coord: (i32, i32, i32), data: T) {
        let leader_coord = (
            if coord.0 % 2 == 0 { coord.0 } else { coord.0 - 1 },
            if coord.1 % 2 == 0 { coord.1 } else { coord.1 - 1 },
            if coord.2 % 2 == 0 { coord.2 } else { coord.2 - 1 }
        );
        // !(maybe unncecessary) Optimisation, same result as above (REQUIRED: index is unsigned, for eg. for (-1,0,0) it will say leader is (0,0,0) which is wrong, it should be (-2,0,0))
        // let leader_coord = ((coord.0 / 2) * 2, (coord.1 / 2) * 2, (coord.2 / 2) * 2);

        if coord == leader_coord {
            match self.map.get(&leader_coord) {
                None => {
                    let index = self.allocate_one_node(data, leader_coord);
                    self.map.insert(leader_coord, index);
                }
                Some(idx) => {
                    // SAFETY: Since self.map.get() is NOT None, that means there is a node at that coord,
                    // and hence the corresponding index MUST be present in arena
                    self.arena.get_mut(*idx).unwrap().set(data);
                }
            }

            return;
        }

        // Invariant: By now, it's establised that coord != cube_center

        if self.map.get(&leader_coord).is_none() {
            // Even if this is not the location that is to be set, this is still required according to current design
            let index = self.default_allocate_one_node(leader_coord);
            self.map.insert(leader_coord, index);
        }

        // ![START] Allocating some nodes in advance

        /*
         * Gaurantees:
         * 1. allocated_node WILL be used, and only once for allocating the particular node
         * 2. Each index will be used ONLY once
         *
         * Notes:
         * 1. Modify coord of allocated_ids[0] & allocated_ids[1] when using
         */

        let extra_allocated_ids = [
            self.default_allocate_one_node(coord),
            self.default_allocate_one_node(coord),
        ];
        let mut i = 0; // points to the 'next' available allocation

        let allocated_node = self.allocate_one_node(data, coord);

        // ![END] Allocating some nodes in advance

        // SAFETY: 1. self.map contains the leader node, as handled by the previous if, so get.unwrap must not fail
        //         2. If self.map contains the index as a value at any coord, self.arena must have allocated it
        let leader = self
            .arena
            .get_mut(self.map.get(&leader_coord).unwrap().clone())
            .unwrap();

        let diff = (
            coord.0 - leader_coord.0,
            coord.1 - leader_coord.1,
            coord.2 - leader_coord.2,
        );
        match diff {
            (1, 0, 0) => {
                // east
                match leader.east {
                    None => {
                        leader.east.replace(allocated_node);
                    }
                    Some(idx) => {
                        // More Correct way is `self.arena.get_mut(idx).unwrap().set(data)`... but `data` was moved earlier for allocating `allocated_node`
                        leader.east.replace(allocated_node);
                        self.arena.remove(idx);
                    }
                }
            }
            (0, 1, 0) => {
                // north
                match leader.north {
                    None => {
                        leader.north.replace(allocated_node);
                    }
                    Some(idx) => {
                        // More Correct way is `self.arena.get_mut(idx).unwrap().set(data)`... but `data` was moved earlier for allocating `allocated_node`
                        leader.north.replace(allocated_node);
                        self.arena.remove(idx);
                    }
                }
            }
            (0, 0, 1) => {
                // sky
                match leader.sky {
                    None => {
                        leader.sky.replace(allocated_node);
                    }
                    Some(idx) => {
                        // More Correct way is `self.arena.get_mut(idx).unwrap().set(data)`... but `data` was moved earlier for allocating `allocated_node`
                        leader.sky.replace(allocated_node);
                        self.arena.remove(idx);
                    }
                }
            }
            (0, 1, 1) => {
                // north-sky
                let leader_north_idx = match leader.north {
                    None => {
                        let index = extra_allocated_ids[i];
                        leader.north.replace(index);

                        // Coordinate of 'north' (ie. y coordinate +1 ) of leader node
                        self.arena.get_mut(index).unwrap().coord =
                            (leader_coord.0, leader_coord.1 + 1, leader_coord.2);
                        i += 1;

                        debug_assert!(i == 1, "By here i (next free extra allocation) must be 1");

                        index
                    }
                    Some(i) => i,
                };

                // SAFETY: If a node contains the index as a neighbour, self.arena must have allocated it
                let leader_north = self.arena.get_mut(leader_north_idx).unwrap();

                match leader_north.sky {
                    None => {
                        leader_north.sky.replace(allocated_node);
                    }
                    Some(idx) => {
                        // More Correct way is `self.arena.get_mut(idx).unwrap().set(data)`... but `data` was moved earlier for allocating `allocated_node`
                        leader_north.sky.replace(allocated_node);
                        self.arena.remove(idx);
                    }
                }
            }
            (1, 0, 1) => {
                // east-sky
                let leader_east_idx = match leader.east {
                    None => {
                        let index = extra_allocated_ids[i];
                        leader.east.replace(index);

                        // Coordinate of 'east' (ie. x coordinate +1 ) of leader node
                        self.arena.get_mut(index).unwrap().coord =
                            (leader_coord.0 + 1, leader_coord.1, leader_coord.2);
                        i += 1;

                        debug_assert!(i == 1, "By here i (next free extra allocation) must be 1");

                        index
                    }
                    Some(i) => i,
                };

                // SAFETY: If a node contains the index as a neighbour, self.arena must have allocated it
                let leader_east = self.arena.get_mut(leader_east_idx).unwrap();

                match leader_east.sky {
                    None => {
                        leader_east.sky.replace(allocated_node);
                    }
                    Some(idx) => {
                        // More Correct way is `self.arena.get_mut(idx).unwrap().set(data)`... but `data` was moved earlier for allocating `allocated_node`
                        leader_east.sky.replace(allocated_node);
                        self.arena.remove(idx);
                    }
                }
            }
            (1, 1, 0) => {
                // north-east
                let leader_north_idx = match leader.north {
                    None => {
                        let index = extra_allocated_ids[i];
                        leader.north.replace(index);

                        // Coordinate of 'north' (ie. y coordinate +1 ) of leader node
                        self.arena.get_mut(index).unwrap().coord =
                            (leader_coord.0, leader_coord.1 + 1, leader_coord.2);
                        i += 1;

                        debug_assert!(i == 1, "By here i (next free extra allocation) must be 1");

                        index
                    }
                    Some(i) => i,
                };

                // SAFETY: If a node contains the index as a neighbour, self.arena must have allocated it
                let leader_north = self.arena.get_mut(leader_north_idx).unwrap();

                match leader_north.east {
                    None => {
                        leader_north.east.replace(allocated_node);
                    }
                    Some(idx) => {
                        // More Correct way is `self.arena.get_mut(idx).unwrap().set(data)`... but `data` was moved earlier for allocating `allocated_node`
                        leader_north.east.replace(allocated_node);
                        self.arena.remove(idx);
                    }
                }
            }
            (1, 1, 1) => {
                // north-east-sky
                let leader_north_idx = match leader.north {
                    None => {
                        let index = extra_allocated_ids[i];
                        leader.north.replace(index);

                        // Coordinate of 'north' (ie. y coordinate +1 ) of leader node
                        self.arena.get_mut(index).unwrap().coord =
                            (leader_coord.0, leader_coord.1 + 1, leader_coord.2);
                        i += 1;

                        debug_assert!(i == 1, "By here i (next free extra allocation) must be 1");

                        index
                    }
                    Some(i) => i,
                };

                // SAFETY: If a node contains the index as a neighbour, self.arena must have allocated it
                let leader_north = self.arena.get_mut(leader_north_idx).unwrap();

                let leader_north_east_idx = match leader_north.east {
                    None => {
                        let index = extra_allocated_ids[i];
                        leader_north.east.replace(index);

                        // Coordinate of 'north-east' (ie. x coordinate +1, y coordinate +1 ) of leader node
                        self.arena.get_mut(index).unwrap().coord =
                            (leader_coord.0 + 1, leader_coord.1 + 1, leader_coord.2);
                        i += 1;

                        debug_assert!(i == 2, "By here i (next free extra allocation) must be 2");

                        index
                    }
                    Some(i) => i,
                };

                // SAFETY: If a node contains the index as a neighbour, self.arena must have allocated it
                let leader_north_east = self.arena.get_mut(leader_north_east_idx).unwrap();

                match leader_north_east.sky {
                    None => {
                        leader_north_east.sky.replace(allocated_node);
                    }
                    Some(idx) => {
                        // More Correct way is `self.arena.get_mut(idx).unwrap().set(data)`... but `data` was moved earlier for allocating `allocated_node`
                        leader_north_east.sky.replace(allocated_node);
                        self.arena.remove(idx);
                    }
                }
            }

            _ => {
                panic!(
                    "Invalid direction, coord {:?} is probably not a neighbour of {:?}",
                    coord, leader_coord
                );
            }
        };

        // Clean up allocated nodes, that were not used
        while i != 2 {
            self.arena.remove(extra_allocated_ids[i]);
            i += 1;
        }
    }

    /**
     * @brief Returns an iterator that allows iterating in ONLY ONE DIRECTION
     *
     * If the direction is NOT known at compile time, use .iter_all_dir()
     */
    pub fn iter<'a, const DIR: Direction>(
        &'a mut self,
        starting_coord: (i32, i32, i32),
    ) -> GraphMatIterator<'a, T, DIR> {
        GraphMatIterator {
            graphmat: self,
            curr_pos: starting_coord,
        }
    }

    /**
     * @brief Returns an iterator that allows iterating in ANY direction
     *
     * If the direction is known at compile time, can also use .iter(), it
     * uses compile-time const generics that provides extra efficiency equaivalent to removal of one `match` statement :)
     */
    pub fn iter_all_dir<'a>(
        &'a mut self,
        starting_coord: (i32, i32, i32),
        starting_dir: Direction,
    ) -> GraphMatFreeIterator<'a, T> {
        GraphMatFreeIterator {
            graphmat: self,
            curr_pos: starting_coord,
            curr_dir: starting_dir,
        }
    }

    pub fn find(&self, value: &T) -> Option<(i32, i32, i32)>
    where
        T: PartialEq,
    {
        // Finding in self.map will be more costly given space optimisation is done there
        // for eg. 100 nodes hai, 10 centers, to har center ke liye recursively search krna hoga
        //         keeping track of all searched nodes, taaki loop me na phase
        for (_, node) in self.arena.iter() {
            if node.get().contains(&value) {
                return Some(node.coord);
            }
        }

        None
    }

    pub fn find_if<UnaryPredicate>(&self, pred: UnaryPredicate) -> Option<(i32, i32, i32)>
    where
        UnaryPredicate: Fn(&T) -> bool,
    {
        for (_, node) in self.arena.iter() {
            // SAFETY: First check assures that node.get() contains something, so node.get().unwrap() must not fail
            if node.get().is_some() && pred(node.get().unwrap()) {
                return Some(node.coord);
            }
        }

        None
    }

    pub fn free_pos(&mut self, coord: (i32, i32, i32)) {
        let index = self.map.get(&coord).clone();

        if let Some(idx) = index {
            let idx = idx.clone();
            self.map.remove(&coord);
            self.arena.remove(idx);
        }
    }

    pub fn free_all<UnaryPredicate>(&mut self, predicate: UnaryPredicate)
    where
        UnaryPredicate: Fn(&T) -> bool,
    {
        let mut to_remove = Vec::new();
        for (coord, idx) in self.map.iter() {
            // SAFETY: Since, self.map contains the coordinate and corresponding `idx`,
            // there must be some existing node at `idx`, so self.arena.get(idx) must return Some(node)
            let node = self.arena.get(*idx).unwrap();
            // SAFETY: First check assures that node.get() contains something, so node.get().unwrap() must not fail
            if node.get().is_some() && predicate(node.get().unwrap()) {
                to_remove.push(coord.clone());
            }
        }

        for coord in to_remove {
            // SAFETY: `to_remove` was filled with self.map keys, so map.get must not fail
            let idx = self.map.get(&coord).unwrap().clone();
            self.arena.remove(idx);
            self.map.remove(&coord);
        }
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.map.reserve((capacity - self.map.len())/8);
        self.arena.reserve(capacity - self.arena.len());
    }
}

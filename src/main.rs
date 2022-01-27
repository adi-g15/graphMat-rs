#![feature(adt_const_params)]   // to use compile time Direction constants in GraphMat::iter()

mod direction;
mod node;
mod graphmat;
mod iterators;

use graphmat::GraphMat;

use crate::direction::Direction;

fn main() {
    println!("Hello, world!");

    let mut matrix = GraphMat::new();

    matrix.get(&(1,2,3));
    matrix.set((2,3,4), 150);
    matrix.get_mut(&(2,3,4));

    println!("{:?}, {:?}",
        matrix.get(&(1,2,3)),
        matrix.get(&(2,3,4))
    );

    matrix.set((4,3,6), 40);
    matrix.set((4,2,6), 39);
    matrix.set((4,1,6), 38);
    matrix.set((4,0,6), 37);

    matrix.set((4,3,5), 13);
    matrix.set((4,2,5), 21);
    matrix.set((4,1,5), 22);
    matrix.set((4,0,5), 23);

    let mut it = matrix.iter::<{Direction::dakshin}>((4,3,6));

    // Will run till it.next() is not None
    while let Some((coord, node)) = it.next() {
        println!("Iterating in loop1: {:?} => {}", coord, node);
    }

    let mut it2 = matrix.iter_all_dir((4,3,6), Direction::adharastha);

    // Will run till it.next() is not None
    while let Some((coord, node)) = it2.next() {
        // Not allowed, agar ye krna h to `node` ka lifetime khatm krna hoga, for eg. clone kar, ya loop ke end me kro ye
        // it2.set_direction(Direction::dakshin);

        println!("Iterating in loop2: {:?} => {}", coord, node);

        it2.set_direction(Direction::dakshin);
    }

    matrix.free_pos((2,3,4));
    matrix.free_all(|n| n == &0);
}

#![feature(adt_const_params)] // to use compile time Direction constants in GraphMat::iter()
#![feature(option_result_contains)] // to use `option.contains(value)` instead of `match option { Some(val) => val == value, None => false }`

mod direction;
mod graphmat;
mod iterators;
mod node;

pub use iterators::{GraphMatIterator, GraphMatFreeIterator};
pub use direction::Direction;
pub use graphmat::GraphMat;

#[test]
fn main_test() {
    let mut matrix = GraphMat::new();

    matrix.get((1, 2, 3));
    matrix.set((2, 3, 4), 150);
    matrix.get_mut((2, 3, 4));

    println!("{:?}, {:?}", matrix.get((1, 2, 3)), matrix.get((2, 3, 4)));

    matrix.set((4, 3, 6), 40);
    matrix.set((4, 2, 6), 39);
    matrix.set((4, 1, 6), 38);
    matrix.set((4, 0, 6), 37);

    matrix.set((4, 3, 5), 13);
    matrix.set((4, 2, 5), 21);
    matrix.set((4, 1, 5), 22);
    matrix.set((4, 0, 5), 23);

    let mut it = matrix.iter::<{ Direction::dakshin }>((4, 3, 6));

    // Will run till it.next() is not None
    while let Some((coord, node)) = it.next() {
        println!("Iterating in loop1: {:?} => {}", coord, node);
    }

    let mut it2 = matrix.iter_all_dir((4, 3, 6), Direction::adharastha);

    // Will run till it.next() is not None
    while let Some((coord, node)) = it2.next() {
        // Not allowed, agar ye krna h to `node` ka lifetime khatm krna hoga, for eg. clone kar, ya loop ke end me kro ye
        // it2.set_direction(Direction::dakshin);

        println!("Iterating in loop2: {:?} => {}", coord, node);

        it2.set_direction(Direction::dakshin);
    }

    // Conditionally free some nodes
    matrix.free_pos((2, 3, 4));
    matrix.free_all(|n| n == &0);
}

#[test]
fn matrix_set_n() {
    let mut matrix = GraphMat::new();
    // matrix.reserve(1000000); // Only improves the time here by 0.8 s

    for i in 0..99 {
        for j in 0..99 {
            for k in 0..99 {
                matrix.set((i, j, k), i + j + k);
            }
        }
    }
}

#[test]
fn matrix_set_n_get() {
    let mut matrix = GraphMat::new();
    matrix.reserve(1000000);

    for i in 0..99 {
        for j in 0..99 {
            for k in 0..99 {
                matrix.set((i, j, k), i + j + k);
            }
        }
    }

    for i in 0..99 {
        for j in 0..99 {
            for k in 0..99 {
                let node = matrix.get((i, j, k));
                assert!(node.is_some());
                assert_eq!(node.unwrap().clone(), i + j + k);
            }
        }
    }

    println!(
        "len: {}, capacity: {}",
        matrix.arena.len(),
        matrix.arena.capacity()
    );
}

#[test]
fn matrix_iterator() {
    let mut matrix = GraphMat::new();
    matrix.reserve(1000000);

    for i in 0..99 {
        for j in 0..99 {
            for k in 0..99 {
                matrix.set((i, j, k), i + j + k);
            }
        }
    }

    // Iterator 'significantly' faster (currently in some directions, it is just an O(1) access)
    for j in 0..99 {
        for k in 0..99 {
            let mut it = matrix.iter::<{ Direction::purva }>((0, j, k));

            let mut i = 0;
            while let Some((coord, val)) = it.next() {
                i += 1;
                assert_eq!(*val, coord.0 + coord.1 + coord.2);
            }
    
            // println!(
            //     "times: {}, len: {}, capacity: {}",
            //     i,
            //     matrix.arena.len(),
            //     matrix.arena.capacity()
            // );
        }
    }
}

#[test]
fn simple_matrix_init() {
    // For benchmarking
    let mut matrix = Vec::new();

    for i in 0..99 {
        matrix.push(Vec::new());
        for j in 0..99 {
            matrix[i].push(Vec::new());
            for k in 0..99 {
                matrix[i][j].push(i + j + k);
            }
        }
    }

    for i in 0..99 {
        for j in 0..99 {
            for k in 0..99 {
                assert_ne!(matrix[i][j][k], 99999);
                assert_eq!(matrix[i][j][k], i + j + k);
            }
        }
    }
}

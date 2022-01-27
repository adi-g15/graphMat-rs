mod direction;
mod node;
mod graphmat;

use graphmat::GraphMat;

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
}

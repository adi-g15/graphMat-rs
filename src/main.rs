mod direction;
mod node;
mod graphmat;

use graphmat::GraphMat;

fn main() {
    println!("Hello, world!");

    let mut matrix = GraphMat::new();

    let data1 = matrix.get(&(1,2,3));
    matrix.set((2,3,4), 150);
    let data2 = matrix.get_mut(&(2,3,4));

    data2.unwrap().borrow_mut().set(4);

    let data2_internal = data2.unwrap().borrow().data;

    println!("{:?} == {:?}", matrix.get(&(2,3,4)), data2_internal);

}

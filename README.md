# GraphMat - Rust

Another one totally inspired from [graphMat](https://github.com/adi-g15/graphMat), but with newer features.

3D Sparse Matrix library, for efficient memory usage, at the cost of a bit more CPU usage.

## Example Usage

```rs
let mut matrix = GraphMat::new();
matrix.reserve(1000000);

// .get() and .set()
matrix.get((1, 2, 3));          // `get` returns None, if the location is not yet assigned/available
matrix.set((2, 3, 4), 150);
matrix.get_mut((2, 3, 4));      // returns a mutable reference

println!("{:?}, {:?}", matrix.get((1, 2, 3)), matrix.get((2, 3, 4)));

// ...some more nodes initialised
matrix.set((4, 3, 6), 40);
matrix.set((4, 2, 6), 39);
matrix.set((4, 1, 6), 38);
matrix.set((4, 0, 6), 37);
matrix.set((4, 3, 5), 13);
matrix.set((4, 2, 5), 21);
matrix.set((4, 1, 5), 22);
matrix.set((4, 0, 5), 23);

// !Directional Iterator 1 - Direction known at compile-time
let mut it = matrix.iter::<{ Direction::dakshin }>((4, 3, 6));

// Will run till it.next() is not None
while let Some((coord, node)) = it.next() {
    println!("Iterating in loop1: {:?} => {}", coord, node);
}

// !Directional Iterator 2 - Dynamic directions
let mut it2 = matrix.iter_all_dir((4, 3, 6), Direction::adharastha);

while let Some((coord, node)) = it2.next() {
    println!("Iterating in loop2: {:?} => {}", coord, node);

    it2.set_direction(Direction::dakshin);
}

// Conditionally free some nodes
matrix.free_pos((2, 3, 4));
matrix.free_all(|n| n == &0);
```

## Features

1. 3D Sparse Matrix
2. Uses arena allocation, so deallocation-then-allocation friendly
3. Provides directional iterators
4. Covers Infinite space (ie. (0,0,0), (-32545435,-34453466,-768324) etc. given how much the index type can index), since a primary reason for this is using in my simulator
5. Conditional free, you can conditionally remove nodes

Cons:
1. Not multi-threading friendly
2. Naive library writer, naive library... ie. it may not be optimised for many cases

### The 'leader' concept

The way i implemented it as sparse is, it keeps a HashMap of keys as coordinate, and value is the Node

But, this can't be done for all nodes, else it is just a HashMap of coordinates to data, no real benefit.

But, some reference is needed, so there is a 'leader' node, through which many other nodes (on average, 7 nodes per leader) are accessed.
This can be extended to reference more, say 17 per leader, but that is less manageable in terms of code, since there are more match statements.

pub mod matrix;
pub mod vector;
pub mod linear_combinations;
pub mod numeric;

fn main() {
    let v = vector::Vector {
        values: vec![1, 2, 3],
    };

    println!("Vector: {:?}", v);

    let m = matrix::Matrix {
        values: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
    };
    println!("Matrix: {:?}", m);
}

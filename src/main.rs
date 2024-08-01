use vector::Vector;

pub mod cos;
pub mod cross_product;
pub mod determinant;
pub mod linear_combinations;
pub mod linear_interpolation;
pub mod matrix;
pub mod matrix_multiplication;
pub mod norm;
pub mod inverse;
pub mod rank;
pub mod numeric;
pub mod row_echelon_form;
pub mod trace;
pub mod transpose;
pub mod vector;

fn main() {
    let v = vector::Vector {
        values: vec![1, 2, 3],
    };

    println!("Vector: {:?}", v);

    let m = matrix::Matrix {
        values: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
    };

    let mut v1 = vector::Vector {
        values: vec![1, 2, 3],
    };

    let v2 = vector::Vector {
        values: vec![4, 5, 6],
    };

    let sum = v1.sub(&v2);

    println!("Sub of vectors: {:?}", sum);
    println!("Matrix: {:?}", m);

    let start = vector::Vector {
        values: vec![0., 0., 0.],
    };

    let end = vector::Vector {
        values: vec![10., 10., 10.],
    };

    let t = 0.5;

    let lerped = Vector::lerp(start, end, t);

    println!("Lerped Vector: {:?}", lerped);
}

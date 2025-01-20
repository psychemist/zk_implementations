/*

Polynomial = 2 * x ** 3 + 4 * x ** 2 + 5

SPARSE:
[(2, 3), (4, 2), (5, 0)]

for tuple in xxx:
    var += tuple[0] * (x ** tuple[1])
    print(var)

DENSE:
[5, 0, 4, 2]

for coeff in range(0, len(yyy)):
    gg += yyy[coeff] *  x  ** coeff
    print(gg)

*/

use std::ops::Index;
struct UnivariatePolySparse {}

struct UnivariatePolyDense {
    coefficient: Vec<f64>,
}

fn evaluate_sparse(vec: &Vec<(u32, u32)>, x: u32) -> u32 {
    let mut answer: u32 = 0;

    for tuple in vec {
        answer += tuple.0 * x.pow(tuple.1);
    }

    return answer;
}

fn evaluate_dense(vec: &Vec<u32>, x: u32) -> u32 {
    let mut result: u32 = 0;

    for index in 0..vec.len() {
        result += vec.index(index) * x.pow(index.try_into().unwrap());
    }

    return result;
}

fn degree_sparse(vec: &Vec<(u32, u32)>) -> u32 {
    let mut deg: u32 = 0;

    for tuple in vec {
        if tuple.1 > deg {
            deg = tuple.1;
        }
    }

    return deg;
    // return vec[0].1;
}

fn degree_dense(vec: &Vec<u32>) -> usize {
    return vec.len() - 1;
}

// fn interpolate_points(points: Vec<(usize, usize)>) -> UnivariatePolySparse {
//     // body
// }

fn interpolate_ordered(ys: Vec<usize>) -> UnivariatePolyDense {
    // for i in 0..ys.len() {}
    todo!()
}

// fn interpolate_coordinates(xs: Vec<usize>, ys: Vec<usize>) -> UnivariatePolySparse {
//     // body
// }

fn main() {
    let sparse_polynomial: Vec<(u32, u32)> = vec![(2, 3), (4, 2), (5, 0)];
    let dense_polynomial: Vec<u32> = vec![5, 0, 4, 2];

    println!(
        "Evaluation of Sparse Polynomial representation: {}",
        evaluate_sparse(&sparse_polynomial, 2)
    );
    println!(
        "Evaluation of Dense Polynomial representation: {}",
        evaluate_dense(&dense_polynomial, 2)
    );

    println!(
        "Number of degrees of Sparse Polynomial: {}",
        degree_sparse(&sparse_polynomial)
    );
    println!(
        "Number of degrees of Dense Polynomial: {}",
        degree_dense(&dense_polynomial)
    );

    println!(
        "Interpolation of Sparse Polynomial: {}",
        // evaluate_sparse(&sparse_polynomial)
        todo!()
    );
    println!(
        "Interpolation of Dense Polynomial: {}",
        // evaluate_dense(&dense_polynomial)
        todo!()
    );
}

#![cfg_attr(not(test), no_std, no_main)]

#[cfg(not(test))]
runtime::binInit!();
#[cfg(test)]
runtime::addtest!();

fn vector_add(a: &[u32], b: &[u32], c: &mut [u32]) {
    for i in 0..a.len() {
        c[i] = a[i] + b[i];
    }
}

fn vector_dot(a: &[u32], b: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    sum
}

fn matrix_mult(a: &[u32], b: &[u32], c: &mut [u32], n: usize) {
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }
}

fn main() {
    let a = [1, 2, 3, 4];
    let b = [5, 6, 7, 8];
    let mut c = [0; 4];

    vector_add(&a, &b, &mut c);
    println!("Vector Add: {:?}", c);

    let dot = vector_dot(&a, &b);
    println!("Vector Dot: {}", dot);

    let m1 = [1, 2, 3, 4];
    let m2 = [5, 6, 7, 8];
    let mut m3 = [0; 4];

    matrix_mult(&m1, &m2, &mut m3, 2);
    println!("Matrix Mult: {:?}", m3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add() {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];
        let mut c = [0; 4];

        vector_add(&a, &b, &mut c);

        assert_eq!(c, [6, 8, 10, 12]);
    }

    #[test]
    fn test_vector_dot() {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];

        let result = vector_dot(&a, &b);

        assert_eq!(result, 70); // 1*5 + 2*6 + 3*7 + 4*8 = 5 + 12 + 21 + 32 = 70
    }

    #[test]
    fn test_matrix_mult() {
        let m1 = [1, 2, 3, 4];
        let m2 = [5, 6, 7, 8];
        let mut result = [0; 4];

        matrix_mult(&m1, &m2, &mut result, 2);

        // 期望结果: [[19, 22], [43, 50]]
        assert_eq!(result, [19, 22, 43, 50]);
    }
}

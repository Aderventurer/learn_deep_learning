use rand::Rng;

fn matrix_multiplication(matrix1: &[Vec<i32>], matrix2: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let m = matrix1.len();
    if m == 0 || matrix1[0].is_empty() || matrix2.is_empty() {
        panic!("Invalid matrix dimensions");
    }
    let n = matrix1[0].len();
    let p = matrix2[0].len();
    
    if matrix2.len() != n {
        panic!("Columns of matrix1 ({}) must equal rows of matrix2 ({})", n, matrix2.len());
    }

    let mut result = vec![vec![0; p]; m];
    
    for i in 0..m {
        for j in 0..p {
            for k in 0..n {
                result[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }
    
    result
}

fn reference_matrix_multiplication(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let m = a.len();
    let n = a[0].len();
    let p = b[0].len();
    assert_eq!(b.len(), n, "Matrix dimensions mismatch");
    
    (0..m).map(|i| {
        (0..p).map(|j| {
            a[i].iter()
                .zip(b.iter().map(|row| row[j]))
                .map(|(&x, y)| x * y)
                .sum()
        }).collect()
    }).collect()
}

fn main() {
    let matrix1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let matrix2 = vec![vec![7, 8], vec![9, 10], vec![11, 12]];
    let result_custom = matrix_multiplication(&matrix1, &matrix2);
    let expected = vec![vec![58, 64], vec![139, 154]];
    assert_eq!(result_custom, expected);
    println!("固定测试通过！");

    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let m = rng.gen_range(1..6);
        let n = rng.gen_range(1..6);
        let p = rng.gen_range(1..6);
        
        let a: Vec<Vec<i32>> = (0..m)
            .map(|_| (0..n).map(|_| rng.gen_range(-10..=10)).collect())
            .collect();
        
        let b: Vec<Vec<i32>> = (0..n)
            .map(|_| (0..p).map(|_| rng.gen_range(-10..=10)).collect())
            .collect();
        
        let custom = matrix_multiplication(&a, &b);
        let reference = reference_matrix_multiplication(&a, &b);
        
        assert_eq!(custom, reference, 
            "测试失败:\nA: {:?}\nB: {:?}\nCustom: {:?}\nReference: {:?}",
            a, b, custom, reference);
    }
    println!("所有随机测试均通过！");
}
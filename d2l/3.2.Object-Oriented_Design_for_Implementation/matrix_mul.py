import numpy as np
def matrix_multiplication(matrix1, matrix2):
    m = len(matrix1)
    n = len(matrix1[0])
    p = len(matrix2[0])
    if n != len(matrix2):
        raise ValueError("列不等于行")
    result = [[0 for _ in range(p)] for _ in range(m)]
    for i in range(m):
        for j in range(p):
            for k in range(n):
                result[i][j] += matrix1[i][k] * matrix2[k][j]
    return result

if __name__ == "__main__":
    matrix1 = [[1, 2, 3], [4, 5, 6]]
    matrix2 = [[7, 8], [9, 10], [11, 12]]
    matrix1 = [[1, 2, 3], [4, 5, 6]]
    matrix2 = [[7, 8], [9, 10], [11, 12]]

    
    result_custom = matrix_multiplication(matrix1, matrix2)
    
    result_numpy = np.dot(np.array(matrix1), np.array(matrix2))

    assert np.array_equal(np.array(result_custom), result_numpy), \
        f"失败: {result_custom} != {result_numpy.tolist()}"
    print("成功")
#deepseek补充
    for _ in range(5):
        
        m = np.random.randint(1, 6)
        n = np.random.randint(1, 6)
        p = np.random.randint(1, 6)
        A = np.random.randint(-10, 10, size=(m, n)).tolist()
        B = np.random.randint(-10, 10, size=(n, p)).tolist()

        res_custom = matrix_multiplication(A, B)
        res_np = np.dot(np.array(A), np.array(B)).tolist()
        assert res_custom == res_np, \
            f"随机测试失败 (尺寸 {m}x{n} * {n}x{p}): {res_custom} != {res_np}"

    print("所有随机测试均通过！")
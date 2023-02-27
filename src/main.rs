/* Given an array of integers, return a new array such that each element at index i of the new array is the product of all the numbers in the original array except the one at i.

For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be [2, 3, 6].

Follow-up: what if you can't use division? */

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [3, 2, 1];

    println!("{:?}", array_product(&arr1)); // [120, 60, 40, 30, 24]
    println!("{:?}", array_product(&arr2)); // [2, 3, 6]
}

fn array_product(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr.len()); // criar um novo array com o mesmo tamanho de arr

    for i in 0..arr.len() {
        let mut product = 1;
        for j in 0..arr.len() {
            if i != j {
                product *= arr[j]; // atribui a product o produto da multiplicação apenas se i != j
            }
        }
        result.push(product); // coloca o resultado no novo array
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array_product() {
        let arr1 = [1, 2, 3, 4, 5];
        let arr2 = [3, 2, 1];
        let arr3 = [4, 5, 6];
    
        assert_eq!(array_product(&arr1), [120, 60, 40, 30, 24]);
        assert_eq!(array_product(&arr2), [2, 3, 6]);
        assert_eq!(array_product(&arr3), [30, 24, 20]);
    }
}
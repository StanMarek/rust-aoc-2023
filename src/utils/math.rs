pub fn multiply_array_elements(array: &[i32]) -> i32 {
    let mut multiplied = 1;
    for i in 0..array.len() {
        multiplied *= array[i];
    }

    return multiplied;
}

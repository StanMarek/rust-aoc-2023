pub fn multiply_array_elements(array: &[i32]) -> i32 {
    let mut multiplied = 1;
    (0..array.len()).for_each(|i| {
        multiplied *= array[i];
    });

    multiplied
}

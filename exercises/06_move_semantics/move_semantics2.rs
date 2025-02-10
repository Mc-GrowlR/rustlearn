fn fill_vec(input: &[i32]) -> Vec<i32> {
    // let mut vec: Vec<i32> = vec.to_vec();

    // vec.push(88);

    // vec
    let mut new_vec = Vec::with_capacity(input.len() + 1);
    new_vec.extend_from_slice(input);
    new_vec.push(88);
    new_vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

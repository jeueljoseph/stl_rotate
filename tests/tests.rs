#[cfg(test)]
mod tests {
    use stl_rotate::Rotate;

    #[test]
    fn test_rotate() {
        let mut v: Vec<usize> = (0..10).collect();
        v.rotate(0..10, 5);
        assert!(v == [5, 6, 7, 8, 9, 0, 1, 2, 3, 4]);
    }
}

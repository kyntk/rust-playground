pub fn compound() {}

#[cfg(test)]
mod test {
    #[test]
    fn test_tuple() {
        let t1 = (88, true);
        assert_eq!(t1.0, 88);
        assert_eq!(t1.1, true);

        let mut t2 = (88, true);
        t2.0 = 100;
        assert_eq!(t2, (100, true));

        let mut t3 = ((0, 5), (10, -1));
        let ((ref mut x1_ptr, ref mut y1_ptr), _) = t3; // ref mutで要素を指す可変のポインタ
        *x1_ptr = 3;
        *y1_ptr = 10;
        assert_eq!(t3, ((3, 10), (10, -1)));
    }

    #[test]
    fn test_array() {}
}
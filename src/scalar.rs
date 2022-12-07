pub fn scalar() {
}

fn unit() {
    println!("hello");
}

#[cfg(test)]
mod tests {
    use super::unit;

    #[test]
    fn test_unit() {
        let ret = unit();
        assert_eq!(ret, ());
        assert_eq!(std::mem::size_of::<()>(), 0);
    }

    #[test]
    fn test_bool() {
        let b1 = true;
        let _b2 = !b1;
        assert_eq!(std::mem::size_of::<bool>(), 1);
    }

    #[test]
    fn test_integer() {
        let _n1 = 10_000; // i32
        let _n2 = 0u8; // u8型をサフィックスで指定
        let n3 = -100_isize;
        let n4 = 10;
        let _m5 = n3 + n4;
        let _h1 = 0xff; // プレフィックスで16進数
        let _o1 = 0o744; // 8進数
        let _b1 = 0b1010_0110_1110_1001; //2進数

        let n6 = b'A'; // ASCII文字コード
        assert_eq!(n6, 65u8);

        // rustup doc --std
        // i32のドキュメントを見る
        assert_eq!(i32::MIN, -2147483648);

        let x: i32 = 2;
        assert_eq!(x.pow(5), 32);

        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<u8>(), 1);
    }

    #[test]
    fn test_integer_overflow() {
        let n1 = std::u8::MAX;
        let n2 = 2u8;
        // panic!
        // let n3 = n1 + n2;
        // println!("{}", n3);

        assert_eq!(n1.checked_add(n2), None);
        assert_eq!(n1.saturating_add(n2), std::u8::MAX);
        assert_eq!(n1.wrapping_add(n2), 1);
        assert_eq!(n1.overflowing_add(n2), (1, true));
    }

    #[test]
    fn test_float() {
        let f1:f64 = 10.3;
        let f2 = -1_234.56f32;
        let _f3:f64 = 578.6E+77; // 指数部の指定

        assert_eq!(f1.ceil(), 11.0);
        assert_eq!(f2.round(), -1_235.0);

        assert_eq!(std::mem::size_of::<f64>(), 8);
    }

    #[test]
    fn test_char() {
        let c1 = 'A';
        let c2 = 'a';
        assert!(c1 < c2);
        assert!(c1.is_uppercase());

        let c3 = '0';
        assert!(c3.is_digit(10));

        assert_eq!(std::mem::size_of::<char>(), 4);
    }
}

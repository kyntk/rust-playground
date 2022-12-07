pub fn reference() {
    let mut n = 0;
    f1(n);
    f2(&mut n);
}

// コピーを束縛する。mainのnには影響を与えない
pub fn f1(mut n: u32) {
    println!("f1: n = {}", n);
    n = 1;
    println!("f1: n = {}", n);
}

pub fn f2(n_ptr: &mut u32) {
    println!("f2: n_ptr = {:p}", n_ptr);

    *n_ptr = 2;
    println!("f2: *n_ptr = {}", *n_ptr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference() {
        let mut n = 0;
        println!("main: n = {}", n);

        f1(n);
        assert_eq!(n, 0);

        f2(&mut n);
        assert_eq!(n, 2);
    }

    #[test]
    fn test_raw_pointer() {
        let c1 = 'A';
        let c1_ptr: *const char = &c1; // *const charで強制的に生ポインタへ
        assert_eq!(unsafe { *c1_ptr }, 'A');

        let mut n1 = 0;
        let n1_ptr: *mut i32 = &mut n1;
        assert_eq!(unsafe { *n1_ptr }, 0);

        unsafe {
            *n1_ptr = 1_000;
            assert_eq!(*n1_ptr, 1_000);
        }
    }

    #[test]
    fn test_function_pointer() {
        fn double(n: i32) -> i32 {
            n + n
        }
        fn abs(n: i32) -> i32 {
            if n >= 0 { n } else { -n }
        }

        let f_bad = double; // 関数定義型だと判定される
        let mut f: fn(i32) -> i32 = double; // 型注釈で、関数名から関数ポインタを得る
        assert_eq!(f(-42), -84);

        f = abs; // fの値を変更
        assert_eq!(f(-42), 42);

        assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());
        assert_eq!(std::mem::size_of_val(&f_bad), 0);
    }
}
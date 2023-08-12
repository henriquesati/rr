use tradutor_binario::lib::*;


#[cfg(test)]
    #[test]
    fn bin_int_test() {
        let result =  from_dec(String::from("bin"), Num::Int(1000));
        println!("result{}", result);
        assert_eq!(
            result,
            String::from("1111101000")
        );
    }
    #[test]
    fn bin_int_test2() {
        let result =  from_dec(String::from("bin"), Num::Int(13897483));
        println!("result{}", result);
        assert_eq!(
            result,
             String::from("110101000000111100001011")
        );
    }
    #[test]
    fn bin_int_test3() {
        let result =  from_dec(String::from("bin"), Num::Int(2746923));
        println!("result{}", result);
        assert_eq!(
            result,
             String::from("1010011110101000101011")
        );
    }

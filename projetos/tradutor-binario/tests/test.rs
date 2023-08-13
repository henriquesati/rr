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
    #[test]
    fn bin_float_test1(){
        let result = from_dec(String::from("bin"), Num::Float(100.0625));
        assert_eq!(
            result,
            String::from("1100100,0001")
        )
    }
    #[test]
    fn oct_test1(){
        let result = from_dec(String::from("oct"), Num::Int(100));
        assert_eq!(
            result,
            String::from("144")
        )

    }

    #[test]
    fn oct_test2(){
        let result = from_dec(String::from("oct"), Num::Int(573648));
        assert_eq!(
            result,
            String::from("2140320")
        )

    }
    #[test]
    fn hex_test1(){
        let result = from_dec(String::from("hex"), Num::Int(573648));
        assert_eq!(
            result,
            String::from("8C0D0")
        )

    }
    #[test]
    fn hex_test2(){
        let result = from_dec(String::from("hex"), Num::Int(1837));
        assert_eq!(
            result,
            String::from("72D")
        )

    }


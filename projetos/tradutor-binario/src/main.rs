use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

#[allow(dead_code)]
enum Num {
    Int(i128),
    Float(f64)
}

#[warn(unused_variables)]
#[allow(dead_code)]
#[allow(non_snake_case)]

fn main() {
    println!("Hello, world!");
    let teste = String::from("bin");
    fromDec(teste, Num::Int(10));


fn fromDec(code: String, num: Num ) {
    let mut base: i128 = 0;
    let stringSlice = code.as_str();
    match stringSlice{
        "bin" =>{
            println!("bin");
            bin(num);
        }
        "oct"=>{
            println!("oct");
            base = 8
        },
        "hex" =>{
            println!("hex");
            base = 16
        },
        _=>{
            println!("none");
        }

    }
        let mut result: Vec<i128> = Vec::new();
        println!("{}", base);
        // while num > 0 {
        //     let resto = num % base;
        //     let quociente = num / base;
        //     result.push(resto);
        //     num = quociente;
        // }
        println!("{:?}", result);
    
}

fn bin(num: Num) {
 
    let mut number: i128 = 0;
    let mut result: Vec<i128> = Vec::new();

    let meu_numero = match num {
        Num::Int(x) => number = x,
        Num::Float(x) => floatBin(x),
    };

    while number > 0{
        let resto = number % 2;
        let quociente = number / 2;
        result.push(resto);
        number = quociente;
    }
    result.reverse();
        println!("{:?}", result);
        let binFormat: Vec<String> = result.iter().map(|num| num.to_string()).collect();
        let binFormat = binFormat.join(", ");
        println!("{:?}", binFormat);

        fn floatBin(num: f64){
           let mnum = 1.2;
           let snum = mnum.to_string();
           let stringRef: &str = snum.as_str();
           let dec: Decimal = Decimal::from_str(stringRef).unwrap();
            println!("{}", dec);
            println!("floatbin");
        }

    
    }
}
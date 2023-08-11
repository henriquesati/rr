use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

#[allow(dead_code)]
enum Num {
    Int(i128),
    Float(f64)
}
fn hidrated(input: String) -> Decimal {
    let inputref: &str = input.as_str();
    let value: Decimal = Decimal::from_str(inputref).unwrap();
    return value
}

#[warn(unused_variables)]
#[allow(dead_code)]
#[allow(non_snake_case)]

fn main() {
    println!("Hello, world!");
    let teste = String::from("bin");
    fromDec(teste, Num::Float(1000.2395));


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
        // let mut result: Vec<i128> = Vec::new();
        // println!("{}", base);
        // // while num > 0 {
        // //     let resto = num % base;
        // //     let quociente = num / base;
        // //     result.push(resto);
        // //     num = quociente;
        // // }
        // println!("{:?}", result);
    
}

fn bin(num: Num) {
    let mut input: i128 = 0;
 
    match num{
        Num::Int(x) =>{
            let input = x;
        
        },
        Num::Float(x)=>{
            let mut count: u16 = 0;
            let mut xToString = x.to_string();
            let mut slice = false; 
            let mut futureFloat: Vec<u8> = vec![];
            futureFloat.push(b'0');

            unsafe{
            let vetor = xToString.as_bytes_mut();
                for byte in vetor{
                    if *byte == b'.'{
                        slice = true;
                    }
                    if slice == true{
                        futureFloat.push(*byte);
                        count += 1;
                    }
                    if count == 5{
                        break;
                    }
                }

            }
            let floatAsString = String::from_utf8(futureFloat).unwrap();
            let float = hidrated(floatAsString);
            println!("{}", float);

           
        }
        _=>{

        } //quantidade ideial com 4.digitos é de 6 bytes
    }
   
    
    let base = 2;
    let mut count = 0;

     while input > 0 && count < 6{
        let resto = input % base;
        let quociente = input / base;
        println!("{}", quociente);
        input = quociente;
        count += 1;
        println!("contador: {}", count);
    }
    
    }
}
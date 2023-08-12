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
    let result: String = fromDec(teste, Num::Float(12.5));
    println!("{}", result);


fn fromDec(code: String, num: Num ) -> String{
    let mut base: i128 = 0;
    let mut input: i128 = 0;
    let stringSlice = code.as_str();
    match stringSlice{
        "bin" =>{
            println!("bin");
            let result = bin(num);
            return result;
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
    match num{
        Num::Float(x) =>{
            input =  x as i128;
        },
        Num::Int(x) =>{
            input = x;

        },
        _ =>{

        }
    }
        
        let mut vetorResultado: Vec<i128> = Vec::new();
        println!("{}", base);
        while input > 0 {
            let resto = input % base;
            let quociente: i128 = input / base;
            vetorResultado.push(resto);
            input = quociente;
        }
        println!("{:?}", vetorResultado);
        let result: String  = vetorResultado.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join("");
        return result;
    
}

fn bin(num: Num) -> String {
 
    match num{
        Num::Int(x) =>{
            let result = binInt(x);
            result
        
        },
        Num::Float(x)=>{
            let intPart: i128 = x.trunc() as i128;
            let resultInt = binInt(intPart);
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
            let resultFloat = binFloat(float);
            let intPlusFloat: String = format!("{}{}{}", resultInt,",", resultFloat);
            intPlusFloat

        }
        _=>{
            let stringtest = String::from("n");
            return stringtest;
        }
    }
   
    
}
fn binInt(num: i128) -> String {
    let mut input =  num;
    let base = 2;
    let mut count = 0;
    let mut vetorBinario: Vec<u16> = Vec::new();

     while input >= 1{
        let resto = input % base;
        let resto = resto as u16;
        let quociente = input / base;
        println!("{}", quociente);
        input = quociente;
        count += 1;
        vetorBinario.push(resto);
        println!("contador: {}", count);
    }
    let result: String  = vetorBinario.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join("");
    return result.chars().rev().collect();
}
fn binFloat(num: Decimal) -> String{ //precisão de 4 digitos
    let base = dec!(2);
    let zero = dec!(0);
    let um: Decimal = dec!(1);
    let mut count = 0;
    let mut resultado = dec!(0);
    let mut vetorBinario: Vec<u16> = Vec::new();

    while count !=4{
        resultado = num * base;
        if resultado == um { vetorBinario.push(1); break;}
        if resultado - um < zero{
            vetorBinario.push(0);
        }
        else{
            vetorBinario.push(1);
            resultado = resultado - um;
        }
    }
    let result: String  = vetorBinario.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join("");
    return result;
}
}
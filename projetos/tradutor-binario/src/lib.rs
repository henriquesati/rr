pub mod lib{
    use std::collections::HashMap;
    
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use rust_decimal::prelude::*;
    
    
    #[allow(dead_code)]
    pub enum Num {
        Int(i128),
        Float(f64)
    }
    fn hidrated(input: String) -> Decimal {
        let inputref: &str = input.as_str();
        let value: Decimal = Decimal::from_str(inputref).unwrap();
        return value
    }
    
    pub fn from_dec(code: String, num: Num ) -> String{
        let mut base: i128 = 0; 
        let mut input: i128 = 0;
        let string_slice = code.as_str();
        match num{
            Num::Float(x) =>{
                input =  x as i128;
            },
            Num::Int(x) =>{
                input = x;
    
            },
            _ =>{
            println!("proposital unreacheable");
            }
        }
        match string_slice{
            "bin" =>{
                let result = bin(num);
                return result;
            }
            "oct"=>{
                base = 8
            },
            "hex" =>{
                base = 16;
                let result = to_hex(input);
                return result;
            },
            _=>{
                println!("unreachable pattern reached at: from_dec");
            }
    
        }     
            let mut vetor_resultado: Vec<u16> = Vec::new();
            while input > 0 {
                let resto = input % base;
                let quociente: i128 = input / base;
                vetor_resultado.push(resto as u16);
                input = quociente;
            }
            println!("{:?}", vetor_resultado);
            vetor_resultado.reverse();
            let result: String  = vetor_resultado.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join("");
            return result;
        
    }
    
    fn to_hex(num: i128) -> String {
    
        let mut input = num;
        let base = 16;
        let mut vetor_resultado: Vec<String> = Vec::new();
        let mut mapa_hexadecimal: HashMap<u8, String> = HashMap::new();
    
        mapa_hexadecimal.insert(10, String::from("A"));
        mapa_hexadecimal.insert(11, String::from("B"));
        mapa_hexadecimal.insert(12, String::from("C"));
        mapa_hexadecimal.insert(13, String::from("D"));
        mapa_hexadecimal.insert(14, String::from("E"));
        mapa_hexadecimal.insert(15, String::from("F"));
        while input > 0 {
            let resto = input % base;
            println!("resto{}", resto);
            let quociente: i128 = input / base;
            println!("quociente:{}", quociente);
            if resto > 9 {
                match mapa_hexadecimal.get(&(resto as u8)) {
                    Some(holder) => {
                        vetor_resultado.push(String::from(holder));
    
                    }
                    None => {
                        println!("isso não deveria acontecer, none reached");
                    }
                }
            }
            else {
                let tmp = std::char::from_digit(resto as u32, 10).unwrap();
                vetor_resultado.push(String::from(tmp));
            }
            input = quociente;
        }
        vetor_resultado.reverse();
        let result: String  = vetor_resultado.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
        return result
    }
    fn bin(num: Num) -> String {
     
        match num{
            Num::Int(x) =>{
                let result = bin_int(x);
                result
            
            },
            Num::Float(x)=>{
                let int_part: i128 = x.trunc() as i128;
                let result_int = bin_int(int_part);
                let mut count: u16 = 0;
                let mut x_to_string = x.to_string();
                let mut slice = false; 
                let mut future_float: Vec<u8> = vec![];
                future_float.push(b'0');
    
                unsafe{
                let vetor = x_to_string.as_bytes_mut();
                    for byte in vetor{
                        if *byte == b'.'{
                            slice = true;
                        }
                        if slice == true{
                            future_float.push(*byte);
                            count += 1;
                        }
                        if count == 5{
                            break;
                        }
                    }
    
                }
        
                let float_as_string = String::from_utf8(future_float).unwrap();
                let float = hidrated(float_as_string);
                let result_float = bin_float(float);
                let int_plus_float: String = format!("{}{}{}", result_int,",", result_float);
                return int_plus_float
            }
            _=>{
                let unreachable = String::from("unreachble pattern reached - error");
                return unreachable;
            }
        }
        
    }
    
    fn bin_int(num: i128) -> String {
        let mut input =  num;
        let base = 2;
        let mut count = 0;
        let mut vetor_binario: Vec<u16> = Vec::new();
    
         while input >= 1{
            let resto = input % base;
            let resto = resto as u16;
            let quociente = input / base;
            println!("{}", quociente);
            input = quociente;
            count += 1;
            vetor_binario.push(resto);
            println!("contador: {}", count);
        }
        let result: String  = vetor_binario.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join("");
        return result.chars().rev().collect();
    }
    
    fn bin_float(num: Decimal) -> String{ //precisão de 4 digitos
        let base = dec!(2);
        let zero = dec!(0);
        let um: Decimal = dec!(1.0000);
        let mut count = 0;
        let mut resultado:Decimal = num;
        let mut vetor_binario: Vec<u16> = Vec::new();
    
        while count !=4{
            resultado = resultado * base;
            if resultado == um { 
                vetor_binario.push(1); 
                break;
            }
            if resultado - um < zero{
                vetor_binario.push(0);
            }
            else{
                vetor_binario.push(1);
                resultado = resultado - um;
            }
            count+=1;
        }
        let result: String  = vetor_binario.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join("");
        return result;
    }
    }
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

 fn main(){


    let  numero1 = 10;
    let numero1string = numero1.to_string();
    let numero1stringref: &str = numero1string.as_str();
    let mut num1: Decimal = Decimal::from_str(numero1stringref).unwrap();
     println!("{}", num1 );

     let numero2 = 1.2;
     let numero2string = numero2.to_string();
     let numero2stringref: &str = numero2string.as_str();
     let num2: Decimal = Decimal::from_str(numero2stringref).unwrap();

    //  let base2: u8 = 2;
    //  let numerobasestring = base2.to_string();
    //  let numerobasestringref = numerobasestring = numerobasestring.as_str();
    //  let numerobase: Decimal = Decimal::from_str(numerobasestringref).unwrap();
    let numerobase = dec!(2);
    let zero = dec!(0);
    let mut count = 0;

     while num1 > zero && count < 6{
        let resto = num1 % numerobase;
        let quociente = num1 / numerobase;
        println!("{}", quociente);
        num1 = quociente;
        count += 1;
        println!("now counnt");
        println!("{}", count);
    }


     println!("floatbin");

 }
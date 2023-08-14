use tradutor_binario::lib::*;
use tradutor_binario::teste::*;

fn main(){
    let result = from_dec(String::from("hex"), Num::Int(14492));
    println!("result{}", result);
    teste1();

}
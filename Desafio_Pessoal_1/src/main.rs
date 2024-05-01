use std::io;

fn main() {
    println!("\n{:=^40}", "Calculadora");

    println!("\nDigite os valores que deseja operar (separados pelo tipo de operação): ");

    let mut valores = String::new();
    io::stdin()
        .read_line(&mut valores)
        .expect("\n\tErro na incerção dos valores\n\t");

    if '+' in valores {
        println!("\n\t Encontrado +\n\t")
    } else{
        println!("\n\tDeu ruim\n\t")
    }

    println!("{:?}", nums);
}
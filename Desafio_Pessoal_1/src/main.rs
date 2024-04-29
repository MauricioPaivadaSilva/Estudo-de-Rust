use std::io;

fn main() {
    println!("\n{:=^40}", "Calculadora");

    println!("\nDigite os valores que deseja operar (separados pelo tipo de operação): ");

    let mut valores = String::new();
    io::stdin()
        .read_line(&mut valores)
        .expect("\n\tErro na incerção dos valores\n\t");

    let nums: Vec<f32> = valores
        .split(" ")
        .map(|c| c.trim().parce().expect("\n\tErr in parce\n"))
        .collect();
}
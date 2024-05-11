use std::io;

fn main() {
    println!("\n{:=^40}", "Calculadora");

    println!("\nDigite os valores que deseja operar (separados pelo tipo de operação e por espaço): ");

    let mut valores = String::new();
    io::stdin()
        .read_line(&mut valores)
        .expect("\n\tErro na incerção dos valores\n\t");

    let nums: Vec<&str> = valores.split(" ").map(|n| n.trim()).collect();

    let mut local = 0;
    let mut soma = 0;

    for n in nums.clone().into_iter() {
        if n == "+" {
            soma = nums[local-1].parse::<i32>().unwrap() + nums[local+1].parse::<i32>().unwrap();
        } else {
            println!("\n\tAinda não planejado");
        }
        local += 1;
    }

    println!("{}", soma);
}

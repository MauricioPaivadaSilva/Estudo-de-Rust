use std::io;

fn main() {
    println!("\n{:=^40}", "Somador");
    println!("\nDigite os valores que deseja somar: (Os valores devem ser separados por vírgula)\nEx.: 1,2,3,50,...");

    let mut valores = String::new();
    io::stdin().read_line(&mut valores).expect("\n\tErr!!\n");

    let nums: Vec<i32> = valores
        .split(",")
        .map(|c| c.trim().parse().expect("\n\tErr in parse\n"))
        .collect();

    let result: i32 = nums.iter().sum();

    println!("\nR.: O total é {}", result);

    println!("\n{}", "-".repeat(47));
}

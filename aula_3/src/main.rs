fn main(){
    let x:u8 = 5; // Especificado 1
    let y = 10_u8; // Especificado 2
    let z = 2; // Não especificado (inferência igual a i32)

    let a = 2.0; // Float não especificado, mas inferido como f64
    
    let b = true; // Utilizando valor bool de forma inferida

    let mut numbers = (x, y, z, a, b);

    println!("\n\t{:?}\n", numbers); // variavel.n = pego o valor da variável na posição n

    numbers.3 += 150.5;
    println!("\n\t{:?}\n", numbers);

    let array: [i32;3] = [1, 2, 3];
    println!("\n\t{:?}\n", array);

    let mut array = [1.1, 2.2, 3.3];
    println!("\n\t{:?}\n", array);

    array[2] = 50.3;
    println!("\n\t{:?}\n", array);
    println!("\n\t{:?}\n", &array[1..3]);
}
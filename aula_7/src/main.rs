/*fn say_hallo(name: &str){
    println!("\n\tHello {name}\n");
}

fn main(){
    say_hallo("Maurício");
}*/

//
//  Convertendo números e somando usando funções
//

fn convert_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}

fn double(n: i32) -> i32 {
    n*2
}

fn main() {
    let input = "56 65 58 48 59 56 87 23";

    let result: Vec<i32> = input.split(' ').map(convert_to_number).map(double).collect();

    println!("\n\tResultado = {:?}", result);
}

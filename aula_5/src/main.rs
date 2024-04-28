// fn main() {
//     let l0 = 'b';
//     let nome: &str = "Maurício"; // str reference

//     let mut s = String::new(); // String -> Vai para a HEAP

//     s.push('M');
//     s.push('a');
//     s.push('u');
//     s.push('r');
//     s.push('i');
//     s.push('c');
//     s.push('i');
//     s.push('o');

//     let mut t = String::new();
//     t.push_str("push_str");

//     println!("\n\tchar => {l0}\n\t&syt => {nome}\n\tString => {s}\n\tString => {t}\n");
// }

use std::io;

fn main() {
    let mut s = String::new();
    println!("\nDigite um texto:");

    io::stdin()
        .read_line(&mut s)
        .expect("\n\tErr reading console\n\t");

    println!("\nVocê digitou: {s}\n");
}

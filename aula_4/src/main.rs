static _Y: u32 = 17; // Aloca a informação juntamente com o binário do programa de forma estática

fn main() {
    let x = 5;
    let z = true;
    let numbers = [1, 2, 3]; // O compilador armazena na meméria stack (tamanho dinâmico)

    let users = get_users(); // Espaço imprevisível (Alocado na HEAP).
}

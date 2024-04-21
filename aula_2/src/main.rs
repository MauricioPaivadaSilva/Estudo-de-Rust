/*
    Variáveis e constantes

    => Deve-se observar todas as operações, pois como é feita a tipagem das variáveis, ele pode dar erro ao final. Ex.: Quando as constantes estavam como i16, não foi possível realizar a multiplicação final, por que elas não suportavam o resultado final. Assim, devo tomar cuidado com isso. Além disso, o Rust não realiza operações entre i8 e i16 por exemplo.
*/

const SECONDS_IN_MINUTE:i32 = 60;

fn main(){
    const MINUTES_IN_HOUR:i32 = 60;
    const SECONDS_IN_HOUR:i32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

    let total = 30; //Variável inicializada.
    let total_in_seconds = total * SECONDS_IN_HOUR;
    println!("Trabalhou {} horas", total_in_seconds);
}
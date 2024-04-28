# Estudo de Rust
Repositório dedicado ao estudo da linguagem Rust.

## Base de estudo

O estudo da linguagem está se dando com base nas aulas da série [Aprenda Rust](https://youtube.com/playlist?list=PLjSf4DcGBdiGCNOrCoFgtj0KrUq1MRUME&si=qkZN1SVDei8BeZek), do canal [CodeShow](https://www.youtube.com/@codeshowbr). Também estou baseando-me na documentação do próprio [Rust](https://www.rust-lang.org/pt-BR).

---

### Aula 1:

Foi realizada a apresentação da linguagem Rust.

Comandos estudados:
| Comando | Ação |
|---|---|
| `rustc` | Compila o arquivo do rust. |
| `cargo new` | Cria um novo projeto usando o gerenciador `cargo`. |
| `cargo run` | Compila e executa o programa rm Rust.|
| `cargo fmt` | Formata todo o código, deixando mais legível. |

### Aula 2:

#### Variáveis e constantes

* Deve-se observar todas as operações, pois como é feita a tipagem das variáveis, ele pode dar erro ao final. Ex.: Quando as constantes estavam como i16, não foi possível realizar a multiplicação final, por que elas não suportavam o resultado final. Assim, devo tomar cuidado com isso.
* Além disso, o Rust não realiza operações entre i8 e i16 por exemplo.

Comandos estudados:
| Comando | Ação |
|---|---|
| `let` | Define a variável (só pode ser definida dentro de um escopo `{}`). |
| `const` | Define como constante (pode ser definida em todo lugar). |
| `:u32` | Define a variável/constante como inteiro sem sinal. Ex.: `let num:u32` |
| `mut` | Permite a mutabilidade da variável (só funciona com variáveis). Ex.: `let mut num:u32` |

P.S.: Normalmente não é definido o tipo de variável, pois é preferivel que o compilador faça a inferência.

### Aula 3

Aula de tipos primitivos em Rust.

#### Tipos primitivos **Inteiros**

| bits | signed | unsigned|
|------|--------|---------|
| 8 | `i8` | `u8` |
| 16 | `i16` | `u16` |
| 32 | `i32` | `u32` |
| 64 | `i64` | `u64` |
| 128 | `i128` | `u128` |
| arch | `isize` | `usize` |

##### Signed

range: $-(2^{n-1})$ até $2^{n-1}-1$.

`i8`: $-128$ até $127$.

##### Unsigned

range: $0$ até $2^{n}-1$.

`u8`: $0$ até $255$

P.S.: Caso não esteja explícito o tipo inteiro que será utilizado, o Rust assume que é o `i32`.

* Pode ser utilizado algumas formas de explicitar o tipo de dado.

| Comando | Tipo |
|---------|------|
| `_` | Facilita a legibilidade de números grandes, mas não interfere no código. Ex.: `123_125` |
| `0x` | Diz que os dados escritos após o _x_ são hexadecimais. Ex.: `0xff` |
| `0o` | Diz que os dados escritos após o _o_ são octais. Ex.: `0o75` |
| `0b` | Diz que os dados escritos após o _b_ são binários. Ex.: `0b1111_000` |
| `b''` | Diz que os dados escritos dentro das aspas simples são _bytes_. Ex.: `b'A'`, nesse caso o _A_ está sendo inserido como o _byte_. |

#### Tipos primitivos **Bool**

| Verdadeiro | Falso |
|---|---|
| `true` | `false` |
| 1 | 0 |

#### Tipos primitivos **Char**

* Aceita até 4 _bytes_ da tabela _unicode_;
* Tem que utilizar aspas simples.

#### Tipo composto **Tupla**

* A tupla aceita tipos diferentes dentro dela.

#### Tipo composto **Matriz**

* Ná matriz só pode ser adicionados tipos iguais.

### Aula 4

Aula sobre memória em Rust.

Comandos estudados:
| Comando | Ação |
|---|---|
| `static` | Define uma variável que será armazenada de forma estática dentro do binário final do programa. |
| `let` | Armazena as variáveis na stack (tamanho dinâmico). |

### Aula 5

Aula sobre textos e caracteres em Rust.

* hexdump: Programa para fazer debug em baixo nível, o hexadecimal do executável. Auxilia em debug.

Comandos estudados:
| Comando | Ação |
|---|---|
| `&str` | String "slice" (string reference). |
| `String::new()` | Cria uma String dentro da HEAP (de tamanho dinâmico) **Tem que usar o `mut`**. |
| `variave.push()` | Adiciona caracteres ('') a String. |
| `variavel.push_str()` | Adiciona palavra ("") a String. |
| `"".to_string()` | Cria um objeto com tipo String. |
| `String::from("")` | Cria uma String a partir de outra. |
| `String::from_iter(variavel)` | Cria uma String a partir de um objeto iterável. |

### Aula 6

Perguntas.

Comandos estudados:
| Comando | Ação |
|---|---|
| `cargo --release` | Compila para distribuição. |
| `.len()` | Retorna a quantidade de char presentes. |
| `.trim()` | Remove char especiais (do começo e do final). |

1. É igual a (2)

```rust
let nums: Vec<&str> = valores.split(",").map(|c| c.trim()).collect();
```

2. 
```rust
fn main(){
    ...
    let nums: Vec<&str> = valores.split(",").map(clean).collect();
    ...
}

pub fn clean(c: &str) -> &str {
    c.trim()
}
```
// Criando uma constante.

const SECONDS_IN_MINUTE: u32 = 60;

// "u32" representa um número inteiro sem sinal.
// Isso significa que ele só pode armazenar valores inteiros positivos (incluindo 0).

// As constantes são imutáveis, ou seja, seu valor não pode ser alterado após a criação.
// Por convenção, seus nomes são escritos em CAIXA ALTA e separados por underline (_).

// Diferente das variáveis comuns, as constantes precisam ter seu tipo definido explicitamente.
// Também não é possível declarar duas constantes com o mesmo nome dentro do mesmo escopo.

//let total = 30;

// Cria uma variável chamada "total" com o valor de 30.

// Porém, essa variável está sendo criada no local errado.
// Não podemos simplesmente declarar variáveis dessa forma fora do escopo principal do programa.

// Para que o código possa ser executado, precisamos definir o início e o fim do escopo principal utilizando "fn main() {}".

fn main () { // Início do escopo. Tudo que for criado aqui existirá enquanto estivermos dentro deste bloco.

    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

    // Isso mostra que constantes não precisam armazenar apenas valores fixos.
    // Elas também podem ser definidas a partir de operações matemáticas realizadas em tempo de compilação.

    let mut total = 30;

    let total_em_segundos = total * SECONDS_IN_HOUR;

    println!("Trabalhou por {} segundos sem se cansar", total_em_segundos);

    total = 44;

    // Aqui alteramos o valor da variável porque ela foi criada com "mut".
    // O "mut" indica que a variável pode receber novos valores durante a execução do programa.

    println!("Trabalhou por {} sem se cansar", total);

    let total = "Quarenta";

    // Aqui acontece algo chamado "shadowing".
    // Não estamos alterando a variável anterior.
    // Estamos criando uma nova variável chamada "total" que substitui a anterior dentro deste escopo.
    // Por isso agora "total" é uma string em vez de um número.

    println!("Trabalhou por {} sem se cansar", total);

    // Demonstrando como funcionam os escopos em Rust.

    let total = 50;

    {
        let total = 66;

        // Esta variável existe apenas dentro deste bloco.
        // Enquanto estivermos aqui, ela "esconde" a variável externa.

        println!("{}", total);
    }

    println!("{}", total);

    // Quando o bloco interno termina, a variável criada nele deixa de existir.
    // Então voltamos a enxergar a variável que estava no escopo externo.
    // Por isso o primeiro println! exibe 66 e o segundo exibe 50.

    // O "{}" funciona como um placeholder, ou seja, um espaço reservado para inserir valores.
    // Após a string, usamos uma vírgula e informamos qual variável será colocada naquele local.

} // Fim do escopo principal.

// Ao chegar ao final do escopo ocorre o "drop".
// Os valores que pertencem a esse escopo deixam de existir e sua memória pode ser liberada.

// O Rust também avisa quando uma variável foi criada mas nunca foi utilizada.

// Caso queira criar uma variável que não será usada de propósito,
// podemos começar seu nome com "_".
// Isso informa ao compilador que a variável pode ficar sem uso.

// Acabei essa aula no timer 12:34.

// ESCALARES E TIPOS COMPOSTOS

// Tipos escalares armazenam apenas um único valor.
// Esse valor pertence a uma categoria específica e conhecida,
// permitindo comparações e operações diretas.

// Tipos escalares:

// - Inteiro (integer)
//   Exemplo: 5

// - Ponto flutuante (floating point)
//   Exemplo: 42.1

// - Booleano (bool)
//   Exemplo: true, false

// - Caractere (char)
//   Exemplo: 'a', 'Z', '@', emojis e outros caracteres Unicode.

// Parei nos tópicos compostos.

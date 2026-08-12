//let total = 30;

// Cria uma variável chamada "total" com o valor de 30.

// Porém, essa variável está sendo criada no local errado.
// Não podemos simplesmente declarar variáveis dessa forma fora do escopo principal do nosso programa.

// Para que o código possa ser executado, precisamos definir o início e o fim do escopo principal utilizando "fn main() {}".

fn main () { // Início do escopo. A variável vai existir e poderá ser utilizada enquanto estivermos dentro desse escopo.

let mut total = 30;

println!("Trabalhou por {} sem se cansar" , total);

total = 44; // Aqui podemos alterar o valor da variável porque usamos "mut" na sua criação.
// O "mut" indica que essa variável pode ter seu valor alterado depois de criada.

println!("Trabalhou por {} sem se cansar" , total);

// O "{}" funciona como um placeholder, ou seja, um espaço reservado onde o valor de uma variável será inserido.
// Depois das aspas, usamos uma vírgula e colocamos o nome da variável que queremos inserir naquele espaço.

} // Fim do escopo.
// Ao chegarmos ao final desse escopo, ocorre o "drop".
// Isso significa que, ao sair desse escopo, as variáveis que pertencem a ele deixam de existir e sua memória pode ser liberada.

// O Rust também informa diretamente no terminal quando uma variável foi criada, mas não está sendo utilizada.

// Caso queira criar de propósito uma variável que não será utilizada, podemos colocar "_" no começo do nome.
// No Rust, o "_" é usado para indicar que aquela variável pode não ser utilizada.

// Acabei essa aula no timer 12:34.

# Rust — Fase 0: Ownership e Borrowing

> Fundamentos antes do projeto OpenEye. Sem isso, nada do resto faz sentido.

---

## 1. Tipo vs Valor (a base de tudo)

Duas coisas diferentes que parecem a mesma coisa:

- **Tipo** = categoria de algo. Ex: "Cachorro" (a categoria).
- **Valor** = a coisa específica, real, dentro dessa categoria. Ex: "Rex, o cachorro do vizinho".

No código:

- `String` (S maiúsculo) = **categoria** "texto que pode mudar". Não é nenhum texto específico ainda.
- `"rust"` = **valor** real. Um texto de verdade, com conteúdo.

Quando escrevemos `s: &String` dentro da definição de uma função, ainda **não existe texto real** — é só a regra dizendo "essa função aceita textos emprestados dessa categoria". O valor real só existe quando alguém *chama* a função de verdade, passando algo como `&s1`.

Analogia: a receita de bolo ("pede farinha") não é o bolo em si. O tipo é a receita, o valor é o bolo pronto.

---

## 2. `String::from("rust")` — dissecado

```rust
let s1 = String::from("rust");
```

- `let` → cria uma variável.
- `s1` → nome da variável.
- `String` → categoria de dado: texto que pode ser modificado (diferente de `&str`, que é texto fixo).
- `::` → "vai lá dentro dessa caixa (`String`) e pega essa ferramenta específica". Diferente de `:`, que só diz "esse aqui é desse tipo".
- `from` → nome de uma **função** que já vem pronta dentro de `String`. Ela constrói um `String` a partir de outra coisa (nesse caso, a partir de um texto fixo).
- `("rust")` → o ingrediente que a função `from` recebe: o texto fixo `"rust"`.

**Tradução final:** "Crie uma variável chamada `s1`. Nela, guarde um `String` (texto que pode mudar), construído a partir do texto `"rust"`, usando a função `from`."

### Diferença `:` vs `::`

| Símbolo | Significado | Exemplo |
|---|---|---|
| `:` (um) | "isso aqui é desse tipo" | `s: &String` |
| `::` (dois) | "acessa algo de dentro de uma caixa/categoria" | `String::from(...)` |

São símbolos sem relação um com o outro — só coincidem de usar `:`.

---

## 3. Ownership (posse) — as 3 regras

1. Cada valor tem **um dono** (uma variável).
2. Só pode existir **um dono por vez**.
3. Quando o dono sai de escopo, o valor é destruído automaticamente (sem garbage collector).

### Por que isso existe

Em Python/JS, existe um processo de fundo (garbage collector) rodando sempre, checando o que ainda tá em uso pra limpar da memória. Isso gasta processamento constante.

Rust decide **em tempo de compilação** (antes do programa rodar) quem é dono de cada valor, e libera a memória automaticamente assim que o dono sai de uso — sem gastar processamento extra em runtime.

Pra isso funcionar sem bug, só pode existir **um dono por vez**. Se duas variáveis fossem donas da mesma coisa, as duas tentariam liberar a mesma memória no final — erro (double free).

### Exemplo 1 — Move (transferência de posse)

```rust
fn main() {
    let s1 = String::from("rust");
    let s2 = s1;
    println!("{}", s1); // ERRO: s1 não é mais válida
}
```

**Por quê:** `String` guarda dado que pode mudar de tamanho (fica no heap, memória dinâmica). Quando fazemos `s2 = s1`, Rust **não copia o conteúdo** — ele **transfere a posse** (`move`) de `s1` pra `s2`. Depois disso, `s1` deixa de ser dona de qualquer coisa. Usá-la de novo é erro.

**Analogia:** `String` é uma caixa de ferramentas emprestada, não um post-it fotocopiável. Quando você entrega a caixa pra outra pessoa, você não fica com uma cópia — você fica sem a caixa.

### Solução — `.clone()`

```rust
fn main() {
    let s1 = String::from("rust");
    let s2 = s1.clone(); // copia real dos dados
    println!("{}", s1); // funciona
    println!("{}", s2); // funciona também — são duas caixas separadas
}
```

`.clone()` duplica os dados de verdade na memória. Custa processamento, por isso só é usado quando você realmente precisa das duas cópias existindo — diferente de Python, onde copiar não é nem uma decisão consciente.

---

## 4. Borrowing (empréstimo) — usar sem tomar posse

### O problema que resolve

Se toda função que recebe um valor "rouba" a posse dele, o programa fica inutilizável rápido — a variável original sempre sumiria depois de qualquer chamada de função.

**Analogia:** você empresta seu carro pro seu amigo checar o pneu. Ele usa o carro, mas devolve — o carro continua seu. Ele não virou dono só porque mexeu nele.

Isso, em Rust, é o **borrow**. Símbolo: `&`.

### Exercício 2 — dissecado linha por linha

```rust
fn tamanho(s: &String) -> usize {
    s.len()
}
```

| Pedaço | Significado |
|---|---|
| `fn` | abreviação de "function" — avisa que uma ação com nome está sendo criada |
| `tamanho` | nome escolhido pra função (poderia ser qualquer nome) |
| `(s: &String)` | parâmetro: um ingrediente chamado `s`, que precisa ser do tipo `&String` |
| `s` | nome escolhido pro parâmetro (poderia ser `texto`, `x`, etc.) |
| `&String` | "referência emprestada a um `String`" — não é posse, é só olhar |
| `->` | "e o que essa função devolve é do tipo..." |
| `usize` | tipo de número usado pra contagens/tamanhos (nunca negativo) |
| `{ ... }` | marca início e fim do corpo da função — o que ela faz de verdade |
| `s.len()` | chama a função `len` (length = tamanho), que já vem pronta em todo `String`, contando quantos caracteres tem |

**Por que `{ }` não pode ficar vazio aqui:** a função prometeu devolver um `usize` (por causa do `-> usize`). Se o corpo tá vazio, ela não devolve nada — quebra a promessa, erro de compilação.

**Por que não tem `return`:** em Rust, a última linha de uma função, **sem ponto e vírgula no final**, já é automaticamente o valor devolvido. Regra específica da linguagem.

### Sem `&` vs com `&`

```rust
fn tamanho(s: String) -> usize { ... }  // sem &: TOMA a posse. Quem chamou perde a variável.
fn tamanho(s: &String) -> usize { ... } // com &: só EMPRESTA. Quem chamou continua dono.
```

Chamando:

```rust
tamanho(nome);   // sem &: nome vira dono-de-nada depois disso
tamanho(&nome);  // com &: nome continua existindo normalmente depois
```

### Regra resumida

- Sem `&` → transferência de posse (a origem morre, vira inválida).
- Com `&` → empréstimo (a origem sobrevive, continua utilizável).

### Solução completa do exercício 2

```rust
fn tamanho(s: &String) -> usize {
    s.len()
}

fn main() {
    let nome = String::from("openeye");
    let n = tamanho(&nome);       // empresta nome, não perde posse
    println!("{}", nome);          // funciona: nome ainda existe
    println!("Tamanho: {}", n);
}
```

---

## Resumo mental rápido

| Conceito | Regra de ouro |
|---|---|
| Ownership | 1 valor = 1 dono por vez. Passar sem `&` transfere a posse. |
| Move | `let s2 = s1` invalida `s1` se for tipo `String` (dado no heap). |
| Clone | `.clone()` força cópia real, os dois continuam válidos. |
| Borrow imutável | `&T` = só olha, não modifica, não toma posse. |
| Tipo vs Valor | Tipo = categoria (`String`). Valor = coisa real (`"rust"`). |
| `:` vs `::` | `:` diz o tipo. `::` acessa algo de dentro de uma categoria. |
| `->` | Indica o tipo que a função devolve. |
| `usize` | Tipo de número pra contagens/tamanhos, nunca negativo. |

---

## Pendente (próxima sessão)

- Exercício 3 — borrow mutável (`&mut`) e a regra de exclusividade (por que só pode existir um `&mut` por vez).
- Exercício 4 — struct + ownership.
- Exercício 5 — lifetimes explícitos (`'a`).

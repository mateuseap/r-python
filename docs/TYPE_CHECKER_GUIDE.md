# Guia de Implementação: type checker de classes no RPython

Fase 2 do projeto de classes. A fase 1 ensinou o parser a reconhecer a sintaxe; esta fase
ensina o **type checker** a decidir se um programa com classes é bem tipado. O
interpretador continua fora do escopo: ele ainda não executa classes.

Documentos relacionados:

- `docs/IMPLEMENTATION_GUIDE.md`: fase 1 (parser).
- `docs/PROPOSTA_CLASSES.md`: proposta completa, em especial a seção 3.5.
- `docs/type_checker.md`: visão geral do type checker antes desta fase.
- `docs/presentation/type-checker.html`: slides desta fase.

PRs: #6 (declaração, métodos e instanciação), #7 (acesso a membros e atribuição a
campo), #8 (runner de demonstração e programas inválidos) e #9 (esta documentação).

---

## 1. Como o type checker funciona

O type checker percorre a AST produzida pelo parser, sem executar nada. Há duas funções
centrais:

| Função | Arquivo | Recebe | Devolve |
|---|---|---|---|
| `check_expr` | `src/type_checker/expression_type_checker.rs` | uma `Expression` e o ambiente | o `Type` da expressão, ou um erro |
| `check_stmt` | `src/type_checker/statement_type_checker.rs` | um `Statement` e o ambiente | um **novo** ambiente, ou um erro |

O **ambiente** (`Environment<Type>`, em `src/environment/environment.rs`) guarda o que
já foi declarado: variáveis (com mutabilidade e tipo), funções, ADTs e, a partir desta
fase, classes. Ele é uma pilha de escopos: `push()` abre um bloco, `pop()` fecha.

`check_stmt` nunca altera o ambiente recebido. Ele devolve uma cópia atualizada: por
exemplo, `val p = Point()` devolve um ambiente onde `p : Point`. Um programa é checado
encadeando esses ambientes, statement por statement.

Erros são `String` com prefixo `[Type Error]` ou `[Name Error]`. O primeiro erro
interrompe a checagem.

---

## 2. Requisitos desta fase

Extraídos da seção 3.5 da proposta.

| ID | Requisito | Estado | PR |
|---|---|---|---|
| REQ-01 | Registrar classes no ambiente | DONE | #6 |
| REQ-02 | Checar campos: nomes únicos, inicializador compatível com o tipo | DONE | #6 |
| REQ-03 | Checar métodos como funções, com `self` em escopo | DONE | #6 |
| REQ-04 | Validar que `self` tem o tipo da classe | DONE | #6 |
| REQ-05 | `Point()` tem tipo `TClass("Point")` | DONE | #6 |
| REQ-06 | `obj.campo`: `obj` é instância e o campo existe; devolve o tipo do campo | DONE | #7 |
| REQ-07 | `obj.metodo(args)`: método existe e argumentos batem; devolve o tipo de retorno | DONE | #7 |
| REQ-08 | `obj.campo = expr`: campo existe, é `var` e recebe valor do tipo certo | DONE | #7 |
| REQ-09 | Testes de casos válidos e inválidos | DONE | #6, #7, #8 |

---

## 3. Gaps encontrados

Comparando os requisitos com o código ao fim da fase 1:

| Gap | Descrição |
|---|---|
| GAP-01 | `ClassDef` era rejeitado com "classes are not supported yet" |
| GAP-02 | O ambiente não tinha onde guardar classes |
| GAP-03 | `Point()` caía em `check_func_call` e falhava com "was called but never declared" |
| GAP-04 | `FieldAccess` e `MethodCall` eram rejeitados com "member access is not supported yet" |
| GAP-05 | `FieldAssignment` era rejeitado com "field assignment is not supported yet" |
| GAP-06 | Qualquer identificador em posição de tipo virava `TClass`, então `Itn` passava sem erro |

---

## 4. O que mudou, arquivo por arquivo

### PR #6: declaração, métodos e instanciação

**`src/environment/environment.rs`**

- Campo novo `classes: HashMap<Name, Class>` em cada `Scope`.
- `map_class`, `lookup_class` e `get_all_classes`, seguindo o mesmo padrão de ADTs e
  funções.
- `set_global_classes`: usado ao checar o corpo de uma função, que roda num ambiente novo.
  Sem isso, uma função não enxergaria as classes declaradas antes dela.

**`src/type_checker/statement_type_checker.rs`**: `check_class_def_stmt`

1. Rejeita classe já declarada.
2. Registra a classe **antes** de checar os membros, para que um campo ou método possa
   citar a própria classe (`self: Point`, ou um método que devolve `Point`).
3. Para cada campo: nome não repetido, tipo conhecido e inicializador compatível
   (`type_fits`).
4. Para cada método: nome não repetido e primeiro parâmetro exatamente `self: <Classe>`.
5. Só depois que todas as assinaturas são conhecidas, checa o corpo de cada método com
   `check_func_def_stmt`. Assim, um método pode chamar outro declarado mais abaixo.
6. Os métodos **não** entram como funções globais: o ambiente devolvido por
   `check_func_def_stmt` é descartado. `get(C())` continua sendo erro.

`check_func_def_stmt` ganhou duas linhas: valida os tipos dos parâmetros e do retorno
com `check_type_is_known`, e copia as classes para o ambiente do corpo.

**`src/type_checker/expression_type_checker.rs`**

- `check_func_call`: se o nome chamado é uma classe, desvia para `check_instantiation`.
- `check_instantiation`: devolve `TClass(nome)` e rejeita argumentos (construtor com
  argumentos é trabalho futuro).
- `type_fits(esperado, atual)`: compatibilidade usada em campos, argumentos, retornos e
  atribuições. Igual à igualdade de tipos, exceto que `Any` dentro do tipo atual é
  aceito. Isso permite `var itens: List[Int] = []` e `return Nothing` em função que
  devolve `Maybe[Int]`.
- `check_type_is_known`: um `TClass(nome)` precisa ser uma classe declarada (ou um ADT,
  já que o parser lê qualquer identificador em posição de tipo como `TClass`). Percorre
  também `List`, `Maybe`, `Result`, tuplas e tipos de função.

### PR #7: acesso a membros e atribuição a campo

**`src/type_checker/expression_type_checker.rs`**

- `class_of`: tipa a expressão e exige `TClass`. Se não for, o erro diz que só
  instâncias de classe têm campos (ou métodos).
- `lookup_field`: encontra a declaração do campo. Usado pelo acesso e pela atribuição.
- `FieldAccess(obj, campo)` devolve o tipo declarado do campo.
- `check_method_call`: encontra o método, ignora o primeiro parâmetro (`self`, já ligado
  a `obj`) e confere quantidade e tipo dos argumentos restantes.

**`src/type_checker/statement_type_checker.rs`**: `check_field_assignment_stmt`

1. O campo precisa existir.
2. O campo precisa ser `var`. Atribuir a um `val` é erro.
3. O valor precisa ser compatível com o tipo do campo.

A mutabilidade é **do campo**, não da variável que guarda o objeto. Por isso
`val p = Point(); p.x = 3` é aceito quando `x` é `var`: a variável `p` não muda, o
campo sim.

**`examples/classes/point_declaration.rpy`**: o método `translate` usava `x = x + dx`,
o que o type checker rejeita (campos só são alcançados por `self`). Passou a usar
`self.x = self.x + dx`.

### PR #8: runner e programas inválidos

- `examples/typecheck.rs`: lê um arquivo, faz o parse e o type check, e imprime o
  primeiro erro. Necessário porque `cargo run` não invoca o type checker.
- `examples/classes/type_errors/`: cinco programas aceitos pelo parser, cada um com um
  único erro de tipo.

---

## 5. Exemplo: checando o programa da proposta

`examples/classes/point_full.rpy`, statement por statement:

| Statement | O que o type checker faz | Ambiente depois |
|---|---|---|
| `class Point: ... end` | registra `Point`; checa `x`, `y`, `init` e `get_x` | `Point` declarada |
| dentro de `init`: `self.x = x` | `self : Point`, `x` é `var Int`, valor `x : Int` | sem mudança |
| dentro de `get_x`: `return self.x` | `self.x : Int` bate com o retorno `Int` | sem mudança |
| `val p = Point()` | `Point` é classe, sem argumentos: `TClass("Point")` | `p : Point` |
| `p.init(3, 4)` | `init` espera `(Int, Int)` depois de `self`; recebe `(Int, Int)` | sem mudança |
| `val px = p.get_x()` | `get_x` devolve `Int` | `px : Int` |
| `assert(px == 3, ...)` | `px == 3` é `Boolean` | sem mudança |

Resultado: o programa é bem tipado.

```bash
cargo run --example typecheck -- examples/classes/point_full.rpy
# examples/classes/point_full.rpy: ok, the program is well typed
```

## 6. Exemplos de erro

| Programa | Mensagem |
|---|---|
| `wrong_self_type.rpy` | `first parameter of method 'area' must be 'self: Square'` |
| `unknown_type.rpy` | `unknown class 'Pont'` |
| `missing_field.rpy` | `class 'Point' has no field 'z'` |
| `val_field_assignment.rpy` | `field 'id' of class 'Account' is a val and cannot be reassigned` |
| `wrong_argument.rpy` | `argument 1 of method 'add' expects Int, found String` |

Um método **sem** `self` nem chega ao type checker: o parser já o rejeita (fase 1). O
que só o type checker pega é `self` com o tipo errado.

---

## 7. Decisões de projeto

**Classes compartilham namespace com variáveis e funções.** Declarar `class Point` onde
já existe variável ou função `Point` é erro, e o contrário também. Sem isso,
`Point()` seria ambíguo entre instanciação e chamada de função.

**Defaults de campos são expressões fechadas.** Podem usar constantes, operadores e
instanciar classes já declaradas, mas não capturam variáveis ou funções externas. Uma
instanciação direta da própria classe (`var next: Node = Node()`) também é rejeitada,
porque nunca terminaria. Recursão com caso base continua possível:
`var next: Maybe[Node] = Nothing`.

**`return` fora de função é erro, não panic.** Retornos usam `type_fits`, então `[]` e
`Nothing` cabem em retornos `List[T]` e `Maybe[T]`.

**`self` não virou palavra reservada.** Se `self` entrasse em `KEYWORDS`, o
`identifier` deixaria de aceitá-lo, e tanto `def m(self: Point)` quanto `self.x` deixariam
de parsear, exigindo mudanças em vários pontos do parser. As regras que importam ficaram
no type checker: `self` é o primeiro parâmetro e tem o tipo da classe. Como parâmetros
são imutáveis, `self = ...` já é erro.

**O parser continua aceitando `.campo` depois de qualquer primário.** Restringir no
parser quebraria `p.x` e `f().y`. O type checker rejeita ponto em quem não é classe, com
mensagem clara. Python e Java também aceitam isso na sintaxe.

**`Point()` continua sendo `FuncCall`.** Distinguir instanciação de chamada exige saber
se o nome é uma classe, o que é informação semântica. O type checker resolve isso
consultando o ambiente, sem nó `New` e sem mudar o parser.

**Classes são registradas antes dos membros, e métodos são checados depois de todas as
assinaturas.** Permite campos e métodos que citam a própria classe e métodos que chamam
outros declarados mais abaixo.

**O type checker não foi ligado ao `cargo run`.** Os exemplos antigos usam
`var _ = print(...)` várias vezes, e o type checker trata a redeclaração de `_` como erro.
Ligá-lo agora quebraria esses exemplos. O runner `examples/typecheck.rs` cobre a
demonstração.

---

## 8. Testes

Todos em `tests/type_checker_classes_tests.rs`. Os programas são escritos em RPython e
passados pelo parser, então cada teste se lê como a linguagem.

| Módulo | O que cobre |
|---|---|
| `class_declaration` | classe válida, `Point()`, argumentos rejeitados, duplicatas, inicializador incompatível, tipo desconhecido, `[]` e `Nothing` em campos, campo de outra classe |
| `declaration_edge_cases` | colisões classe/variável/função, defaults fechados, ciclo direto, recursão por `Maybe`, retornos `Nothing`/`[]`, `return` fora de função |
| `methods` | `self` válido e inválido, método duplicado, corpo checado, parâmetro com tipo desconhecido, método que cria instância, método não vira função global |
| `class_types_outside_classes` | parâmetro e retorno de função com classe desconhecida, função que instancia classe |
| `field_access` | tipo do campo, campo de instanciação, aritmética, campo inexistente, ponto em não-classe, encadeamento |
| `method_call` | tipo de retorno, `self` implícito, método chamando método, método inexistente, quantidade e tipo de argumentos, ponto em não-classe, corpo usando campo inexistente |
| `field_assignment` | `var` aceito, `val` rejeitado, tipo errado, campo inexistente, `self.campo = v`, `val` não congela campos `var` |
| raiz | exemplos `point_full.rpy` e `point_declaration.rpy` tipam; os cinco programas inválidos falham com a mensagem esperada |

```bash
cargo test --test type_checker_classes_tests   # 57 testes
cargo test                                     # 693 testes
```

---

## 9. Limitações atuais

- O interpretador não executa classes: `cargo run` em um programa com classe termina com
  `not implemented yet`.
- Construtor com argumentos não existe: `Point(1, 2)` é erro de tipo.
- Sem herança: `TClass` só é compatível com a mesma classe.
- Comentários `#` não são aceitos pelo parser.
- O type checker não roda no `cargo run` (ver seção 7).

## 10. Fora do escopo desta fase (FUTURE WORK)

- Interpretador: representar instâncias em runtime e executar `FieldAccess`,
  `MethodCall` e `FieldAssignment`.
- Construtor com argumentos e o nó `New`.
- Herança, sobrescrita, visibilidade e métodos estáticos.
- Ligar o type checker ao `cargo run`, liberando `_` para redeclaração.
- Pretty print completo de classes.

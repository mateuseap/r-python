# Guia de Implementação: sintaxe de classes no parser do RPython

Este documento explica o que foi feito na fase 1 do projeto de classes, cujo escopo,
definido pelo professor, é **somente o parser**. O objetivo é permitir explicar a
implementação sem depender de decorar o código.

PRs: #1 (exemplos `.rpy`), #2 (PR A, declaração de classe), #3 (PR B, acesso a membros), #4 (esta documentação).

Documentos relacionados:

- `docs/PROPOSTA_CLASSES.md`: proposta completa (todas as fases).
- `docs/BRIEFING_REUNIAO.md`: briefing da primeira reunião.
- `docs/PARSER.md`: visão geral do parser antes desta fase.
- `docs/presentation/index.html`: slides da apresentação.

---

## 1. Como o RPython funciona hoje

RPython é uma linguagem estaticamente tipada com sintaxe parecida com Python,
implementada em Rust. O programa passa por estas etapas:

```text
arquivo .rpy  (String)
   |
   v
parser       src/parser/          texto  ->  Vec<Statement>  (AST)
   |
   v
type checker src/type_checker/    AST    ->  Ok / erro de tipo
   |
   v
interpreter  src/interpreter/     AST    ->  execução
```

O driver é `src/main.rs`: lê o arquivo, chama `parser::parse` e executa com
`execute_block`. Para rodar:

```bash
cargo run -- examples/hello.rpy
```

### Não existe lexer separado

O parser usa a biblioteca de **combinadores** `nom`. Não há uma fase de tokenização:
cada combinador consome diretamente caracteres da string de entrada. Os "tokens" são
produzidos por combinadores pequenos em `src/parser/parser_common.rs`:

| Combinador | O que reconhece |
|---|---|
| `keyword("class")` | a palavra inteira `class`, com espaços opcionais em volta, e garante que não é prefixo de identificador (`classe` não casa) |
| `identifier` | `[a-zA-Z_][a-zA-Z0-9_]*`, rejeitando palavras de `KEYWORDS` (`src/parser/keywords.rs`) |
| `char(':')`, `char(';')`, `char('.')` | um caractere literal |
| `multispace0` | espaços e quebras de linha (zero ou mais) |

Todo parser tem o tipo `fn(&str) -> IResult<&str, T>`. Em caso de sucesso devolve
`Ok((resto_da_entrada, valor))`. Em caso de falha devolve `Err(nom::Err::Error(..))`,
o que permite que `alt` tente a próxima alternativa (backtracking).

---

## 2. Onde começa o parsing

`src/parser/mod.rs`, função `parse`:

```rust
pub fn parse(input: &str) -> IResult<&str, Vec<Statement>> {
    // multispace0, lista de parse_statement separados por ';', ';' final opcional
}
```

Um programa é uma lista de statements separados por `;`.

### Statements: `parse_statement` (`src/parser/parser_stmt.rs`)

É um `alt` com uma alternativa por tipo de statement. A ordem importa: a primeira
alternativa que casa vence. Statements que começam com palavra-chave vêm antes; as
alternativas genéricas (atribuição e expressão solta) ficam por último para não
consumirem palavras-chave como se fossem identificadores.

Ordem atual (as linhas marcadas com `+` foram adicionadas nesta fase):

```text
  parse_var_declaration_statement
  parse_val_declaration_statement
  parse_if_chain_statement
  parse_if_else_statement
  parse_while_statement
  parse_for_statement
  parse_assert*_statement (5 variantes)
  parse_test_function_definition_statement
+ parse_class_definition_statement
  parse_function_definition_statement
  parse_return_statement
  parse_break_statement
  parse_continue_statement
+ parse_field_assignment_statement
  parse_assignment_statement
  parse_expression_statement
```

### Blocos

`parse_block` reconhece `:` + lista de statements separados por `;` + `end` e produz
`Statement::Block(Vec<Statement>)`. Funções (`def`), `while`, `for` e `test` usam
`parse_block`. O `if/elif/else` usa `parse_inner_block`, que não consome `end`, porque
só o último ramo fecha a construção.

### Funções

`parse_function_definition_statement` reconhece
`def nome(arg: Tipo, ...) -> Tipo <bloco>` e produz `Statement::FuncDef(Function)`.
Cada parâmetro é lido por `parse_formal_argument` (`nome : tipo`), e o tipo por
`parse_type` (`src/parser/parser_type.rs`).

### Expressões: escada de precedência (`src/parser/parser_expr.rs`)

Cada nível chama o nível de precedência maior e combina os resultados com
`fold_many0`, o que gera associatividade à esquerda:

```text
parse_expression
  parse_or            or
  parse_and           and
  parse_not           not
  parse_relational    < <= > >= == !=
  parse_add_sub       + -
  parse_term          * /
  parse_factor        (nesta fase: primário seguido de sufixos .nome / .nome(args))
  parse_primary       literais, listas, chamadas f(x), variáveis, parênteses, lambda
```

### Erros de parsing

Não há mensagens de erro customizadas: o parser devolve o erro do `nom`
(`ErrorKind::Tag`, `Char`, ...). Quando um programa é aceito só em parte, `parse`
termina com sucesso mas o resto não consumido fica em `rest`; `main.rs` imprime
`Aviso: código não consumido`. Os testes desta fase usam essas duas formas para
verificar sintaxe inválida: ou o parser falha, ou ele não produz o nó esperado / não
consome toda a entrada.

---

## 3. Requisitos desta fase

Extraídos de `docs/PROPOSTA_CLASSES.md` (seções 3.2, 3.3 e 3.4, Fase 1) e limitados
pela orientação do professor (somente parser).

| ID | Requisito | Fonte | Estado | PR |
|---|---|---|---|---|
| REQ-01 | `class` é palavra-chave reservada | Proposta 3.2 regra 1, 3.4 | DONE | A |
| REQ-02 | Declaração `class Nome: ... end` | Proposta 3.2 regra 2 | DONE | A |
| REQ-03 | Corpo aceita somente `val`, `var` e `def` | Proposta 3.2 regra 3 | DONE | A |
| REQ-04 | Método deve ter `self` como primeiro parâmetro | Proposta 3.2 regra 4 | DONE (só o nome; o tipo fica para o type checker) | A |
| REQ-05 | Nome de classe pode ser usado como tipo (`self: Point`) | Proposta 3.2 regra 5, 3.3 `TClass` | DONE | A |
| REQ-06 | AST: `Class`, `FieldDeclaration`, `ClassDef`, `TClass` | Proposta 3.3 | DONE | A |
| REQ-07 | Acesso a campo `obj.campo` / `self.campo` | Proposta 3.2 regra 6, 3.4 | DONE | B |
| REQ-08 | Chamada de método `obj.metodo(args)` | Proposta 3.1 item 4, 3.4 | DONE | B |
| REQ-09 | Atribuição a campo `obj.campo = expr` | Proposta 3.3, 3.4 | DONE | B |
| REQ-10 | Testes de parser para a nova sintaxe | Proposta 4 Fase 1 | DONE | A e B |
| REQ-11 | Instanciação `Nome()` | Proposta 3.2 regra 7 | Sem mudança necessária: já é `FuncCall("Nome", [])`. O nó `New` exige saber se o nome é classe, o que é informação semântica. FUTURE WORK | - |

---

## 4. Gaps encontrados

Comparando os requisitos com o parser da branch `develop`:

| Gap | Descrição | Resolvido em |
|---|---|---|
| GAP-01 | `class` não era reservada; `class = 1` era uma atribuição válida | PR A |
| GAP-02 | Não havia nó de AST para declaração de classe | PR A |
| GAP-03 | Nenhuma regra reconhecia `class Nome: ... end` | PR A |
| GAP-04 | `parse_type` não aceitava nome de classe: `def f(self: Point)` falhava | PR A |
| GAP-05 | O caractere `.` só existia dentro de números reais; `p.x` parava em `p` | PR B |
| GAP-06 | Atribuição aceitava só identificador no lado esquerdo | PR B |
| GAP-07 | Não havia testes de parsing de classes | PR A e B |

---

## 5. Trabalho que já existia

| Item | O que faz | Escopo atual? | Decisão |
|---|---|---|---|
| Exploração inicial | Protótipo local de classes só com campos, indo do parser ao interpretador | Extrapola: implementa semântica e runtime | Usado só como estudo e apresentado na reunião. A implementação desta fase foi feita do zero a partir de `develop`, limitada ao parser. |
| PR #1 | Exemplos `.rpy` (hello, grades, math, statistics) | Base para entender a sintaxe | Mergeado |
| Branches `upstream/*` | Trabalhos antigos do repositório original | Não relacionados a classes | Ignoradas |
| Issues | Desabilitadas no fork | - | A rastreabilidade fica nesta tabela e nos PRs |

---

## 6. O que mudou, arquivo por arquivo

### PR A (#2): declaração de classe (`feature/parser-class-declaration`)

**`src/parser/keywords.rs`**: `"class"` adicionado a `KEYWORDS`. Efeito: `identifier`
rejeita `class`, então `class` não pode mais ser nome de variável, função ou classe.

**`src/parser/parser_common.rs`**: constante `CLASS_KEYWORD = "class"`.

**`src/ir/ast.rs`**:

```rust
pub struct FieldDeclaration {
    pub name: Name,
    pub field_type: Type,
    pub mutable: bool,               // var = true, val = false
    pub initializer: Box<Expression>,
}

pub struct Class {
    pub name: Name,
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<Function>,      // reaproveita a struct Function existente
}

enum Type      { ..., TClass(Name) }
enum Statement { ..., ClassDef(Class) }
```

Por que `methods: Vec<Function>`: um método é sintaticamente igual a uma função. Usar a
mesma struct permite reaproveitar o parser de função e, no futuro, o type checker de
função.

Por que o inicializador é obrigatório: a proposta mostra todos os campos com valor
(`val x: Int = 0`), e o construtor implícito `Point()` da proposta depende desses
valores default.

**`src/parser/parser_stmt.rs`**: três funções novas e uma linha em `parse_statement`.

- `parse_class_field`: `(val | var) IDENT : tipo = expr`. Produz `FieldDeclaration` com
  `mutable = (kw == "var")`. Diferente de `var x = 0` comum, o tipo é obrigatório.
- `parse_class_method`: chama `parse_function_definition_statement` (sem duplicar a
  regra de função) e confere que o primeiro parâmetro se chama `self`. Se não, falha
  com `ErrorKind::Verify`. A checagem de que `self` tem o tipo da classe é semântica e
  fica para o type checker.
- `parse_class_definition_statement`: `class IDENT : membros end`, onde membros é
  `separated_list0(';', alt((campo, método)))`, igual à forma de `parse_block`.
  Cada membro vira um `ClassMember` (enum privado) e no final é separado em `fields` e
  `methods`.
- Em `parse_statement`, `parse_class_definition_statement` entra entre `test` e `def`,
  junto das outras alternativas guiadas por palavra-chave.

**`src/parser/parser_type.rs`**: `parse_class_type`, a última alternativa de
`parse_type`. Um identificador em posição de tipo vira `Type::TClass(nome)`. Ficando por
último, `Int`, `List[...]`, `Maybe[...]` etc. continuam tendo prioridade.

### PR B (#3): acesso a membros (`feature/parser-member-access`)

**`src/parser/parser_common.rs`**: constante `DOT_CHAR = '.'`.

**`src/ir/ast.rs`**:

```rust
enum Expression {
    ...,
    FieldAccess(Box<Expression>, Name),                  // obj.campo
    MethodCall(Box<Expression>, Name, Vec<Expression>),  // obj.metodo(args)
}
enum Statement {
    ...,
    FieldAssignment(Box<Expression>, Name, Box<Expression>),  // obj.campo = expr
}
```

O objeto é uma `Expression` (e não um `Name`) para permitir encadeamento:
`a.b.c`, `self.origem.x`, `f(1).y`.

**`src/parser/parser_expr.rs`**: o antigo `parse_factor` foi renomeado para
`parse_primary` (conteúdo idêntico) e um novo `parse_factor` foi criado:

```rust
pub fn parse_factor(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_primary(input)?;
    fold_many0(
        preceded(char('.'), pair(identifier, opt(parse_actual_arguments))),
        move || init.clone(),
        |acc, (name, args)| match args {
            Some(args) => Expression::MethodCall(Box::new(acc), name.to_string(), args),
            None => Expression::FieldAccess(Box::new(acc), name.to_string()),
        },
    )(input)
}
```

Por que nesse lugar: `parse_factor` é o nível de maior precedência. Colocando o sufixo
`.` ali, ele liga mais forte que qualquer operador, então `p.x + 1` é
`Add(FieldAccess(p, x), 1)` e não `FieldAccess(p, x + 1)`. O `fold_many0` repete
enquanto houver `.`, produzindo associação à esquerda.

`3.14` continua sendo `CReal`: `parse_number` consome o número real inteiro antes de o
sufixo ser tentado, e um sufixo exige identificador depois do ponto.

**`src/parser/parser_stmt.rs`**: `parse_field_assignment_statement`:

```text
parse_factor   "="   parse_expression
```

Depois de ler, exige que o alvo seja `FieldAccess(obj, campo)` e devolve
`FieldAssignment(obj, campo, expr)`. Se o alvo for outra coisa (`x`, `p.f()`), falha e
`alt` tenta a próxima alternativa. Por isso ela vem antes de
`parse_assignment_statement`: para `x = 1`, ela falha e a atribuição normal casa,
sem mudança de comportamento.

### Mudanças fora do parser (mínimas, obrigatórias)

Rust exige `match` exaustivo. Como `Type`, `Expression` e `Statement` ganharam
variantes, alguns `match` deixaram de compilar. Só foram adicionados os braços
necessários, sem semântica:

| Arquivo | Braço adicionado |
|---|---|
| `src/type_checker/statement_type_checker.rs` | `ClassDef` e `FieldAssignment` devolvem `Err("[Type Error] ... not supported yet")` |
| `src/type_checker/expression_type_checker.rs` | `FieldAccess` e `MethodCall` devolvem erro do mesmo tipo |
| `src/pretty_print/pretty_type.rs` | `TClass(n)` imprime `n` |
| `src/pretty_print/pretty_statements.rs` | `ClassDef` imprime `class Nome: ... end` (placeholder); `FieldAssignment` imprime `obj.campo = expr;` |
| `src/pretty_print/pretty_expressions.rs` | `FieldAccess` e `MethodCall` em forma linear |
| `src/ir/ast.rs` | `Display` de `TClass` |

O interpretador não precisou mudar: `execute` e `eval` já têm um braço `_` que devolve
`"not implemented yet"`.

---

## 7. Exemplo: entrada, caminho no parser, nó produzido

### `class Point`

```text
class Point:
    var x: Int = 0;
    def get_x(self: Point) -> Int:
        return self.x;
    end;
end
```

| Entrada consumida | Função | Resultado |
|---|---|---|
| (todas as alternativas anteriores falham sem consumir) | `parse_statement` | tenta `parse_class_definition_statement` |
| `class` | `keyword(CLASS_KEYWORD)` | ok |
| `Point` | `identifier` | `"Point"` |
| `:` | `char(COLON_CHAR)` | abre o corpo |
| `var x: Int = 0` | `parse_class_field` | `FieldDeclaration { x, TInteger, mutable: true, CInt(0) }` |
| `;` | separador do `separated_list0` | |
| `def get_x(` | `parse_class_method` chama `parse_function_definition_statement` | |
| `self: Point` | `parse_formal_argument`, `parse_type`, `parse_class_type` | `FormalArgument(self, TClass("Point"))` |
| `-> Int` | `parse_type` | `TInteger` |
| `: return self.x; end` | `parse_block`, `parse_return_statement`, `parse_factor` | `Block([Return(FieldAccess(Var(self), "x"))])` |
| | `parse_class_method` verifica `params[0] == self` | `Function get_x` |
| `; end` | `opt(';')`, `keyword(END_KEYWORD)` | |
| | `map` separa membros | `Statement::ClassDef(Class { .. })` |

AST real (saída de `{:#?}` resumida):

```text
ClassDef(Class {
    name: "Point",
    fields: [FieldDeclaration { name: "x", field_type: TInteger, mutable: true, initializer: CInt(0) }],
    methods: [Function {
        name: "get_x", kind: TInteger,
        params: [FormalArgument { argument_name: "self", argument_type: TClass("Point") }],
        body: Some(Block([Return(FieldAccess(Var("self"), "x"))])),
    }],
})
```

### Expressão com membros

```text
p.x + f(1).y * 2
```

```text
Add(
  FieldAccess(Var("p"), "x"),
  Mul(
    FieldAccess(FuncCall("f", [CInt(1)]), "y"),
    CInt(2)))
```

### Programa completo da proposta

`examples/classes/point_full.rpy` (exemplo da seção 3.2 da proposta) é consumido por
inteiro e gera 5 statements:

```text
ClassDef(Point { fields: [x, y], methods: [init, get_x] })
ValDeclaration("p", FuncCall("Point", []))
ExprStmt(MethodCall(Var("p"), "init", [CInt(3), CInt(4)]))
ValDeclaration("px", MethodCall(Var("p"), "get_x", []))
Assert(EQ(Var("px"), CInt(3)), CString("x deveria ser 3"))
```

Dentro de `init`, `self.x = x` vira `FieldAssignment(Var("self"), "x", Var("x"))`.

Executar o arquivo com `cargo run -- examples/classes/point_full.rpy` termina com
`Erro em tempo de execução: not implemented yet`. Isso é esperado: o parser aceita o
programa e o interpretador ainda não sabe executar classes.

---

## 8. Testes

Todos em `tests/parser_tests.rs`, usando a infraestrutura existente (`cargo test`).

| Teste | Verifica |
|---|---|
| `class_tests::test_empty_class` | `class Empty: end` |
| `class_tests::test_class_with_val_and_var_fields` | `mutable` correto para `val` e `var`, tipos e inicializadores |
| `class_tests::test_class_with_method` | método com `self: TClass("Counter")`, corpo e tipo de retorno |
| `class_tests::test_class_example_file` | `examples/classes/point_declaration.rpy` |
| `class_tests::test_invalid_class_declarations` | rejeita: sem nome, nome reservado, sem `:`, sem `end`, campo sem tipo, statement solto no corpo, método sem `self`, método sem parâmetros |
| `class_tests::test_class_keyword_is_reserved` | `class = 1` falha |
| `member_access_tests::test_field_access` | `p.x` |
| `member_access_tests::test_chained_access_is_left_associative` | `a.b.c()` |
| `member_access_tests::test_method_call_with_args` | `self.move(1, y)` |
| `member_access_tests::test_member_access_binds_tighter_than_operators` | `p.x + f(1).y * 2` |
| `member_access_tests::test_real_literal_is_not_member_access` | `2.5` continua `CReal` |
| `member_access_tests::test_field_assignment` | `self.a.b = 3` |
| `member_access_tests::test_plain_assignment_unchanged` | `x = 1` continua `Assignment` |
| `member_access_tests::test_method_call_statement` | `p.reset()` como statement |
| `member_access_tests::test_invalid_member_syntax` | `p.`, `p.1`, `p.if` não são consumidos; `p.f() = 1` não é `FieldAssignment` |
| `member_access_tests::test_member_access_example_file` | `examples/classes/member_access.rpy` |
| `test_full_class_program_parses` | `examples/classes/point_full.rpy` inteiro |

Arquivos de exemplo novos em `examples/classes/`:

- `point_declaration.rpy`: só a declaração (campos e métodos).
- `member_access.rpy`: leitura, escrita, chamada e encadeamento.
- `point_full.rpy`: exemplo completo da proposta.

Para rodar:

```bash
cargo test                       # todos os testes
cargo test --test parser_tests   # só os testes de parser
cargo fmt -- --check             # mesmo check do CI
```

---

## 9. Limitações atuais (conhecidas)

- `parse_type` aceita qualquer identificador como `TClass`. O parser não sabe se a
  classe existe; isso é trabalho do type checker. Consequência: um erro de digitação
  em tipo (`def f(a: Itn) -> Int`) agora é aceito pelo parser como `TClass("Itn")`.
- O parser só verifica o **nome** do primeiro parâmetro (`self`), não o tipo.
- Membros da classe precisam ser separados por `;`, como statements em blocos.
- Campos exigem tipo e inicializador.
- `Point()` é `FuncCall`, sem nó próprio.
- O pretty print de `ClassDef` é um placeholder (`class Nome: ... end`).
- Sem espaço entre objeto e ponto: `p . x` não é aceito (espaço depois do ponto
  também não).

## 10. Fora do escopo desta fase (FUTURE WORK)

- Type checking de classes, campos, métodos, `self` e atribuição a `val`.
- Nó `New` para instanciação e construtor com argumentos.
- Representação de objetos em runtime (`ClassInstance`) e execução de métodos.
- Herança, visibilidade, métodos estáticos, sobrescrita.
- Pretty print completo de classes.
- Registro de classes no `Environment`.

A exploração inicial apresentada na reunião tem um esboço de parte disso e pode servir
de referência para as próximas fases.

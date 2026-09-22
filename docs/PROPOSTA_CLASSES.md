# Proposta: Extensão de RPython para Suporte a Classes

## 1. Análise do Repositório RPython

### 1.1 Arquitetura Atual

O RPython é uma linguagem estaticamente tipada, interpretada, com sintaxe inspirada em Python. A arquitetura atual é:

```
src/
├── ir/ast.rs              # AST: Expression, Statement, Type, Function
├── parser/                # Parser combinador com nom
│   ├── parser_expr.rs     # Expressões
│   ├── parser_stmt.rs     # Statements
│   ├── parser_type.rs     # Tipos
│   └── parser_common.rs   # Identificadores, keywords, tokens
├── type_checker/          # Checagem estática de tipos
│   ├── expression_type_checker.rs
│   └── statement_type_checker.rs
├── interpreter/           # Interpretador tree-walking
│   ├── expression_eval.rs
│   └── statement_execute.rs
├── pretty_print/          # AST → código fonte formatado
├── environment/           # Tabelas de símbolos com escopo
└── stdlib/                # Metabuiltins (I/O, conversões, etc.)
```

### 1.2 Limitações Identificadas (relacionadas à proposta)

Do README, as limitações mais relevantes:

| # | Limitação | Impacto na proposta |
|---|-----------|---------------------|
| 10 | **No objects or methods** — não há classes, interfaces ou chamadas de método | **É exatamente o que vamos implementar** |
| 2 | **No pattern matching** — ADTs podem ser construídos mas não destruídos | Inspiração para construção de objetos |
| 1 | **No module system** — todo código em um único arquivo | Classes serão top-level no arquivo |
| 9 | **No exceptions** — erros via `Maybe`/`Result` | Podemos reaproveitar para métodos que falham |

Outras limitações técnicas observadas no código:
- O `Environment` já suporta escopo com `push()`/`pop()`, variáveis, funções, ADTs e testes
- Não há representação de objetos/instâncias no interpretador
- Não há acesso a atributos (`.` operator) no parser nem na AST
- O tipo `Type` não tem variantes para tipos de classe

---

## 2. Comparação: AST RPython vs AST Python

### 2.1 AST RPython (atual)

**Fonte:** [`src/ir/ast.rs`](https://github.com/UnBCIC-TP2/r-python/blob/main/src/ir/ast.rs)

```rust
pub enum Expression {
    CTrue, CFalse, CInt(i32), CReal(f64), CString(String), CVoid,
    Var(Name),
    FuncCall(Name, Vec<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    EQ(Box<Expression>, Box<Expression>),
    NEQ(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    GTE(Box<Expression>, Box<Expression>),
    LTE(Box<Expression>, Box<Expression>),
    COk(Box<Expression>), CErr(Box<Expression>),
    CJust(Box<Expression>), CNothing,
    Unwrap(Box<Expression>),
    IsError(Box<Expression>), IsNothing(Box<Expression>),
    Propagate(Box<Expression>),
    Lambda(Function),
    ListValue(Vec<Expression>),
    Tuple(Vec<Expression>),
    Constructor(Name, Vec<Box<Expression>>),
}

pub enum Statement {
    VarDeclaration(Name, Box<Expression>),
    ValDeclaration(Name, Box<Expression>),
    Assignment(Name, Box<Expression>),
    IfThenElse(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    IfChain { branches: Vec<(Box<Expression>, Box<Statement>)>, else_branch: Option<Box<Statement>> },
    While(Box<Expression>, Box<Statement>),
    For(Name, Box<Expression>, Box<Statement>),
    Break, Continue,
    Block(Vec<Statement>),
    Sequence(Box<Statement>, Box<Statement>),
    Assert(Box<Expression>, Box<Expression>),
    AssertTrue(Box<Expression>, Box<Expression>),
    AssertFalse(Box<Expression>, Box<Expression>),
    AssertEQ(Box<Expression>, Box<Expression>, Box<Expression>),
    AssertNEQ(Box<Expression>, Box<Expression>, Box<Expression>),
    TestDef(Function),
    ModTestDef(Name, Box<Statement>),
    AssertFails(String),
    FuncDef(Function),
    Return(Box<Expression>),
    TypeDeclaration(Name, Vec<ValueConstructor>),
    ExprStmt(Box<Expression>),
    MetaStmt(String),
}

pub enum Type {
    TInteger, TBool, TReal, TString, TVoid, TAny,
    TFunction(Box<Type>, Vec<Type>),
    TList(Box<Type>),
    TTuple(Vec<Type>),
    TMaybe(Box<Type>),
    TResult(Box<Type>, Box<Type>),
    TAlgebraicData(Name, Vec<ValueConstructor>),
}
```

### 2.2 AST Python (referência)

**Fonte:** [`ast` — Python 3.12](https://docs.python.org/pt-br/3.12/library/ast.html)

A gramática abstrata do Python define classes via `ClassDef`:

```asdl
stmt = ...
     | ClassDef(identifier name,
                  expr* bases,
                  keyword* keywords,
                  stmt* body,
                  expr* decorator_list,
                  type_param* type_params)
     | FunctionDef(identifier name, arguments args,
                   stmt* body, expr* decorator_list, expr? returns,
                   string? type_comment, type_param* type_params)
     ...

expr = ...
     | Attribute(expr value, identifier attr, expr_context ctx)
     | Call(expr func, expr* args, keyword* keywords)
     | Name(identifier id, expr_context ctx)
     ...
```

### 2.3 Análise Comparativa

| Aspecto | Python AST | RPython AST (atual) | O que precisa ser adicionado |
|---------|-----------|---------------------|------------------------------|
| **Classe** | `ClassDef(name, bases, body, ...)` | ❌ Não existe | `ClassDef` statement + struct `Class` |
| **Atributo** | `Attribute(value, attr, ctx)` | ❌ Não existe | `FieldAccess` expression |
| **Método** | `FunctionDef` dentro de `ClassDef.body` | `FuncDef` (apenas global) | Permitir `def` dentro de `class` |
| **Construtor** | `__init__` método especial | ❌ Não existe | Método `init` especial ou construtor implícito |
| **Instanciação** | `Call(func=Name('Classe'), args=...)` | `FuncCall` | `New(name, args)` expression |
| **Tipo de classe** | Implícito (tipagem dinâmica) | ❌ Não existe | `TClass(Name)` no enum `Type` |
| **`self`** | Primeiro parâmetro explícito | ❌ Não aplica | `self` como primeiro parâmetro obrigatório |

**Observação importante:** RPython é estaticamente tipado, então a representação de classes precisa ser mais explícita que Python. Cada classe define um novo tipo.

---

## 3. Proposta: Extensão de RPython para Suporte a Classes

### 3.1 Escopo da Primeira Entrega

A proposta é dividida em duas fases. A **primeira entrega** foca na estrutura sintática e semântica básica, **sem herança**:

**Na primeira entrega:**
1. ✅ Declaração de classes com campos (`val`/`var`) e métodos (`def`)
2. ✅ `self` explícito como primeiro parâmetro de métodos
3. ✅ Acesso a campos: `self.campo` e `obj.campo`
4. ✅ Chamada de métodos: `self.metodo(args)` e `obj.metodo(args)`
5. ✅ Construtor implícito: `NomeDaClasse()` cria instância com valores default
6. ✅ Tipagem estática: `TClass(Name)` como novo tipo
7. ✅ Checagem de tipos para acesso a campos e chamada de métodos
8. ✅ Interpretação: criação de instâncias e resolução de `self`
9. ✅ Pretty print e testes

**Para entregas futuras:**
- Herança simples (`class B : A : ... end`)
- Construtor com parâmetros (`NomeDaClasse(arg1, arg2)`)
- Encapsulamento (`private`/`public`)
- Métodos estáticos
- Sobrescrita de métodos (`override`)

### 3.2 Sintaxe Proposta

```text
# Declaração de classe
class Point:
    val x: Int = 0;
    val y: Int = 0;
    
    def init(self: Point, x: Int, y: Int) -> Unit:
        self.x = x;
        self.y = y;
    end;
    
    def distance(self: Point, other: Point) -> Real:
        # ... cálculo da distância
        return 0.0;
    end;
end;

# Uso
val p1 = Point();
p1.init(3, 4);
val px = p1.x;           # acesso a campo
val d = p1.distance(p2); # chamada de método
```

**Regras sintáticas:**
1. `class` é palavra-chave nova
2. Corpo da classe segue após `:` e termina com `end`
3. Dentro da classe podem haver declarações `val`, `var` e `def`
4. Todo método deve ter `self` como **primeiro** parâmetro
5. O tipo de `self` é o nome da classe em que está definido
6. Campos são acessíveis via `self.campo` dentro da classe e `obj.campo` fora
7. Instanciação: `NomeDaClasse()` (sem argumentos na primeira entrega)

### 3.3 Alterações na AST

#### `src/ir/ast.rs`

```rust
// Representa uma classe
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub name: Name,
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<Function>,
}

// Representa um campo de classe
#[derive(Clone, Debug, PartialEq)]
pub struct FieldDeclaration {
    pub name: Name,
    pub field_type: Type,
    pub mutable: bool,
    pub initializer: Option<Box<Expression>>,
}

// Novas variantes em Expression
pub enum Expression {
    // ... variantes existentes ...
    
    // Acesso a campo: obj.field
    FieldAccess(Box<Expression>, Name),
    
    // Chamada de método: obj.method(args)
    MethodCall(Box<Expression>, Name, Vec<Expression>),
    
    // Criação de instância: ClassName()
    New(Name, Vec<Expression>),
}

// Novas variantes em Statement
pub enum Statement {
    // ... variantes existentes ...
    
    // Definição de classe
    ClassDef(Class),
    
    // Atribuição a campo: obj.field = expr
    FieldAssignment(Box<Expression>, Name, Box<Expression>),
}

// Novas variantes em Type
pub enum Type {
    // ... variantes existentes ...
    
    // Tipo de classe: TClass("Point")
    TClass(Name),
}
```

### 3.4 Alterações no Parser

#### `src/parser/parser_common.rs`
- Adicionar `CLASS_KEYWORD = "class"` em keywords
- Adicionar `"class"` ao vetor `KEYWORDS`

#### `src/parser/parser_stmt.rs`
- Novo parser `parse_class_definition_statement`:
  ```
  class <ident> : <campos_e_metodos> end
  ```
- Reaproveita `parse_statement` para o corpo, mas filtra apenas `val`/`var`/`def`
- Novo parser `parse_field_assignment_statement` para `obj.campo = expr`

#### `src/parser/parser_expr.rs`
- Modificar `parse_factor` para suportar acesso a campo:
  ```
  <primary> . <ident>   => FieldAccess
  <primary> . <ident>(args) => MethodCall
  <ident>()             => New (se ident for nome de classe)
  ```

### 3.5 Alterações no Type Checker

#### `src/type_checker/expression_type_checker.rs`
- `check_expr(FieldAccess(obj, field))`:
  1. Avalia tipo de `obj` → deve ser `TClass(name)`
  2. Busca classe `name` no environment
  3. Verifica se `field` existe na classe
  4. Retorna o tipo do campo
- `check_expr(MethodCall(obj, method, args))`:
  1. Avalia tipo de `obj` → deve ser `TClass(name)`
  2. Busca classe `name` no environment
  3. Verifica se `method` existe
  4. Verifica tipos dos argumentos (incluindo `self` implícito)
  5. Retorna tipo de retorno do método
- `check_expr(New(class_name, args))`:
  1. Verifica se `class_name` é uma classe declarada
  2. Verifica se argumentos batem com construtor (se houver)
  3. Retorna `TClass(class_name)`

#### `src/type_checker/statement_type_checker.rs`
- `check_stmt(ClassDef(class))`:
  1. Registra a classe no environment
  2. Checa cada campo (tipos válidos, inicializadores compatíveis)
  3. Checa cada método como `FuncDef`, mas com `self` no escopo
  4. Valida que `self` é do tipo correto
- `check_stmt(FieldAssignment(obj, field, expr))`:
  1. Verifica se `obj` é `TClass`
  2. Verifica se campo existe e é mutável (`var`)
  3. Verifica compatibilidade de tipos

### 3.6 Alterações no Interpretador

#### `src/interpreter/expression_eval.rs`
- `eval(FieldAccess(obj, field))`:
  1. Avalia `obj` → deve resultar em uma instância
  2. Representação de instância: novo tipo de `Expression` ou estrutura no environment
  3. Retorna valor do campo
- `eval(MethodCall(obj, method, args))`:
  1. Avalia `obj` → obtém instância
  2. Busca método na definição da classe
  3. Cria novo environment com `self` bound à instância
  4. Executa corpo do método
- `eval(New(class_name, args))`:
  1. Busca definição da classe
  2. Cria instância com campos inicializados (valores default ou do construtor)
  3. Retorna a instância

**Representação de instância:**
```rust
// Nova variante em Expression (ou nova estrutura separada)
pub enum Expression {
    // ... existentes ...
    
    // Representa uma instância de classe em tempo de execução
    ClassInstance(Name, Vec<(Name, Expression)>), // nome da classe, (campo, valor)
}
```

#### `src/interpreter/statement_execute.rs`
- `execute(ClassDef(class))`:
  1. Registra definição da classe no environment
  2. Semelhante a `FuncDef`
- `execute(FieldAssignment(obj_expr, field, expr))`:
  1. Avalia `obj_expr` → instância
  2. Avalia `expr` → valor
  3. Atualiza campo na instância

### 3.7 Alterações no Pretty Print

#### `src/pretty_print/pretty_statements.rs`
- Implementar `ToDoc` para `Statement::ClassDef`
- Implementar `ToDoc` para `Statement::FieldAssignment`
- Implementar `ToDoc` para `FieldDeclaration`

#### `src/pretty_print/pretty_expressions.rs`
- Implementar `ToDoc` para `Expression::FieldAccess`
- Implementar `ToDoc` para `Expression::MethodCall`
- Implementar `ToDoc` para `Expression::New`
- Implementar `ToDoc` para `Expression::ClassInstance`

### 3.8 Alterações no Environment

#### `src/environment/environment.rs`
- Adicionar campo `classes: HashMap<Name, Class>` nas scopes
- Adicionar métodos:
  - `map_class(name, class)`
  - `lookup_class(name) -> Option<Class>`
  - `get_all_classes() -> HashMap<Name, Class>`

---

## 4. Plano de Implementação (Primeira Entrega)

### Fase 1: AST e Parser (Semana 1)
- [ ] Adicionar structs `Class`, `FieldDeclaration` em `ast.rs`
- [ ] Adicionar variantes `ClassDef`, `FieldAssignment` em `Statement`
- [ ] Adicionar variantes `FieldAccess`, `MethodCall`, `New` em `Expression`
- [ ] Adicionar `TClass` em `Type`
- [ ] Adicionar keyword `class` e `CLASS_KEYWORD`
- [ ] Implementar `parse_class_definition_statement`
- [ ] Implementar `parse_field_access` e `parse_method_call` em `parser_expr.rs`
- [ ] Implementar `parse_field_assignment` em `parser_stmt.rs`
- [ ] Adicionar testes de parser para classes

### Fase 2: Type Checker (Semana 2)
- [ ] Implementar `check_class_def_stmt` em `statement_type_checker.rs`
- [ ] Implementar `check_field_access` em `expression_type_checker.rs`
- [ ] Implementar `check_method_call` em `expression_type_checker.rs`
- [ ] Implementar `check_new` em `expression_type_checker.rs`
- [ ] Implementar `check_field_assignment_stmt` em `statement_type_checker.rs`
- [ ] Adicionar testes de type checker para classes

### Fase 3: Interpreter (Semana 2-3)
- [ ] Adicionar `Expression::ClassInstance` para representação em runtime
- [ ] Implementar `eval_field_access` em `expression_eval.rs`
- [ ] Implementar `eval_method_call` em `expression_eval.rs`
- [ ] Implementar `eval_new` em `expression_eval.rs`
- [ ] Implementar `execute(ClassDef)` e `execute(FieldAssignment)` em `statement_execute.rs`
- [ ] Adicionar `classes` ao `Environment`
- [ ] Adicionar testes de interpretação para classes

### Fase 4: Pretty Print e Testes de Integração (Semana 3)
- [ ] Implementar `ToDoc` para novos nós de AST
- [ ] Adicionar testes de pretty print
- [ ] Criar programas de exemplo com classes
- [ ] Executar `cargo test` e corrigir regressões

### Fase 5: Documentação e Revisão (Semana 4)
- [ ] Atualizar README com sintaxe de classes
- [ ] Criar exemplos em `examples/`
- [ ] Revisar cobertura de testes
- [ ] Preparar apresentação

---

## 5. Exemplo Completo (objetivo da primeira entrega)

```text
class Point:
    val x: Int = 0;
    val y: Int = 0;
    
    def init(self: Point, x: Int, y: Int) -> Unit:
        self.x = x;
        self.y = y;
    end;
    
    def get_x(self: Point) -> Int:
        return self.x;
    end;
end;

val p = Point();
p.init(3, 4);
val px = p.get_x();
assert(px == 3, "x should be 3");
```

---

## 6. Riscos e Mitigações

| Risco | Mitigação |
|-------|-----------|
| Parser de `obj.field` conflitar com sintaxe existente | Usar `.` como operador de baixa precedência, após `parse_factor` |
| `self` precisa ser tratado especialmente em todos os métodos | Validar no type checker que primeiro parâmetro é `self` |
| Representação de instâncias no interpretador é complexa | Começar com `Expression::ClassInstance` simples, similar a `Tuple` |
| Interação com lambdas e funções de primeira classe | Por ora, classes não são funções; `ClassName()` é `New`, não `FuncCall` |
| Escopo de campos vs variáveis locais | Usar o `Environment` com push/pop em métodos, `self` como variável especial |

---

## 7. Referências

- RPython AST: https://github.com/UnBCIC-TP2/r-python/blob/main/src/ir/ast.rs
- Python AST docs: https://docs.python.org/pt-br/3.12/library/ast.html
- RPython README (limitações): https://github.com/UnBCIC-TP2/r-python/blob/main/README.md

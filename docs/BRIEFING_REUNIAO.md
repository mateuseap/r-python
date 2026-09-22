# Briefing: Extensão de Classes no RPython
## Reunião — TP2 / UnB

---

## 1. Contexto: O que é RPython?

- **RPython** = linguagem estaticamente tipada com sintaxe parecida com Python
- Feita em Rust, para ensino de implementação de linguagens (parser, type checker, interpreter)
- Atualmente tem: variáveis, funções, loops, listas, tuplas, ADTs, lambdas, testes inline
- **NÃO tem:** classes, objetos, métodos, herança, acesso a atributos (`obj.campo`)

---

## 2. Objetivo do Trabalho

**Adicionar suporte a classes no RPython.**

Isso significa:
- Declarar classes com campos (`val`/`var`) e métodos (`def`)
- Instanciar objetos (`NomeDaClasse()`)
- Acessar campos (`obj.x`, `self.x`)
- Chamar métodos (`obj.metodo(args)`, `self.metodo(args)`)
- `self` explícito como primeiro parâmetro de todo método

**Primeira entrega:** apenas estrutura básica (sem herança, sem encapsulamento, sem métodos estáticos).

---

## 3. Como o RPython funciona hoje (em 30 segundos)

```text
def soma(a: Int, b: Int) -> Int:
    return a + b;
end;

val r = soma(2, 3);
```

1. Parser (`nom`) → AST
2. Type Checker → valida tipos
3. Interpreter (tree-walking) → executa a AST

Todo o código está em `src/` com ~2700 linhas Rust.

---

## 4. O que precisa mudar

| Componente | O que adicionar |
|-----------|-----------------|
| **AST** (`ir/ast.rs`) | `Class`, `FieldAccess`, `MethodCall`, `New`, `TClass` |
| **Parser** (`parser/`) | keyword `class`, `.` para campo/método, `()` para instanciar |
| **Type Checker** (`type_checker/`) | Validar `self`, existência de campos/métodos, tipos |
| **Interpreter** (`interpreter/`) | Criar instâncias, resolver `self`, acessar campos |
| **Pretty Print** (`pretty_print/`) | Imprimir código com classes |
| **Testes** (`tests/`) | Parser, type checker, interpreter, integração |

---

## 5. Sintaxe proposta (primeira entrega)

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
assert(px == 3, "x deveria ser 3");
```

Regras:
- `class Nome : ... end`
- Dentro da classe: só `val`, `var`, `def`
- Todo método começa com `self: NomeDaClasse`
- `self.campo` dentro da classe, `obj.campo` fora
- Instanciação: `NomeDaClasse()` (sem args na primeira entrega)

---

## 6. Divisão de trabalho sugerida

| Pessoa | Foco | Arquivos principais |
|--------|------|---------------------|
| **Mateus** | AST + Parser + Integração | `ir/ast.rs`, `parser/*.rs` |
| **Pessoa 2** | Type Checker | `type_checker/*.rs` |
| **Pessoa 3** | Interpreter | `interpreter/*.rs`, `environment/` |
| **Pessoa 4** | Pretty Print + Testes | `pretty_print/*.rs`, `tests/*.rs` |

**Dependências:** AST → Parser → (Type Checker || Interpreter) → Pretty Print → Testes

Sugestão: começar pela AST + Parser juntos, depois Type Checker e Interpreter em paralelo.

---

## 7. Repositório e workflow

- **Fork:** https://github.com/mateuseap/r-python
- **Upstream:** https://github.com/UnBCIC-TP2/r-python (repo original)
- Cada um faz fork do fork, ou trabalha em branches no fork principal
- PRs revisados antes de merge na `main`
- Testes: `cargo test` (270+ testes existentes, não podem quebrar)

---

## 8. Primeiros passos (próxima semana)

1. Todo mundo clona o fork e roda `cargo test`
2. Definir quem fica com cada componente
3. Criar branch `feature/classes` no fork
4. Começar pela AST (Mateus puxa isso)
5. Segunda reunião: review da AST + início do parser

---

## 9. Dúvidas comuns

**Q: Herança vai ser implementada?**
A: Sim, mas depois da primeira entrega. Começamos sem herança.

**Q: E encapsulamento (public/private)?**
A: Também fica para depois. Tudo é público na primeira entrega.

**Q: O RPython é em Rust, preciso saber Rust?**
A: Sim, mas o código é didático. O padrão é claro: enum + match. Não usa traits complexos.

**Q: Como testamos?**
A: Testes unitários inline em cada módulo (`#[cfg(test)]`) + testes de integração em `tests/`.

---

## 10. Material de apoio

- **Proposta completa:** `PROPOSTA_CLASSES.md` no fork
- **RPython README:** https://github.com/UnBCIC-TP2/r-python/blob/main/README.md
- **Python AST ref:** https://docs.python.org/pt-br/3.12/library/ast.html
- **Código AST RPython:** `src/ir/ast.rs`

---

## Perguntas pra reunião

1. Quem fica com Type Checker?
2. Quem fica com Interpreter?
3. Quem fica com Pretty Print + Testes?
4. Qual dia/horário da próxima reunião?
5. Todo mundo consegue rodar `cargo test`?

# Programas com erro de tipo

Cada arquivo é aceito pelo parser e tem um único erro, que o type checker rejeita antes da execução.

| Arquivo | Erro esperado |
|---|---|
| `wrong_self_type.rpy` | `self` de `area` tem tipo `Int`, não `Square` |
| `unknown_type.rpy` | `Pont` não é uma classe declarada (erro de digitação de `Point`) |
| `missing_field.rpy` | `Point` não declara o campo `z` |
| `val_field_assignment.rpy` | `id` foi declarado com `val` e não pode ser alterado |
| `wrong_argument.rpy` | `add` espera `Int`, mas recebe `String` |

Para ver a mensagem de cada um:

```bash
cargo run --example typecheck -- examples/classes/type_errors/wrong_self_type.rpy
```

Os mesmos casos são verificados em `tests/type_checker_classes_tests.rs`.

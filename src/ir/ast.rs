use std::fmt;

// Type alias for variable and function names
pub type Name = String;

// Represents a function in the AST
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Name,
    pub kind: Type,
    pub params: Vec<FormalArgument>,
    pub body: Option<Box<Statement>>,
}

impl Function {
    // Creates a new function with default values
    pub fn new() -> Function {
        return Function {
            name: "__main__".to_string(),
            kind: Type::TVoid,
            params: Vec::new(),
            body: None,
        };
    }
}

// Represents a formal argument in a function definition
#[derive(Debug, PartialEq, Clone)]
pub struct FormalArgument {
    pub argument_name: Name,
    pub argument_type: Type,
}

impl FormalArgument {
    // Creates a new formal argument
    pub fn new(argument_name: Name, argument_type: Type) -> Self {
        FormalArgument {
            argument_name,
            argument_type,
        }
    }
}

//Represents function signature
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct FuncSignature {
    pub name: Name,
    pub argument_types: Vec<Type>,
}

impl FuncSignature {
    pub fn new() -> FuncSignature {
        FuncSignature {
            name: "".to_string(),
            argument_types: vec![],
        }
    }
    pub fn from_func(func: &Function) -> FuncSignature {
        FuncSignature {
            name: func.name.clone(),
            argument_types: func
                .params
                .iter()
                .map(|arg| arg.argument_type.clone())
                .collect(),
        }
    }
}

impl fmt::Display for FuncSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.argument_types
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

// Represents the types that can be used in the AST
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Type {
    TInteger,
    TBool,
    TReal,
    TString,
    TVoid,
    TFunction(Box<Type>, Vec<Type>),
    TList(Box<Type>),
    TTuple(Vec<Type>),
    TMaybe(Box<Type>),
    TResult(Box<Type>, Box<Type>), // Ok, Error
    TAny,
    TAlgebraicData(Name, Vec<ValueConstructor>),
    // Named class type, e.g. `self: Point` (parsed only; no semantics yet)
    TClass(Name),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::TInteger => write!(f, "Int"),
            Type::TBool => write!(f, "Boolean"),
            Type::TReal => write!(f, "Real"),
            Type::TString => write!(f, "String"),
            Type::TVoid => write!(f, "Unit"),
            Type::TAny => write!(f, "Any"),

            Type::TList(inner) => write!(f, "List[{}]", inner),

            Type::TTuple(elements) => {
                let types = elements
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Tuple[{}]", types)
            }

            Type::TMaybe(inner) => write!(f, "Maybe[{}]", inner),

            Type::TResult(ok, err) => write!(f, "Result[{}, {}]", ok, err),

            Type::TFunction(ret, params) => {
                let params_str = params
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "fn({}) -> {}", params_str, ret)
            }

            Type::TAlgebraicData(name, _constructors) => {
                write!(f, "{}", name)
            }

            Type::TClass(name) => write!(f, "{}", name),
        }
    }
}

// Represents a value constructor for an algebraic data type
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ValueConstructor {
    pub name: Name,
    pub types: Vec<Type>,
}

impl ValueConstructor {
    // Creates a new value constructor
    pub fn new(name: Name, types: Vec<Type>) -> Self {
        ValueConstructor { name, types }
    }
}

// A field declared in a class body: `val x: Int = 0` or `var x: Int = 0`
#[derive(Clone, Debug, PartialEq)]
pub struct FieldDeclaration {
    pub name: Name,
    pub field_type: Type,
    pub mutable: bool,
    pub initializer: Box<Expression>,
}

// A class declaration: `class Name: <fields and methods> end`
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub name: Name,
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<Function>,
}

// Represents expressions in the AST
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    // Constants
    CTrue,
    CFalse,
    CInt(i32),
    CReal(f64),
    CString(String),
    CVoid,

    // Variable reference
    Var(Name),

    // Function call
    FuncCall(Name, Vec<Expression>),

    // Arithmetic expressions over numbers
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),

    // Boolean expressions over booleans
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),

    // Relational expressions over numbers
    EQ(Box<Expression>, Box<Expression>),
    NEQ(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    GTE(Box<Expression>, Box<Expression>),
    LTE(Box<Expression>, Box<Expression>),

    // Error-related expressions
    COk(Box<Expression>),
    CErr(Box<Expression>),
    CJust(Box<Expression>),
    CNothing,
    Unwrap(Box<Expression>),
    IsError(Box<Expression>),
    IsNothing(Box<Expression>),
    Propagate(Box<Expression>),

    //Lambda expression
    Lambda(Function),

    // List value
    ListValue(Vec<Expression>),

    // Tuple value
    Tuple(Vec<Expression>),

    // Constructor
    Constructor(Name, Vec<Box<Expression>>),
}

// Represents statements in the AST
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VarDeclaration(Name, Box<Expression>),
    ValDeclaration(Name, Box<Expression>),
    Assignment(Name, Box<Expression>),
    IfThenElse(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    IfChain {
        branches: Vec<(Box<Expression>, Box<Statement>)>,
        else_branch: Option<Box<Statement>>,
    },
    While(Box<Expression>, Box<Statement>),
    For(Name, Box<Expression>, Box<Statement>),
    Break,
    Continue,
    Block(Vec<Statement>),
    Sequence(Box<Statement>, Box<Statement>),
    Assert(Box<Expression>, Box<Expression>), //Segundo expression deve ser String
    AssertTrue(Box<Expression>, Box<Expression>), //Segundo expression deve ser String
    AssertFalse(Box<Expression>, Box<Expression>), //Segundo expression deve ser String
    AssertEQ(Box<Expression>, Box<Expression>, Box<Expression>), //Terceiro expression deve ser String
    AssertNEQ(Box<Expression>, Box<Expression>, Box<Expression>), //Terceiro expression deve ser String
    TestDef(Function),
    ModTestDef(Name, Box<Statement>),
    AssertFails(String),
    FuncDef(Function),
    Return(Box<Expression>),
    TypeDeclaration(Name, Vec<ValueConstructor>),
    /// Avalia uma expressão apenas por seus efeitos colaterais (ex: chamada de função).
    ExprStmt(Box<Expression>),
    MetaStmt(String),
    // Class declaration (parsed only; no type checking or execution yet)
    ClassDef(Class),
}

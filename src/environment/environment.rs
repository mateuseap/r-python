use crate::ir::ast::{Class, FuncSignature, Function, Name, ValueConstructor};
use indexmap::IndexMap;
use std::collections::{HashMap, LinkedList};
use std::fmt::Debug;

#[derive(Clone)]
pub struct Scope<A> {
    pub variables: HashMap<Name, (bool, A)>,
    pub functions: HashMap<FuncSignature, Function>,
    pub adts: HashMap<Name, Vec<ValueConstructor>>,
    pub tests: IndexMap<Name, Function>,
    pub classes: HashMap<Name, Class>,
}

impl<A: Clone> Scope<A> {
    fn new() -> Scope<A> {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
            adts: HashMap::new(),
            tests: IndexMap::new(),
            classes: HashMap::new(),
        }
    }

    fn map_variable(&mut self, var: Name, mutable: bool, value: A) {
        self.variables.insert(var, (mutable, value));
    }

    fn map_function(&mut self, function: Function) {
        let func_signature = FuncSignature::from_func(&function);
        self.functions.insert(func_signature, function);
    }

    fn map_test(&mut self, test: Function) {
        self.tests.insert(test.name.clone(), test);
    }

    fn map_adt(&mut self, name: Name, adt: Vec<ValueConstructor>) {
        self.adts.insert(name.clone(), adt);
    }

    fn lookup_var(&self, var: &Name) -> Option<(bool, A)> {
        self.variables
            .get(var)
            .map(|(mutable, value)| (*mutable, value.clone()))
    }

    fn update_var(&mut self, var: &Name, value: A) -> bool {
        if let Some((mutable, slot)) = self.variables.get_mut(var) {
            *slot = value;
            let _ = mutable;
            true
        } else {
            false
        }
    }

    fn remove_var(&mut self, var: &Name) -> bool {
        self.variables.remove(var).is_some()
    }

    fn lookup_function(&self, func_signature: &FuncSignature) -> Option<&Function> {
        self.functions.get(func_signature)
    }

    fn lookup_function_by_name(&self, name: &Name) -> Option<&Function> {
        self.functions.iter().find_map(|(signature, function)| {
            if &signature.name == name {
                Some(function)
            } else {
                None
            }
        })
    }

    fn lookup_test(&self, name: &Name) -> Option<&Function> {
        self.tests.get(name)
    }

    fn lookup_adt(&self, name: &Name) -> Option<&Vec<ValueConstructor>> {
        self.adts.get(name)
    }

    fn map_class(&mut self, class: Class) {
        self.classes.insert(class.name.clone(), class);
    }

    fn lookup_class(&self, name: &Name) -> Option<&Class> {
        self.classes.get(name)
    }
}

#[derive(Clone)]
pub struct Environment<A: Clone + Debug> {
    pub current_func: FuncSignature,
    pub globals: Scope<A>,
    pub stack: LinkedList<Scope<A>>,
}

impl<A: Clone + Debug> Environment<A> {
    pub fn new() -> Environment<A> {
        Environment {
            current_func: FuncSignature::new(),
            globals: Scope::new(),
            stack: LinkedList::new(),
        }
    }

    pub fn get_current_func(&self) -> FuncSignature {
        self.current_func.clone()
    }

    pub fn get_current_scope(&self) -> &Scope<A> {
        self.stack.front().unwrap_or(&self.globals)
    }

    pub fn set_current_func(&mut self, func_signature: &FuncSignature) {
        self.current_func = func_signature.clone();
    }

    pub fn set_global_functions(&mut self, global_functions: HashMap<FuncSignature, Function>) {
        self.globals.functions = global_functions;
    }

    pub fn set_global_classes(&mut self, global_classes: HashMap<Name, Class>) {
        self.globals.classes = global_classes;
    }

    pub fn map_variable(&mut self, var: Name, mutable: bool, value: A) {
        match self.stack.front_mut() {
            None => self.globals.map_variable(var, mutable, value),
            Some(top) => top.map_variable(var, mutable, value),
        }
    }

    pub fn map_function(&mut self, function: Function) {
        match self.stack.front_mut() {
            None => self.globals.map_function(function),
            Some(top) => top.map_function(function),
        }
    }

    pub fn map_test(&mut self, test: Function) {
        match self.stack.front_mut() {
            None => self.globals.map_test(test),
            Some(top) => top.map_test(test),
        }
    }

    pub fn map_adt(&mut self, name: Name, cons: Vec<ValueConstructor>) {
        match self.stack.front_mut() {
            None => self.globals.map_adt(name, cons),
            Some(top) => top.map_adt(name, cons),
        }
    }

    pub fn lookup(&self, var: &Name) -> Option<(bool, A)> {
        for scope in self.stack.iter() {
            if let Some(value) = scope.lookup_var(var) {
                return Some(value);
            }
        }
        self.globals.lookup_var(var)
    }

    /// Update an existing variable in the nearest scope where it's defined.
    /// Returns true if the variable existed and was updated; false otherwise.
    pub fn update_existing_variable(&mut self, var: &Name, value: A) -> bool {
        for scope in self.stack.iter_mut() {
            if scope.update_var(var, value.clone()) {
                return true;
            }
        }
        self.globals.update_var(var, value)
    }

    /// Remove a variable from the nearest scope where it's defined.
    /// Returns true if something was removed; false otherwise.
    pub fn remove_variable(&mut self, var: &Name) -> bool {
        for scope in self.stack.iter_mut() {
            if scope.remove_var(var) {
                return true;
            }
        }
        self.globals.remove_var(var)
    }

    pub fn lookup_function(&self, func_signature: &FuncSignature) -> Option<&Function> {
        for scope in self.stack.iter() {
            if let Some(func) = scope.lookup_function(func_signature) {
                return Some(func);
            }
        }
        self.globals.lookup_function(func_signature)
    }

    pub fn lookup_var_or_func(&self, name: &Name) -> Option<FuncOrVar<A>> {
        for scope in self.stack.iter() {
            if let Some(value) = scope.lookup_var(name) {
                return Some(FuncOrVar::Var(value));
            }
            if let Some(func) = scope.lookup_function_by_name(name) {
                return Some(FuncOrVar::Func(func.clone()));
            }
        }
        if let Some(value) = self.globals.lookup_var(name) {
            return Some(FuncOrVar::Var(value));
        }
        if let Some(func) = self.globals.lookup_function_by_name(name) {
            return Some(FuncOrVar::Func(func.clone()));
        }
        None
    }

    pub fn lookup_test(&self, name: &Name) -> Option<&Function> {
        for scope in self.stack.iter() {
            if let Some(test) = scope.lookup_test(name) {
                return Some(test);
            }
        }
        self.globals.lookup_test(name)
    }

    pub fn get_all_tests(&self) -> Vec<Function> {
        let mut tests = Vec::new();
        for scope in self.stack.iter() {
            for test in scope.tests.values() {
                tests.push(test.clone());
            }
        }
        for test in self.globals.tests.values() {
            tests.push(test.clone());
        }
        tests
    }

    pub fn lookup_adt(&self, name: &Name) -> Option<&Vec<ValueConstructor>> {
        for scope in self.stack.iter() {
            if let Some(cons) = scope.lookup_adt(name) {
                return Some(cons);
            }
        }
        self.globals.lookup_adt(name)
    }

    pub fn map_class(&mut self, class: Class) {
        match self.stack.front_mut() {
            None => self.globals.map_class(class),
            Some(top) => top.map_class(class),
        }
    }

    pub fn lookup_class(&self, name: &Name) -> Option<&Class> {
        for scope in self.stack.iter() {
            if let Some(class) = scope.lookup_class(name) {
                return Some(class);
            }
        }
        self.globals.lookup_class(name)
    }

    pub fn get_all_classes(&self) -> HashMap<Name, Class> {
        let mut all_classes = self.globals.classes.clone();
        for scope in self.stack.iter().rev() {
            all_classes.extend(scope.classes.clone());
        }
        all_classes
    }

    pub fn scoped_function(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn push(&mut self) {
        self.stack.push_front(Scope::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop_front();
    }

    pub fn get_all_variables(&self) -> Vec<(Name, (bool, A))> {
        let mut vars = Vec::new();

        for scope in self.stack.iter() {
            for (name, value) in &scope.variables {
                if !vars.iter().any(|(n, _)| n == name) {
                    vars.push((name.clone(), value.clone()));
                }
            }
        }

        for (name, value) in &self.globals.variables {
            if !vars.iter().any(|(n, _)| n == name) {
                vars.push((name.clone(), value.clone()));
            }
        }
        vars
    }

    /// The type checker ensures that each function is defined only once.
    pub fn get_all_functions(&self) -> HashMap<FuncSignature, Function> {
        let mut all_functions = HashMap::new();
        for (func_signature, func) in &self.globals.functions {
            all_functions.insert(func_signature.clone(), func.clone());
        }
        for scope in self.stack.iter().rev() {
            for (func_signature, func) in &scope.functions {
                all_functions.insert(func_signature.clone(), func.clone());
            }
        }
        all_functions
    }
}

pub enum FuncOrVar<A: Clone + Debug> {
    Func(Function),
    Var((bool, A)),
}

pub struct TestResult {
    pub name: Name,
    pub result: bool,
    pub error: Option<String>,
}

impl TestResult {
    pub fn new(name: Name, result: bool, error: Option<String>) -> Self {
        TestResult {
            name,
            result,
            error,
        }
    }
}

use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use crate::compiler::parser::Compiler;

#[derive(Clone, Debug)]
pub enum SymbolCategory {
    Field,
    Static,
    Var,
    Arg,
}

#[derive(Clone, Debug)]
pub struct SymbolCategoryError;

impl FromStr for SymbolCategory {
    type Err = SymbolCategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "field" => Ok(SymbolCategory::Field),
            "static" => Ok(SymbolCategory::Static),
            "var" => Ok(SymbolCategory::Var),
            "arg" => Ok(SymbolCategory::Arg),
            _ => Err(SymbolCategoryError),
        }
    }
}

impl Display for SymbolCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Field => write!(f, "this"),
            Self::Static => write!(f, "static"),
            Self::Var => write!(f, "local"),
            Self::Arg => write!(f, "argument"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub var_type: String,
    pub kind: SymbolCategory,
    pub index: u16,
}

impl Symbol {
    pub fn new(var_type: String, kind: SymbolCategory, index: u16) -> Self {
        Symbol {
            var_type,
            kind,
            index,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolIndexes {
    field: u16,
    static_: u16,
    var: u16,
    arg: u16,
}

impl SymbolIndexes {
    pub fn new() -> Self {
        SymbolIndexes {
            field: 0,
            static_: 0,
            var: 0,
            arg: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
    pub indices: SymbolIndexes,
    pub table: BTreeMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            indices: SymbolIndexes::new(),
            table: BTreeMap::new(),
        }
    }

    pub fn get_index(&mut self, symbol: &str) -> u16 {
        match symbol {
            "field" => self.indices.field,
            "static" => self.indices.static_,
            "var" => self.indices.var,
            "arg" => self.indices.arg,
            _ => panic!("Came across an unfamiliar symbol: '{}'", symbol),
        }
    }

    pub fn increment_index(&mut self, symbol: &str) {
        match symbol {
            "field" => self.indices.field += 1,
            "static" => self.indices.static_ += 1,
            "var" => self.indices.var += 1,
            "arg" => self.indices.arg += 1,
            _ => panic!("Came across an unfamiliar symbol: '{}'", symbol),
        }
    }

    pub fn insert_symbol(
        &mut self,
        symbol_name: String,
        symbol_type: String,
        symbol_kind: String,
        symbol_index: u16,
    ) {
        let symbol = Symbol::new(
            symbol_type,
            SymbolCategory::from_str(&symbol_kind).unwrap(),
            symbol_index,
        );
        self.table.insert(symbol_name, symbol);
    }

    pub fn get_symbol(&mut self, symbol_name: &str) -> Option<&Symbol> {
        self.table.get(symbol_name)
    }
}

impl Compiler {
    pub fn get_symbol(&mut self, symbol_name: &str) -> Option<&Symbol> {
        let subroutine_symbol = self.subroutine_symbol_table.get_symbol(symbol_name);
        if subroutine_symbol.is_none() {
            let class_symbol = self.class_symbol_table.get_symbol(symbol_name);
            return class_symbol;
        }
        subroutine_symbol
    }
}

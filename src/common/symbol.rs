#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Symbol(pub usize);

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Symbol({})", self.0)
    }
}

pub struct SymbolPool {
    strings: Vec<String>,
    map: std::collections::HashMap<String, Symbol>,
}

impl SymbolPool {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            map: std::collections::HashMap::new(),
        }
    }

    /// Intern a string and return its corresponding Symbol.
    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&symbol) = self.map.get(s) {
            return symbol;
        }
        let symbol = Symbol(self.strings.len());
        self.strings.push(s.to_string());
        self.map.insert(s.to_string(), symbol);
        symbol
    }

    /// Resolve a Symbol back to its string slice.
    pub fn resolve(&self, symbol: Symbol) -> &str {
        &self.strings[symbol.0]
    }

    pub fn contains(&self, s: &str) -> bool {
        self.map.contains_key(s)
    }

    pub fn get(&self, s: &str) -> Option<Symbol> {
        self.map.get(s).cloned()
    }

    pub fn print_contents(&self) {
        println!("SymbolPool contents:");
        for (i, s) in self.strings.iter().enumerate() {
            println!("  Symbol({}): '{}'", i, s);
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(pub usize);

pub struct StringPool {
    strings: Vec<String>,
    map: std::collections::HashMap<String, Symbol>,
}

impl StringPool {
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
}
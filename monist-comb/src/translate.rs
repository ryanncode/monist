
use crate::ir::Comb;

/// A translator that converts the generated combinator tree (Comb)
/// into a Bend syntax string suitable for HVM2 execution.
pub struct BendTranslator<'a> {
    comb: &'a Comb,
}

impl<'a> BendTranslator<'a> {
    pub fn new(comb: &'a Comb) -> Self {
        Self { comb }
    }

    fn translate_comb(comb: &Comb) -> String {
        match comb {
            Comb::S => "S".to_string(),
            Comb::K => "K".to_string(),
            Comb::I => "I".to_string(),
            Comb::B => "B".to_string(),
            Comb::C => "C".to_string(),
            Comb::T => "T".to_string(),
            Comb::Terminal(s) => s.clone(),
            Comb::Limit(_, _, inner) => Self::translate_comb(inner),
            Comb::Var(v) => v.clone(),
            Comb::App(l, r) => format!("{}({})", Self::translate_comb(l), Self::translate_comb(r)),
            Comb::Eq => "Eq".to_string(),
            Comb::Mem => "Mem".to_string(),
            Comb::Neg => "Neg".to_string(),
            Comb::Conj => "Conj".to_string(),
            Comb::Disj => "Disj".to_string(),
            Comb::Impl => "Impl".to_string(),
            Comb::Forall => "Forall".to_string(),
        }
    }

    /// Generates the Bend code.
    /// This translates the combinator nodes and foundational definitions.
    pub fn translate(&self) -> String {
        let mut source = String::new();
        source.push_str("// Auto-generated Bend combinator syntax\n\n");
        
        // Foundational combinator definitions
        source.push_str("def S(x):\n  return lambda y: lambda z: (x(z))(y(z))\n\n");
        source.push_str("def K(x):\n  return lambda y: x\n\n");
        source.push_str("def I(x):\n  return x\n\n");
        source.push_str("def B(x):\n  return lambda y: lambda z: x(y(z))\n\n");
        source.push_str("def C(x):\n  return lambda y: lambda z: (x(z))(y)\n\n");
        source.push_str("def T(x):\n  return x\n\n");

        source.push_str("def main():\n");
        
        let expr = Self::translate_comb(self.comb);
        source.push_str(&format!("  return {}\n", expr));

        source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_comb() {
        let comb = Comb::App(Box::new(Comb::S), Box::new(Comb::K));
        let translator = BendTranslator::new(&comb);
        let output = translator.translate();
        
        assert!(output.contains("def S(x):"));
        assert!(output.contains("def K(x):"));
        assert!(output.contains("def main():"));
        assert!(output.contains("return S(K)"));
    }
}

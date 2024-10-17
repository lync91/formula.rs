/// Defines Excel Functions.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Function {
    Abs,
    Sum,
    Product,
    Average,
    Or,
    And,
    Xor,
    Not,
    Negate,
    Days,
    Right,
    Left,
    Iff,
    IsBlank,
}

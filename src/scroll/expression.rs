use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    /// Instructs the named entity to moan its remembered data value, and to keep remembering it.
    Moan(Option<&'a str>),
    /// Boolean operator that evaluates to true if the entity is currently remembering a data value equal to the given variable, false otherwise.
    Remembering(Option<&'a str>, Value),
    /// This operator pops the top two value off the statement stack, divides the second value by the top value, and puts the result back on the statement stack.
    Rend,
    /// This operator replaces the top value of the statement stack with its negative.
    Turn,
    /// This is not an operator defined in the ZOMBIE language. It represents any concrete value occuring in the code.
    Value(Value),
}

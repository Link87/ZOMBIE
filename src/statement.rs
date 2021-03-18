use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    cmd: StatementCmd,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementCmd {
    SayNamed(String, Value),
    Say(Value),
    RememberNamed(String, Value),
    Remember(Value),
}

impl Statement {
    pub fn new(cmd: StatementCmd) -> Statement {
        Statement { cmd }
    }

    pub fn cmd<'a>(&'a self) -> &'a StatementCmd {
        &self.cmd
    }
}

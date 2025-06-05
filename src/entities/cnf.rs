use crate::{entities::clause::Clause, solver::SolveError};

use super::thesis::AtomicThesis;

#[derive(Clone, PartialEq, Debug)]
pub struct Cnf<'a> {
    pub clauses: Vec<Clause<'a>>,
}

impl<'a> Cnf<'a> {
    pub fn new(clauses: Vec<Clause<'a>>) -> Self {
        Self { clauses }
    }

    pub fn assign(mut self, thesis: &AtomicThesis, value: bool) -> Result<Self, SolveError> {
        let mut clauses: Vec<Clause> = self.clauses.clone();
        clauses = clauses
            .into_iter()
            .map(|c| c.assign(thesis, value))
            .collect::<Result<Vec<_>, _>>()?;

        clauses.retain(|c| !c.is_empty());

        self.clauses = clauses;

        Ok(self)
    }

    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }
}

impl<'a> std::fmt::Display for Cnf<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "(")?;
        for c in self.clauses.iter() {
            writeln!(f, "  {} and", c.to_string())?
        }
        write!(f, ")")?;
        Ok(())
    }
}

use crate::{
    entities::thesis::{AtomicThesis, Thesis},
    solver::SolveError,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Clause<'a> {
    pub theses: Vec<&'a Thesis<'a>>,
}

impl<'a> Clause<'a> {
    pub fn new(theses: Vec<&'a Thesis>) -> Self {
        Self { theses }
    }

    pub fn assign(mut self, atomic_thesis: &AtomicThesis, value: bool) -> Result<Self, SolveError> {
        if self
            .theses
            .iter()
            .any(|t| t.atomic().equals(atomic_thesis) && t.is_positive() == value)
        {
            self.theses.clear();
            Ok(self)
        } else {
            self.theses
                .retain(|t| !(t.atomic().equals(atomic_thesis) && t.is_positive() != value));
            // P に F をアサインしたら F よりその項のみ除去
            // not P に T をアサインしたら F よりその項のみ除去
            if self.theses.is_empty() {
                Err(SolveError::Unsat)
            } else {
                Ok(self)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.theses.is_empty()
    }
}

impl<'a> std::fmt::Display for Clause<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;
        for t in self.theses.iter() {
            write!(f, "{} or ", t.to_string())?
        }
        write!(f, ")")?;
        Ok(())
    }
}

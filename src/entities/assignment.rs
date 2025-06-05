use crate::entities::thesis::AtomicThesis;

#[derive(Clone, Debug)]
pub struct Assignment<'a> {
    pub thesis: &'a AtomicThesis,
    pub value: Option<bool>,
}

impl<'a> Assignment<'a> {
    pub fn new(thesis: &'a AtomicThesis, value: Option<bool>) -> Self {
        Self { thesis, value }
    }
}

impl<'a> std::fmt::Display for Assignment<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.value {
            Some(v) => write!(f, "{} -> {}", self.thesis.name, v)?,
            None => write!(f, "{} -> None", self.thesis.name)?,
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Assignments<'a> {
    pub assigns: Vec<Assignment<'a>>,
}

impl<'a> Assignments<'a> {
    pub fn assign(mut self, atomic_thesis: &'a AtomicThesis, value: bool) -> Self {
        for i in 0..self.assigns.len() {
            match self.assigns.get(i) {
                Some(a) => {
                    if a.thesis.equals(&atomic_thesis) {
                        self.assigns.remove(i);
                        self.assigns
                            .push(Assignment::new(atomic_thesis, Some(value)));
                        break;
                    }
                }
                None => {}
            }
        }
        self
    }
}

impl<'a> std::fmt::Display for Assignments<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for a in self.assigns.iter() {
            write!(f, "{}, ", a.to_string())?
        }
        write!(f, "]")?;
        Ok(())
    }
}

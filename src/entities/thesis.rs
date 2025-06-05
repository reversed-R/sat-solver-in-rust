#[derive(Clone, PartialEq, Debug)]
pub enum Thesis<'a> {
    Positive(&'a AtomicThesis),
    Negative(&'a AtomicThesis),
}

impl<'a> Thesis<'a> {
    pub fn not(&self) -> Self {
        match self {
            Self::Positive(a) => Self::Negative(a),
            Self::Negative(a) => Self::Positive(a),
        }
    }

    pub fn atomic(&self) -> &AtomicThesis {
        match self {
            Self::Positive(a) => a,
            Self::Negative(a) => a,
        }
    }

    pub fn is_positive(&self) -> bool {
        match self {
            Self::Positive(_) => true,
            Self::Negative(_) => false,
        }
    }
}

impl<'a> std::fmt::Display for Thesis<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Positive(a) => &write!(f, "{}", &a.name)?,
            Self::Negative(a) => &write!(f, "not {}", &a.name)?,
        };
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct AtomicThesis {
    pub name: String,
}

impl AtomicThesis {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn equals(&self, value: &Self) -> bool {
        self.name == value.name
    }
}

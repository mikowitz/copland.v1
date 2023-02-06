use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive)]
pub enum IntervalSize {
    Unison = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Sixth = 6,
    Seventh = 7,
    Octave = 8,
}

impl IntervalSize {
    pub fn to_float(self) -> f32 {
        match self {
            Self::Unison => 0.,
            Self::Second => 2.,
            Self::Third => 4.,
            Self::Fourth => 5.,
            Self::Fifth => 7.,
            Self::Sixth => 9.,
            Self::Seventh => 11.,
            Self::Octave => 12.,
        }
    }

    pub fn can_be_perfect(self) -> bool {
        matches!(self, Self::Unison | Self::Fourth | Self::Fifth | Self::Octave)
    }

    pub fn staff_spaces(self) -> i32 {
        self as i32 - 1
    }
}

impl fmt::Display for IntervalSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = *self as i32;
        write!(f, "{size}")
    }
}

impl From<i32> for IntervalSize {
    fn from(value: i32) -> Self {
        match num::FromPrimitive::from_i32(value) {
            Some(size) => size,
            None => todo!(),
        }
    }
}

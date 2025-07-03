use crate::{cubic::Cubic, geometry::PointTransformer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeatureType {
    Edge,
    Corner { convex: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Feature {
    pub ty: FeatureType,
    pub cubics: Vec<Cubic>,
}

impl Feature {
    pub const fn edge(cubics: Vec<Cubic>) -> Self {
        Self { cubics, ty: FeatureType::Edge }
    }

    pub const fn corner(cubics: Vec<Cubic>, convex: bool) -> Self {
        Self {
            cubics,
            ty: FeatureType::Corner { convex },
        }
    }

    #[must_use]
    pub fn transformed<T: PointTransformer>(self, f: &T) -> Self {
        match self.ty {
            FeatureType::Edge => Self {
                cubics: self.cubics.into_iter().map(|cubic| cubic.transformed(f)).collect(),
                ty: FeatureType::Edge,
            },
            FeatureType::Corner { convex } => Self {
                cubics: self.cubics.into_iter().map(|cubic| cubic.transformed(f)).collect(),
                ty: FeatureType::Corner { convex },
            },
        }
    }

    pub const fn is_corner(&self) -> bool {
        matches!(self.ty, FeatureType::Corner { .. })
    }

    pub fn is_corner_and<F: FnOnce(bool) -> bool>(&self, func: F) -> bool {
        if let FeatureType::Corner { convex } = &self.ty {
            func(*convex)
        } else {
            false
        }
    }
}

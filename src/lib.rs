#![allow(clippy::cast_precision_loss)]
#![doc = include_str!("../README.md")]

mod cubic;
mod feature;
mod feature_mapper;
pub mod geometry;
mod mapper;
mod measured_polygon;
mod measurer;
mod morph;
pub mod path;
mod rounded_polygon;
pub(crate) mod util;

pub use self::{
    cubic::Cubic,
    feature::{Feature, FeatureType},
    mapper::DoubleMapper,
    measured_polygon::{MeasuredPolygon, ProgressableFeature},
    measurer::Measurer,
    morph::Morph,
    rounded_polygon::{CornerRounding, RoundedPoint, RoundedPolygon},
};

use std::fmt::Display;

use crate::class_name::{fmt_class_name, ClassName};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MaskType {
    Default,
    Squircle,
    Heart,
    Hexagon,
    Hexagon2,
    Decagon,
    Pentagon,
    Diamond,
    Square,
    Circle,
    Parallelogram,
    Parallelogram2,
    Parallelogram3,
    Parallelogram4,
    Star,
    Star2,
    Triangle,
    Triangle2,
    Triangle3,
    Triangle4,
}

impl ClassName for MaskType {
    fn has_class_name(self) -> bool {
        self != MaskType::Default
    }

    fn class_name(self) -> String {
        match self {
            MaskType::Default => "".to_string(),
            MaskType::Squircle => "mask-squircle".to_string(),
            MaskType::Heart => "mask-heart".to_string(),
            MaskType::Hexagon => "mask-hexagon".to_string(),
            MaskType::Hexagon2 => "mask-hexagon-2".to_string(),
            MaskType::Decagon => "mask-decagon".to_string(),
            MaskType::Pentagon => "mask-pentagon".to_string(),
            MaskType::Diamond => "mask-diamond".to_string(),
            MaskType::Square => "mask-square".to_string(),
            MaskType::Circle => "mask-circle".to_string(),
            MaskType::Parallelogram => "mask-parallelogram".to_string(),
            MaskType::Parallelogram2 => "mask-parallelogram-2".to_string(),
            MaskType::Parallelogram3 => "mask-parallelogram-3".to_string(),
            MaskType::Parallelogram4 => "mask-parallelogram-4".to_string(),
            MaskType::Star => "mask-star".to_string(),
            MaskType::Star2 => "mask-star-2".to_string(),
            MaskType::Triangle => "mask-triangle".to_string(),
            MaskType::Triangle2 => "mask-triangle-2".to_string(),
            MaskType::Triangle3 => "mask-triangle-3".to_string(),
            MaskType::Triangle4 => "mask-triangle-4".to_string(),
        }
    }
}

impl Display for MaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MaskCrop {
    Default,
    First,
    Last,
}

impl ClassName for MaskCrop {
    fn has_class_name(self) -> bool {
        self != MaskCrop::Default
    }

    fn class_name(self) -> String {
        match self {
            MaskCrop::Default => "".to_string(),
            MaskCrop::First => "mask-half-1".to_string(),
            MaskCrop::Last => "mask-half-2".to_string(),
        }
    }
}

impl Display for MaskCrop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

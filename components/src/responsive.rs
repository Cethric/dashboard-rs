use std::fmt::Display;

use crate::class_name::{fmt_class_name, ClassName};

#[derive(Debug, Copy, Clone)]
pub enum Responsive<T>
where
    T: ClassName + Copy + Clone,
{
    Default(T),
    ExtraExtraLarge(T),
    ExtraLarge(T),
    Large(T),
    Medium(T),
    Small(T),
}

impl<T> ClassName for Responsive<T>
where
    T: ClassName + Copy + Clone,
{
    fn has_class_name(self) -> bool {
        match self {
            Responsive::Default(item) => item.has_class_name(),
            Responsive::ExtraExtraLarge(item) => item.has_class_name(),
            Responsive::ExtraLarge(item) => item.has_class_name(),
            Responsive::Large(item) => item.has_class_name(),
            Responsive::Medium(item) => item.has_class_name(),
            Responsive::Small(item) => item.has_class_name(),
        }
    }

    fn class_name(self) -> String {
        match self {
            Responsive::Default(item) => {
                if !item.has_class_name() {
                    String::default()
                } else {
                    item.class_name()
                }
            }
            Responsive::ExtraExtraLarge(item) => {
                if !item.has_class_name() {
                    String::default()
                } else {
                    format!("2xl:{}", item.class_name())
                }
            }
            Responsive::ExtraLarge(item) => {
                if !item.has_class_name() {
                    String::default()
                } else {
                    format!("xl:{}", item.class_name())
                }
            }
            Responsive::Large(item) => {
                if !item.has_class_name() {
                    String::default()
                } else {
                    format!("lg:{}", item.class_name())
                }
            }
            Responsive::Medium(item) => {
                if !item.has_class_name() {
                    String::default()
                } else {
                    format!("md:{}", item.class_name())
                }
            }
            Responsive::Small(item) => {
                if !item.has_class_name() {
                    String::default()
                } else {
                    format!("sm:{}", item.class_name())
                }
            }
        }
    }
}

impl<T> Display for Responsive<T>
where
    T: ClassName + Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_class_name(self, f)
    }
}

pub struct ResponsiveVec<T>(pub Vec<T>)
where
    T: ClassName + Copy + Clone;

impl<T> Display for ResponsiveVec<T>
where
    T: ClassName + Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "");
        }
        for item in &self.0[0..self.0.len() - 1] {
            if item.has_class_name() {
                debug_assert!(write!(f, " {}", item.class_name()).is_ok_and(|_| true));
            }
        }

        let item = self.0[self.0.len() - 1];
        if item.has_class_name() {
            write!(f, " {}", item.class_name())
        } else {
            write!(f, "")
        }
    }
}

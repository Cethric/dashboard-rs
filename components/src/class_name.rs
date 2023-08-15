pub trait ClassName {
    fn has_class_name(self) -> bool;
    fn class_name(self) -> String;
}

pub fn fmt_class_name<T>(item: &T, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
where
    T: ClassName + Copy,
{
    if item.has_class_name() {
        write!(f, " {}", item.class_name())
    } else {
        write!(f, "")
    }
}

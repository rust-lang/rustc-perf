use crate::values;

pub(crate) trait ColorEn {
    type Other: ColorEn;

    fn reify() -> values::Color;
}
pub(crate) struct White;
pub(crate) struct Black;

impl ColorEn for White {
    type Other = Black;

    fn reify() -> values::Color {
        values::Color::White
    }
}
impl ColorEn for Black {
    type Other = White;

    fn reify() -> values::Color {
        values::Color::Black
    }
}

// Reduced from rust-lang/trait-system-refactor-initiative#254.
//
// This exercises nested RPIT parser combinators. The original regression
// made the next solver significantly slower than the old solver.

type ParseResult<'src, Output> = Result<(&'src str, Output), String>;

pub trait Parser<'src, Output>: Sized + Copy {
    fn go(self, input: &'src str) -> ParseResult<'src, Output>;

    fn then<U, B: Parser<'src, U>>(self, other: B) -> impl Parser<'src, (Output, U)> {
        move |input: &'src str| -> ParseResult<'src, (Output, U)> {
            let (input, first) = self.go(input)?;
            let (rest, second) = other.go(input)?;
            Ok((rest, (first, second)))
        }
    }

    fn to<U: Copy>(self, value: U) -> impl Parser<'src, U> {
        move |input: &'src str| -> ParseResult<'src, U> { Ok((input, value)) }
    }
}

pub fn keyword<'src>(keyword: &'static str) -> impl Parser<'src, &'src str> {
    move |input: &'src str| -> ParseResult<'src, &'src str> {
        if input.starts_with(keyword) {
            let rest = &input[keyword.len()..];
            Ok((rest, &input[..keyword.len()]))
        } else {
            Err(format!("Expected keyword '{}'", keyword))
        }
    }
}

impl<'src, F, Output> Parser<'src, Output> for F
where
    F: Fn(&'src str) -> ParseResult<'src, Output>,
    F: Copy,
{
    fn go(self, input: &'src str) -> ParseResult<'src, Output> {
        self(input)
    }
}

#[derive(Copy, Clone)]
struct PrimitiveType(&'static str);

// Keeping the RPIT opaque is important to this regression case.
fn primitive_type_parser<'src>() -> impl Parser<'src, PrimitiveType> {
    keyword("a")
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .then(keyword("a"))
        .to(PrimitiveType("a"))
}

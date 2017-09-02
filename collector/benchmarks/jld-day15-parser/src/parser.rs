use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

use combine::{digit,letter,sep_by,many1,char,string,not_followed_by,any,
              Parser,ParserExt,ParseError};

use ::{Num,Stats};

pub enum SemanticError {
    DupStat(String, Num, Num),
    MissingStat(&'static str),
}
pub type SyntaxError<'a> = ParseError<&'a str>;

struct MaybeStats(Result<Stats, SemanticError>);
impl FromIterator<(String,Num)> for MaybeStats {
    fn from_iter<T>(ii: T) -> Self where T: IntoIterator<Item=(String,Num)> {
        let squash_plist = move || {
            let mut tab: HashMap<String,Num> = HashMap::new();
            for (key, val) in ii {
                if let Some(oldval) = tab.insert(key.clone(), val) {
                    return Err(SemanticError::DupStat(key, oldval, val));
                }
            }
            let get = |key| {
                tab.get(key).ok_or(SemanticError::MissingStat(key))
            };
            Ok(Stats {
                capacity: *try!(get("capacity")),
                durability: *try!(get("durability")),
                flavor: *try!(get("flavor")),
                texture: *try!(get("texture")),
                calories: *try!(get("calories")),
            })
        };
        MaybeStats(squash_plist())
    }
}

pub fn parse(s: &str) -> Result<(String, Result<Stats, SemanticError>), SyntaxError> {
    let num = many1(digit().or(char('-'))).and_then(|s: String| Num::from_str(&s));
    let prop = (many1(letter())).skip(char(' ')).and(num);
    let plist = sep_by::<MaybeStats, _, _>(prop, string(", "));
    let mut ingred = many1(letter()).skip(string(": ")).and(plist).skip(not_followed_by(any()));

    let ((name, MaybeStats(ms)), rest) =  try!(ingred.parse(s));
    assert!(rest == "");
    Ok((name, ms))
}

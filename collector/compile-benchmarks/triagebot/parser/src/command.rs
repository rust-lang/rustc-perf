use crate::error::Error;
use crate::ignore_block::IgnoreBlocks;
use crate::token::Tokenizer;
use regex::Regex;

pub mod assign;
pub mod close;
pub mod concern;
pub mod nominate;
pub mod note;
pub mod ping;
pub mod prioritize;
pub mod relabel;
pub mod second;
pub mod shortcut;
pub mod transfer;

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Relabel(Result<relabel::RelabelCommand, Error<'a>>),
    Assign(Result<assign::AssignCommand, Error<'a>>),
    Ping(Result<ping::PingCommand, Error<'a>>),
    Nominate(Result<nominate::NominateCommand, Error<'a>>),
    Prioritize(Result<prioritize::PrioritizeCommand, Error<'a>>),
    Second(Result<second::SecondCommand, Error<'a>>),
    Shortcut(Result<shortcut::ShortcutCommand, Error<'a>>),
    Close(Result<close::CloseCommand, Error<'a>>),
    Note(Result<note::NoteCommand, Error<'a>>),
    Concern(Result<concern::ConcernCommand, Error<'a>>),
    Transfer(Result<transfer::TransferCommand, Error<'a>>),
}

#[derive(Debug)]
pub struct Input<'a> {
    all: &'a str,
    parsed: usize,
    ignore: IgnoreBlocks,
    /// A pattern for finding the start of a command based on the name of the
    /// configured bots.
    bot_re: Regex,
}

fn parse_single_command<'a, T, F, M>(
    parse: F,
    mapper: M,
    tokenizer: &Tokenizer<'a>,
) -> Option<(Tokenizer<'a>, Command<'a>)>
where
    F: FnOnce(&mut Tokenizer<'a>) -> Result<Option<T>, Error<'a>>,
    M: FnOnce(Result<T, Error<'a>>) -> Command<'a>,
    T: std::fmt::Debug,
{
    let mut tok = tokenizer.clone();
    let res = parse(&mut tok);
    log::info!("parsed {:?} command: {:?}", std::any::type_name::<T>(), res);
    match res {
        Ok(None) => None,
        Ok(Some(v)) => Some((tok, mapper(Ok(v)))),
        Err(err) => Some((tok, mapper(Err(err)))),
    }
}

impl<'a> Input<'a> {
    pub fn new(input: &'a str, bot: Vec<&'a str>) -> Input<'a> {
        let bots: Vec<_> = bot.iter().map(|bot| format!(r"(?:@{bot}\b)")).collect();
        let bot_re = Regex::new(&format!(
            r#"(?i)(?P<review>\br\?)|{bots}"#,
            bots = bots.join("|")
        ))
        .unwrap();
        Input {
            all: input,
            parsed: 0,
            ignore: IgnoreBlocks::new(input),
            bot_re,
        }
    }

    fn parse_command(&mut self) -> Option<Command<'a>> {
        let tok = Tokenizer::new(&self.all[self.parsed..]);
        log::info!("identified potential command");

        let mut success = vec![];

        let original_tokenizer = tok.clone();

        success.extend(parse_single_command(
            relabel::RelabelCommand::parse,
            Command::Relabel,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            assign::AssignCommand::parse,
            Command::Assign,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            note::NoteCommand::parse,
            Command::Note,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            concern::ConcernCommand::parse,
            Command::Concern,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            ping::PingCommand::parse,
            Command::Ping,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            nominate::NominateCommand::parse,
            Command::Nominate,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            prioritize::PrioritizeCommand::parse,
            Command::Prioritize,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            second::SecondCommand::parse,
            Command::Second,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            shortcut::ShortcutCommand::parse,
            Command::Shortcut,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            close::CloseCommand::parse,
            Command::Close,
            &original_tokenizer,
        ));
        success.extend(parse_single_command(
            transfer::TransferCommand::parse,
            Command::Transfer,
            &original_tokenizer,
        ));

        if success.len() > 1 {
            panic!(
                "succeeded parsing {:?} to multiple commands: {:?}",
                &self.all[self.parsed..],
                success
            );
        }

        let (mut tok, c) = success.pop()?;
        // if we errored out while parsing the command do not move the input forwards
        if c.is_ok() {
            self.parsed += tok.position();
        }
        Some(c)
    }

    /// Parses command for `r?`
    fn parse_review(&mut self) -> Option<Command<'a>> {
        let tok = Tokenizer::new(&self.all[self.parsed..]);
        match parse_single_command(assign::AssignCommand::parse_review, Command::Assign, &tok) {
            Some((mut tok, command)) => {
                self.parsed += tok.position();
                Some(command)
            }
            None => {
                log::warn!("expected r? parser to return something: {:?}", self.all);
                None
            }
        }
    }
}

impl<'a> Iterator for Input<'a> {
    type Item = Command<'a>;

    fn next(&mut self) -> Option<Command<'a>> {
        loop {
            let caps = self.bot_re.captures(&self.all[self.parsed..])?;
            let m = caps.get(0).unwrap();
            if self
                .ignore
                .overlaps_ignore((self.parsed + m.start())..(self.parsed + m.end()))
                .is_some()
            {
                log::info!("command overlaps ignored block; ignore: {:?}", self.ignore);
                self.parsed += m.end();
                continue;
            }

            self.parsed += m.end();
            if caps.name("review").is_some() {
                if let Some(command) = self.parse_review() {
                    return Some(command);
                }
            } else if let Some(command) = self.parse_command() {
                return Some(command);
            }
        }
    }
}

impl<'a> Command<'a> {
    pub fn is_ok(&self) -> bool {
        match self {
            Command::Relabel(r) => r.is_ok(),
            Command::Assign(r) => r.is_ok(),
            Command::Ping(r) => r.is_ok(),
            Command::Nominate(r) => r.is_ok(),
            Command::Prioritize(r) => r.is_ok(),
            Command::Second(r) => r.is_ok(),
            Command::Shortcut(r) => r.is_ok(),
            Command::Close(r) => r.is_ok(),
            Command::Note(r) => r.is_ok(),
            Command::Concern(r) => r.is_ok(),
            Command::Transfer(r) => r.is_ok(),
        }
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

#[test]
fn errors_outside_command_are_fine() {
    let input = "haha\" unterminated quotes @bot labels +bug. Terminating after the command";
    let mut input = Input::new(input, vec!["bot"]);
    assert!(input.next().unwrap().is_ok());
}

#[test]
fn code_1() {
    let input = "`@bot modify label: +bug.`";
    let mut input = Input::new(input, vec!["bot"]);
    assert!(input.next().is_none());
}

#[test]
fn code_2() {
    let input = "```
    @bot modify labels: +bug.
    ```";
    let mut input = Input::new(input, vec!["bot"]);
    assert!(input.next().is_none());
}

#[test]
fn resumes_after_code() {
    // Handles a command after an ignored block.
    let input = "```
@bot modify labels: +bug.
```

@bot claim
    ";
    let mut input = Input::new(input, vec!["bot"]);
    assert!(matches!(input.next(), Some(Command::Assign(Ok(_)))));
    assert_eq!(input.next(), None);
}

#[test]
fn edit_1() {
    let input_old = "@bot modify labels: +bug.";
    let mut input_old = Input::new(input_old, vec!["bot"]);
    let input_new = "Adding labels: @bot modify label +bug. some other text";
    let mut input_new = Input::new(input_new, vec!["bot"]);
    assert_eq!(input_old.next(), input_new.next());
}

#[test]
fn edit_2() {
    let input_old = "@bot label bug.";
    let mut input_old = Input::new(input_old, vec!["bot"]);
    let input_new = "@bot modify labels to: +bug.";
    let mut input_new = Input::new(input_new, vec!["bot"]);
    assert_eq!(input_old.next(), input_new.next());
}

#[test]
fn move_input_along() {
    let input = "@bot labels: +bug. Afterwards, delete the world.";
    let mut input = Input::new(input, vec!["bot"]);
    assert!(input.next().unwrap().is_ok());
    assert_eq!(&input.all[input.parsed..], " Afterwards, delete the world.");
}

#[test]
fn move_input_along_1() {
    let input = "@bot modify labels\": +bug. Afterwards, delete the world.";
    let mut input = Input::new(input, vec!["bot"]);
    assert!(input.next().unwrap().is_err());
    // don't move input along if parsing the command fails
    assert_eq!(&input.all[..input.parsed], "@bot");
}

#[test]
fn multiname() {
    let input = "@rustbot label to: +bug. Afterwards, delete the world. @triagebot prioritize";
    let mut input = Input::new(input, vec!["triagebot", "rustbot"]);
    assert!(dbg!(input.next().unwrap()).is_ok());
    assert_eq!(
        &input.all[input.parsed..],
        " Afterwards, delete the world. @triagebot prioritize"
    );
    assert!(input.next().unwrap().is_ok());
    assert!(input.next().is_none());
}

#[test]
fn review_commands() {
    for (input, name) in [
        ("r? @octocat", "octocat"),
        ("r? octocat", "octocat"),
        ("R? @octocat", "octocat"),
        ("can I r? someone?", "someone"),
        ("Please r? @octocat can you review?", "octocat"),
        ("r? rust-lang/compiler", "rust-lang/compiler"),
        ("r? @D--a--s-h", "D--a--s-h"),
    ] {
        let mut input = Input::new(input, vec!["bot"]);
        assert_eq!(
            input.next(),
            Some(Command::Assign(Ok(assign::AssignCommand::RequestReview {
                name: name.to_string()
            })))
        );
        assert_eq!(input.next(), None);
    }
}

#[test]
fn review_errors() {
    use std::error::Error;
    for input in ["r?", "r? @", "r? @ user", "r?:user", "r?! @foo", "r?\nline"] {
        let mut input = Input::new(input, vec!["bot"]);
        let err = match input.next() {
            Some(Command::Assign(Err(err))) => err,
            c => panic!("unexpected {:?}", c),
        };
        assert_eq!(
            err.source().unwrap().downcast_ref(),
            Some(&assign::ParseError::NoUser)
        );
        assert_eq!(input.next(), None);
    }
}

#[test]
fn review_ignored() {
    // Checks for things that shouldn't be detected.
    for input in [
        "r",
        "reviewer? abc",
        "r foo",
        "<a>\n r? @bot\n</a>",
        "<!--\nr? foo\n-->",
    ] {
        let mut input = Input::new(input, vec!["bot"]);
        assert_eq!(input.next(), None);
    }
}

#[test]
fn concern() {
    let input = "@bot concern this is my concern";
    let mut input = Input::new(input, vec!["bot"]);
    assert_eq!(
        input.next(),
        Some(Command::Concern(Ok(concern::ConcernCommand::Concern {
            title: "this is my concern".to_string()
        })))
    );
}

#[test]
fn resolve() {
    let input = "@bot resolve this is my concern";
    let mut input = Input::new(input, vec!["bot"]);
    assert_eq!(
        input.next(),
        Some(Command::Concern(Ok(concern::ConcernCommand::Resolve {
            title: "this is my concern".to_string()
        })))
    );
}

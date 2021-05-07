use rustyline::{
    completion::{Completer, FilenameCompleter, Pair},
    error::ReadlineError,
    highlight::{Highlighter, MatchingBracketHighlighter},
    hint::{Hinter, HistoryHinter},
    validate::{MatchingBracketValidator, Validator},
    Context,
};
use rustyline_derive::Helper;
use std::borrow::Cow::{self, Borrowed, Owned};
//use colored::*;

#[derive(Helper)]
pub struct CustomHelper {
    pub completer: FilenameCompleter,
    pub highlighter: MatchingBracketHighlighter,
    pub validator: MatchingBracketValidator,
    pub hinter: HistoryHinter,
    pub styled_prompt: String,
}

impl Completer for CustomHelper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Highlighter for CustomHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, prompt: &'p str, default: bool) -> Cow<'b, str> {
        if default {
            Borrowed(&self.styled_prompt)
        } else {
            Borrowed(prompt)
        }
    }
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        //Owned(hint.color(.hinting_color).to_string())
        Owned(hint.to_string())
    }
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Hinter for CustomHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Validator for CustomHelper {}

//impl Validator for CustomHelper {
//fn validate(&self, ctx: &mut ValidationContext<'_>) -> rustyline::Result<validate::ValidationResult> {
//self.validator.validate(ctx)
//}
//fn validate_while_typing(&self) -> bool {
//self.validator.validate_while_typing()
//}
//}

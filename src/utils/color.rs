use std::fmt::Display;

const ESCAPE: &str = "\x1b";
const CLEAR: &str = "\x1b[0m";

pub trait ColoredOutput {
    fn orange(&self) -> String;
    fn on_red(&self) -> String;
    fn on_purple(&self) -> String;
}

impl<T: Display> ColoredOutput for T {
    fn orange(&self) -> String {
        format!("{ESCAPE}[38;5;214m{self}{CLEAR}")
    }

    fn on_red(&self) -> String {
        format!("{ESCAPE}[41m{self}{CLEAR}")
    }

    fn on_purple(&self) -> String {
        format!("{ESCAPE}[45m{self}{CLEAR}")
    }
}

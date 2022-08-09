#[derive(Clone)]
pub enum Formatter {
    RegexReplace(),
    StringReplace(),
    Trim,
    AddSpaceToLineEnd,
}

#[derive(Clone)]
pub struct Formatters {
    preprocessors: Vec<Formatter>,
    formatters: Vec<Formatter>,
}

impl Formatters {
    pub fn empty() -> Formatters {
        Formatters {
            preprocessors: vec![],
            formatters: vec![],
        }
    }

    pub fn run_preprocessors(&self, value: String) -> String {
        self.preprocessors
            .clone()
            .into_iter()
            .fold(value, |v, f| f.run(v))
    }

    pub fn run(&self, value: String) -> String {
        self.formatters
            .clone()
            .into_iter()
            .fold(value, |v, f| f.run(v))
    }
}

impl Formatter {
    pub fn run(&self, value: String) -> String {
        //TODO implement formatters!
        value
    }
}

use regex::Regex;

#[derive(Clone)]
pub enum Formatter {
    RegexReplace(RegexReplaceFormatter),
    StringReplace(),
    Trim,
    AddSpaceToLineEnd,
}

#[derive(Clone)]
pub struct RegexReplaceFormatter {
    pattern: &'static str,
    replacement: &'static str,
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

    pub fn default() -> Formatters {
        Formatters {
            preprocessors: vec![Formatter::Trim, Formatter::AddSpaceToLineEnd],
            formatters: vec![
                Formatter::Trim,
                Formatter::RegexReplace(RegexReplaceFormatter {
                    pattern: "^(\\* )",
                    replacement: "",
                }),
                Formatter::RegexReplace(RegexReplaceFormatter {
                    pattern: "^([0-9]\\. )|^([0-9][0-9]\\. )",
                    replacement: "",
                }),
            ],
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
    pub fn run(&self, mut value: String) -> String {
        match self {
            Formatter::RegexReplace(rr) => {
                let re = Regex::new(rr.pattern).unwrap();

                re.replace_all(&value, rr.replacement).to_string()
            }
            Formatter::StringReplace() => todo!(),
            Formatter::Trim => value.trim().to_string(),
            Formatter::AddSpaceToLineEnd => {
                value.push(' ');
                value
            }
        }

        //TODO implement formatters!
        //value
    }
}

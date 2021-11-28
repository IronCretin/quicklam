use ariadne::{Color, Label, Report, ReportKind, Source};
use lalrpop_util::ParseError;
use rustyline::{error::ReadlineError, Editor};

use quicklam::error::Error;
use quicklam::parse;

fn main() {
    // let mut exp = parse(r"(\f x. f (f (f x))) (\f x. f (f (f x))) S Z").unwrap();
    // exp.reduce_all();
    // println!("{}", exp);

    let mut rl = Editor::<()>::new();
    // ignore error if no prev history
    let _ = rl.load_history(".history");
    'repl: loop {
        match rl.readline(">>> ") {
            Ok(mut line) => {
                if line.is_empty() {
                    continue;
                }
                let term = loop {
                    match parse(line.as_str()) {
                        Ok(term) => {
                            rl.add_history_entry(line.as_str());
                            break term;
                        }
                        Err(ParseError::UnrecognizedEOF { .. }) => match rl.readline("... ") {
                            Ok(l) => {
                                rl.add_history_entry(l.as_str());
                                line += &l;
                            }
                            Err(ReadlineError::Interrupted) => {
                                println!("^C");
                                rl.add_history_entry(line.as_str());
                                continue 'repl;
                            }
                            Err(ReadlineError::Eof) => {
                                println!("^D");
                                break 'repl;
                            }
                            Err(err) => {
                                println!("Error: {:?}", err);
                                break 'repl;
                            }
                        },
                        Err(e) => {
                            let span = match e {
                                ParseError::InvalidToken { location, .. } => location..location + 1,
                                ParseError::UnrecognizedEOF { .. } => unreachable!(),
                                ParseError::UnrecognizedToken {
                                    token: (location, ..),
                                    ..
                                } => location..location + 1,
                                ParseError::ExtraToken {
                                    token: (start, _, end),
                                    ..
                                } => start..end,
                                ParseError::User {
                                    error: Error { start, end, .. },
                                } => start..end,
                            };
                            Report::build(ReportKind::Error, (), 0)
                                .with_message("Parse error")
                                .with_label(
                                    Label::new(span)
                                        .with_message(e.to_string())
                                        .with_color(Color::Red),
                                )
                                .finish()
                                .eprint(Source::from(&line))
                                .unwrap();
                            continue 'repl;
                        }
                    }
                };
                println!("{}", term);
                for t in term.reduce_iter() {
                    println!("==>\n{}", t);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(".history").unwrap();
}

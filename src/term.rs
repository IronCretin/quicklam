use std::fmt::{self, Display};
use std::rc::Rc;

use Term::*;

/// A lambda calculus term, in De Bruijn index notation.
#[derive(Debug, Clone)]
pub enum Term<'a> {
    /// A variable. The number indicates the corresponding lambda abstraction.
    Var(usize),
    /// An arbitrary free variable
    Free(&'a str),
    /// Function application.
    App(Rc<Term<'a>>, Rc<Term<'a>>),
    /// A lambda abstraction. The name has no meaning except for printing.
    Lam(&'a str, Rc<Term<'a>>),
}

impl Default for Term<'_> {
    fn default() -> Self {
        Term::Var(0)
    }
}

impl PartialEq for Term<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Var(i), Var(j)) => i == j,
            (Free(s), Free(t)) => s == t,
            (App(l1, r1), App(l2, r2)) => l1 == l2 && r1 == r2,
            (Lam(_, b1), Lam(_, b2)) => b1 == b2,
            _ => false,
        }
    }
}

impl Display for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        enum Position {
            Root,
            Left,
            Right,
            Body,
        }
        use Position::*;
        fn go<'a>(
            t: &'a Term<'_>,
            vars: &mut Vec<&'a str>,
            pos: Position,
            f: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            if matches!(pos, Body) && !matches!(t, Lam(_, _)) {
                f.write_str(". ")?;
            }
            match t {
                &Var(i) => {
                    if i < vars.len() {
                        f.write_str(vars[vars.len() - 1 - i])?
                    } else {
                        write!(f, "#{}", i)?
                    }
                }
                &Free(s) => f.write_str(s)?,
                App(l, r) => {
                    if matches!(pos, Right) {
                        f.write_str("(")?;
                    }
                    go(l, vars, Left, f)?;
                    f.write_str(" ")?;
                    go(r, vars, Right, f)?;
                    if matches!(pos, Right) {
                        f.write_str(")")?;
                    }
                }
                Lam(x, b) => {
                    if matches!(pos, Left) {
                        f.write_str("(")?;
                    }
                    if !matches!(pos, Body) {
                        f.write_str("λ")?;
                    } else {
                        f.write_str(" ")?;
                    }
                    f.write_str(x)?;
                    vars.push(*x);
                    go(b, vars, Body, f)?;
                    vars.pop();
                    if matches!(pos, Left) {
                        f.write_str(")")?;
                    }
                }
            }
            Ok(())
        }
        go(self, &mut Vec::new(), Root, f)
    }
}

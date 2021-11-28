use std::rc::Rc;

use crate::term::Term;
use Term::*;

impl<'a> Term<'a> {
    // fn free(&self, n: usize) -> bool {
    //     match self {
    //         Var(i) => *i == n,
    //         Free(_) => false,
    //         App(f, x) => f.free(n) || x.free(n),
    //         Lam(_, b) => b.free(n + 1),
    //     }
    // }
    /// Increment all variables >= `n`
    fn inc(&mut self, n: usize) {
        match self {
            Var(i) => {
                if *i >= n {
                    *i += 1
                }
            }
            Free(_) => {}
            App(f, x) => {
                Rc::make_mut(f).inc(n);
                Rc::make_mut(x).inc(n);
            }
            Lam(_, b) => Rc::make_mut(b).inc(n + 1),
        }
    }
    /// substitute `r` for `n`, decrementing all variables above.
    fn sub_dec(&mut self, n: usize, r: &Term<'a>) {
        match self {
            Var(i) => {
                if *i == n {
                    *self = r.clone();
                } else if *i > n {
                    *i -= 1
                }
            }
            Free(_) => {}
            App(f, x) => {
                Rc::make_mut(f).sub_dec(n, r);
                Rc::make_mut(x).sub_dec(n, r);
            }
            Lam(_, b) => {
                let mut r = r.clone();
                r.inc(n);
                Rc::make_mut(b).sub_dec(n + 1, &r);
            }
        }
    }
    pub fn reduce_step(self: &mut Rc<Self>) -> bool {
        match self.as_ref() {
            Var(_) => false,
            Free(_) => false,
            _ => match Rc::make_mut(self) {
                App(f, x) => match Rc::make_mut(f) {
                    Lam(_, b) => {
                        Rc::make_mut(b).sub_dec(0, x);
                        *self = b.clone();
                        true
                    }
                    _ => f.reduce_step() || x.reduce_step(),
                },
                Lam(_, b) => b.reduce_step(),
                _ => unreachable!(),
            },
        }
    }
    pub fn reduce_iter(self: Rc<Self>) -> Reductions<'a> {
        Reductions {
            current: Some(self),
        }
    }
    pub fn reduce_all(self: &mut Rc<Self>) {
        while self.reduce_step() {
            // println!("{} \n==>", self);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reductions<'a> {
    current: Option<Rc<Term<'a>>>,
}
impl<'a> Iterator for Reductions<'a> {
    type Item = Rc<Term<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.as_mut()?.reduce_step() {
            self.current.clone()
        } else {
            self.current = None;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn church() {
        let mut exp = parse(r"(\f x. f (f x)) (\f x. f (f x)) S Z").unwrap();
        exp.reduce_all();
        assert_eq!(exp, parse("S (S (S (S Z)))").unwrap());
    }
    #[test]
    fn big_church() {
        let mut exp = parse(r"(\f x. (f (f x))) (\f x. f (f (f x))) S Z").unwrap();
        exp.reduce_all();
        assert_eq!(exp, parse("S (S (S (S (S (S (S (S (S Z))))))))").unwrap());
    }
    #[test]
    fn ski() {
        let mut exp = parse(r"(\S K. S K K x) (\x y z. x z (y z)) (\x y. x)").unwrap();
        exp.reduce_all();
        assert_eq!(exp, parse(r"x").unwrap());
    }
}

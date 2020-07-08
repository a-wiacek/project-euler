use crate::utils::numeric::digits::undigits;
use itertools::Itertools;
use num::{rational::Ratio, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, One};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Clone, Eq, PartialEq)]
struct ExpNode<T> {
    fun: Op,
    left: Box<Exp<T>>,
    right: Box<Exp<T>>,
}

#[derive(Clone, Eq, PartialEq)]
enum Exp<T> {
    ExpNode(ExpNode<T>),
    ExpLeaf(T),
}

impl<T> Exp<T>
where
    T: Clone + One + Integer + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv,
{
    fn eval(&self) -> Option<Ratio<T>> {
        match self {
            Exp::ExpLeaf(leaf) => Some(Ratio::new_raw(leaf.clone(), T::one())),
            Exp::ExpNode(node) => {
                let ExpNode { fun, left, right } = node;
                if let Some(l) = left.eval() {
                    if let Some(r) = right.eval() {
                        return match fun {
                            Op::Add => Ratio::<T>::checked_add,
                            Op::Sub => Ratio::<T>::checked_sub,
                            Op::Mul => Ratio::<T>::checked_mul,
                            Op::Div => Ratio::<T>::checked_div,
                        }(&l, &r);
                    }
                }
                None
            }
        }
    }
}

fn generate_exps_with_leaves(leaves: &[i32]) -> Vec<Exp<i32>> {
    match leaves.len() {
        1 => vec![Exp::ExpLeaf(leaves[0])],
        _ => {
            let mut exps: Vec<Exp<i32>> = Vec::new();
            for &fun in &[Op::Add, Op::Sub, Op::Mul, Op::Div] {
                for i in 1..leaves.len() {
                    let lefts = generate_exps_with_leaves(&leaves[..i]);
                    let rights = generate_exps_with_leaves(&leaves[i..]);
                    for left in &lefts {
                        for right in &rights {
                            exps.push(Exp::ExpNode(ExpNode {
                                fun,
                                left: Box::new(left.clone()),
                                right: Box::new(right.clone()),
                            }))
                        }
                    }
                }
            }
            exps
        }
    }
}

fn score(digits: Vec<i32>) -> usize {
    let mut values: Vec<i32> = digits
        .into_iter()
        .permutations(4)
        .flat_map(|perm| generate_exps_with_leaves(&perm[..]))
        .flat_map(|exp| exp.eval())
        .filter(Ratio::is_integer)
        .map(|val| val.to_integer())
        .filter(|val| val.is_positive())
        .collect();
    values.sort();
    values.dedup();
    values
        .into_iter()
        .zip(1..)
        .take_while(|(x, y)| x == y)
        .count()
}

pub fn euler093() -> String {
    undigits(
        &(1..10)
            .combinations(4)
            .max_by_key(|digits| score(digits.clone()))
            .unwrap(),
    )
    .to_string()
}

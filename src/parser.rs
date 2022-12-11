use nom::sequence::tuple;

use nom::branch::alt;

use nom::multi::separated_list0;

use nom::character::complete::digit1;

use nom::combinator::map_res;

use nom::bytes::complete::tag;

use nom::sequence::preceded;

use nom::IResult;

use std::str::FromStr;

use core::fmt;
use std::fmt::Display;

pub enum Op {
    Add(u64),
    Prod(OptionProd),
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        match self {
            Op::Add(n) => {
                s.push_str("Add ");
                s.push_str(&n.to_string());
            }
            Op::Prod(opt) => {
                s.push_str("Multiply");
                match opt {
                    OptionProd::Old => s.push_str(" by self"),
                    OptionProd::Num(n) => {
                        let mut p = " by ".to_string();
                        p.push_str(&n.to_string());
                        s.push_str(&p);
                    }
                }
            }
        }
        write!(f, "{}", s)
    }
}

pub enum Test {
    Div(u64),
}
impl Test {
    pub(crate) fn unwrap(&self) -> u64 {
        match self {
            Test::Div(n) => *n,
        }
    }
}

impl Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "Divides by ".to_string();
        match self {
            Test::Div(n) => {
                s.push_str(&n.to_string());
            }
        }
        write!(f, "{}", s)
    }
}

pub enum OptionProd {
    Old,
    Num(u64),
}

pub enum Conditional {
    True(u64),
    False(u64),
}

impl Conditional {
    pub fn unwrap(&self) -> u64 {
        match self {
            Conditional::True(n) => *n,
            Conditional::False(n) => *n,
        }
    }
}

impl Display for Conditional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        match self {
            Conditional::True(n) => {
                s.push_str("throw to monkey ");
                s.push_str(&n.to_string());
            }
            Conditional::False(n) => {
                s.push_str("throw to monkey ");
                s.push_str(&n.to_string());
            }
        }
        write!(f, "{}", s)
    }
}

pub struct Monkey {
    pub(crate) id: u64,
    pub(crate) starting_items: Vec<u64>,
    pub(crate) operation: Op,
    pub(crate) test: Test,
    pub(crate) cond_true: Conditional,
    pub(crate) cond_false: Conditional,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items = "[".to_string();
        for item in &self.starting_items {
            items.push_str(&item.to_string());
            items.push_str(", ");
        }
        items.pop();
        items.pop();
        items.push(']');

        write!(
            f,
            "(\nid: {}\n items: {}\n operations: {}\n test: {}\n true: {}\n false: {})",
            self.id, items, self.operation, self.test, self.cond_true, self.cond_false
        )
    }
}

pub fn parse_id(i: &str) -> IResult<&str, u64> {
    let (input, n) = preceded(tag("Monkey "), map_res(digit1, u64::from_str))(i)?;
    // println!("success id");
    Ok((input, n))
}

pub fn parse_items(i: &str) -> IResult<&str, Vec<u64>> {
    let (input, n) = preceded(
        tag(":\r\n  Starting items: "),
        separated_list0(tag(", "), map_res(digit1, u64::from_str)),
    )(i)?;

    // println!("success items");
    Ok((input, n))
}

pub fn parse_op(i: &str) -> IResult<&str, Op> {
    let (input, _) = tag("\r\n  Operation: new = old ")(i)?;
    // println!("success op");
    let (input, n) = alt((parse_op_add, parse_op_prod))(input)?;
    Ok((input, n))
}

pub fn parse_op_add(i: &str) -> IResult<&str, Op> {
    let (input, _) = tag("+ ")(i)?;
    let (input, n) = map_res(digit1, u64::from_str)(input)?;
    Ok((input, Op::Add(n)))
}

pub fn parse_op_prod_old(i: &str) -> IResult<&str, Op> {
    let (input, _) = tag("old")(i)?;
    Ok((input, Op::Prod(OptionProd::Old)))
}

pub fn parse_op_prod_num(i: &str) -> IResult<&str, Op> {
    let (input, n) = map_res(digit1, u64::from_str)(i)?;
    Ok((input, Op::Prod(OptionProd::Num(n))))
}

pub fn parse_op_prod(i: &str) -> IResult<&str, Op> {
    let (input, _) = tag("* ")(i)?;
    let (input, n) = alt((parse_op_prod_num, parse_op_prod_old))(input)?;
    Ok((input, n))
}

pub fn parse_test(i: &str) -> IResult<&str, Test> {
    let (input, _) = tag("\r\n  Test: divisible by ")(i)?;
    let (input, n) = map_res(digit1, u64::from_str)(input)?;
    // println!("success test");
    Ok((input, Test::Div(n)))
}

pub fn parse_conditional_true(i: &str) -> IResult<&str, Conditional> {
    let (input, _) = tag("\r\n    If true: throw to monkey ")(i)?;
    let (input, n) = map_res(digit1, u64::from_str)(input)?;
    // println!("success true");
    Ok((input, Conditional::True(n)))
}

pub fn parse_conditional_false(i: &str) -> IResult<&str, Conditional> {
    let (input, _) = tag("\r\n    If false: throw to monkey ")(i)?;
    let (input, n) = map_res(digit1, u64::from_str)(input)?;
    // println!("success false");
    Ok((input, Conditional::False(n)))
}

pub fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (input, (id, starting_items, operation, test, cond_true, cond_false)) = tuple((
        parse_id,
        parse_items,
        parse_op,
        parse_test,
        parse_conditional_true,
        parse_conditional_false,
    ))(i)?;
    Ok((
        input,
        Monkey {
            id,
            starting_items,
            operation,
            test,
            cond_true,
            cond_false,
        },
    ))
}

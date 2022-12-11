use std::fmt::{Display, Formatter};
use std::mem::swap;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::map;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{preceded, tuple};

use crate::util::{default_solution, parse_usize};

#[derive(Debug, Copy, Clone)]
enum Operand {
    Old,
    Const(usize),
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Plus,
    Multiply,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Square,
    IncrementBy(usize),
    MultiplyBy(usize)
}

impl Operation {
    fn perform(self, old: usize) -> usize {
        match self {
            Operation::Square => old * old,
            Operation::IncrementBy(n) => old + n,
            Operation::MultiplyBy(n) => old * n
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    number: usize,
    operation: Operation,
    module: usize,
    goto_on_zero: usize,
    goto_on_nonzero: usize,
    inventory: Vec<usize>,
    inspections_performed: i64
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Monkey {}: {:?}; performed {} inspections", self.number, self.inventory, self.inspections_performed))
    }
}

impl Monkey {
    fn distribute_inventory<Calm>(monkeys: &mut [Monkey], self_idx: usize, great_monkey_modulus: usize, calm: Calm) where Calm: Fn(usize) -> usize {
        let mut this = Monkey { number: usize::MAX, operation: Operation::MultiplyBy(0), module: 1, goto_on_nonzero: 0, goto_on_zero: 0, inventory: Vec::new(), inspections_performed: 0  };
        swap(&mut this, &mut monkeys[self_idx]);


        for initial_worry_level in &this.inventory {
            this.inspections_performed += 1;
            let after_inspect = this.operation.perform(*initial_worry_level);
            let after_calm_down = calm(after_inspect);

            let target = if after_calm_down % this.module == 0 {
                &mut monkeys[this.goto_on_zero].inventory
            } else {
                &mut monkeys[this.goto_on_nonzero].inventory
            };

            target.push(after_calm_down % great_monkey_modulus)
        }

        this.inventory.clear();

        swap(&mut this, &mut monkeys[self_idx]);
        assert!(this.inventory.is_empty())
    }
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    map(one_of("+*"), |ch| if ch == '*' { Operator::Multiply } else { Operator::Plus })(input)
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(tag("old"), |_| Operand::Old),
        map(parse_usize, |v| Operand::Const(v))
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(tuple((
        tag("  Operation: new = old "),
        parse_operator,
        space1,
        parse_operand
    )), |(_, op, _, rhs)| match (op, rhs) {
        (Operator::Multiply, Operand::Old) => Operation::Square,
        (Operator::Multiply, Operand::Const(n)) => Operation::MultiplyBy(n),
        (Operator::Plus, Operand::Const(n)) => Operation::IncrementBy(n),
        (Operator::Plus, Operand::Old) => Operation::MultiplyBy(2)
    })(input)
}

fn parse_throw_target(input: &str) -> IResult<&str, usize> {
    preceded(
        tuple((
            tag("    If "),
            alt((tag("true"), tag("false"))),
            tag(": throw to monkey "))
        ),
        parse_usize,
    )(input)
}

fn parse_inventory(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tag("  Starting items: "),
        separated_list0(tag(", "), parse_usize),
    )(input)
}

fn parse_test(input: &str) -> IResult<&str, usize> {
    preceded(tag("  Test: divisible by "), parse_usize)(input)
}

fn parse_header(input: &str) -> IResult<&str, usize> {
    map(tuple((tag("Monkey "), parse_usize, tag(":"))), |tpl| tpl.1)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            parse_header,
            line_ending,
            parse_inventory,
            line_ending,
            parse_operation,
            line_ending,
            parse_test,
            line_ending,
            parse_throw_target,
            line_ending,
            parse_throw_target,
            line_ending)
        ),
        | (number, _, inventory, _, operation, _, module, _, goto_on_zero, _, goto_on_nonzero, _) | Monkey {
            number,
            operation,
            module,
            goto_on_zero,
            goto_on_nonzero,
            inventory,
            inspections_performed: 0
        }
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(line_ending,parse_monkey)(input)
}

fn solve_problem(mut source: Vec<Monkey>) -> (i64, i64){
    let great_monkey_modulus = source.iter().fold(1, |lcm, monkey|{
        num::integer::lcm(lcm, monkey.module)
    });

    let mut part1 = source.clone();
    let monkeys = &mut part1;
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            Monkey::distribute_inventory(monkeys, i, great_monkey_modulus, |x| x / 3);
        }
    }

    monkeys.sort_by_key(|m|-m.inspections_performed);
    let part1 = monkeys[0].inspections_performed * monkeys[1].inspections_performed;


    let monkeys = &mut source;
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            Monkey::distribute_inventory(monkeys, i, great_monkey_modulus, |x|x);
        }
    }

    monkeys.sort_by_key(|m|-m.inspections_performed);
    let part2 = monkeys[0].inspections_performed * monkeys[1].inspections_performed;
    (part1, part2)
}

default_solution!(parse_input, solve_problem);
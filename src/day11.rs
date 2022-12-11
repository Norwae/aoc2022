use std::fmt::{Display, Formatter};
use std::mem::swap;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, one_of, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, terminated, tuple};

use crate::util::{default_solution, parse_i64, parse_usize};

#[derive(Debug, Copy, Clone)]
enum Operand {
    Old,
    Const(usize),
}

impl Operand {
    fn eval(self, old: usize) -> usize {
        match self {
            Operand::Old => old,
            Operand::Const(v) => v
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Plus,
    Multiply,
}

impl Operator {
    fn perform(&self, lhs: Operand, rhs: Operand, old: usize) -> usize {
        let lhs = lhs.eval(old);
        let rhs = rhs.eval(old);

        match self {
            Operator::Plus => lhs + rhs,
            Operator::Multiply => lhs * rhs
        }
    }
}

#[derive(Debug)]
struct Assignment {
    to: usize,
    items: Vec<usize>
}

impl Assignment {
    fn transfer_to(mut self, monkeys: &mut [Monkey]) {
        monkeys[self.to].inventory.append(&mut self.items);
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    number: usize,
    operator: Operator,
    left_operand: Operand,
    right_operand: Operand,
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
    fn distribute_inventory(&mut self, calmdown_factor: usize, great_monkey_modulus: usize) -> (Assignment, Assignment){
        let mut zero = Assignment {
            to: self.goto_on_zero,
            items: Vec::new()
        };
        let mut nonzero = Assignment {
            to: self.goto_on_nonzero,
            items: Vec::new()
        };
        let mut inventory = Vec::new();
        swap(&mut inventory, &mut self.inventory);

        for initial_worry_level in inventory {
            self.inspections_performed += 1;
            let after_inspect = self.operator.perform(self.left_operand, self.right_operand, initial_worry_level);
            let after_calm_down = after_inspect / calmdown_factor;

            let target = if after_calm_down % self.module == 0 {
                &mut zero.items
            } else {
                &mut nonzero.items
            };

            target.push(after_calm_down % great_monkey_modulus)
        }


        (zero, nonzero)
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

fn parse_operation(input: &str) -> IResult<&str, (Operand, Operator, Operand)> {
    map(tuple((
        tag("  Operation: new = "),
        parse_operand,
        space1,
        parse_operator,
        space1,
        parse_operand
    )), |(_, lhs, _, op, _, rhs)| (lhs, op, rhs))(input)
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
        | (number, _, inventory, _, (left_operand, operator, right_operand), _, module, _, goto_on_zero, _, goto_on_nonzero, _) | Monkey {
            number,
            operator,
            left_operand,
            right_operand,
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
            let (a1, a2) = monkeys[i].distribute_inventory(3, great_monkey_modulus);
            a1.transfer_to(monkeys);
            a2.transfer_to(monkeys);
        }
    }

    monkeys.sort_by_key(|m|-m.inspections_performed);
    let part1 = monkeys[0].inspections_performed * monkeys[1].inspections_performed;


    let monkeys = &mut source;
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let (a1, a2) = monkeys[i].distribute_inventory(1, great_monkey_modulus);
            a1.transfer_to(monkeys);
            a2.transfer_to(monkeys);
        }
    }

    monkeys.sort_by_key(|m|-m.inspections_performed);
    let part2 = monkeys[0].inspections_performed * monkeys[1].inspections_performed;
    (part1, part2)
}

default_solution!(parse_input, solve_problem);
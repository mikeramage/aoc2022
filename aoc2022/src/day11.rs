use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{vec_deque, HashMap, VecDeque},
    usize,
};

use crate::utils;

#[derive(Clone, Debug)]
enum Operator {
    PLUS,
    TIMES,
}

#[derive(Clone, Debug)]
enum Operand {
    OLD,
    NUMBER(usize),
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    operator: Operator,
    operand: Operand,
    test_divisibility_factor: usize,
    test_true_monkey: usize,
    test_false_monkey: usize,
    item_list: VecDeque<Item>,
    items_inspected: usize,
}

impl Monkey {
    pub fn new(
        id: usize,
        operator: Operator,
        operand: Operand,
        test_divisibility_factor: usize,
        test_true_monkey: usize,
        test_false_monkey: usize,
        item_list: VecDeque<Item>,
    ) -> Monkey {
        let monkey = Monkey {
            id,
            operator,
            operand,
            test_divisibility_factor,
            test_true_monkey,
            test_false_monkey,
            item_list,
            items_inspected: 0,
        };
        monkey
    }

    pub fn inspect_item(&mut self) {
        if !self.item_list.is_empty() {
            let mut item: Item = self.item_list.pop_front().unwrap();
            item.update_worry_level(&self.operator, &self.operand);
            self.item_list.push_front(item);
            self.items_inspected += 1;
        }
    }

    pub fn get_bored(&mut self, prime_factor_product: usize) {
        if !self.item_list.is_empty() {
            let mut item: Item = self.item_list.pop_front().unwrap();
            if prime_factor_product == 0 {
                item.relax_worry_level();
            } else {
                item.manage_worry_level(prime_factor_product);
            }
            self.item_list.push_front(item);
        }
    }

    pub fn give_item(&self, monkey: &mut Monkey, item: Item) {
        monkey.receive_item(item);
    }

    pub fn receive_item(&mut self, item: Item) {
        self.item_list.push_back(item);
    }

    pub fn test_item(&mut self, monkeys: &mut HashMap<usize, Monkey>) {
        if !self.item_list.is_empty() {
            let item: Item = self.item_list.pop_front().unwrap();
            let worry_level = item.get_worry_level();
            if worry_level % self.test_divisibility_factor == 0 {
                self.give_item(monkeys.get_mut(&self.test_true_monkey).unwrap(), item);
            } else {
                self.give_item(monkeys.get_mut(&self.test_false_monkey).unwrap(), item);
            }
        }
    }

    pub fn got_items(&self) -> bool {
        !self.item_list.is_empty()
    }

    pub fn items_inspected(&self) -> usize {
        self.items_inspected
    }
}

#[derive(Clone, Debug)]
struct Item {
    worry_level: usize,
}

impl Item {
    pub fn new(worry_level: usize) -> Item {
        let item = Item { worry_level };
        item
    }

    pub fn update_worry_level(&mut self, operator: &Operator, operand: &Operand) {
        match operator {
            Operator::PLUS => match operand {
                Operand::OLD => self.worry_level += self.worry_level,
                Operand::NUMBER(x) => self.worry_level += x,
            },
            Operator::TIMES => match operand {
                Operand::OLD => self.worry_level *= self.worry_level,
                Operand::NUMBER(x) => self.worry_level *= x,
            },
        }
    }

    pub fn relax_worry_level(&mut self) {
        self.worry_level /= 3;
    }

    pub fn manage_worry_level(&mut self, prime_factor_product: usize) {
        self.worry_level %= prime_factor_product;
    }

    pub fn get_worry_level(&self) -> usize {
        self.worry_level
    }
}

///Day 10 solution
pub fn day11() -> (usize, usize) {
    let monkey_config = utils::parse_input::<String>("input/day11.txt");
    let re_monkey_id = Regex::new(r"^Monkey (?P<id>\d+):$").unwrap();
    let re_items = Regex::new(r"^\s+Starting items: (?P<item_list>.+)$").unwrap();
    let re_operation =
        Regex::new(r"^\s+Operation: new = old (?P<operator>[\+\*]) (?P<operand>\d+|old)$").unwrap();
    let re_test = Regex::new(r"^\s+Test: divisible by (?P<divisibility_factor>\d+)$").unwrap();
    let re_true_monkey = Regex::new(r"^\s+If true: throw to monkey (?P<true_monkey>\d+)$").unwrap();
    let re_false_monkey =
        Regex::new(r"^\s+If false: throw to monkey (?P<false_monkey>\d+)$").unwrap();

    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();
    let mut items: VecDeque<Item> = VecDeque::new();
    let mut monkey_id: usize = 0;
    let mut operator: Operator = Operator::PLUS;
    let mut operand: Operand = Operand::OLD;
    let mut test_divisibility_factor: usize = 0;
    let mut test_true_monkey: usize = 0;
    let mut test_false_monkey: usize = 0;

    for line in monkey_config {
        if re_monkey_id.is_match(&line) {
            monkey_id = re_monkey_id
                .captures(&line)
                .unwrap()
                .name("id")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
        } else if re_items.is_match(&line) {
            let items_str = re_items
                .captures(&line)
                .unwrap()
                .name("item_list")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            for item in items_str {
                items.push_back(Item::new(item));
            }
        } else if re_operation.is_match(&line) {
            let operator_str = re_operation
                .captures(&line)
                .unwrap()
                .name("operator")
                .unwrap()
                .as_str();
            operator = match operator_str {
                "*" => Operator::TIMES,
                "+" => Operator::PLUS,
                _ => panic!("Unrecognised operator {}", operator_str),
            };
            let operand_str = re_operation
                .captures(&line)
                .unwrap()
                .name("operand")
                .unwrap()
                .as_str();
            operand = match operand_str {
                "old" => Operand::OLD,
                number => Operand::NUMBER(number.parse::<usize>().unwrap()),
                //Panics if can't parse into a usize
            }
        } else if re_test.is_match(&line) {
            test_divisibility_factor = re_test
                .captures(&line)
                .unwrap()
                .name("divisibility_factor")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
        } else if re_true_monkey.is_match(&line) {
            test_true_monkey = re_true_monkey
                .captures(&line)
                .unwrap()
                .name("true_monkey")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
        } else if re_false_monkey.is_match(&line) {
            test_false_monkey = re_false_monkey
                .captures(&line)
                .unwrap()
                .name("false_monkey")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
        } else if line.trim().is_empty() {
            //End of monkey
            monkeys.insert(
                monkey_id,
                Monkey::new(
                    monkey_id,
                    operator,
                    operand,
                    test_divisibility_factor,
                    test_true_monkey,
                    test_false_monkey,
                    items,
                ),
            );
            items = VecDeque::new();
            operator = Operator::PLUS;
            operand = Operand::OLD;
        }
    }

    //The last monkey's config is not followed by a blank line
    monkeys.insert(
        monkey_id,
        Monkey::new(
            monkey_id,
            operator,
            operand,
            test_divisibility_factor,
            test_true_monkey,
            test_false_monkey,
            items,
        ),
    );

    let num_rounds: usize = 20;
    let num_rounds2: usize = 10000;
    let mut monkeys2 = monkeys.clone();

    // Part 1
    for _round in 0..num_rounds {
        for ii in 0..monkeys.len() {
            let mut monkey = monkeys.remove(&ii).unwrap();
            while monkey.got_items() {
                monkey.inspect_item();
                monkey.get_bored(0);
                monkey.test_item(&mut monkeys);
            }
            monkeys.insert(ii, monkey);
        }
    }

    // Part 2
    let prime_factor_product = monkeys
        .values()
        .map(|x| x.test_divisibility_factor)
        .product();
    for round in 0..num_rounds2 {
        for ii in 0..monkeys2.len() {
            let mut monkey = monkeys2.remove(&ii).unwrap();
            while monkey.got_items() {
                monkey.inspect_item();
                monkey.get_bored(prime_factor_product);
                monkey.test_item(&mut monkeys2);
            }
            monkeys2.insert(ii, monkey);
        }
    }

    let part1: usize = monkeys
        .values()
        .map(|x| x.items_inspected())
        .sorted()
        .rev()
        .chunks(2)
        .into_iter()
        .next()
        .unwrap()
        .product();

    let part2: usize = monkeys2
        .values()
        .map(|x| x.items_inspected())
        .sorted()
        .rev()
        .chunks(2)
        .into_iter()
        .next()
        .unwrap()
        .product();

    (part1, part2)
}

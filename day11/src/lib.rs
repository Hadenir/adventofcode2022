use std::{cmp::Ordering, fmt::Debug};

pub mod parser;

pub fn solve_part_1(mut monkeys: Vec<Monkey>) -> u64 {
    let mut business = vec![0; monkeys.len()];

    for _ in 0..20 {
        for turn in 0..monkeys.len() {
            let (prev_monkeys, next_monkeys) = monkeys.split_at_mut(turn);
            let (curr_monkey, next_monkeys) = next_monkeys.split_at_mut(1);
            let monkey = &mut curr_monkey[0];

            for mut item in monkey.items.drain(..) {
                item.worry = (monkey.operation)(item.worry) / 3;

                let target = (monkey.test)(&item.worry);
                match target.cmp(&turn) {
                    Ordering::Less => prev_monkeys[target].items.push(item),
                    Ordering::Greater => next_monkeys[target - turn - 1].items.push(item),
                    _ => panic!("Monkey cannot throw item at itself"),
                }

                business[turn] += 1;
            }
        }
    }

    business.sort_by(|a, b| b.cmp(a));
    business[0] * business[1]
}

pub fn solve_part_2((scm, mut monkeys): (Worry, Vec<Monkey>)) -> u64 {
    let mut business = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for turn in 0..monkeys.len() {
            let (prev_monkeys, next_monkeys) = monkeys.split_at_mut(turn);
            let (curr_monkey, next_monkeys) = next_monkeys.split_at_mut(1);
            let monkey = &mut curr_monkey[0];

            for mut item in monkey.items.drain(..) {
                item.worry = (monkey.operation)(item.worry) % scm;

                let target = (monkey.test)(&item.worry);
                match target.cmp(&turn) {
                    Ordering::Less => prev_monkeys[target].items.push(item),
                    Ordering::Greater => next_monkeys[target - turn - 1].items.push(item),
                    _ => panic!("Monkey cannot throw item at itself"),
                }

                business[turn] += 1;
            }
        }
    }

    business.sort_by(|a, b| b.cmp(a));
    business[0] * business[1]
}

type Worry = u64;
type OperationFn = Box<dyn Fn(Worry) -> Worry>;
type TestFn = Box<dyn Fn(&Worry) -> usize>;

#[derive(PartialEq, Eq, Clone)]
struct Item {
    worry: Worry,
}

pub struct Monkey {
    items: Vec<Item>,
    operation: OperationFn,
    test: TestFn,
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Item").field(&self.worry).finish()
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish_non_exhaustive()
    }
}

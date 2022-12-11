mod parser;
use nom::{bytes::complete::tag, multi::separated_list0};
use parser::{Conditional, Op, OptionProd, Test};

fn main() {
    let input = include_str!("real.text");
    let (_, mut monkeys) = separated_list0(tag("\r\n\r\n"), parser::parse_monkey)(input).unwrap();
    let mut count_inspections = vec![0; monkeys.len()];
    let mut common_denom = 1;
    for monkey in &monkeys {
        common_denom *= monkey.test.unwrap();
    }

    // println!("{:?}", monkeys[0]);
    for _ in 0..10000 {
        for ind in 0..monkeys.len() {
            let mut move_ind = Vec::new();
            let mut new_values = Vec::new();
            count_inspections[ind] += monkeys[ind].starting_items.len();
            for item in monkeys[ind].starting_items.iter() {
                let mut new_item = *item;
                match &monkeys[ind].operation {
                    Op::Add(n) => new_item += n,
                    Op::Prod(opt) => match opt {
                        OptionProd::Old => new_item *= new_item,
                        OptionProd::Num(n) => new_item *= *n,
                    },
                }
                // new_item /= 3;
                let Test::Div(div) = &monkeys[ind].test;
                let mut id = 0;
                if new_item % div == 0 {
                    match &monkeys[ind].cond_true {
                        Conditional::True(n) => id = *n,
                        Conditional::False(_) => (),
                    }
                } else {
                    match &monkeys[ind].cond_false {
                        Conditional::True(_) => (),
                        Conditional::False(n) => id = *n,
                    }
                }
                new_item %= common_denom;
                move_ind.push(id);
                new_values.push(new_item);
            }
            monkeys[ind].starting_items.clear();
            while !move_ind.is_empty() {
                let monkey_id = move_ind.pop().unwrap();
                let item = new_values.pop().unwrap();
                monkeys[monkey_id as usize].starting_items.push(item);
            }
        }
    }

    // println!("{:?}", monkeys[0]);
    // Monkey 0: 20, 23, 27, 26
    // println!("{}", count_inspections[0]);
    // println!("{}", count_inspections[1]);
    // println!("{}", count_inspections[2]);
    // println!("{}", count_inspections[3]);
    count_inspections.sort();
    count_inspections.reverse();
    println!(
        "Monkey Business: {}",
        count_inspections[0] * count_inspections[1]
    );
}


use std::env;
use std::process::exit;
use rand::{Rng, thread_rng};



struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {

        let mut winning_numbers: Vec<usize> = Vec::with_capacity(take);
        let mut rng = thread_rng();
        let mut i = 0;
        while i < take {
            let num = rng.gen_range(0..=from);
            if winning_numbers.is_empty() {
                winning_numbers.push(num);
                i = i +1;
            }
            else if !winning_numbers.contains(&num) {
                winning_numbers.push(num);
                i = i + 1;
            }
        }

        Lotto {
            take,
            from,
            numbers: winning_numbers,
        }
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    format!("{} of {}: {:?}", lotto.take, lotto.from, lotto.numbers)
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("To less arguments provided!");
        exit(1);
    }
    else if args.len() == 3 {
        let take: usize = args[1].parse().expect("Could not parse the takes!");
        let from: usize = args[2].parse().expect("Could not parse the range (out of X)!");

        if take > from {
            println!("The pool ({}) must be grater than the numbers ({}) you like to choose!", from, take);
            exit(1);
        }

        let lotto = Lotto::new(take, from);
        println!("{}", format_lotto_results(&lotto));
        exit(0);
    }
    else if (args.len() - 1) % 2 == 0 {
        let mut runs = (args.len() - 1) / 2;
        let mut i = 1;

        while runs != 0 {
            let take: usize = args[i].parse().expect("Could not parse the takes!");
            i = i + 1;
            let from: usize = args[i].parse().expect("Could not parse the range (out of X)!");
            i = i + 1;

            if take > from {
                println!("The pool ({}) must be grater than the numbers ({}) you like to choose!", from, take);
                exit(1);
            }

            let lotto = Lotto::new(take, from);

            println!("{} ", format_lotto_results(&lotto));
            runs = runs - 1;

        }
    }

    // let take: usize = args[1].parse().expect("Could not parse the takes!");
    // let from: usize = args[2].parse().expect("Could not parse the range (out of X)!");
    //
    // let lotto = Lotto::new(take, from);
    //
    // println!("{}", format_lotto_results(&lotto));
    exit(0);
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.numbers;

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.numbers;
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}

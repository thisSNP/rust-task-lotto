
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
    todo!("Implement CLI")
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

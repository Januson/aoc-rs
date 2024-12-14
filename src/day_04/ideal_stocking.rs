use std::ops::Range;
use std::thread;

struct AdventCoinMiner {
    secret_key: String,
}

impl AdventCoinMiner {
    fn new(secret_key: &str) -> Self {
        AdventCoinMiner {
            secret_key: secret_key.to_string(),
        }
    }

    fn mine(&self, difficulty: usize) -> u32 {
        let target = "0".repeat(difficulty);
        let scale = 10u32.pow(difficulty as u32);

        thread::scope(|scope| {
            let mut threads = vec![];

            for i in 0..10 {
                let target = &target;
                let range = (i * scale)..((i + 1) * scale);
                threads.push(scope.spawn(move || {
                    self.mine_coin(target, range)
                }));
            }

            threads.into_iter()
                .map(|child| child.join())
                .filter_map(|x| x.ok())
                .filter_map(|x| x)
                .min()
                .expect("No solution found")
        })
    }

    fn mine_coin(&self, target: &str, range: Range<u32>) -> Option<u32> {
        for i in range {
            let coin = format!("{}{}", self.secret_key, i);
            let hash = hash(coin);
            if hash.starts_with(&target) {
                return Some(i);
            }
        }
        None
    }
}

fn hash(input: String) -> String {
    format!("{:x}", md5::compute(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_04/input.txt");

        let miner = AdventCoinMiner::new(input);

        assert_eq!(miner.mine(5), 346386);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/year_2015/day_04/input.txt");

        let miner = AdventCoinMiner::new(input);

        assert_eq!(miner.mine(6), 9958218);
    }

    #[test]
    fn test_mining() {
        let miner = AdventCoinMiner::new("abcdef");
        assert_eq!(miner.mine(5), 609043);
    }
}

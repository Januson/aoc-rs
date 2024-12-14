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
        for i in 0.. {
            let coin = format!("{}{}", self.secret_key, i);
            let digest = md5::compute(coin);
            let hash = format!("{:x}", digest);
            if hash.starts_with(&target) {
                return i;
            }
        }
        panic!("No solution found");
    }
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
    fn test_mining() {
        let miner = AdventCoinMiner::new("abcdef");
        assert_eq!(miner.mine(5), 609043);
    }
}

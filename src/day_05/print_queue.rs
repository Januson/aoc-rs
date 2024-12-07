use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct OrderingRule(u8, u8);

impl FromStr for OrderingRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('|');
        let first = parts.next().unwrap().parse().unwrap();
        let second = parts.next().unwrap().parse().unwrap();

        Ok(OrderingRule(first, second))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Update(Vec<u8>);

impl FromStr for Update {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(',')
            .map(|s| s.parse().expect(format!("Failed to parse number: {}", s).as_str()))
            .collect();

        Ok(Update(values))
    }
}

impl Update {
    fn meets(&self, ordering: &OrderingRule) -> bool {
        if !self.0.contains(&ordering.0) || !self.0.contains(&ordering.1) {
            return true;
        }
        let first = self.0.iter().position(|&r| r == ordering.0).unwrap();
        let second = self.0.iter().position(|&r| r == ordering.1).unwrap();

        first < second
    }

    fn middle(&self) -> u8 {
        let length = self.0.len() / 2;
        *self.0.iter().nth(length).unwrap()
    }

    fn fixed(&self, ordering: &HashMap<u8, Vec<u8>>) -> Update {
        let mut pages = self.0.clone();
        let default = vec![];
        pages.sort_by(|a, b| {
            let order_a = ordering.get(&a).unwrap_or(&default);
            let order_b = ordering.get(&b).unwrap_or(&default);
            if order_a.contains(b) {
                Ordering::Less
            } else if order_b.contains(a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        Update(pages)
    }
}

struct PrintQueueUpdate {
    ordering: Vec<OrderingRule>,
    updates: Vec<Update>,
}

impl FromStr for PrintQueueUpdate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, "\n\n");
        let ordering = parts.next().unwrap().lines()
            .map(|s| s.parse().unwrap())
            .collect();
        let updates = parts.next().unwrap().lines()
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(PrintQueueUpdate {
            ordering,
            updates,
        })
    }
}

impl PrintQueueUpdate {
    fn correct_updates(&self) -> Vec<Update> {
        let mut correct_updates = vec![];

        for update in &self.updates {
            let meets_all_rules = &self.ordering.iter().all(|rule| update.meets(rule));
            if *meets_all_rules {
                correct_updates.push((*update).clone());
            }
        }

        correct_updates
    }

    fn corrupted_updates(&self) -> Vec<Update> {
        let mut correct_updates = vec![];

        for update in self.updates.clone() {
            let meets_all_rules = &self.ordering.iter().all(|rule| update.meets(rule));
            if !*meets_all_rules {
                correct_updates.push(update);
            }
        }

        correct_updates
    }

    fn checksum(&self) -> u64 {
        let updates = self.correct_updates();
        Self::calculate_checksum(&updates)
    }

    fn calculate_checksum(updates: &[Update]) -> u64 {
        updates.iter()
            .map(|x| x.middle() as u64)
            .sum()
    }

    fn fixed_checksum(&self) -> u64 {
        let mut ordering: HashMap<u8, Vec<u8>> = HashMap::new();
        for order in &self.ordering {
            ordering.entry(order.0).or_insert(vec![]).push(order.1);
        }

        let corrupted = self.corrupted_updates();
        let fixed = corrupted.iter()
            .map(|update| update.fixed(&ordering))
            .collect::<Vec<Update>>();
        Self::calculate_checksum(&fixed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/day_05/input.txt");
        let update = PrintQueueUpdate::from_str(input).unwrap();

        assert_eq!(update.checksum(), 5091);
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../../input/day_05/input.txt");
        let update = PrintQueueUpdate::from_str(input).unwrap();

        assert_eq!(update.fixed_checksum(), 4681);
    }

    #[test]
    fn test_ordering_parsing() {
        let rule = "47|53".parse::<OrderingRule>().unwrap();

        assert_eq!(rule.0, 47);
        assert_eq!(rule.1, 53);
    }

    #[test]
    fn test_update_pages_parsing() {
        let rule = "75,47,61,53,29".parse::<Update>().unwrap();

        assert_eq!(rule.0, vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_queue_update_parsing() {
        let rule = "47|53\n\n75,47,61,53,29".parse::<PrintQueueUpdate>().unwrap();

        assert_eq!(rule.ordering, vec![OrderingRule(47, 53)]);
        assert_eq!(rule.updates, vec![Update(vec![75, 47, 61, 53, 29])]);
    }

    #[test]
    fn test_update_fixing() {
        let queue_update = INPUT.parse::<PrintQueueUpdate>().unwrap();
        let update = Update(vec![75, 97, 47, 61, 53]);

        let mut ordering: HashMap<u8, Vec<u8>> = HashMap::new();
        for order in queue_update.ordering {
            ordering.entry(order.0).or_insert(vec![]).push(order.1);
        }

        let fixed = update.fixed(&ordering);

        assert_eq!(fixed.0, vec![97, 75, 47, 61, 53]);
    }

    #[test]
    fn test_full() {
        let update = INPUT.parse::<PrintQueueUpdate>().unwrap();

        let correct = update.correct_updates();
        assert_eq!(correct, vec![
            Update(vec![75, 47, 61, 53, 29]),
            Update(vec![97, 61, 53, 29, 13]),
            Update(vec![75, 29, 13]),
        ]);

        let result: u64 = update.checksum();
        assert_eq!(result, 143);

        let result: u64 = update.fixed_checksum();
        assert_eq!(result, 123);
    }

    const INPUT: &str = "\
            47|53\n\
            97|13\n\
            97|61\n\
            97|47\n\
            75|29\n\
            61|13\n\
            75|53\n\
            29|13\n\
            97|29\n\
            53|29\n\
            61|53\n\
            97|53\n\
            61|29\n\
            47|13\n\
            75|47\n\
            97|75\n\
            47|61\n\
            75|61\n\
            47|29\n\
            75|13\n\
            53|13\n\
            \n\
            75,47,61,53,29\n\
            97,61,53,29,13\n\
            75,29,13\n\
            75,97,47,61,53\n\
            61,13,29\n\
            97,13,75,29,47\n\
        ";
}

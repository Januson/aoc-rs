struct Program {
    memory: Memory,
}

struct Memory {
    memory: Vec<i32>,
}

impl Memory {
    fn new(data: &str) -> Memory {
        Memory {
            memory: data.split(',').into_iter()
                .map(|n| n.parse().expect("An integer was expected!"))
                .collect()
        }
    }

    fn read(&self, index: i32) -> i32 {
        self.memory[index as usize]
    }

    fn insert(&mut self, index: i32, value: i32) {
        self.memory[index as usize] = value;
    }
}

impl Program {
    fn new(memory: Memory) -> Program {
        Program {
            memory,
        }
    }

    fn run(&mut self) {
        let mut i = 0;
        loop {
            let op = self.memory.read(i);
            match op {
                1 => {
                    let x = self.memory.read(self.memory.read(i + 1));
                    let y = self.memory.read(self.memory.read(i + 2));
                    let z = self.memory.read(i + 3);
                    self.memory.insert(z, x + y);
                }
                2 => {
                    let x = self.memory.read(self.memory.read(i + 1));
                    let y = self.memory.read(self.memory.read(i + 2));
                    let z = self.memory.read(i + 3);
                    self.memory.insert(z, x * y);
                }
                99 => break,
                _ => panic!("Error: {}!", op),
            }
            i += 4;
        }
    }

    fn state(&self) -> &Vec<i32> {
        &self.memory.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        let mut program = Program::new(Memory::new("1,0,0,0,99"));

        program.run();

        assert_eq!(program.state(), &vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn simple_multiplication() {
        let mut program = Program::new(Memory::new("2,3,0,3,99"));

        program.run();

        assert_eq!(program.state(), &vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn store_result_after_ending_optcode() {
        let mut program = Program::new(Memory::new("2,4,4,5,99,0"));

        program.run();

        assert_eq!(program.state(), &vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn recalculate_ending_optcode() {
        let mut program = Program::new(Memory::new("1,1,1,4,99,5,6,0,99"));

        program.run();

        assert_eq!(program.state(), &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn first_part() {
        let input = include_str!("../../inputs/year_2019/day_02/input.txt");
        let mut memory = Memory::new(input);
        memory.insert(1, 12);
        memory.insert(2, 2);
        let mut program = Program::new(memory);

        program.run();

        assert_eq!(program.state()[0], 5098658);
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../inputs/year_2019/day_02/input.txt");
        let mut memory = Memory::new(input);
        memory.insert(1, 50);
        memory.insert(2, 64);
        let mut program = Program::new(memory);

        program.run();

        let state = program.state();
        assert_eq!(state[0], 19690720);
        assert_eq!(100 * state[1] + state[2], 5064);
    }
}
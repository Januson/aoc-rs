use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Signal(u16);

impl From<u16> for Signal {
    fn from(value: u16) -> Self {
        Signal(value)
    }
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Signal(s.parse::<u16>().expect(format!("Expected a number, got: {}", s).as_str())))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Wire(u16);

impl Wire {
    fn new(s: &str) -> Wire {
        let bytes = s.as_bytes();
        if bytes.len() > 2 || bytes.iter().any(|&b| b > 127) {
            panic!("Supports only up to two ASCII characters, but was: {}", s);
        }
        let mut packed = 0u16;
        for &b in bytes {
            packed = (packed << 8) | (b as u16);
        }
        Self(packed)
    }

    fn as_str(&self) -> String {
        let bytes = [
            ((self.0 >> 8) & 0xFF) as u8,
            (self.0 & 0xFF) as u8,
        ];
        String::from_utf8(bytes.iter().filter(|&&b| b != 0).copied().collect()).unwrap()
    }
}

impl Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
struct Circuit {
    connections: HashMap<Wire, Connection>,
}

impl Circuit {
    fn connection(&self, wire: &Wire) -> &Connection {
        self.connections.get(wire).expect(format!("Wire not found: {:?}", wire).as_str())
    }

    fn from_str(s: &str) -> Result<Self, ()> {
        let connections = s.lines()
            .map(|line| Connection::from_str(line).expect("Invalid instruction"))
            .map(|instruction| (instruction.target, instruction))
            .collect();

        Ok(Circuit { connections })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LogicGate {
    Input(String),
    And(String, String),
    Or(String, String),
    LShift(String, u8),
    RShift(String, u8),
    Not(String),
}

#[derive(Debug, PartialEq, Eq)]
struct Connection {
    gate: LogicGate,
    target: Wire,
}

impl Connection {
    fn new(gate: LogicGate, target: Wire) -> Self {
        Connection {
            gate,
            target,
        }
    }

    fn from_str(s: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let target = Wire::new(parts[1]);

        let operation = match parts[0].split(' ').collect::<Vec<&str>>().as_slice() {
            [a] => LogicGate::Input(a.to_string()),
            [a, "AND", b] => LogicGate::And(a.to_string(), b.to_string()),
            [a, "OR", b] => LogicGate::Or(a.to_string(), b.to_string()),
            [a, "LSHIFT", b] => LogicGate::LShift(a.to_string(), b.parse().expect("Expected a number")),
            [a, "RSHIFT", b] => LogicGate::RShift(a.to_string(), b.parse().expect("Expected a number")),
            ["NOT", a] => LogicGate::Not(a.to_string()),
            _ => panic!("Invalid instruction: {}", s),
        };

        Ok(Connection::new(operation, target))
    }
}

struct BobbyTables {
    circuit: Circuit,
    cache: HashMap<Wire, u16>,
}

impl BobbyTables {
    fn new(circuit: Circuit) -> Self {
        BobbyTables {
            circuit,
            cache: HashMap::new(),
        }
    }

    fn signal_on(&mut self, wire: Wire) -> u16 {
        BobbyTables::signal(wire, &self.circuit, &mut self.cache)
    }

    fn signal(
        wire: Wire,
        circuit: &Circuit,
        cache: &mut HashMap<Wire, u16>,
    ) -> u16 {
        if let Some(result) = cache.get(&wire) {
            return *result;
        }

        let connection = circuit.connection(&wire);
        let signal = match &connection.gate {
            LogicGate::Input(value) => {
                value.parse::<u16>().unwrap_or_else(|_| {
                    BobbyTables::signal(Wire::new(value), circuit, cache)
                })
            },
            LogicGate::Not(value) => {
                let signal = value.parse::<u16>().unwrap_or_else(|_| {
                    BobbyTables::signal(Wire::new(value), circuit, cache)
                });
                !signal
            }
            LogicGate::And(a, b) => {
                let signal_a = a.parse::<u16>().unwrap_or_else(|_| {
                    BobbyTables::signal(Wire::new(a), circuit, cache)
                });
                let signal_b = b.parse::<u16>().unwrap_or_else(|_| {
                    BobbyTables::signal(Wire::new(b), circuit, cache)
                });
                signal_a & signal_b
            },
            LogicGate::Or(a, b) => {
                let signal_a = a.parse::<u16>().unwrap_or_else(|_| {
                    BobbyTables::signal(Wire::new(a), circuit, cache)
                });
                let signal_b = b.parse::<u16>().unwrap_or_else(|_| {
                    BobbyTables::signal(Wire::new(b), circuit, cache)
                });
                signal_a | signal_b
            },
            LogicGate::LShift(value, n) => {
                let signal = value.parse::<u16>().unwrap_or_else(|_| {
                    BobbyTables::signal(Wire::new(value), circuit, cache)
                });
                signal << n
            },
            LogicGate::RShift(value, n) => {
                let signal = value.parse::<u16>().unwrap_or_else(|_| {
                    BobbyTables::signal(Wire::new(value), circuit, cache)
                });
                signal >> n
            },
        };

        cache.insert(wire, signal);
        signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1() {
        let input = include_str!("../../input/year_2015/day_07/input.txt");

        let circuit = Circuit::from_str(input).unwrap();
        let mut bobby_tables = BobbyTables::new(circuit);

        assert_eq!(bobby_tables.signal_on(Wire::new("a")), 3176);
    }

    static SAMPLE: &str = "\
        123 -> x\n\
        456 -> y\n\
        x AND y -> d\n\
        x OR y -> e\n\
        x LSHIFT 2 -> f\n\
        y RSHIFT 2 -> g\n\
        NOT x -> h\n\
        NOT y -> i\n\
        ";

    #[test]
    fn test_example() {
        let circuit = Circuit::from_str(SAMPLE).unwrap();
        let mut bobby_tables = BobbyTables::new(circuit);

        assert_eq!(bobby_tables.signal_on(Wire::new("d")), 72);
        assert_eq!(bobby_tables.signal_on(Wire::new("e")), 507);
        assert_eq!(bobby_tables.signal_on(Wire::new("f")), 492);
        assert_eq!(bobby_tables.signal_on(Wire::new("g")), 114);
        assert_eq!(bobby_tables.signal_on(Wire::new("h")), 65412);
        assert_eq!(bobby_tables.signal_on(Wire::new("i")), 65079);
        assert_eq!(bobby_tables.signal_on(Wire::new("x")), 123);
        assert_eq!(bobby_tables.signal_on(Wire::new("y")), 456);
    }

    #[test]
    fn test_parsing_connections_send() {
        let input = "123 -> x";

        let connection = Connection::from_str(input).unwrap();

        assert_eq!(connection, Connection {
            gate: LogicGate::Input("123".to_string()),
            target: Wire::new("x"),
        });
    }

    #[test]
    fn test_parsing_connections_and() {
        let input = "a AND b -> x";

        let connection = Connection::from_str(input).unwrap();

        assert_eq!(connection, Connection {
            gate: LogicGate::And("a".to_string(), "b".to_string()),
            target: Wire::new("x"),
        });
    }

    #[test]
    fn test_parsing_connections_or() {
        let input = "a OR c -> x";

        let connection = Connection::from_str(input).unwrap();

        assert_eq!(connection, Connection {
            gate: LogicGate::Or("a".to_string(), "c".to_string()),
            target: Wire::new("x"),
        });
    }

    #[test]
    fn test_parsing_connections_not() {
        let input = "NOT c -> x";

        let connection = Connection::from_str(input).unwrap();

        assert_eq!(connection, Connection {
            gate: LogicGate::Not("c".to_string()),
            target: Wire::new("x"),
        });
    }

    #[test]
    fn test_parsing_connections_lshift() {
        let input = "a LSHIFT 2 -> x";

        let connection = Connection::from_str(input).unwrap();

        assert_eq!(connection, Connection {
            gate: LogicGate::LShift("a".to_string(), 2),
            target: Wire::new("x"),
        });
    }

    #[test]
    fn test_parsing_connections_rshift() {
        let input = "a RSHIFT 2 -> x";

        let connection = Connection::from_str(input).unwrap();

        assert_eq!(connection, Connection {
            gate: LogicGate::RShift("a".to_string(), 2),
            target: Wire::new("x"),
        });
    }

    #[test]
    fn test_send_signal() {
        let circuit = Circuit::from_str(SAMPLE).unwrap();
        let mut bobby_tables = BobbyTables::new(circuit);

        assert_eq!(bobby_tables.signal_on(Wire::new("x")), 123);
        assert_eq!(bobby_tables.signal_on(Wire::new("y")), 456);
    }

    #[test]
    fn test_and_signal() {
        let circuit = Circuit::from_str(SAMPLE).unwrap();
        let mut bobby_tables = BobbyTables::new(circuit);

        assert_eq!(bobby_tables.signal_on(Wire::new("d")), 72);
    }

    #[test]
    fn test_or_signal() {
        let circuit = Circuit::from_str(SAMPLE).unwrap();
        let mut bobby_tables = BobbyTables::new(circuit);

        assert_eq!(bobby_tables.signal_on(Wire::new("e")), 507);
    }

    #[test]
    fn test_not_signal() {
        let circuit = Circuit::from_str(SAMPLE).unwrap();
        let mut bobby_tables = BobbyTables::new(circuit);

        assert_eq!(bobby_tables.signal_on(Wire::new("h")), 65412);
        assert_eq!(bobby_tables.signal_on(Wire::new("i")), 65079);
    }

    #[test]
    fn test_lshift_signal() {
        let circuit = Circuit::from_str(SAMPLE).unwrap();
        let mut bobby_tables = BobbyTables::new(circuit);

        assert_eq!(bobby_tables.signal_on(Wire::new("f")), 492);
    }

    #[test]
    fn test_rshift_signal() {
        let circuit = Circuit::from_str(SAMPLE).unwrap();
        let mut bobby_tables = BobbyTables::new(circuit);

        assert_eq!(bobby_tables.signal_on(Wire::new("g")), 114);
    }
}

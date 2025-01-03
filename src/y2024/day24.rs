use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug, PartialEq, Eq)]
struct Connection<'a> {
    gate: Gate,
    wire_1: &'a str,
    wire_2: &'a str,
    output_wire: &'a str,
}

impl<'a> Connection<'a> {
    fn parse(line: &'a str) -> Self {
        let (gate_part, output_wire) = line.split_once(" -> ").unwrap();
        let mut parts = gate_part.split_whitespace();
        let wire_1 = parts.next().unwrap();
        let gate_type = parts.next().unwrap();
        let wire_2 = parts.next().unwrap();

        let gate = match gate_type {
            "AND" => Gate::AND,
            "OR" => Gate::OR,
            "XOR" => Gate::XOR,
            _ => panic!("Unknown gate type"),
        };

        Connection {
            gate,
            wire_1,
            wire_2,
            output_wire,
        }
    }

    #[inline]
    fn is_direct(&self) -> bool {
        self.wire_1.starts_with('x') || self.wire_2.starts_with('x')
    }

    #[inline]
    fn is_output(&self) -> bool {
        self.output_wire.starts_with('z')
    }

    #[inline]
    fn has_input(&self, input: &str) -> bool {
        self.wire_1 == input || self.wire_2 == input
    }

    #[inline]
    fn has_output(&self, output: &str) -> bool {
        self.output_wire == output
    }
}

fn parse_circuit(input: &str) -> (HashMap<&str, i32>, Vec<Connection>, usize) {
    let mut wires = HashMap::new();
    let mut gates = Vec::new();
    let mut max_z_wire = 0;

    let (init_values, gate_defs) = input.split_once("\n\n").unwrap();

    for line in init_values.lines() {
        let (wire, value) = line.split_once(": ").unwrap();
        wires.insert(wire, value.parse().unwrap());
    }

    for line in gate_defs.lines() {
        let connection = Connection::parse(line);
        if connection.output_wire.starts_with('z') {
            if let Ok(num) = connection.output_wire[1..].parse::<usize>() {
                max_z_wire = max_z_wire.max(num);
            }
        }
        gates.push(connection);
    }

    (wires, gates, max_z_wire)
}

fn simulate_circuit<'a>(wires: &mut HashMap<&'a str, i32>, gates: &[Connection<'a>]) {
    let mut changed = true;
    while changed {
        changed = false;
        for gate in gates {
            if wires.contains_key(gate.output_wire) {
                continue;
            }

            if let (Some(&input1), Some(&input2)) = (wires.get(gate.wire_1), wires.get(gate.wire_2)) {
                let result = match gate.gate {
                    Gate::AND => input1 & input2,
                    Gate::OR => input1 | input2,
                    Gate::XOR => input1 ^ input2,
                };
                wires.insert(gate.output_wire, result);
                changed = true;
            }
        }
    }
}

fn combine_z_wires(wires: &HashMap<&str, i32>, max_z_wire: usize) -> i64 {
    let mut result = 0i64;
    for i in (0..=max_z_wire).rev() {
        if let Some(&value) = wires.get(&format!("z{:02}", i).as_str()) {
            result = (result << 1) | (value as i64);
        }
    }
    result
}

pub fn part1(input: &str) -> i64 {
    let (mut wires, gates, max_z_wire) = parse_circuit(input);
    simulate_circuit(&mut wires, &gates);
    combine_z_wires(&wires, max_z_wire)
}

pub fn part2(input: &str) -> String {
    let (init_wires, gates, max_z) = parse_circuit(input);
    let mut suspicious_outputs = HashSet::new();

    /*
    * FULL ADDER
    * (first bits aren't a full adder)
    * (for last FA, COUT is the extra output)
    *
    * A    XOR B    -> VAL0     <= FAGate0
    * A    AND B    -> VAL1     <= FAGate1
    * VAL0 AND CIN  -> VAL2     <= FAGate2
    * VAL0 XOR CIN  -> SUM      <= FAGate3
    * VAL1 OR  VAL2 -> COUT     <= FAGate4
    */

    //check FAGate0 gates for zXXs
    //each of these should be a An XOR Bn -> VAL0n
    //except for the first one, which should be x00 XOR y00 -> z00
    let fa_gate0s: Vec<_> = gates.iter().filter(|g| g.is_direct() && g.gate == Gate::XOR).collect();
    for g in &fa_gate0s {
        if g.has_input("x00") {
            if !g.has_output("z00") {
                suspicious_outputs.insert(g.output_wire);
            }
            continue;
        } else if g.has_output("z00") {
            suspicious_outputs.insert(g.output_wire);
        }

        if g.is_output() {
            suspicious_outputs.insert(g.output_wire);
        }
    }

    //check all XOR gates that are indirect (FAGate3)
    //each of these should be outputting to a zXX
    let fa_gate3s: Vec<_> = gates.iter().filter(|g| !g.is_direct() && g.gate == Gate::XOR).collect();
    for g in &fa_gate3s {
        if !g.is_output() {
            suspicious_outputs.insert(g.output_wire);
        }
    }

    //check all output gates
    //each of these should be VAL0 XOR CIN -> SUM
    //except for the last one, which should be VAL1 OR VAL2 -> COUT
    let output_gates = gates.iter().filter(|g| g.is_output());
    for g in output_gates {
        if g.output_wire == format!("z{:02}", max_z) {
            if g.gate != Gate::OR {
                suspicious_outputs.insert(g.output_wire);
            }
            continue;
        } else if g.gate != Gate::XOR {
            suspicious_outputs.insert(g.output_wire);
        }
    }

    // All FAGate0 gates MUST output to a FAGate3 gate
    let mut check_next = Vec::new();
    for gate in fa_gate0s {
        if suspicious_outputs.contains(gate.output_wire) {
            continue;
        }
        if gate.output_wire == "z00" {
            continue;
        }

        let matches: Vec<_> = fa_gate3s.iter()
            .filter(|g| g.has_input(gate.output_wire))
            .collect();
        
        if matches.is_empty() {
            check_next.push(gate);
            suspicious_outputs.insert(gate.output_wire);
        }
    }

    // Check what the flagged gates should be
    for gate in check_next {
        let intended_result = format!("z{}", &gate.wire_1[1..]);
        
        let matches: Vec<_> = fa_gate3s.iter()
            .filter(|g| g.has_output(&intended_result))
            .collect();

        if matches.len() != 1 {
            panic!("Critical Error! Is your input correct?");
        }

        let matched_gate = matches[0];
        let to_check = [matched_gate.wire_1, matched_gate.wire_2];

        // One of these should come from an OR gate
        let or_matches: Vec<_> = gates.iter()
            .filter(|g| g.gate == Gate::OR && to_check.contains(&g.output_wire))
            .collect();

        if or_matches.len() != 1 {
            panic!("Critical Error! This solver isn't complex enough");
        }

        let or_match_output = or_matches[0].output_wire;
        
        // The correct output is the one that isn't or_match_output
        let correct_output = to_check.iter()
            .find(|&&output| output != or_match_output)
            .unwrap();

        suspicious_outputs.insert(correct_output);
    }

    if suspicious_outputs.len() != 8 {
        panic!("Critical Error! This solver isn't complex enough");
    }

    suspicious_outputs.iter().sorted().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!(part1(input), 2024);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/day24.txt").expect("Input file should exist");
        assert_eq!(part1(&input), 51745744348272);
        assert_eq!(part2(&input), "bfq,bng,fjp,hkh,hmt,z18,z27,z31");
    }
}

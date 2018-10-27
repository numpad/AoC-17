use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Op {
	Inc(i32),
	Dec(i32),
}

impl Op {
	pub fn new(op: &str, value: &str) -> Option<Op> {
		let value: i32 = value.parse().ok()?;

		match op {
			"inc" => Some(Op::Inc(value)),
			"dec" => Some(Op::Dec(value)),
			_ => None
		}
	}
}

#[derive(Debug, PartialEq)]
enum Cmp {
	Greater(i32),
	Less(i32),
	GreaterEquals(i32),
	LessEquals(i32),
	Equals(i32),
	NotEquals(i32),
}

impl Cmp {
	pub fn new(op: &str, value: &str) -> Option<Cmp> {
		let value: i32 = value.parse().ok()?;

		match op {
			">"  => Some(Cmp::Greater(value)),
			"<"  => Some(Cmp::Less(value)),
			">=" => Some(Cmp::GreaterEquals(value)),
			"<=" => Some(Cmp::LessEquals(value)),
			"==" => Some(Cmp::Equals(value)),
			"!=" => Some(Cmp::NotEquals(value)),
			_ => None
		}
	}
}

#[derive(Debug, PartialEq)]
struct Instr<'a> {
	assign_var: &'a str,
	op: Op,
	cmp_var: &'a str,
	cmp: Cmp,
}

impl<'a> Instr<'a> {
	pub fn from_str(line: &'a str) -> Option<Instr<'a>> {
		let tokens: Vec<&'a str> = line.split_whitespace().collect();
		if tokens.len() != 7 {
			return None;
		}

		Some(Instr {
			assign_var: tokens[0],
			op: Op::new(tokens[1], tokens[2])?,
			cmp_var: tokens[4],
			cmp: Cmp::new(tokens[5], tokens[6])?,
		})
	}
}

#[test]
fn create_instructions() {
	// check if fails to parse wrong instruction
	assert_eq!(Instr::from_str("abcdef"), None);

	let lines = vec![
		"b inc 5 if a > 1",
		"a inc 1 if b < 5<",
		"c dec -10 if a >= 1",
		"c inc -20 if c == 10"
	];

	
	let i = Instr::from_str(lines[0]).expect("failed to parse correct instruction");

	assert_eq!(i.assign_var, "b");
	assert_eq!(i.op, Op::Inc(5));
	assert_eq!(i.cmp_var, "a");
	assert_eq!(i.cmp, Cmp::Greater(1));
}

struct Registers<'a> {
	registers: HashMap<&'a str, i32>,
}

impl<'a> Registers<'a> {
	pub fn new() -> Registers<'a> {
		Registers {
			registers: HashMap::new(),
		}
	}

	pub fn process(&mut self, instr: &Instr) {

	}

}

fn main() {
	
}

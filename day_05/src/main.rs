
struct Jumper {
	offsets: Vec<i32>,
	cursor: i32,
}

impl Jumper {
	pub fn from_vec(offsets: Vec<i32>) -> Jumper {
		Jumper {
			offsets,
			cursor: 0,
		}
	}

	fn get_offset(&self) -> Option<i32> {
		match self.offsets.get(self.cursor as usize) {
			Some(off) => Some(*off),
			None => None,
		}
	}
}

impl Iterator for Jumper {
	type Item = i32;

	fn next(&mut self) -> Option<Self::Item> {
		let offset = self.get_offset();

		match offset {
			Some(off) => {
				/* part 2, part 1 is simply the else case */
				if self.offsets[self.cursor as usize] >= 3 {
					self.offsets[self.cursor as usize] -= 1;
				} else {
					self.offsets[self.cursor as usize] += 1;
				}
				self.cursor += off;
				Some(self.cursor - off)
			},
			None => None,
		}
	}
}

#[test]
fn test_jump() {
	let instr = vec![0, 3, 0, 1, -3];
	let mut jump = Jumper::from_vec(instr);

	assert_eq!(jump.next(), Some( 0));
	assert_eq!(jump.next(), Some( 0));
	assert_eq!(jump.next(), Some( 1));
	assert_eq!(jump.next(), Some( 4));
	assert_eq!(jump.next(), Some( 1));
	assert_eq!(jump.next(), None);
}

fn main() {
	let instr = String::from_utf8(include_bytes!("input").to_vec())
		.unwrap()
		.lines()
		.map(|offset| offset.parse().unwrap() )
		.collect();

	let jump = Jumper::from_vec(instr);
	
	println!("counter: {}", jump.count());
}

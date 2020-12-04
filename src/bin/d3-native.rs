/*
arboreal genetics and biome stability

movement: 3 right, 1 down
*/
fn main() {
	let content = read_file(".\\assets\\3d-input.txt");
	let lines = vec_from_string(content);
	let answer = solution_one(lines);

	println!("Number of tress: {}", answer);
}

fn solution_one(lines: Vec<String>) -> i32 {
	// (row, col)
	let mut index = (0, 0);
	let travel = (1, 3);
	let mut trees = 0;

	while index.0 < lines.len() {
		let line = &lines[index.0];

		if let Some('#') = line.chars().nth(index.1 % line.len()) {
			trees += 1;
		}

		index = (index.0 + travel.0, index.1 + travel.1);
	}

	trees
}

fn read_file(path: &str) -> String {
	let content = std::fs::read_to_string(path).expect("Couldn't read file.");

	content.to_string()
}

fn vec_from_string(text: String) -> Vec<String> {
	text.trim_end().split('\n').map(|x| String::from(x)).collect::<Vec<String>>()
}
use std::fmt::Display;
use std::slice::Iter;

pub enum State
{
	Solved,
	Unsolved(u32)
}

pub struct Sudoku
{
	size: usize,
	board: Vec<u8>,
	pub neighbours: Vec<Sudoku>
}

impl Sudoku
{
	pub fn new(size: u8) -> Sudoku
	{
		// Will result in bad behaviour if size = 0xFF but idc
		Sudoku { 
			size: 9, 
			board: vec![0; (size * size).into()],
			neighbours: vec![]
		}
	}

	pub fn fill_random(&mut self)
	{
		// Fill board with numbers in order
		// TODO: Come up with proper randomized board
		let mut crap: Vec<u8> = vec![];
		crap.append(&mut vec![1, 2, 3, 1, 2, 3, 1, 2, 3]);
		crap.append(&mut vec![4, 5, 6, 4, 5, 6, 4, 5, 6]);
		crap.append(&mut vec![7, 8, 9, 7, 8, 9, 7, 8, 9]);

		self.board = vec![];
		self.board.append(&mut crap.clone());
		self.board.append(&mut crap.clone());
		self.board.append(&mut crap);
	}

	pub fn new_random(size: u8) -> Sudoku 
	{
		let mut sudoku = Sudoku::new(size);
		sudoku.fill_random();

		sudoku
	}

	pub fn iter_neighbours(&mut self) -> Iter<Sudoku>
	{
		let indices = [0, 3, 6, 27, 30, 33, 54, 57, 60];
		let offsets = [0, 1, 2, 9, 10, 11, 18, 19, 20];
		self.neighbours.clear();

		for s in indices 
		{
			for i in 0..9
			{
				for j in i+1..9
				{
					let mut new_sudoku = self.clone();
					new_sudoku.swap(s + offsets[i], s + offsets[j]);

					self.neighbours.push(new_sudoku);
				}
			}
		}

		self.neighbours.iter()
	}

	pub fn swap(&mut self, left: usize, right: usize)
	{
		self.board.swap(left, right);
	}

	fn check_rows(&self) -> u32 
	{
		let mut errors = 0u32;

		for row in self.board.chunks(self.size)
		{
			for i in 0..self.size
			{
				for j in i+1..self.size
				{
					errors += if row[i] == row[j] { 1 } else { 0 };
				}
			}
		}

		errors
	}

	fn check_cols(&self) -> u32
	{
		let mut errors = 0u32;

		for col in 0..self.size
		{
			for i in 0..self.size
			{
				for j in i+1..self.size
				{
					errors += if self.board[col + i * self.size] == self.board[col + j * self.size] { 1 } else { 0 };
				}
			}
		}

		errors
	}

	pub fn errors(&self) -> u32
	{
		let mut errors = 0u32;

		errors += self.check_rows();
		errors += self.check_cols();

		errors
	}

	pub fn solved(&self) -> State
	{
		let errors = self.errors();

		match errors
		{
			0 => State::Solved,
			_ => State::Unsolved(errors)
		}
	}
}

impl Clone for Sudoku
{
	fn clone(&self) -> Self 
	{
		Sudoku { 
			size: self.size, 
			board: self.board.clone(),
			neighbours: vec![]
		}
	}
}

impl Display for Sudoku
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		for i in 0..self.size
		{
			for j in 0..self.size
			{
				write!(f, "{} ", self.board[i * self.size + j])?;
			}
			writeln!(f, "")?;
		}

		write!(f, "")
	}
}

impl PartialEq for Sudoku
{
    fn eq(&self, other: &Self) -> bool 
	{
		for i in 0..self.size
		{
			if self.board[i] != other.board[i] {
				return false;
			}
		}

		true
	}
}

impl Eq for Sudoku {}
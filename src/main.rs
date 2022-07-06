mod sudoku;
use sudoku::{Sudoku, State};

use rand::seq::SliceRandom;

fn main() {
    let mut sudoku = Sudoku::new_random(9);

    let mut pos = 0;
    let mut taboo = vec![Sudoku::new(9); 1024];

    let mut counter = 0;

    loop
    {
        match sudoku.solved()
        {
            State::Solved => { println!("Solved!"); break; },
            State::Unsolved(err) => 
            {
                println!("Iteration {}: {} errors", counter, err);
                if counter % 15 == 0
                {
                    println!("{}", sudoku);
                }
            }
        }

        let mut neighbours: Vec<(Sudoku, i32)> = vec![];

        for neighbour in sudoku.iter_neighbours()
        {
            if taboo.contains(neighbour) {
                continue;
            }

            let errors = neighbour.errors() as i32;
            neighbours.push((neighbour.clone(), errors));
        }

        neighbours.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        for i in 1..neighbours.len()
        {
            if neighbours[i].1 != neighbours[0].1
            {
                neighbours = neighbours.chunks(i).next().unwrap().to_vec();
                break;
            }
        }

        taboo[pos] = sudoku.clone();
        pos = (pos + 1) % taboo.len();
        sudoku = neighbours.choose(&mut rand::thread_rng()).unwrap().0.clone();
        counter += 1;
    }

    println!("{}", sudoku)
    
}

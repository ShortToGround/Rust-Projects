// Sudoku Solver

fn is_valid(puzzle: [[usize; 9]; 9], row: usize, col: usize, guess: usize) -> bool {
        // First check the row to see if the number is already there
        for i in 0..9 {
            // If the number is in the row, this guess is invalid
            if puzzle[row][i] == guess{
                return false;
            }
        }
        // Then check the column to see if the number is there
        for i in 0..9 {
            if puzzle[i][col] == guess{
                // If the number is in the column, this guess is invalid
                return false;
            }
        }
        // Then check the current 3x3 grid to see if the number is there
        // this can be broken up into a 3x3 xy axis
    
        // First we find our y value
        let sudoku_chunk_row: usize = row / 3;
    
        // Then our x value
        let sudoku_chunk_col: usize = col / 3;
    
        // Now we search that 3x3 grid, starting with the top left cell of the chunk and ending at the bottom right cell
        let mut i: usize = sudoku_chunk_row * 3;
        let mut j: usize = sudoku_chunk_col * 3;

        while i < ((sudoku_chunk_row * 3) + 3) {
            while j < ((sudoku_chunk_col * 3) + 3) {
                if puzzle[i][j] == guess {
                    return false;
                }
                j += 1;
            }
            j = sudoku_chunk_col * 3;
            i += 1;
        }

        // If we haven't returned a 0 by now then this is a valid number
        return true;
}

fn find_next_blank(puzzle: [[usize; 9]; 9]) -> (usize, usize){
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] == 0{
                return (row, col);
            }
        }
    }
    // If we never find a blank space, then return -1, -1.
    return (10, 10);
}

fn print_puzzle(puzzle: [[usize; 9]; 9]) -> bool {
        // Prints the entire puzzle and makes it nice and easy to read
        println!("+-----------------------+");
        let mut i: usize = 0;

        for row in puzzle {
            print!("| ");
            let mut j: usize = 0;

            for cell in row {
                if cell == 0 {
                    print!("-");
                }
                else{
                    print!("{}",cell);
                }
                if (((j + 1) % 3) == 0) && (j != 8){
                    print!(" | ");
                }
                else if j != 8{
                    print!(" ");
                }
                j += 1;
            }

            println!(" |");

            if (((i + 1 )% 3) == 0) && (i != 8){
                println!("+-----------------------+");
            }
            i += 1;
        }
        println!("+-----------------------+");
        return true;
}

fn find_solution(mut puzzle: [[usize; 9]; 9]) -> bool {
    let (row, col) = find_next_blank(puzzle);
    if row == 10 {
        print_puzzle(puzzle);
        return true;
    }
    else{
        for guess in 1..10 {
            if is_valid(puzzle, row, col, guess){
                puzzle[row][col] = guess;
                //println!("Is valid!");
                if find_solution(puzzle){
                    return true;
                }
            }
            else{
                //println!("Is not valid!");
                puzzle[row][col] = 0;
            }
        }
    }
    return false;
}

fn main(){

    let puzzle: [[usize; 9]; 9] = [
        [4,0,2,9,0,0,6,0,7],
        [3,9,0,0,0,0,2,4,8],
        [7,0,0,0,0,1,9,0,3],
        [5,4,0,0,0,0,0,0,0],
        [0,0,3,7,0,9,5,0,4],
        [8,2,0,0,5,0,0,9,6],
        [0,0,5,0,0,6,0,7,0],
        [0,0,4,8,0,0,0,0,9],
        [0,0,8,0,0,0,4,0,0]
    ];


    if !find_solution(puzzle){
        println!("No solution found...\n");
    }
    return ();
}

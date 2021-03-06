use std::time::Instant;

// Shows which indices are in contention with which other indices.
const CONTENTION_INDICES: [[i8; 20]; 81] = [
    [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 27, 36, 45, 54, 63, 72 ],
    [ 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 28, 37, 46, 55, 64, 73 ],
    [ 0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 29, 38, 47, 56, 65, 74 ],
    [ 0, 1, 2, 4, 5, 6, 7, 8, 12, 13, 14, 21, 22, 23, 30, 39, 48, 57, 66, 75 ],
    [ 0, 1, 2, 3, 5, 6, 7, 8, 12, 13, 14, 21, 22, 23, 31, 40, 49, 58, 67, 76 ],
    [ 0, 1, 2, 3, 4, 6, 7, 8, 12, 13, 14, 21, 22, 23, 32, 41, 50, 59, 68, 77 ],
    [ 0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 17, 24, 25, 26, 33, 42, 51, 60, 69, 78 ],
    [ 0, 1, 2, 3, 4, 5, 6, 8, 15, 16, 17, 24, 25, 26, 34, 43, 52, 61, 70, 79 ],
    [ 0, 1, 2, 3, 4, 5, 6, 7, 15, 16, 17, 24, 25, 26, 35, 44, 53, 62, 71, 80 ],
    [ 0, 1, 2, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 27, 36, 45, 54, 63, 72 ],
    [ 0, 1, 2, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 28, 37, 46, 55, 64, 73 ],
    [ 0, 1, 2, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20, 29, 38, 47, 56, 65, 74 ],
    [ 3, 4, 5, 9, 10, 11, 13, 14, 15, 16, 17, 21, 22, 23, 30, 39, 48, 57, 66, 75 ],
    [ 3, 4, 5, 9, 10, 11, 12, 14, 15, 16, 17, 21, 22, 23, 31, 40, 49, 58, 67, 76 ],
    [ 3, 4, 5, 9, 10, 11, 12, 13, 15, 16, 17, 21, 22, 23, 32, 41, 50, 59, 68, 77 ],
    [ 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 24, 25, 26, 33, 42, 51, 60, 69, 78 ],
    [ 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 24, 25, 26, 34, 43, 52, 61, 70, 79 ],
    [ 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 24, 25, 26, 35, 44, 53, 62, 71, 80 ],
    [ 0, 1, 2, 9, 10, 11, 19, 20, 21, 22, 23, 24, 25, 26, 27, 36, 45, 54, 63, 72 ],
    [ 0, 1, 2, 9, 10, 11, 18, 20, 21, 22, 23, 24, 25, 26, 28, 37, 46, 55, 64, 73 ],
    [ 0, 1, 2, 9, 10, 11, 18, 19, 21, 22, 23, 24, 25, 26, 29, 38, 47, 56, 65, 74 ],
    [ 3, 4, 5, 12, 13, 14, 18, 19, 20, 22, 23, 24, 25, 26, 30, 39, 48, 57, 66, 75 ],
    [ 3, 4, 5, 12, 13, 14, 18, 19, 20, 21, 23, 24, 25, 26, 31, 40, 49, 58, 67, 76 ],
    [ 3, 4, 5, 12, 13, 14, 18, 19, 20, 21, 22, 24, 25, 26, 32, 41, 50, 59, 68, 77 ],
    [ 6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 33, 42, 51, 60, 69, 78 ],
    [ 6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 34, 43, 52, 61, 70, 79 ],
    [ 6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 35, 44, 53, 62, 71, 80 ],
    [ 0, 9, 18, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 54, 63, 72 ],
    [ 1, 10, 19, 27, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 55, 64, 73 ],
    [ 2, 11, 20, 27, 28, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 56, 65, 74 ],
    [ 3, 12, 21, 27, 28, 29, 31, 32, 33, 34, 35, 39, 40, 41, 48, 49, 50, 57, 66, 75 ],
    [ 4, 13, 22, 27, 28, 29, 30, 32, 33, 34, 35, 39, 40, 41, 48, 49, 50, 58, 67, 76 ],
    [ 5, 14, 23, 27, 28, 29, 30, 31, 33, 34, 35, 39, 40, 41, 48, 49, 50, 59, 68, 77 ],
    [ 6, 15, 24, 27, 28, 29, 30, 31, 32, 34, 35, 42, 43, 44, 51, 52, 53, 60, 69, 78 ],
    [ 7, 16, 25, 27, 28, 29, 30, 31, 32, 33, 35, 42, 43, 44, 51, 52, 53, 61, 70, 79 ],
    [ 8, 17, 26, 27, 28, 29, 30, 31, 32, 33, 34, 42, 43, 44, 51, 52, 53, 62, 71, 80 ],
    [ 0, 9, 18, 27, 28, 29, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 54, 63, 72 ],
    [ 1, 10, 19, 27, 28, 29, 36, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 55, 64, 73 ],
    [ 2, 11, 20, 27, 28, 29, 36, 37, 39, 40, 41, 42, 43, 44, 45, 46, 47, 56, 65, 74 ],
    [ 3, 12, 21, 30, 31, 32, 36, 37, 38, 40, 41, 42, 43, 44, 48, 49, 50, 57, 66, 75 ],
    [ 4, 13, 22, 30, 31, 32, 36, 37, 38, 39, 41, 42, 43, 44, 48, 49, 50, 58, 67, 76 ],
    [ 5, 14, 23, 30, 31, 32, 36, 37, 38, 39, 40, 42, 43, 44, 48, 49, 50, 59, 68, 77 ],
    [ 6, 15, 24, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 51, 52, 53, 60, 69, 78 ],
    [ 7, 16, 25, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 44, 51, 52, 53, 61, 70, 79 ],
    [ 8, 17, 26, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 51, 52, 53, 62, 71, 80 ],
    [ 0, 9, 18, 27, 28, 29, 36, 37, 38, 46, 47, 48, 49, 50, 51, 52, 53, 54, 63, 72 ],
    [ 1, 10, 19, 27, 28, 29, 36, 37, 38, 45, 47, 48, 49, 50, 51, 52, 53, 55, 64, 73 ],
    [ 2, 11, 20, 27, 28, 29, 36, 37, 38, 45, 46, 48, 49, 50, 51, 52, 53, 56, 65, 74 ],
    [ 3, 12, 21, 30, 31, 32, 39, 40, 41, 45, 46, 47, 49, 50, 51, 52, 53, 57, 66, 75 ],
    [ 4, 13, 22, 30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 50, 51, 52, 53, 58, 67, 76 ],
    [ 5, 14, 23, 30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 49, 51, 52, 53, 59, 68, 77 ],
    [ 6, 15, 24, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 52, 53, 60, 69, 78 ],
    [ 7, 16, 25, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 53, 61, 70, 79 ],
    [ 8, 17, 26, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 62, 71, 80 ],
    [ 0, 9, 18, 27, 36, 45, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74 ],
    [ 1, 10, 19, 28, 37, 46, 54, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74 ],
    [ 2, 11, 20, 29, 38, 47, 54, 55, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74 ],
    [ 3, 12, 21, 30, 39, 48, 54, 55, 56, 58, 59, 60, 61, 62, 66, 67, 68, 75, 76, 77 ],
    [ 4, 13, 22, 31, 40, 49, 54, 55, 56, 57, 59, 60, 61, 62, 66, 67, 68, 75, 76, 77 ],
    [ 5, 14, 23, 32, 41, 50, 54, 55, 56, 57, 58, 60, 61, 62, 66, 67, 68, 75, 76, 77 ],
    [ 6, 15, 24, 33, 42, 51, 54, 55, 56, 57, 58, 59, 61, 62, 69, 70, 71, 78, 79, 80 ],
    [ 7, 16, 25, 34, 43, 52, 54, 55, 56, 57, 58, 59, 60, 62, 69, 70, 71, 78, 79, 80 ],
    [ 8, 17, 26, 35, 44, 53, 54, 55, 56, 57, 58, 59, 60, 61, 69, 70, 71, 78, 79, 80 ],
    [ 0, 9, 18, 27, 36, 45, 54, 55, 56, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74 ],
    [ 1, 10, 19, 28, 37, 46, 54, 55, 56, 63, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74 ],
    [ 2, 11, 20, 29, 38, 47, 54, 55, 56, 63, 64, 66, 67, 68, 69, 70, 71, 72, 73, 74 ],
    [ 3, 12, 21, 30, 39, 48, 57, 58, 59, 63, 64, 65, 67, 68, 69, 70, 71, 75, 76, 77 ],
    [ 4, 13, 22, 31, 40, 49, 57, 58, 59, 63, 64, 65, 66, 68, 69, 70, 71, 75, 76, 77 ],
    [ 5, 14, 23, 32, 41, 50, 57, 58, 59, 63, 64, 65, 66, 67, 69, 70, 71, 75, 76, 77 ],
    [ 6, 15, 24, 33, 42, 51, 60, 61, 62, 63, 64, 65, 66, 67, 68, 70, 71, 78, 79, 80 ],
    [ 7, 16, 25, 34, 43, 52, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 71, 78, 79, 80 ],
    [ 8, 17, 26, 35, 44, 53, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 78, 79, 80 ],
    [ 0, 9, 18, 27, 36, 45, 54, 55, 56, 63, 64, 65, 73, 74, 75, 76, 77, 78, 79, 80 ],
    [ 1, 10, 19, 28, 37, 46, 54, 55, 56, 63, 64, 65, 72, 74, 75, 76, 77, 78, 79, 80 ],
    [ 2, 11, 20, 29, 38, 47, 54, 55, 56, 63, 64, 65, 72, 73, 75, 76, 77, 78, 79, 80 ],
    [ 3, 12, 21, 30, 39, 48, 57, 58, 59, 66, 67, 68, 72, 73, 74, 76, 77, 78, 79, 80 ],
    [ 4, 13, 22, 31, 40, 49, 57, 58, 59, 66, 67, 68, 72, 73, 74, 75, 77, 78, 79, 80 ],
    [ 5, 14, 23, 32, 41, 50, 57, 58, 59, 66, 67, 68, 72, 73, 74, 75, 76, 78, 79, 80 ],
    [ 6, 15, 24, 33, 42, 51, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 79, 80 ],
    [ 7, 16, 25, 34, 43, 52, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 80 ],
    [ 8, 17, 26, 35, 44, 53, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79 ],
];

// This function was used to auto-generate the 81x20 array of contention indices above.
fn _generate_contention_indices()
{
    // Denotes what box a particular cell is in.
    let box_indices: [i8; 81] = [
        1, 1, 1,  2, 2, 2,  3, 3, 3,
        1, 1, 1,  2, 2, 2,  3, 3, 3,
        1, 1, 1,  2, 2, 2,  3, 3, 3,

        4, 4, 4,  5, 5, 5,  6, 6, 6,
        4, 4, 4,  5, 5, 5,  6, 6, 6,
        4, 4, 4,  5, 5, 5,  6, 6, 6,

        7, 7, 7,  8, 8, 8,  9, 9, 9,
        7, 7, 7,  8, 8, 8,  9, 9, 9,
        7, 7, 7,  8, 8, 8,  9, 9, 9,
    ];

    let thing: Vec<Vec<i8>> = (0..81)
        .map(|i: i8| {
            let row_index: i8 = i - (i % 9);
            let column_index: i8 = i % 9;

            let mut row_indices: Vec<i8> = (row_index..row_index+9).filter(|&v| v != i).collect();
            let mut column_indices: Vec<i8> = (column_index..81).step_by(9).take(9).filter(|&v| v != i).collect();
            let mut box_indices: Vec<i8> = (0..81).filter(|&v| box_indices[v as usize] == box_indices[i as usize]).filter(|&v| v != i).collect();

            let mut indices = vec![];
            indices.append(&mut row_indices);
            indices.append(&mut column_indices);
            indices.append(&mut box_indices);

            indices.sort();
            indices.dedup();

            indices
        })
        .collect();

    for v in thing
    {
        for i in v
        {
            print!("{} ", i);
        }
        println!();
    }
}

fn solve_puzzle(puzzle: [i8; 81]) -> Vec<[i8; 81]>
{
    let mut solution_base: [i8; 81] = puzzle.clone();

    let mut available = get_available(&puzzle);

    let mut only_one: Vec<(usize, i8)> =
        available.iter()
            .filter(|(_, v)| v.len() == 1)
            .map(|(i, v)| (*i, v[0]))
            .collect();

    while only_one.len() > 0
    {
        for (i, v) in only_one
        {
            solution_base[i] = v;
        }

        available = get_available(&solution_base);

        only_one =
            available.iter()
                .filter(|(_, v)| v.len() == 1)
                .map(|(i, v)| (*i, v[0]))
                .collect();
    }

    if !solution_base.contains(&0)
    {
        return vec![solution_base];
    }
    else
    {
        let num_open_spots = solution_base.iter().filter(|&&v| v == 0).collect::<Vec<&i8>>().len();
        let num_steps = num_open_spots + 1;
        let mut steps: Vec<Vec<[i8; 81]>> = Vec::with_capacity(num_steps);
            
        steps.push(vec![solution_base]);

        for i in 1..num_steps
        {
            // for each puzzle in the prior step, get the first open spot, generate all available numbers, and generate
            // new puzzles with the first open spot taken by each available number. After the puzzles are generated,
            // filter out the ones that have open spots with no available numbers.
            
            let adjusted_puzzles = steps[i - 1]
                .iter()
                .map(|p| {
                    let first_available_index =
                        p.iter().enumerate()
                            .filter(|(_, &v)| v == 0)
                            .map(|(i, _)| i)
                            .next()
                            .unwrap();
                    
                    let available_numbers = get_available_at_index(p, first_available_index);
                    
                    available_numbers.iter()
                        .map(|&v| {
                            let mut cloned_puzzle = p.clone();
                            cloned_puzzle[first_available_index] = v;
                            cloned_puzzle
                        })
                        .collect::<Vec<[i8; 81]>>()
                })
                .flatten();
                
            let solvable: Vec<[i8; 81]> =
                adjusted_puzzles
                    .filter(|p| {
                        !get_available(p)
                            .iter()
                            .any(|(_, nums)| nums.len() == 0)
                    })
                    .collect();
            
            steps.push(solvable);
        }

        steps[steps.len() - 1].clone()
    }
}

/// Gets the available numbers in each index. Returns a vector of tuples where the first element is
/// the index in the original puzzle and the second element is a vector of available numbers.
fn get_available(p: &[i8; 81]) -> Vec<(usize, Vec<i8>)>
{
    let empty_spots: Vec<(usize, i8)> = p.iter().enumerate().filter(|(_, &v)| v == 0).map(|(i, v)| (i, *v)).collect();
    let non_empty_spots: Vec<(usize, i8)> = p.iter().enumerate().filter(|(_, &v)| v != 0).map(|(i, v)| (i, *v)).collect();

    let available_numbers: Vec<(usize, Vec<i8>)> =
        empty_spots
            .iter()
            .map(|(i, _)| {
                let mut contended_numbers =
                    non_empty_spots
                        .iter()
                        .filter(|(j, _)| CONTENTION_INDICES[*i].contains(&(*j as i8)))
                        .map(|(_, v)| *v)
                        .collect::<Vec<i8>>();

                contended_numbers.sort();
                contended_numbers.dedup();

                let open_numbers = (1..10).filter(|v| !contended_numbers.contains(v)).collect::<Vec<i8>>();

                (*i, open_numbers)
            })
            .collect();

    available_numbers
}

/// Gets all available numbers for a puzzle at a given index.
fn get_available_at_index(p: &[i8; 81], i: usize) -> Vec<i8>
{
    let non_empty_spots: Vec<(usize, i8)> =
        p
            .iter()
            .enumerate()
            .filter(|(_, &v)| v != 0)
            .map(|(i, v)| (i, *v))
            .collect();

    let mut contended_numbers =
        non_empty_spots
            .iter()
            .filter(|(j, _)| CONTENTION_INDICES[i].contains(&(*j as i8)))
            .map(|(_, v)| *v)
            .collect::<Vec<i8>>();

    contended_numbers.sort();
    contended_numbers.dedup();

    (1..10).filter(|v| !contended_numbers.contains(v)).collect::<Vec<i8>>()
}

fn main()
{
    // TODO: Replace with some form of input (file might be best).
    let puzzle: [i8; 81] = [
        2, 0, 0,  1, 0, 0,  0, 0, 6,
        8, 0, 0,  0, 6, 0,  9, 0, 4,
        0, 9, 0,  5, 0, 8,  0, 2, 0,

        0, 0, 0,  0, 0, 7,  2, 4, 0,
        0, 0, 0,  0, 0, 0,  0, 0, 0,
        0, 4, 6,  3, 0, 0,  0, 0, 0,

        0, 8, 0,  7, 0, 5,  0, 6, 0,
        7, 0, 5,  0, 1, 0,  0, 0, 3,
        1, 0, 0,  0, 0, 2,  0, 0, 9,
    ];

    let _puzzle: [i8; 81] = [
        0, 0, 6,  0, 4, 9,  3, 0, 0,
        0, 9, 0,  8, 0, 0,  5, 1, 4,
        0, 0, 0,  0, 1, 0,  0, 0, 0,

        8, 0, 4,  0, 0, 2,  0, 0, 1,
        0, 7, 5,  0, 8, 1,  2, 3, 0,
        3, 0, 1,  0, 5, 6,  0, 0, 8,

        0, 0, 0,  0, 2, 0,  0, 0, 5,
        0, 5, 9,  0, 0, 0,  0, 0, 0,
        1, 0, 3,  6, 9, 5,  4, 0, 0,
    ];

    println!("Input puzzle:\n");
    print_puzzle(puzzle);

    let timer = Instant::now();
    let solutions: Vec<[i8; 81]> = solve_puzzle(puzzle);
    let elapsed = timer.elapsed();
    println!("\nTime taken to solve:\n\t{} seconds\n\t{} milliseconds\n\t{} nanoseconds\n", elapsed.as_secs(), elapsed.as_millis(), elapsed.as_nanos());

    if solutions.len() == 0
    {
        println!("No solutions found.");
    }
    else
    {
        println!("Solution(s):\n");
        for solution in solutions
        {
            print_puzzle(solution);
            println!("");
        }
    }
}

fn print_puzzle(p: [i8; 81])
{
    println!("???-----------------???");
    println!("|{} {} {}|{} {} {}|{} {} {}|", p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8]);
    println!("|{} {} {}|{} {} {}|{} {} {}|", p[9], p[10], p[11], p[12], p[13], p[14], p[15], p[16], p[17]);
    println!("|{} {} {}|{} {} {}|{} {} {}|", p[18], p[19], p[20], p[21], p[22], p[23], p[24], p[25], p[26]);
    println!("???-----------------???");
    println!("|{} {} {}|{} {} {}|{} {} {}|", p[27], p[28], p[29], p[30], p[31], p[32], p[33], p[34], p[35]);
    println!("|{} {} {}|{} {} {}|{} {} {}|", p[36], p[37], p[38], p[39], p[40], p[41], p[42], p[43], p[44]);
    println!("|{} {} {}|{} {} {}|{} {} {}|", p[45], p[46], p[47], p[48], p[49], p[50], p[51], p[52], p[53]);
    println!("???-----------------???");
    println!("|{} {} {}|{} {} {}|{} {} {}|", p[54], p[55], p[56], p[57], p[58], p[59], p[60], p[61], p[62]);
    println!("|{} {} {}|{} {} {}|{} {} {}|", p[63], p[64], p[65], p[66], p[67], p[68], p[69], p[70], p[71]);
    println!("|{} {} {}|{} {} {}|{} {} {}|", p[72], p[73], p[74], p[75], p[76], p[77], p[78], p[79], p[80]);
    println!("???-----------------???");
}
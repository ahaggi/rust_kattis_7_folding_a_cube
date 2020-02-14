use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;


/*
    THIS SOL USES BITWISE-OPERATIONS, i.e. cube: Vec<u8>, where u8 represent a num with 8 bits which is a row
    For an alternative approach, see the other file "alt_sol.rs", which uses cube: Vec<Vec<u8>> where every row is a vec.
*/

fn main() {
    let stdin = io::stdin();

    let mut cube: Vec<Vec<u8>> = vec![vec![0; 6]; 6];

    for i in 0..6 {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read line");

        for (j, ch) in line.trim().chars().enumerate() {
            if ch == '#' {
                cube[i][j] = 1;
            }
        }
    }

    let mut res = false;
    let mut nr_of_refl = 0;
    while !res && nr_of_refl < 2 {
        cube.retain(|v| v.iter().fold(false, |acc, x| *x != 0 || acc));
        res = is_cube(&cube);

        let mut nr_of_rotaion = 1;
        // try rotation
        while !res && nr_of_rotaion < 4 {
            if !res {
                // println!("rotarion nr {}", nr_of_rotaion);
                cube = rotete_90_cube(cube);
                // paint_the_shape(&cube);

                cube.retain(|v| v.iter().fold(false, |acc, x| *x != 0 || acc));
                res = is_cube(&cube);
            }
            nr_of_rotaion += 1;
        }
        cube = reflect_cube(cube);
        nr_of_refl += 1;
    }

    if res {
        println!("can fold");
    } else {
        println!("cannot fold")
    }
}

fn is_cube(cube: &Vec<Vec<u8>>) -> bool {
    let mut is_cube = false;
    match cube.len() {
        2 => is_cube = with_2_rows(&cube),
        3 => is_cube = with_3_rows(&cube),
        _ => (),
    }

    is_cube
}

fn rotete_90_cube(cube: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut cube_r: Vec<Vec<u8>> = vec![vec![0; cube.len()]; 6];

    for i in 0..cube.len() {
        for j in 0..cube[0].len() {
            cube_r[j][i] = cube[i][j]
        }
    }

    cube_r.reverse();
    cube_r
}

fn reflect_cube(cube: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut cube_x: Vec<Vec<u8>> = cube;

    for row in &mut cube_x {
        row.reverse();
    }
    cube_x
}

fn with_2_rows(cube: &Vec<Vec<u8>>) -> bool {
    let mut is_cube = false;

    let mut fst_line_inds: Vec<usize> = Vec::with_capacity(6);
    for (i, n) in cube[0].iter().enumerate() {
        if *n == 1 {
            fst_line_inds.push(i)
        }
    }

    let fst_line_connected: bool =
        fst_line_inds[fst_line_inds.len() - 1] - fst_line_inds[0] == fst_line_inds.len() - 1;
    if fst_line_connected && fst_line_inds.len() == 3 {
        // iff the fst line contains 3 squares and the snd line contains 3 squares
        let fst_start = fst_line_inds[0];
        let fst_end = fst_line_inds[fst_line_inds.len() - 1];
        is_cube = is_shape_0(cube, fst_start, fst_end);
    }

    is_cube
}

fn with_3_rows(cube: &Vec<Vec<u8>>) -> bool {
    // This will work due to all the # form a connected component, meaning it is possible to reach any # from any other # without touching a .
    let mut is_cube = false;

    let mut mid_line_inds: Vec<usize> = Vec::with_capacity(6);

    for (i, n) in cube[1].iter().enumerate() {
        if *n == 1 {
            mid_line_inds.push(i)
        }
    }
    let mid_line_connected: bool =
        mid_line_inds[mid_line_inds.len() - 1] - mid_line_inds[0] == mid_line_inds.len() - 1;

    if mid_line_connected {
        let mid_start = mid_line_inds[0];
        let mid_end = mid_line_inds[mid_line_inds.len() - 1];

        if mid_line_inds.len() == 4 {
            // Shapes with connected mid line of length 4
            is_cube = is_shape_1(&cube, mid_start);
            if !is_cube {
                is_cube = is_shape_2(&cube, mid_start);
            }
            if !is_cube {
                is_cube = is_shape_3(&cube, mid_start);
            }
            if !is_cube {
                is_cube = is_shape_4(&cube, mid_start);
            }
            if !is_cube {
                is_cube = is_shape_5(&cube, mid_start);
            }
            if !is_cube {
                is_cube = is_shape_6(&cube, mid_start);
            }
        } else if mid_line_inds.len() == 3 {
            // Shapes with connected mid line of length 3
            is_cube = is_shape_7(&cube, mid_end);
            if !is_cube {
                is_cube = is_shape_8(&cube, mid_end);
            }
            if !is_cube {
                is_cube = is_shape_9(&cube, mid_end);
            }
        } else if mid_line_inds.len() == 2 {
            // Shapes with connected mid line of length 2
            is_cube = is_shape_10(&cube, mid_start, mid_end);
        }
    }
    is_cube
}

/***************** 2 rows shapes with each row of length ****************/
/************************************************************************/
fn is_shape_0(cube: &Vec<Vec<u8>>, fst_start: usize, fst_end: usize) -> bool {
    /*
        ###
          ###
    */
    let mut is_shape_0 = false;
    if fst_end + 2 <= cube[1].len() - 1 {
        // the above indicate that the snd row is of len = 3, start at fst_end and ends at fst_end+2
        is_shape_0 = cube[0][fst_start] == 1
            && cube[0][fst_end] == 1
            && cube[1][fst_end] == 1
            && cube[1][fst_end + 2] == 1
    }
    is_shape_0
}

/*********** 3 rows shapes with connected mid row of length 4 ***********/
/************************************************************************/
fn is_shape_1(cube: &Vec<Vec<u8>>, mid_start: usize) -> bool {
    /*
        #
        ####
        #
    */
    cube[0][mid_start] == 1 && cube[2][mid_start] == 1
}
fn is_shape_2(cube: &Vec<Vec<u8>>, mid_start: usize) -> bool {
    /*
         #
        ####
         #
    */
    cube[0][mid_start + 1] == 1 && cube[2][mid_start + 1] == 1
}
fn is_shape_3(cube: &Vec<Vec<u8>>, mid_start: usize) -> bool {
    /*
        #
        ####
         #
    */
    cube[0][mid_start] == 1 && cube[2][mid_start + 1] == 1
}
fn is_shape_4(cube: &Vec<Vec<u8>>, mid_start: usize) -> bool {
    /*
        #
        ####
           #
    */
    // mid_start + 3 = end_start  = the 4.th index
    cube[0][mid_start] == 1 && cube[2][mid_start + 3] == 1
}
fn is_shape_5(cube: &Vec<Vec<u8>>, mid_start: usize) -> bool {
    /*
          #
        ####
         #
    */
    // mid_start + 2 = end_start -1 = the 3.th index
    cube[0][mid_start + 2] == 1 && cube[2][mid_start + 1] == 1
}
fn is_shape_6(cube: &Vec<Vec<u8>>, mid_start: usize) -> bool {
    /*
          #
        ####
        #
    */
    // mid_start + 2 = end_start -1 = the 3.th index
    cube[0][mid_start + 2] == 1 && cube[2][mid_start] == 1
}

/*********** 3 rows shapes with connected mid row of length 3 ***********/
/************************************************************************/

fn is_shape_7(cube: &Vec<Vec<u8>>, mid_end: usize) -> bool {
    /*
          #
        ###
          ##
    */
    let mut is_shape_7 = false;

    // To prevent index out of range
    // cube[1].len()-1 = 5
    if mid_end + 1 <= cube[1].len() - 1 {
        is_shape_7 = cube[0][mid_end] == 1 && cube[2][mid_end] == 1 && cube[2][mid_end + 1] == 1;
    }
    is_shape_7
}
fn is_shape_8(cube: &Vec<Vec<u8>>, mid_end: usize) -> bool {
    /*
         #
        ###
          ##
    */
    let mut is_shape_8 = false;

    // To prevent index out of range
    // cube[1].len()-1 = 5
    if mid_end + 1 <= cube[1].len() - 1 {
        is_shape_8 =
            cube[0][mid_end - 1] == 1 && cube[2][mid_end] == 1 && cube[2][mid_end + 1] == 1;
    }
    is_shape_8
}

fn is_shape_9(cube: &Vec<Vec<u8>>, mid_end: usize) -> bool {
    /*
          ##
        ###
        #
    */
    let mut is_shape_9 = false;

    // To prevent index out of range
    // cube[1].len()-1 = 5
    if mid_end + 1 <= cube[1].len() - 1 {
        is_shape_9 =
            cube[0][mid_end] == 1 && cube[0][mid_end + 1] == 1 && cube[2][mid_end - 2] == 1;
    }
    is_shape_9
}

/*********** 3 rows shapes with connected mid row of length 2 ***********/
/************************************************************************/
fn is_shape_10(cube: &Vec<Vec<u8>>, mid_start: usize, mid_end: usize) -> bool {
    /*
        ##
         ##
          ##
    */
    let mut is_shape_10 = false;

    // To prevent index out of range
    // cube[1].len()-1 = 5
    if mid_start > 0 && mid_end + 1 <= cube[1].len() - 1 {
        is_shape_10 = cube[0][mid_start] == 1
            && cube[0][mid_start - 1] == 1
            && cube[2][mid_end] == 1
            && cube[2][mid_end + 1] == 1;
    }
    is_shape_10
}

fn paint_the_shape(cube: &Vec<Vec<u8>>) {
    for row in cube {
        for elm in row {
            if *elm == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

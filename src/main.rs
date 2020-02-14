use std::io;


/*
    THIS SOL USES BITWISE-OPERATIONS, i.e. cube: Vec<u8>, where u8 represent a num with 8 bits which is a row
    For an alternative approach, see the other file "alt_sol.rs", which uses cube: Vec<Vec<u8>> where every row is a vec.
*/

fn main() {
    let stdin = io::stdin();

    let mut cube: Vec<u8> = vec![0; 6];
    for i in 0..6 {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read line");
        for (j, ch) in line.trim().chars().enumerate() {
            if ch == '#' {
                cube[i] |= 1 << 8 - 1 - j;
            }
        }
    }

    let mut res = false;
    let mut nr_of_refl = 0;
    while !res && nr_of_refl < 2 {
        cube.retain(|num| *num != 0);
        res = is_cube(&cube);

        let mut nr_of_rotaion = 1;
        // try rotation
        while !res && nr_of_rotaion < 4 {
            if !res {
                // println!("rotarion nr {}", nr_of_rotaion);
                cube = rotete_90_cube(cube);
                // paint_the_shape(&cube);

                cube.retain(|num| *num != 0);
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

fn reverse_bits(num: u8) -> u8 {
    // https://www.geeksforgeeks.org/write-an-efficient-c-program-to-reverse-bits-of-a-number/
    // take a look at http://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetParallel
    //                https://stackoverflow.com/questions/26145499/bitwise-bitcounts-formula-meaning
    let mut num = num;
    let mut count = 7;
    let mut reverse_num = num;
    num >>= 1;
    while num != 0 {
        reverse_num <<= 1;
        reverse_num |= num & 1;
        num >>= 1;
        count -= 1;
    }
    reverse_num <<= count;
    // println!("{:08b}", reverse_num);
    reverse_num
}

fn rotete_90_cube(cube: Vec<u8>) -> Vec<u8> {
    let mut cube_r: Vec<u8> = vec![0; 8];
    for i in 0..cube.len() {
        for j in 0..8 {
            // if cube[i] & 1<<(8 -1- j) != 0{
            // cube_r[8 -1- j] |= 1<< (8 -1- i)
            cube_r[8 - 1 - j] |= ((cube[i] & 1 << (8 - 1 - j)) >> (8 - 1 - j)) << (8 - 1 - i)

            // }
        }
    }
    paint_shape(&cube_r);
    cube_r
}

fn reflect_cube(cube: Vec<u8>) -> Vec<u8> {
    let mut cube_x: Vec<u8> = cube;
    for num in &mut cube_x {
        *num = reverse_bits(*num);
    }
    paint_shape(&cube_x);

    cube_x
}

fn is_cube(cube: &Vec<u8>) -> bool {
    let mut is_cube = false;
    match cube.len() {
        2 => is_cube = with_2_rows(&cube),
        3 => is_cube = with_3_rows(&cube),
        _ => (),
    }

    is_cube
}

fn with_2_rows(cube: &Vec<u8>) -> bool {
    let mut is_cube = false;

    let mut fst_line_inds: Vec<usize> = Vec::with_capacity(8);

    // Notice that the order of bits in binary is
    // |07|06|05|04|03|02|01|00
    // which is not the same as the order of vector/array
    // 00|01|02|03|04|05|06|07
    // but that does not matter we will rotate the input/shape anyway

    let fst_row = cube[0];
    let mut cnt: usize = 0;
    while cnt < 8 {
        if fst_row & 1 << cnt != 0 {
            fst_line_inds.push(cnt);
        }
        cnt += 1;
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

fn with_3_rows(cube: &Vec<u8>) -> bool {
    // This will work due to all the # form a connected component, meaning it is possible to reach any # from any other # without touching a .
    let mut is_cube = false;

    let mut mid_line_inds: Vec<usize> = Vec::with_capacity(8);

    let mid_row = cube[1];
    let mut cnt: usize = 0;
    while cnt < 8 {
        if mid_row & 1 << cnt != 0 {
            mid_line_inds.push(cnt);
        }
        cnt += 1;
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
fn is_shape_0(cube: &Vec<u8>, fst_start: usize, fst_end: usize) -> bool {
    /* reflection of
        ###
          ###
    */
    let mut is_shape_0 = false;
    if fst_end + 2 <= 8 - 1 {
        // the above indicate that the snd row is of len = 3, start at fst_end and ends at fst_end+2
        is_shape_0 = cube[0] & 1 << fst_start != 0
            && cube[0] & 1 << fst_end != 0
            && cube[1] & 1 << fst_end != 0
            && cube[1] & 1 << fst_end + 2 != 0
    }
    is_shape_0
}

// /*********** 3 rows shapes with connected mid row of length 4 ***********/
// /************************************************************************/
fn is_shape_1(cube: &Vec<u8>, mid_start: usize) -> bool {
    /* reflection of
        #
        ####
        #
    */
    cube[0] & 1 << mid_start != 0 && cube[2] & 1 << mid_start != 0
}
fn is_shape_2(cube: &Vec<u8>, mid_start: usize) -> bool {
    /* reflection of
         #
        ####
         #
    */
    cube[0] & 1 << (mid_start + 1) != 0 && cube[2] & 1 << (mid_start + 1) != 0
}
fn is_shape_3(cube: &Vec<u8>, mid_start: usize) -> bool {
    /* reflection of
        #
        ####
         #
    */
    cube[0] & 1 << mid_start != 0 && cube[2] & 1 << (mid_start + 1) != 0
}
fn is_shape_4(cube: &Vec<u8>, mid_start: usize) -> bool {
    /* reflection of
        #
        ####
           #
    */
    // mid_start + 3 = end_start  = the 4.th index

    cube[0] & 1 << mid_start != 0 && cube[2] & 1 << (mid_start + 3) != 0
}
fn is_shape_5(cube: &Vec<u8>, mid_start: usize) -> bool {
    /* reflection of
          #
        ####
         #
    */
    // mid_start + 2 = end_start -1 = the 3.th index

    cube[0] & 1 << (mid_start + 2) != 0 && cube[2] & 1 << (mid_start + 1) != 0
}
fn is_shape_6(cube: &Vec<u8>, mid_start: usize) -> bool {
    /* reflection of
          #
        ####
        #
    */
    // mid_start + 2 = end_start -1 = the 3.th index

    cube[0] & 1 << (mid_start + 2) != 0 && cube[2] & 1 << (mid_start) != 0
}

/*********** 3 rows shapes with connected mid row of length 3 ***********/
/************************************************************************/

fn is_shape_7(cube: &Vec<u8>, mid_end: usize) -> bool {
    /* reflection of
          #
        ###
          ##
    */
    let mut is_shape_7 = false;

    // To prevent index out of range
    // cube[1].len()-1 = 5

    if mid_end + 1 <= 8 - 1 {
        is_shape_7 = cube[0] & 1 << (mid_end) != 0
            && cube[2] & 1 << (mid_end) != 0
            && cube[2] & 1 << (mid_end + 1) != 0;
    }
    is_shape_7
}
fn is_shape_8(cube: &Vec<u8>, mid_end: usize) -> bool {
    /* reflection of

         #
        ###
          ##
    */
    let mut is_shape_8 = false;

    // To prevent index out of range
    // cube[1].len()-1 = 5
    if mid_end + 1 <= 8 - 1 {
        is_shape_8 = cube[0] & 1 << (mid_end - 1) != 0
            && cube[2] & 1 << mid_end != 0
            && cube[2] & 1 << (mid_end + 1) != 0;
    }
    is_shape_8
}

fn is_shape_9(cube: &Vec<u8>, mid_end: usize) -> bool {
    /* reflection of

          ##
        ###
        #
    */
    let mut is_shape_9 = false;

    // To prevent index out of range
    // cube[1].len()-1 = 5
    if mid_end + 1 <= 8 - 1 {
        is_shape_9 = cube[0] & 1 << mid_end != 0
            && cube[0] & 1 << (mid_end + 1) != 0
            && cube[2] & 1 << (mid_end - 2) != 0;
    }
    is_shape_9
}

/*********** 3 rows shapes with connected mid row of length 2 ***********/
/************************************************************************/
fn is_shape_10(cube: &Vec<u8>, mid_start: usize, mid_end: usize) -> bool {
    /* reflection of

        ##
         ##
          ##
    */
    let mut is_shape_10 = false;

    // To prevent index out of range
    // cube[1].len()-1 = 5
    if mid_start > 0 && mid_end + 1 <= 8 - 1 {
        is_shape_10 = cube[0] & 1 << mid_start != 0
            && cube[0] & 1 << (mid_start - 1) != 0
            && cube[2] & 1 << (mid_end) != 0
            && cube[2] & 1 << (mid_end + 1) != 0;
    }
    is_shape_10
}

fn paint_shape(cube: &Vec<u8>) {
    for num in cube {
        let output = format!("{:08b}", num);
        let output = output.replace("1", "#").replace("0", ".");
        println!("{}", output.as_str());
    }

    println!("\n\n");
}

fn calc_level(serial: i64, x: i64, y: i64) -> i64 {
    // Find the fuel cell's rack ID, which is its X coordinate plus 10.
    let rack_id = x + 10;

    // Begin with a power level of the rack ID times the Y coordinate.
    let mut level = y * rack_id;

    // Increase the power level by the value of the grid serial number (your
    // puzzle input).
    level += serial;

    // Set the power level to itself multiplied by the rack ID.
    level *= rack_id;

    // Keep only the hundreds digit of the power level (so 12345 becomes 3;
    // numbers with no hundreds digit become 0).
    level = (level / 100) % 10;

    //Subtract 5 from the power level.
    level -= 5;

    level
}

fn make_grid(serial: i64) -> Box<[[i64; 300]; 300]> {
    let mut grid = Box::new([[0; 300]; 300]);
    for y in 0..300 {
        for x in 0..300 {
            (*grid)[y][x] = calc_level(serial, x as i64, y as i64);
        }
    }

    grid
}

fn calc_col_power(grid: &Box<[[i64; 300]; 300]>, x: usize, y: usize, size: usize) -> i64 {
    let mut power = 0;
    for y1 in y..(y + size) {
        power += grid[y1][x];
    }
    power
}

fn calc_row_power(grid: &Box<[[i64; 300]; 300]>, x: usize, y: usize, size: usize) -> i64 {
    let mut power = 0;
    for x1 in x..(x + size) {
        power += grid[y][x1];
    }
    power
}

fn calc_section_power(grid: &Box<[[i64; 300]; 300]>, x: usize, y: usize, size: usize) -> i64 {
    let mut power = 0;

    for y1 in y..(y + size) {
        for x1 in x..(x + size) {
            power += (*grid)[y1][x1];
        }
    }

    power
}

fn find_max(grid: &Box<[[i64; 300]; 300]>, size: usize) -> (i64, i64, i64) {
    let mut max_level = std::i64::MIN;
    let mut coord = (-1, -1);

    // Value of the last x = -;
    let mut level0 = 0;
    for y in 0..(300 - size) {
        // Value of the last area.
        let mut last_level = 0;
        for x in 0..(300 - size) {
            let level = if x == 0 {
                level0 = if y == 0 {
                    // x,y == 0, 0
                    calc_section_power(&grid, x, y, size)
                } else {
                    // x == 0, y > 1.  Subtract the previous row, add the next.
                    level0 - calc_row_power(&grid, x, y - 1, size)
                        + calc_row_power(&grid, x, y + size - 1, size)
                };
                level0
            } else {
                // subtract the previous column, add the next
                last_level - calc_col_power(&grid, x - 1, y, size)
                    + calc_col_power(&grid, x + size - 1, y, size)
            };
            last_level = level;
            if level > max_level {
                max_level = level;
                coord = (x as i64, y as i64);
            }
        }
    }

    (coord.0, coord.1, max_level)
}

fn find_max2(grid: &Box<[[i64; 300]; 300]>) -> (i64, i64, i64, usize) {
    let mut max_level = std::i64::MIN;
    let mut max_size = 0;
    let mut coord = (-1, -1);

    for size in 1..=300 {
        let (x, y, level) = find_max(&grid, size);
        if level > max_level {
            max_level = level;
            coord = (x, y);
            max_size = size;
        }
    }

    (coord.0, coord.1, max_level, max_size)
}

fn main() {
    let serial = 7315;
    let grid = make_grid(serial);
    let pt1 = find_max(&grid, 3);
    let pt2 = find_max2(&grid);
    println!("Part 1: {},{} with a power value of {}", pt1.0, pt1.1, pt1.2);
    println!("Part 1: {},{},{} with a power value of {}", pt2.0, pt2.1, pt2.3, pt2.2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_level_test() {
        assert_eq!(4, calc_level(8, 3, 5));
        assert_eq!(-5, calc_level(57, 122, 79));
        assert_eq!(0, calc_level(39, 217, 196));
        assert_eq!(4, calc_level(71, 101, 153));
    }

    #[test]
    fn find_max_test1() {
        assert_eq!((33, 45, 29), find_max(&make_grid(18), 3));
    }

    #[test]
    fn find_max_test2() {
        assert_eq!((21, 61, 30), find_max(&make_grid(42), 3));
    }

    #[test]
    fn find_max2_test1() {
        assert_eq!((90, 269, 113, 16), find_max2(&make_grid(18)));
    }

    #[test]
    fn find_max2_test2() {
        assert_eq!((232, 251, 119, 12), find_max2(&make_grid(42)));
    }
}

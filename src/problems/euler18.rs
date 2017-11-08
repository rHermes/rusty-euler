// Euler problem 18

// how i transformed the triangle:

// cat wow.txt | sed -e 's/ /,/g' | sed -e 's/^/[/' | sed -e 's/$/],/' | sed -re 's/([^0-9])0+([0-9])+([^0-9])/\1\2\3/g'

fn solve_longest_path(rows: Vec<Vec<u8>>) -> u64 {
    use std::cmp::max;

    let n = rows.len();
    let mut i = n - 2;

    let mut current: Vec<u64> = rows[n - 1].iter().map(|x| *x as u64).collect();

    loop {

        // now we

        let row = &rows[i];

        let mut j = 0;
        while j < row.len() {

            current[j] = row[j] as u64 + max(current[j], current[j + 1]);

            j += 1;
        }


        if i == 0 {
            break;
        }
        i -= 1;
    }

    current[0]
}

pub fn run() -> u64 {
    let ins: Vec<Vec<u8>> = vec![
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 4, 82, 47, 65],
        vec![19, 1, 23, 75, 3, 34],
        vec![88, 2, 77, 73, 7, 63, 67],
        vec![99, 65, 4, 28, 6, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
    ];

    solve_longest_path(ins)
}

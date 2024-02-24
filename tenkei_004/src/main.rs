use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut a = vec![vec![1; w]; h];
    let mut b = vec![vec![0; w]; h];

    let mut column_sum = vec![0; w];

    for i in 0..h {
        let mut row_sum = 0;

        for j in 0..w {
            input! {
                aij: i32,
            }
            a[i][j] = aij;

            row_sum += aij;
            column_sum[j] += aij;
        }

        for j in 0..w {
            b[i][j] += row_sum;
        }
    }

    for j in 0..w {
        for i in 0..h {
            b[i][j] += column_sum[j] - a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", b[i][j]);
        }
        println!();
    }
}

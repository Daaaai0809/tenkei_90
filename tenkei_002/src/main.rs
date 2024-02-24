use proconio::input;

fn main() {
    input! {
        n: i32,
    };

    for i in 0..(1 << n) {
        let mut s = String::new();

        let mut left_count = 0;
        let mut right_count = 0;

        for j in (0..n).rev() {
            if i & (1 << j) == 0 {
                s.push('(');
                left_count += 1;
            } else {
                s.push(')');
                right_count += 1;
            }

            if left_count < right_count {
                break;
            }
        }

        if left_count == right_count {
            println!("{}", s);
        }
    }
}


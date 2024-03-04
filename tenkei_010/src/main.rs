use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut c = vec![0; 1_000_000];
    let mut p = vec![0; 1_000_000];

    let mut sum1 = vec! [0; 1_000_000];
    let mut sum2 = vec! [0; 1_000_000];

    for i in 0..n {
        input! {
            ci: i64,
            pi: i64,
        };
        
        c[i] = ci.clone();
        p[i] = pi.clone();

        match ci {
            1 => {
                if i > 0 {
                    sum1[i+1] = sum1[i] + pi;
                    sum2[i+1] = sum2[i];
                } else {
                    sum1[i+1] = pi;
                }
            },
            2 => {
                if i > 0 {
                    sum1[i+1] = sum1[i];
                    sum2[i+1] = sum2[i] + pi;
                } else {
                    sum2[i+1] = pi;
                }
            },
            _ => (),
        };
    }

    input! {
        q: usize,
    };

    let mut l: Vec<i64> = vec![0; q];
    let mut r: Vec<i64> = vec![0; q];

    for i in 0..q {
        input! {
            li: i64,
            ri: i64,
        };

        l[i] = li;
        r[i] = ri;
    }

    for i in 0..q {
        let ans1 = sum1[r[i] as usize] - sum1[l[i] as usize - 1];
        let ans2 = sum2[r[i] as usize] - sum2[l[i] as usize - 1];

        println!("{} {}", ans1, ans2);
    }
}

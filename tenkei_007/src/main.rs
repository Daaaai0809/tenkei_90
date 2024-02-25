use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        b: [i64; q],
    };

    // aを昇順にソート
    let mut a_cp = a;
    a_cp.sort();

    for _b in b {
        let mut left = 0;
        let mut right = n;
        let mut i = n;

        // 二分探索
        while left < right {
            let mid = (left + right) / 2;
            if a_cp[mid] < _b {
                left = mid + 1;
            } else {
                right = mid;
                i = mid;
            }
        }

        // a[i]とbとの差が最小の差を出力
        let ans =  if i == 0 {
            (a_cp[i] - _b).abs()
        } else if i == n {
            (a_cp[i - 1] - _b).abs()
        } else {
            (a_cp[i] - _b).abs().min((a_cp[i - 1] - _b).abs())
        };

        println!("{}", ans.abs());
    }
}

use proconio::input;

fn main() {
   input! {
        _n: usize,
        s: String,
   };

   let _mod = 1_000_000_007;
   let mut dp = vec![vec![0; 8]; 100_009];

   dp[0][0] = 1;

   for (i, c) in s.chars().enumerate() {
         for j in 0..8 {
            dp[i + 1][j] += dp[i][j];

            let _ = match c {
               'a' => {
                  match j {
                     0 => dp[i + 1][j + 1] += dp[i][j],
                     _ => continue,                      
                  }
               }
               't' => {
                  match j {
                     1 => dp[i + 1][j + 1] += dp[i][j],
                     _ => continue,                      
                  }
               }
               'c' => {
                  match j {
                     2 => dp[i + 1][j + 1] += dp[i][j],
                     _ => continue,                      
                  }
               }
               'o' => {
                  match j {
                     3 => dp[i + 1][j + 1] += dp[i][j],
                     _ => continue,                      
                  }
               }
               'd' => {
                  match j {
                     4 => dp[i + 1][j + 1] += dp[i][j],
                     _ => continue,                      
                  }
               }
               'e' => {
                  match j {
                     5 => dp[i + 1][j + 1] += dp[i][j],
                     _ => continue,                      
                  }
               }
               'r' => {
                  match j {
                     6 => dp[i + 1][j + 1] += dp[i][j],
                     _ => continue,                      
                  }
               }
               _ => continue,
            };
         }
   
         for j in 0..8 {
            dp[i + 1][j] %= _mod;
         }
   }

   println!("{}", dp[s.len()][7]);
}

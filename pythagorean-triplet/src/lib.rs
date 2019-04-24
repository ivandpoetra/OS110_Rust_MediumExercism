use std::collections::HashSet;

// a**2 + b**2 = c**2
// a < b < c
// Example 3**2 + 4**2 = 9 + 16 = 25 = 5**2

pub fn find(sum: u32) -> HashSet<[u32; 3]> {

    let mut result = HashSet::new();
    
    for c in 1..sum {
        for b in 1..c {
            if c + b < sum {
                let a = sum - b - c;
                if a < b && a*a + b*b == c*c {
                    result.insert([a, b, c]);
                }
            }
        }
    }
    result
}

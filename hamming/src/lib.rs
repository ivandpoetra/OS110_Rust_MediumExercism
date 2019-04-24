/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
/// Mentor: M. Luthfi A.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    
    // convert string to char for calling ([i])
    let s1: Vec<char> = s1.chars().collect();
	let s2: Vec<char> = s2.chars().collect();

    if s1.len() != s2.len(){
        return None
    }

    else{
        let mut count = 0;
        for i in 0..s1.len(){
            if s1[i] != s2[i]{
                count += 1;
            }
        }
        Some(count) 
    }

}
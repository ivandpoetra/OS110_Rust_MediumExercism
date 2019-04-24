#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0{
        None
    }

    else if num > 0 {
        if aquilot_sum(num) == num {
            return Some(Classification::Perfect)
        }
        else if aquilot_sum(num) > num {
            Some(Classification::Abundant)
        }
        else if aquilot_sum(num) < num {
            Some(Classification::Deficient)
        }
        else {
            None
        }
    }
    
    else {
        None
    }
}

pub fn aquilot_sum(num: u64) -> u64{
    let mut aquilot_sum = 0;
    for i in 1..num{
        if num%i == 0 {
            aquilot_sum = aquilot_sum + i;
        }
        else{
            aquilot_sum = aquilot_sum;
        }
    }
    return aquilot_sum    
}

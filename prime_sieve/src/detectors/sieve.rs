pub fn prime_range(range_min: usize, range_max: usize) -> Vec<usize> {
    let mut is_prime = vec![true; range_max+1];
    is_prime[0] = false;
    if range_max >= 1 { is_prime[1] = false }
 
    for num in range_min..range_max+1 {
        if is_prime[num] {
            let mut multiple = num*num;
            while multiple <= range_max {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }
 
    is_prime.iter().enumerate()
    .filter_map(|(pr, &is_pr)| if is_pr {Some(pr)} else {None})
    .collect()
}
 
pub fn findsum() -> i32 {
    let mut total_sum: i32 = 0;
    for i in 0..1000+1 {
    	if i%3 == 0 || i%5 == 0 {
    	    total_sum = total_sum+i;
    	}
    }
    return total_sum;
}

#[cfg(test)]
mod test {
	use functions; 
    #[test]
    fn my_first_unit_test() {
    	assert_eq!(234168, functions::findsum());
    }
}
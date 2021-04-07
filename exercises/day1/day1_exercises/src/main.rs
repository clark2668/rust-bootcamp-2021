/// Calculates the mean of an array
///
/// Calculates and returns the mean of the array `x` as a f64
fn find_mean(x: &[f64]) -> f64{
	let mut mean: f64 = 0.;
	for num in x.iter(){ mean += num; }
	return mean/(x.len() as f64);
}

/// Figures out if a number is prime or not (only interesting for an integer)
///
/// Returns whether a number is prime or not
fn is_prime(x: i32) -> bool{

	// Do this really stupidly. Between 2 -> X-1, check if there any divisor of X
	// which provides a zero modulus. If not, then the number is prime.
	// TODO: speed this up by only checking up to x/2.
	let mut answer : bool = true;
	for divisor in 2 .. x-1{
		if x % divisor == 0{

			answer = false;
		}
	}
	return answer
}


fn main() {
	
	// calculate th emean of an array
	let x = [4., 10., 30., 40.];
	let mean = find_mean(&x);
	println!("Mean: {}", mean);

	// check if a number is prime
	let number = 7;
	let answer = is_prime(number);
	println!("Is {} prime? : {}", number, answer);



}

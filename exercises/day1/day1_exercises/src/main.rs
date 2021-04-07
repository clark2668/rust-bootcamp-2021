
fn find_mean(x: &[f64]) -> f64{
	let mut mean: f64 = 0.;
	for num in x.iter(){ mean += num; }
	return mean/(x.len() as f64);
}

fn main() {
	let x = [4., 10., 30., 40.];
	let mean = find_mean(&x);
	println!("Mean: {}", mean);

}

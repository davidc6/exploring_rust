use algorithms::ttl::run_ttl;

fn main() {
    let data = vec![[10, 10], [20, 10], [10, 30]];
    let queries = [15, 50, 40, 30];

    let result_ttl = run_ttl(&data, &queries);
    println!("{:?}", result_ttl);
}

// To run the benchmark and open the HTML report:
// cargo bench --bench bench_main --profile dev
// open target/criterion/report/index.html

mod lit_actions;

criterion::criterion_main! {
    lit_actions::benches,
}

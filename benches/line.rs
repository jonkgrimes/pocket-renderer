#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn line_benchmark(bench: &mut Bencher) {
    bench.iter(|| {
        
        (0..1000).fold(0, |x, y| x + y)
    })
}

benchmark_group!(benches, line_benchmark);
benchmark_main!(benches);
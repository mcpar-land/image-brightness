use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
	let image_data = std::fs::read("./testing/signatures.png").unwrap();

	c.bench_function("image_brightness", |b| {
		b.iter(|| image_brightness::image_brightness(&image_data).unwrap())
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

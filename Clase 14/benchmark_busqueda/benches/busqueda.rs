use criterion::{
	criterion_group,
	criterion_main,
	criterion,
	black_box
};

use benchmark_busqueda::buscar;

fn benchmark_busqueda(c: &mut Criterion) {
	let n = 10_000_000;
	let datos: Vec<i32> = (0..n).collect();
	let objetivo = n - 1;

	c.bench_function(
		"busqueda_lineal_grande",
		|b| {
			b.iter(|| {
				buscar(
					black_box(&datos),
					black_box(objetivo)
				);
			})
		}
	);
}

criterion_group!(
	benches,
	benchmark_busqueda
);

criterion_main!(
	benches
);
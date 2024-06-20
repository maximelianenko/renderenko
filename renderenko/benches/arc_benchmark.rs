use std::{path::Path, sync::Arc};

use criterion::{ black_box, criterion_group, criterion_main, Criterion};
use renderenko::{misc::mesh::load_from_obj, types::MeshV4Plus};


fn render_and_save(
    mesh: Arc<MeshV4Plus>
) {
    drop(mesh)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mesh = Arc::new(load_from_obj(Path::new("data/model/test/ferris.obj")).unwrap());
    c.bench_function("render 20", |b| b.iter(|| render_and_save(black_box(mesh.clone()))));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
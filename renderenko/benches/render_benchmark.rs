use std::path::Path;

use criterion::{ criterion_group, criterion_main, Criterion};
use renderenko::builder::{self, RenderenkoBuilded};


fn render_and_save(
    builded: &RenderenkoBuilded
) {
    let _ = builded.render().resize(500,500);
}

fn criterion_benchmark(c: &mut Criterion) {
    let builded = builder::Renderenko::new()
    .mesh(Path::new("data/model/test/ferris.obj")).unwrap()
    .texture(Path::new("data/textures/test/ferris.png")).unwrap()
    .size(100, 100)
    .build();
    // .render()
    // .resize(500,500)
    // .save(Path::new("out"));
    // let mesh = load_from_obj(Path::new("data/model/alex.obj"));
    // // // println!("{:?}",mesh);
    // // // let smth:Vector2<f32> = Vector2::new()
    // if mesh.is_err() {
    //     println!("{}","Error");
    //     return;
    // }
    // let tris:MeshV4Plus = mesh.unwrap();
    // let width:f32 = 100.;
    // let height:f32 = 100.;
    // let aspect_ratio:f32 = height / width;
    // let far:f32 = 1000.;
    // let near:f32 = 0.1;
    // let fov_angle:f32 = 30.;
    // // let pixmap_texture_unwrapped = Pixmap::load_png("data/textures/maximelianenko.png");
    // // let pixmap_texture = pixmap_texture_unwrapped.unwrap();
    // let texture = Reader::open("data/textures/default/maximelianenko.png")
    //     .unwrap()
    //     .decode()
    //     .unwrap();
    // let matrix_projection:Matrix4<f32> = matrix_projection(fov_angle, aspect_ratio, near, far);
    // let matrix_view = matrix_view(
    //     Vector4::new(0.0,0.0,0.0,1.0),
    //     Vector4::new(0.0,3.0,0.0,1.0),
    //     Vector4::new(0.0,0.0,1.0,1.0),
    //     0.0,
    //     0.0
    // );
    // let matrix_world = matrix_world(
    //     0.0,
    //     -15.0,
    //     0.0,
    //     0.25,
    //     1.25
    // );
    // let resize = ResizeImage {
    //     width: 500,
    //     height: 500
    // };
    c.bench_function("render 20", |b| b.iter(|| render_and_save(&builded)));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
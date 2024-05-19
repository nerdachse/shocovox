use criterion::{criterion_group, criterion_main};
use rand::Rng;
use shocovox_rs::{octree::Octree, octree::V3c};

#[cfg(feature = "raytracing")]
use shocovox_rs::octree::raytracing::Ray;

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let mut rng = rand::thread_rng();

    #[cfg(feature = "raytracing")]
    {
        let tree_size = 8;
        let mut tree = shocovox_rs::octree::Octree::<u32, 4>::new(tree_size)
            .ok()
            .unwrap();
        tree.insert(&V3c::new(1, 3, 3), rng.gen_range(0..500))
            .ok()
            .unwrap();
        for x in 0..tree_size {
            for y in 0..tree_size {
                for z in 0..tree_size {
                    if x < (tree_size / 4)
                        || y < (tree_size / 4)
                        || z < (tree_size / 4)
                        || ((tree_size / 2) <= x && (tree_size / 2) <= y && (tree_size / 2) <= z)
                    {
                        // tree.insert(&V3c::new(x, y, z), RGB::new(100, 80, 151)).ok().unwrap();
                        tree.insert(&V3c::new(x, y, z), rng.gen_range(0..500))
                            .ok()
                            .unwrap();
                    }
                }
            }
        }

        c.bench_function("cpu get_by_ray", |b| {
            let viewport_size_width = 128;
            let viewport_size_height = 128;
            let radius = 2. * tree_size as f32;
            let angle: f32 = 40.;
            let origin = V3c::new(angle.sin() * radius, radius, angle.cos() * radius);
            let viewport = Ray {
                direction: (V3c::unit(0.) - origin).normalized(),
                origin,
            };
            let viewport_up_direction = V3c::new(0., 1., 0.);
            let viewport_right_direction =
                viewport_up_direction.cross(viewport.direction).normalized();
            let viewport_width = 4.;
            let viewport_height = 4.;
            let viewport_fov = 3.;
            let pixel_width = viewport_width as f32 / viewport_size_width as f32;
            let pixel_height = viewport_height as f32 / viewport_size_height as f32;
            let viewport_bottom_left = viewport.origin + (viewport.direction * viewport_fov)
                - (viewport_up_direction * (viewport_height / 2.))
                - (viewport_right_direction * (viewport_width / 2.));

            b.iter(|| {
                for y in 0..viewport_size_width {
                    for x in 0..viewport_size_height {
                        //from the origin of the camera to the current point of the viewport
                        let glass_point = viewport_bottom_left
                            + viewport_right_direction * x as f32 * pixel_width
                            + viewport_up_direction * y as f32 * pixel_height;
                        let ray = Ray {
                            origin: viewport.origin,
                            direction: (glass_point - viewport.origin).normalized(),
                        };
                        tree.get_by_ray(&ray);
                    }
                }
            })
        });
    }

    let tree_size = 64;
    let mut tree = shocovox_rs::octree::Octree::<u32>::new(tree_size)
        .ok()
        .unwrap();

    c.bench_function("octree insert", |b| {
        b.iter(|| {
            tree.insert(
                &V3c::new(
                    rng.gen_range(0..tree_size),
                    rng.gen_range(0..tree_size),
                    rng.gen_range(0..tree_size),
                ),
                rng.gen_range(0..500),
            )
            .ok()
        });
    });

    c.bench_function("octree clear", |b| {
        b.iter(|| {
            tree.clear(&V3c::new(
                rng.gen_range(0..tree_size),
                rng.gen_range(0..tree_size),
                rng.gen_range(0..tree_size),
            ))
            .ok()
            .unwrap();
        });
    });

    c.bench_function("octree get", |b| {
        b.iter(|| {
            tree.get(&V3c::new(
                rng.gen_range(0..tree_size),
                rng.gen_range(0..tree_size),
                rng.gen_range(0..tree_size),
            ));
        });
    });

    c.bench_function("octree save", |b| {
        b.iter(|| {
            tree.save("test_junk_octree").ok().unwrap();
        });
    });

    c.bench_function("octree load", |b| {
        b.iter(|| {
            let _tree_copy = Octree::<u32>::load("test_junk_octree").ok().unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

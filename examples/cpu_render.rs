#[cfg(feature = "raytracing")]
#[derive(Default, Clone, Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

#[cfg(feature = "raytracing")]
impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}

#[cfg(feature = "raytracing")]
impl shocovox_rs::octree::VoxelData for RGB {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    fn color(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

#[cfg(feature = "raytracing")]
use rand::Rng;

#[cfg(feature = "raytracing")]
use shocovox_rs::spatial::{math::V3c, raytracing::Ray};

#[cfg(feature = "raytracing")]
#[show_image::main]
fn main() {
    // fill octree with data
    let tree_size = 4;
    let viewport_resolution_width = 128;
    let viewport_resolution_height = 128;
    let mut tree = shocovox_rs::octree::Octree::<RGB>::new(tree_size)
        .ok()
        .unwrap();

    tree.insert(&V3c::new(1, 3, 3), RGB::new(100, 80, 151)).ok();
    for x in 0..tree_size {
        for y in 0..tree_size {
            for z in 0..tree_size {
                if x < (tree_size / 4)
                    || y < (tree_size / 4)
                    || z < (tree_size / 4)
                    || ((tree_size / 2) <= x && (tree_size / 2) <= y && (tree_size / 2) <= z)
                {
                    // tree.insert(&V3c::new(x, y, z), RGB::new(100, 80, 151)).ok();
                    tree.insert(
                        &V3c::new(x, y, z),
                        RGB::new(
                            (255 as f32 * x as f32 / tree_size as f32) as u8,
                            (255 as f32 * y as f32 / tree_size as f32) as u8,
                            (255 as f32 * z as f32 / tree_size as f32) as u8,
                        ),
                    )
                    .ok();
                }
            }
        }
    }

    // for y in 0..tree_size {
    //     for x in 0..(tree_size / 4) {
    //         tree.insert(
    //             &V3c::new(x, y, y),
    //             RGB {
    //                 r: (255 as f32 * y as f32 / tree_size as f32) as u8,
    //                 g: (255 as f32 * y as f32 / tree_size as f32) as u8,
    //                 b: (255 as f32 * y as f32 / tree_size as f32) as u8,
    //             },
    //         )
    //         .ok();
    //         tree.insert(
    //             &V3c::new(x + 1, y, y),
    //             RGB {
    //                 r: 255,
    //                 g: (255 as f32 * y as f32 / tree_size as f32) as u8,
    //                 b: (255 as f32 * y as f32 / tree_size as f32) as u8,
    //             },
    //         )
    //         .ok();
    //         tree.insert(
    //             &V3c::new(x + 2, y, y),
    //             RGB {
    //                 r: (255 as f32 * y as f32 / tree_size as f32) as u8,
    //                 g: 255,
    //                 b: (255 as f32 * y as f32 / tree_size as f32) as u8,
    //             },
    //         )
    //         .ok();
    //         tree.insert(
    //             &V3c::new(x + 3, y, y),
    //             RGB {
    //                 r: (255 as f32 * y as f32 / tree_size as f32) as u8,
    //                 g: (255 as f32 * y as f32 / tree_size as f32) as u8,
    //                 b: 255,
    //             },
    //         )
    //         .ok();
    //     }
    // }

    use show_image::create_window;
    let window = create_window("image", Default::default()).ok().unwrap();

    let radius = 2. * tree_size as f32;
    let mut rng = rand::thread_rng();
    let mut angle = 40.;
    let mut velos = V3c::new(-0.05, 0., 0.);

    loop {
        //generate a random number to add to velos
        velos = velos
            + V3c::new(
                (-5 + rng.gen_range(0..10)) as f32 / 2000.,
                (-5 + rng.gen_range(0..10)) as f32 / 2000.,
                (-5 + rng.gen_range(0..10)) as f32 / 2000.,
            );
        angle = angle + velos.x;

        // Set the viewport
        let origin = V3c::new(
            angle.sin() * radius,
            radius, //angles.y.sin() * radius,
            angle.cos() * radius,
        );
        let viewport = Ray {
            direction: (V3c::unit(0.) - origin).normalized(),
            origin,
        };

        let viewport_up_direction = V3c::new(0., 1., 0.); //TODO: up is actually left?!
        let viewport_right_direction = viewport_up_direction.cross(viewport.direction).normalized();
        let viewport_width = 4.;
        let viewport_height = 4.;
        let viewport_fov = 3.;
        let pixel_width = viewport_width as f32 / viewport_resolution_width as f32;
        let pixel_height = viewport_height as f32 / viewport_resolution_height as f32;
        let viewport_bottom_left = viewport.origin + (viewport.direction * viewport_fov)
            - (viewport_up_direction * (viewport_height / 2.))
            - (viewport_right_direction * (viewport_width / 2.));

        // define light
        let diffuse_light_normal = V3c::new(0., -1., 1.).normalized();

        use image::ImageBuffer;
        use image::Rgb;
        let mut img = ImageBuffer::new(viewport_resolution_width, viewport_resolution_height);

        // cast each ray for a hit
        //TODO: Do it in parallel
        // use rayon::iter::ParallelIterator;
        // use rayon::iter::IntoParallelIterator;
        // (0..viewport_resolution_width).into_par_iter().for_each(|y|
        for y in 0..viewport_resolution_width {
            for x in 0..viewport_resolution_height {
                let actual_y_in_image = viewport_resolution_height - y - 1;
                //from the origin of the camera to the current point of the viewport
                let glass_point = viewport_bottom_left
                    + viewport_right_direction * x as f32 * pixel_width
                    + viewport_up_direction * y as f32 * pixel_height;
                let ray = Ray {
                    origin: viewport.origin,
                    direction: (
                        glass_point - viewport.origin
                        // + V3c::new( // random offset
                        //     rng.gen_range(-10..10) as f32 / 1000.,
                        //     rng.gen_range(-10..10) as f32 / 1000.,
                        //     rng.gen_range(-10..10) as f32 / 1000.,
                        // )
                    )
                        .normalized(),
                };
                // print!("   {ray:?}   ");

                use std::io::Write;
                std::io::stdout().flush().ok().unwrap();
                // if x == 362 && y == 142 {
                //     img.put_pixel(x, actual_y_in_image, Rgb([0, 255, 255]));
                //     println!("{ray:?}");
                //     break 'outer;
                // }

                if let Some(hit) = tree.get_by_ray(&ray) {
                    let (data, _, normal) = hit;
                    //Because both vector should be normalized, the dot product should be 1*1*cos(angle)
                    //That means it is in range -1, +1, which should be accounted for
                    let diffuse_light_strength =
                        1. - (normal.dot(&diffuse_light_normal) / 2. + 0.5);
                    img.put_pixel(
                        x,
                        actual_y_in_image,
                        Rgb([
                            (data.r as f32 * diffuse_light_strength) as u8,
                            (data.g as f32 * diffuse_light_strength) as u8,
                            (data.b as f32 * diffuse_light_strength) as u8,
                        ]),
                    );
                } else {
                    img.put_pixel(x, actual_y_in_image, Rgb([0, 0, 0]));
                }
                print!(
                    "\r   progress: {}   ",
                    (x + y * viewport_resolution_height) as f32
                        / (viewport_resolution_height * viewport_resolution_width) as f32
                );
            }
        }

        use show_image::{ImageInfo, ImageView};
        // Display results //TODO: do it in a more sophisticated way
        println!("Done!");
        let binding = img.into_raw();
        let image = ImageView::new(
            ImageInfo::rgb8(viewport_resolution_width, viewport_resolution_height),
            &binding,
        );

        // Create a window with default options and display the image.
        window.set_image("image-001", image).ok();
        // TODO if window is closed, exit!
    }
}

#[cfg(not(feature = "raytracing"))]
fn main() {} //nothing to do when the feature is not enabled

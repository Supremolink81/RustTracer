use image::{Rgb, RgbImage, open, DynamicImage};

static mut TEXTURE_LIST : Vec<Texture> = vec![];

pub mod vec_class;
pub mod ray_class;
pub mod hitting;
pub mod camera;
pub mod materials;
pub mod bvh;
pub mod textures;
pub mod tree;

//Custom modules
use crate::vec_class::{Vec3, Color, Point3};
use crate::hitting::Hittable;
use crate::camera::Camera;
use crate::materials::{Material};
use crate::tree::Tree;
use crate::textures::{Texture, Perlin};

//Utilities
use rand::Rng;
use rayon::prelude::*;

struct Pixel {
    x : u32,
    y : u32,
    data : [u8 ; 3],
}

fn add_texture(t : Texture) -> usize {
    unsafe {
        TEXTURE_LIST.push(t);
        TEXTURE_LIST.len()-1
    }
}

fn clamp(x : f32, minimum : f32, maximum : f32) -> f32 {
    if x < minimum {
        return minimum;
    }
    if x > maximum {
        return maximum;
    }
    x
}

fn get_color(pixel_color : Color, samples : i32) -> (u8, u8, u8) {
    let r = (pixel_color.x / samples as f32).sqrt();
    let g = (pixel_color.y / samples as f32).sqrt();
    let b = (pixel_color.z / samples as f32).sqrt();
    (
     (255.0 * clamp(r, 0.0, 0.999)) as u8, 
     (255.0 * clamp(g, 0.0, 0.999)) as u8, 
     (255.0 * clamp(b, 0.0, 0.999)) as u8,
    )
}

fn scene() -> Tree {
    let mut objs : Vec<Hittable> = vec![];

    //Images
    let sun_img = open("images/sunmap.jpeg").unwrap();
    let mercury_img = open("images/mercurymap.jpeg").unwrap();
    let venus_img = open("images/venusmap.jpeg").unwrap();
    let earth_img = open("images/earthmap.jpeg").unwrap();
    let mars_img = open("images/marsmap.jpeg").unwrap();

    //Materials
    let sun_mat = Material::Light(add_texture(Texture::Image(sun_img.clone().into_bytes(), sun_img.clone().width(), sun_img.height())));
    let mercury_mat = Material::Lambertian(add_texture(Texture::Image(mercury_img.clone().into_bytes(), mercury_img.clone().width(), mercury_img.height())));
    let venus_mat = Material::Lambertian(add_texture(Texture::Image(venus_img.clone().into_bytes(), venus_img.clone().width(), venus_img.height())));
    let earth_mat = Material::Lambertian(add_texture(Texture::Image(earth_img.clone().into_bytes(), earth_img.clone().width(), earth_img.height())));
    let mars_mat = Material::Lambertian(add_texture(Texture::Image(mars_img.clone().into_bytes(), mars_img.clone().width(), mars_img.height())));

    //Generate objects
    let sun = Hittable::Sphere(sun_mat, Point3::new(278.0, 278.0, 0.0), 100.0);
    let mercury = Hittable::Sphere(mercury_mat, Point3::new(180.0, 180.0, -50.0), 10.0);
    let venus = Hittable::Sphere(venus_mat, Point3::new(260.0, 450.0, 20.0), 25.0);
    let earth = Hittable::Sphere(earth_mat, Point3::new(450.0, 200.0, 10.0), 30.0);
    let mars = Hittable::Sphere(mars_mat, Point3::new(100.0, 300.0, -25.0), 15.0);

    objs.push(sun);
    objs.push(mercury);
    objs.push(venus);
    objs.push(earth);
    objs.push(mars);
    
    Tree::build(&mut objs)
}

fn main() {

    //Image settings
    let aspect_ratio : f32 = 1.0;
    let image_width : u32 = 800;
    let image_height = ((image_width as f32) / aspect_ratio) as u32;

    //Camera settings
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist = 20.0;
    let aperture = 0.0;

    //World setup
    let world : Tree = scene();
    let samples_per_pixel = 1000;
    let max_depth = 1000;
    let cam = Camera::new(lookfrom.clone(), lookat.clone(), vup, 40.0, aspect_ratio, aperture, dist);
    let mut img = RgbImage::new(image_width, image_height);
    println!("P3\n{} {}\n255\n", image_width, image_height);

    let mut xy : Vec<(u32, u32)> = vec![];
    for x in 0..image_width {
        for y in 0..image_height {
            xy.push((x, y));
        }
    }

    //Render image
    let img_pixels = xy.into_par_iter().map(|(i, j)| {
        let mut pixel : Color = Color{x : 0.0, y : 0.0, z : 0.0};
        let mut rng = rand::thread_rng();

        for _s in 0..samples_per_pixel {
            let u : f32 = (i as f32 + rng.gen_range(-1.0..1.0)) / (image_width as f32 - 1.0);
            let v : f32 = (j as f32 + rng.gen_range(-1.0..1.0)) / (image_height as f32 - 1.0);
            let r = cam.get_ray(u, v);
            pixel += r.ray_color(&world, max_depth);
        }

        let (ir, ig, ib) = get_color(pixel, samples_per_pixel);
        Pixel{x : i, y : image_height - j - 1, data : [ir, ig, ib]}
    }).collect::<Vec<_>>();
    
    for pix in img_pixels {
        img.put_pixel(pix.x, pix.y, Rgb(pix.data));
    }

    img.save("imageTest.png").expect("Failed to save image");
}
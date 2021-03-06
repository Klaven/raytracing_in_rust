use super::vec::Vec3;
use super::ray::Ray;

extern crate rand;
use rand::Rng;

pub fn drand48()->f32{
    let random_float: f32 = rand::thread_rng().gen();
    random_float
}

pub struct Camera{
    origin: Vec3,
    lower_left_corner: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius:f32
}
impl Camera{
    pub fn new(look_from: Vec3, look_at:Vec3, vup: Vec3, vfov: f32, aspect:f32, aperture: f32, focus_dist: f32)-> Self{
        let lens_radius = aperture/2.0;
        let theta = vfov*std::f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect*half_height;
        let origin = look_from;
        let w = Vec3::make_unit_vector(look_from-look_at);
        let u = Vec3::make_unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        let mut lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
        lower_left_corner = origin-half_width*focus_dist*u-half_height*focus_dist*v-focus_dist*w;
        let horizontal = 2.0*half_width*focus_dist*u;
        let vertical = 2.0*half_height*focus_dist*v;

        Camera{
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius
        }
    }

    pub fn get_ray(&self, u: f32, v:f32)-> Ray{
        let rd:Vec3 = self.lens_radius*random_in_unit_sphere();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin+offset, self.lower_left_corner+u*self.horizontal+v*self.vertical-self.origin-offset)
    }
}

fn random_in_unit_sphere()-> Vec3{
    let mut p:Vec3;
    while{
        p = 2.0*Vec3::new(drand48(), drand48(), drand48())-Vec3::new(1.0, 1.0, 1.0);
        p.squared_length() >= 1.0
    }{}
    return p;
}
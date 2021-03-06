// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Rect;
use crate::Vec3;
use glib::translate::*;

glib::glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Point3D(Boxed<ffi::graphene_point3d_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_point3d_get_type(), ptr as *mut _) as *mut ffi::graphene_point3d_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_point3d_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || ffi::graphene_point3d_get_type(),
    }
}

impl Point3D {
    pub fn cross(&self, b: &Point3D) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_point3d_cross(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn distance(&self, b: &Point3D) -> (f32, Vec3) {
        unsafe {
            let mut delta = Vec3::uninitialized();
            let ret = ffi::graphene_point3d_distance(
                self.to_glib_none().0,
                b.to_glib_none().0,
                delta.to_glib_none_mut().0,
            );
            (ret, delta)
        }
    }

    pub fn dot(&self, b: &Point3D) -> f32 {
        unsafe { ffi::graphene_point3d_dot(self.to_glib_none().0, b.to_glib_none().0) }
    }

    fn equal(&self, b: &Point3D) -> bool {
        unsafe {
            from_glib(ffi::graphene_point3d_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    pub fn init(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::graphene_point3d_init(self.to_glib_none_mut().0, x, y, z);
        }
    }

    pub fn init_from_point(&mut self, src: &Point3D) {
        unsafe {
            ffi::graphene_point3d_init_from_point(self.to_glib_none_mut().0, src.to_glib_none().0);
        }
    }

    pub fn init_from_vec3(&mut self, v: &Vec3) {
        unsafe {
            ffi::graphene_point3d_init_from_vec3(self.to_glib_none_mut().0, v.to_glib_none().0);
        }
    }

    pub fn interpolate(&self, b: &Point3D, factor: f64) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_point3d_interpolate(
                self.to_glib_none().0,
                b.to_glib_none().0,
                factor,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn length(&self) -> f32 {
        unsafe { ffi::graphene_point3d_length(self.to_glib_none().0) }
    }

    pub fn near(&self, b: &Point3D, epsilon: f32) -> bool {
        unsafe {
            from_glib(ffi::graphene_point3d_near(
                self.to_glib_none().0,
                b.to_glib_none().0,
                epsilon,
            ))
        }
    }

    pub fn normalize(&self) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_point3d_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn normalize_viewport(&self, viewport: &Rect, z_near: f32, z_far: f32) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_point3d_normalize_viewport(
                self.to_glib_none().0,
                viewport.to_glib_none().0,
                z_near,
                z_far,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn scale(&self, factor: f32) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_point3d_scale(self.to_glib_none().0, factor, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        unsafe {
            let mut v = Vec3::uninitialized();
            ffi::graphene_point3d_to_vec3(self.to_glib_none().0, v.to_glib_none_mut().0);
            v
        }
    }

    pub fn zero() -> Point3D {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_point3d_zero()) }
    }
}

impl PartialEq for Point3D {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Point3D {}

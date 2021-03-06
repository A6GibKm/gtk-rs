// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use crate::RelationType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Relation(Object<ffi::AtkRelation, ffi::AtkRelationClass>);

    match fn {
        get_type => || ffi::atk_relation_get_type(),
    }
}

impl Relation {
    pub fn new(targets: &[Object], relationship: RelationType) -> Relation {
        assert_initialized_main_thread!();
        let n_targets = targets.len() as i32;
        unsafe {
            from_glib_full(ffi::atk_relation_new(
                targets.to_glib_none().0,
                n_targets,
                relationship.to_glib(),
            ))
        }
    }
}

pub const NONE_RELATION: Option<&Relation> = None;

pub trait RelationExt: 'static {
    fn add_target<P: IsA<Object>>(&self, target: &P);

    fn get_relation_type(&self) -> RelationType;

    fn get_target(&self) -> Vec<Object>;

    fn remove_target<P: IsA<Object>>(&self, target: &P) -> bool;

    fn set_property_relation_type(&self, relation_type: RelationType);

    fn set_property_target(&self, target: Option<&glib::ValueArray>);

    fn connect_property_relation_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Relation>> RelationExt for O {
    fn add_target<P: IsA<Object>>(&self, target: &P) {
        unsafe {
            ffi::atk_relation_add_target(
                self.as_ref().to_glib_none().0,
                target.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_relation_type(&self) -> RelationType {
        unsafe {
            from_glib(ffi::atk_relation_get_relation_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_target(&self) -> Vec<Object> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::atk_relation_get_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_target<P: IsA<Object>>(&self, target: &P) -> bool {
        unsafe {
            from_glib(ffi::atk_relation_remove_target(
                self.as_ref().to_glib_none().0,
                target.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_property_relation_type(&self, relation_type: RelationType) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"relation-type\0".as_ptr() as *const _,
                glib::Value::from(&relation_type).to_glib_none().0,
            );
        }
    }

    fn set_property_target(&self, target: Option<&glib::ValueArray>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"target\0".as_ptr() as *const _,
                glib::Value::from(target).to_glib_none().0,
            );
        }
    }

    fn connect_property_relation_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_relation_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkRelation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Relation>,
        {
            let f: &F = &*(f as *const F);
            f(&Relation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::relation-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_relation_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_target_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkRelation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Relation>,
        {
            let f: &F = &*(f as *const F);
            f(&Relation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::target\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Relation")
    }
}

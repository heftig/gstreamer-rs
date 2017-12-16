// This file was generated by gir (d50d839) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct NetTimeProvider(Object<ffi::GstNetTimeProvider, ffi::GstNetTimeProviderClass>): [
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_net_time_provider_get_type(),
    }
}

impl NetTimeProvider {
    pub fn new<'a, P: IsA<gst::Clock>, Q: Into<Option<&'a str>>>(clock: &P, address: Q, port: i32) -> NetTimeProvider {
        assert_initialized_main_thread!();
        let address = address.into();
        let address = address.to_glib_none();
        unsafe {
            from_glib_none(ffi::gst_net_time_provider_new(clock.to_glib_none().0, address.0, port))
        }
    }

    pub fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <bool as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "active".to_glib_none().0, Value::from(&active).to_glib_none().0);
        }
    }

    pub fn get_property_address(&self) -> Option<String> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <String as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "address".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn get_property_clock(&self) -> Option<gst::Clock> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <gst::Clock as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "clock".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn get_property_port(&self) -> i32 {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <i32 as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "port".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn connect_property_active_notify<F: Fn(&NetTimeProvider) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&NetTimeProvider) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(notify_active_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_address_notify<F: Fn(&NetTimeProvider) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&NetTimeProvider) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::address",
                transmute(notify_address_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_clock_notify<F: Fn(&NetTimeProvider) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&NetTimeProvider) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::clock",
                transmute(notify_clock_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_port_notify<F: Fn(&NetTimeProvider) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&NetTimeProvider) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::port",
                transmute(notify_port_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for NetTimeProvider {}
unsafe impl Sync for NetTimeProvider {}

unsafe extern "C" fn notify_active_trampoline(this: *mut ffi::GstNetTimeProvider, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&NetTimeProvider) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_address_trampoline(this: *mut ffi::GstNetTimeProvider, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&NetTimeProvider) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_clock_trampoline(this: *mut ffi::GstNetTimeProvider, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&NetTimeProvider) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_port_trampoline(this: *mut ffi::GstNetTimeProvider, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&NetTimeProvider) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
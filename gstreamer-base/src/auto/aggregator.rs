// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
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
    pub struct Aggregator(Object<ffi::GstAggregator, ffi::GstAggregatorClass>): [
        gst::Element => gst_ffi::GstElement,
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_aggregator_get_type(),
    }
}

unsafe impl Send for Aggregator {}
unsafe impl Sync for Aggregator {}

pub trait AggregatorExt {
    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //fn get_allocator(&self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams);

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn get_buffer_pool(&self) -> Option<gst::BufferPool>;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn get_latency(&self) -> gst::ClockTime;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime);

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn set_src_caps(&self, caps: &gst::Caps);

    fn get_property_start_time(&self) -> u64;

    fn set_property_start_time(&self, start_time: u64);

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_start_time_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Aggregator> + IsA<glib::object::Object>> AggregatorExt for O {
    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //fn get_allocator(&self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams) {
    //    unsafe { TODO: call ffi::gst_aggregator_get_allocator() }
    //}

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn get_buffer_pool(&self) -> Option<gst::BufferPool> {
        unsafe {
            from_glib_full(ffi::gst_aggregator_get_buffer_pool(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn get_latency(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_aggregator_get_latency(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime) {
        unsafe {
            ffi::gst_aggregator_set_latency(self.to_glib_none().0, min_latency.to_glib(), max_latency.to_glib());
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn set_src_caps(&self, caps: &gst::Caps) {
        unsafe {
            ffi::gst_aggregator_set_src_caps(self.to_glib_none().0, caps.to_glib_none().0);
        }
    }

    fn get_property_start_time(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "start-time".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_start_time(&self, start_time: u64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "start-time".to_glib_none().0, Value::from(&start_time).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::latency",
                transmute(notify_latency_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_start_time_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::start-time",
                transmute(notify_start_time_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
unsafe extern "C" fn notify_latency_trampoline<P>(this: *mut ffi::GstAggregator, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Aggregator> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Aggregator::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_start_time_trampoline<P>(this: *mut ffi::GstAggregator, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Aggregator> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Aggregator::from_glib_borrow(this).downcast_unchecked())
}

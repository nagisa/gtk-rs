// This file was generated by gir (4b09025) from gir-files (71d73f0)
// DO NOT EDIT

use AppInfo;
use File;
use ffi;
use glib;
#[cfg(feature = "v2_36")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v2_36")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v2_36")]
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v2_36")]
use libc;
#[cfg(feature = "v2_36")]
use std::boxed::Box as Box_;
#[cfg(feature = "v2_36")]
use std::mem::transmute;

glib_wrapper! {
    pub struct AppLaunchContext(Object<ffi::GAppLaunchContext>);

    match fn {
        get_type => || ffi::g_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    pub fn new() -> AppLaunchContext {
        unsafe {
            from_glib_full(ffi::g_app_launch_context_new())
        }
    }
}

pub trait AppLaunchContextExt {
    fn get_display<P: IsA<AppInfo>>(&self, info: &P, files: &[File]) -> Option<String>;

    fn get_environment(&self) -> Vec<String>;

    fn get_startup_notify_id<P: IsA<AppInfo>>(&self, info: &P, files: &[File]) -> Option<String>;

    fn launch_failed(&self, startup_notify_id: &str);

    fn setenv(&self, variable: &str, value: &str);

    fn unsetenv(&self, variable: &str);

    #[cfg(feature = "v2_36")]
    fn connect_launch_failed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v2_36")]
    fn connect_launched<F: Fn(&Self, &AppInfo, &glib::Variant) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<AppLaunchContext> + IsA<glib::object::Object>> AppLaunchContextExt for O {
    fn get_display<P: IsA<AppInfo>>(&self, info: &P, files: &[File]) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_app_launch_context_get_display(self.to_glib_none().0, info.to_glib_none().0, files.to_glib_none().0))
        }
    }

    fn get_environment(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_launch_context_get_environment(self.to_glib_none().0))
        }
    }

    fn get_startup_notify_id<P: IsA<AppInfo>>(&self, info: &P, files: &[File]) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_app_launch_context_get_startup_notify_id(self.to_glib_none().0, info.to_glib_none().0, files.to_glib_none().0))
        }
    }

    fn launch_failed(&self, startup_notify_id: &str) {
        unsafe {
            ffi::g_app_launch_context_launch_failed(self.to_glib_none().0, startup_notify_id.to_glib_none().0);
        }
    }

    fn setenv(&self, variable: &str, value: &str) {
        unsafe {
            ffi::g_app_launch_context_setenv(self.to_glib_none().0, variable.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn unsetenv(&self, variable: &str) {
        unsafe {
            ffi::g_app_launch_context_unsetenv(self.to_glib_none().0, variable.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_36")]
    fn connect_launch_failed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "launch-failed",
                transmute(launch_failed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v2_36")]
    fn connect_launched<F: Fn(&Self, &AppInfo, &glib::Variant) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &AppInfo, &glib::Variant) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "launched",
                transmute(launched_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_36")]
unsafe extern "C" fn launch_failed_trampoline<P>(this: *mut ffi::GAppLaunchContext, startup_notify_id: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<AppLaunchContext> {
    callback_guard!();
    let f: &Box_<Fn(&P, &str) + 'static> = transmute(f);
    f(&AppLaunchContext::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(startup_notify_id))
}

#[cfg(feature = "v2_36")]
unsafe extern "C" fn launched_trampoline<P>(this: *mut ffi::GAppLaunchContext, info: *mut ffi::GAppInfo, platform_data: *mut glib_ffi::GVariant, f: glib_ffi::gpointer)
where P: IsA<AppLaunchContext> {
    callback_guard!();
    let f: &Box_<Fn(&P, &AppInfo, &glib::Variant) + 'static> = transmute(f);
    f(&AppLaunchContext::from_glib_none(this).downcast_unchecked(), &from_glib_none(info), &from_glib_none(platform_data))
}

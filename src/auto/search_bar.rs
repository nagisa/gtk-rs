// This file was generated by gir (9bd51ed) from gir-files (db49619)
// DO NOT EDIT

use Bin;
use Container;
#[cfg(feature = "v3_10")]
use Entry;
use Widget;
use ffi;
#[cfg(feature = "v3_10")]
use gdk;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SearchBar(Object<ffi::GtkSearchBar>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_search_bar_get_type(),
    }
}

impl SearchBar {
    #[cfg(feature = "v3_10")]
    pub fn new() -> SearchBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_bar_new()).downcast_unchecked()
        }
    }
}

#[cfg(feature = "v3_10")]
impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SearchBarExt {
    #[cfg(feature = "v3_10")]
    fn connect_entry<P: IsA<Entry>>(&self, entry: &P);

    #[cfg(feature = "v3_10")]
    fn get_search_mode(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_show_close_button(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn handle_event(&self, event: &gdk::Event) -> bool;

    #[cfg(feature = "v3_10")]
    fn set_search_mode(&self, search_mode: bool);

    #[cfg(feature = "v3_10")]
    fn set_show_close_button(&self, visible: bool);

    fn get_property_search_mode_enabled(&self) -> bool;

    fn set_property_search_mode_enabled(&self, search_mode_enabled: bool);

    fn get_property_show_close_button(&self) -> bool;

    fn set_property_show_close_button(&self, show_close_button: bool);

    fn connect_property_search_mode_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<SearchBar> + IsA<glib::object::Object>> SearchBarExt for O {
    #[cfg(feature = "v3_10")]
    fn connect_entry<P: IsA<Entry>>(&self, entry: &P) {
        unsafe {
            ffi::gtk_search_bar_connect_entry(self.to_glib_none().0, entry.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_search_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_get_search_mode(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_get_show_close_button(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_handle_event(self.to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_search_mode(&self, search_mode: bool) {
        unsafe {
            ffi::gtk_search_bar_set_search_mode(self.to_glib_none().0, search_mode.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_show_close_button(&self, visible: bool) {
        unsafe {
            ffi::gtk_search_bar_set_show_close_button(self.to_glib_none().0, visible.to_glib());
        }
    }

    fn get_property_search_mode_enabled(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "search-mode-enabled".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_search_mode_enabled(&self, search_mode_enabled: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "search-mode-enabled".to_glib_none().0, Value::from(&search_mode_enabled).to_glib_none().0);
        }
    }

    fn get_property_show_close_button(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-close-button".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_close_button(&self, show_close_button: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-close-button".to_glib_none().0, Value::from(&show_close_button).to_glib_none().0);
        }
    }

    fn connect_property_search_mode_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::search-mode-enabled",
                transmute(notify_search_mode_enabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-close-button",
                transmute(notify_show_close_button_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_search_mode_enabled_trampoline<P>(this: *mut ffi::GtkSearchBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchBar::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_close_button_trampoline<P>(this: *mut ffi::GtkSearchBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchBar::from_glib_none(this).downcast_unchecked())
}

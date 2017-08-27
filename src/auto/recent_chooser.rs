// This file was generated by gir (9bd51ed) from gir-files (db49619)
// DO NOT EDIT

use Error;
use RecentFilter;
use RecentInfo;
use RecentSortType;
use ffi;
use glib;
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
    pub struct RecentChooser(Object<ffi::GtkRecentChooser>);

    match fn {
        get_type => || ffi::gtk_recent_chooser_get_type(),
    }
}

pub trait RecentChooserExt {
    fn add_filter(&self, filter: &RecentFilter);

    fn get_current_item(&self) -> Option<RecentInfo>;

    fn get_current_uri(&self) -> Option<String>;

    fn get_filter(&self) -> Option<RecentFilter>;

    fn get_items(&self) -> Vec<RecentInfo>;

    fn get_limit(&self) -> i32;

    fn get_local_only(&self) -> bool;

    fn get_select_multiple(&self) -> bool;

    fn get_show_icons(&self) -> bool;

    fn get_show_not_found(&self) -> bool;

    fn get_show_private(&self) -> bool;

    fn get_show_tips(&self) -> bool;

    fn get_sort_type(&self) -> RecentSortType;

    fn get_uris(&self) -> Vec<String>;

    fn list_filters(&self) -> Vec<RecentFilter>;

    fn remove_filter(&self, filter: &RecentFilter);

    fn select_all(&self);

    fn select_uri(&self, uri: &str) -> Result<(), Error>;

    fn set_current_uri(&self, uri: &str) -> Result<(), Error>;

    fn set_filter<'a, P: Into<Option<&'a RecentFilter>>>(&self, filter: P);

    fn set_limit(&self, limit: i32);

    fn set_local_only(&self, local_only: bool);

    fn set_select_multiple(&self, select_multiple: bool);

    fn set_show_icons(&self, show_icons: bool);

    fn set_show_not_found(&self, show_not_found: bool);

    fn set_show_private(&self, show_private: bool);

    fn set_show_tips(&self, show_tips: bool);

    //fn set_sort_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, sort_func: /*Unknown conversion*//*Unimplemented*/RecentSortFunc, sort_data: P, data_destroy: Q);

    fn set_sort_type(&self, sort_type: RecentSortType);

    fn unselect_all(&self);

    fn unselect_uri(&self, uri: &str);

    fn connect_item_activated<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_recent_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_show_not_found_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_show_private_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_show_tips_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_sort_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<RecentChooser> + IsA<glib::object::Object>> RecentChooserExt for O {
    fn add_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_add_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn get_current_item(&self) -> Option<RecentInfo> {
        unsafe {
            from_glib_full(ffi::gtk_recent_chooser_get_current_item(self.to_glib_none().0))
        }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_recent_chooser_get_current_uri(self.to_glib_none().0))
        }
    }

    fn get_filter(&self) -> Option<RecentFilter> {
        unsafe {
            from_glib_none(ffi::gtk_recent_chooser_get_filter(self.to_glib_none().0))
        }
    }

    fn get_items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_recent_chooser_get_items(self.to_glib_none().0))
        }
    }

    fn get_limit(&self) -> i32 {
        unsafe {
            ffi::gtk_recent_chooser_get_limit(self.to_glib_none().0)
        }
    }

    fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_local_only(self.to_glib_none().0))
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_select_multiple(self.to_glib_none().0))
        }
    }

    fn get_show_icons(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_icons(self.to_glib_none().0))
        }
    }

    fn get_show_not_found(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_not_found(self.to_glib_none().0))
        }
    }

    fn get_show_private(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_private(self.to_glib_none().0))
        }
    }

    fn get_show_tips(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_tips(self.to_glib_none().0))
        }
    }

    fn get_sort_type(&self) -> RecentSortType {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_sort_type(self.to_glib_none().0))
        }
    }

    fn get_uris(&self) -> Vec<String> {
        unsafe {
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_full_num(ffi::gtk_recent_chooser_get_uris(self.to_glib_none().0, &mut length), length as usize);
            ret
        }
    }

    fn list_filters(&self) -> Vec<RecentFilter> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_recent_chooser_list_filters(self.to_glib_none().0))
        }
    }

    fn remove_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_remove_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_recent_chooser_select_all(self.to_glib_none().0);
        }
    }

    fn select_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_chooser_select_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_current_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_chooser_set_current_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_filter<'a, P: Into<Option<&'a RecentFilter>>>(&self, filter: P) {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            ffi::gtk_recent_chooser_set_filter(self.to_glib_none().0, filter.0);
        }
    }

    fn set_limit(&self, limit: i32) {
        unsafe {
            ffi::gtk_recent_chooser_set_limit(self.to_glib_none().0, limit);
        }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_local_only(self.to_glib_none().0, local_only.to_glib());
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_select_multiple(self.to_glib_none().0, select_multiple.to_glib());
        }
    }

    fn set_show_icons(&self, show_icons: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_icons(self.to_glib_none().0, show_icons.to_glib());
        }
    }

    fn set_show_not_found(&self, show_not_found: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_not_found(self.to_glib_none().0, show_not_found.to_glib());
        }
    }

    fn set_show_private(&self, show_private: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_private(self.to_glib_none().0, show_private.to_glib());
        }
    }

    fn set_show_tips(&self, show_tips: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_tips(self.to_glib_none().0, show_tips.to_glib());
        }
    }

    //fn set_sort_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, sort_func: /*Unknown conversion*//*Unimplemented*/RecentSortFunc, sort_data: P, data_destroy: Q) {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_set_sort_func() }
    //}

    fn set_sort_type(&self, sort_type: RecentSortType) {
        unsafe {
            ffi::gtk_recent_chooser_set_sort_type(self.to_glib_none().0, sort_type.to_glib());
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_recent_chooser_unselect_all(self.to_glib_none().0);
        }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_recent_chooser_unselect_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn connect_item_activated<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "item-activated",
                transmute(item_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-changed",
                transmute(selection_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::filter",
                transmute(notify_filter_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::limit",
                transmute(notify_limit_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::local-only",
                transmute(notify_local_only_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_recent_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::recent-manager",
                transmute(notify_recent_manager_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::select-multiple",
                transmute(notify_select_multiple_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-icons",
                transmute(notify_show_icons_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_not_found_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-not-found",
                transmute(notify_show_not_found_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_private_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-private",
                transmute(notify_show_private_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_tips_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-tips",
                transmute(notify_show_tips_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_sort_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::sort-type",
                transmute(notify_sort_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn item_activated_trampoline<P>(this: *mut ffi::GtkRecentChooser, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn selection_changed_trampoline<P>(this: *mut ffi::GtkRecentChooser, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_filter_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_limit_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_local_only_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_recent_manager_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_select_multiple_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_icons_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_not_found_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_private_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_tips_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_sort_type_trampoline<P>(this: *mut ffi::GtkRecentChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

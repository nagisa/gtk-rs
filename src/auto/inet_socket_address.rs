// This file was generated by gir (https://github.com/gtk-rs/gir @ 72ba992)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use InetAddress;
use SocketAddress;
use SocketConnectable;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct InetSocketAddress(Object<ffi::GInetSocketAddress, ffi::GInetSocketAddressClass>): SocketAddress, SocketConnectable;

    match fn {
        get_type => || ffi::g_inet_socket_address_get_type(),
    }
}

impl InetSocketAddress {
    pub fn new(address: &InetAddress, port: u16) -> InetSocketAddress {
        unsafe {
            SocketAddress::from_glib_full(ffi::g_inet_socket_address_new(address.to_glib_none().0, port)).downcast_unchecked()
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn new_from_string(address: &str, port: u32) -> InetSocketAddress {
        unsafe {
            SocketAddress::from_glib_full(ffi::g_inet_socket_address_new_from_string(address.to_glib_none().0, port)).downcast_unchecked()
        }
    }
}

pub trait InetSocketAddressExt {
    fn get_address(&self) -> Option<InetAddress>;

    fn get_flowinfo(&self) -> u32;

    fn get_port(&self) -> u16;

    fn get_scope_id(&self) -> u32;

    fn connect_property_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flowinfo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scope_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<InetSocketAddress> + IsA<glib::object::Object>> InetSocketAddressExt for O {
    fn get_address(&self) -> Option<InetAddress> {
        unsafe {
            from_glib_none(ffi::g_inet_socket_address_get_address(self.to_glib_none().0))
        }
    }

    fn get_flowinfo(&self) -> u32 {
        unsafe {
            ffi::g_inet_socket_address_get_flowinfo(self.to_glib_none().0)
        }
    }

    fn get_port(&self) -> u16 {
        unsafe {
            ffi::g_inet_socket_address_get_port(self.to_glib_none().0)
        }
    }

    fn get_scope_id(&self) -> u32 {
        unsafe {
            ffi::g_inet_socket_address_get_scope_id(self.to_glib_none().0)
        }
    }

    fn connect_property_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::address",
                transmute(notify_address_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_flowinfo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::flowinfo",
                transmute(notify_flowinfo_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::port",
                transmute(notify_port_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scope_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::scope-id",
                transmute(notify_scope_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_address_trampoline<P>(this: *mut ffi::GInetSocketAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InetSocketAddress> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InetSocketAddress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_flowinfo_trampoline<P>(this: *mut ffi::GInetSocketAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InetSocketAddress> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InetSocketAddress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_port_trampoline<P>(this: *mut ffi::GInetSocketAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InetSocketAddress> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InetSocketAddress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_scope_id_trampoline<P>(this: *mut ffi::GInetSocketAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<InetSocketAddress> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&InetSocketAddress::from_glib_borrow(this).downcast_unchecked())
}

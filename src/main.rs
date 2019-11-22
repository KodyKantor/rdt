extern crate libc;

use core::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::ffi::{ CString, CStr };
use std::ptr;
use libc::size_t;
use std::thread;
use std::time::Duration;

#[repr(C)]
struct UsdtProbe { _private: [u8; 0] }

#[repr(C)]
struct UsdtProbedef {
    name: *const c_char,
    function: *const c_char,
    argc: size_t,
    types: *const c_char,
    probe: *const UsdtProbe,
    next: *const UsdtProbedef,
    refcnt: c_int,
}

#[repr(C)]
struct UsdtProvider {
    name: *const c_char,
    module: *const c_char,
    probedefs: c_void,
    error: *const c_char,
    enabled: c_int,
    file: *mut c_void,
}

#[link(name = "usdt", kind = "static")]
extern {
    fn usdt_create_provider(
        name: *const c_char,
        module: *const c_char) -> *const UsdtProvider;
    fn usdt_create_probe(
        func: *const c_char,
        name: *const c_char,
        argc: size_t,
        types: *const *const c_char) -> *const UsdtProbedef;
    fn usdt_provider_add_probe(
        provider: *const UsdtProvider,
        probedef: *const UsdtProbedef) -> c_int;
    fn usdt_provider_enable(provider: *const UsdtProvider) -> c_int;
    fn usdt_fire_probe(
        probe: *const UsdtProbe,
        argc: size_t,
        argv: *const *const c_char);
    fn usdt_is_enabled(
        probe: *const UsdtProbe) -> c_int;
}

fn main() {
    println!("Hello, world!");
    let name = CString::new("rustprov").expect("CString::new failed");
    let module = CString::new("rustmod").expect("CString::new failed");
    let func = CString::new("rustfunc").expect("CString::new failed");
    let fname = CString::new("rustname").expect("CString::new failed");
    let types = CString::new("").expect("CString::new failed");
    let args = CString::new("test").expect("CString::new failed");
    unsafe {
        let prov = usdt_create_provider(name.as_ptr(), module.as_ptr());
        println!("provider... name: {:?}, mod: {:?}",
            CStr::from_ptr((*prov).name),
            CStr::from_ptr((*prov).module));

        let probedef = usdt_create_probe(func.as_ptr(), fname.as_ptr(), 0,
            &types.as_ptr());
        println!("probe... func: {:?}, name: {:?}",
            CStr::from_ptr((*probedef).function),
            CStr::from_ptr((*probedef).name));

        let created = usdt_provider_add_probe(prov, probedef);
        println!("created: {}", created);

        let enabled = usdt_provider_enable(prov);
        println!("enabled: {}, {}", enabled, (*prov).enabled);

        loop {
            println!("firing probes...");

            let enabled = usdt_is_enabled((*probedef).probe);
            println!("enabled: {}", enabled);

            usdt_fire_probe((*probedef).probe, 1, &args.as_ptr());
            println!("sleeping");
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

extern crate libc;

use core::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::ffi::{CString};
use libc::size_t;
use std::thread;
use std::time::Duration;

#[repr(C)]
struct UsdtProbe {
    isenabled_addr: extern fn(c_void)->c_int,
    probe_addr: *const c_void,
}

#[repr(C)]
struct UsdtProbedef {
    name: *const c_char,
    function: *const c_char,
    argc: size_t,
    types: [*const c_char; 32],
    probe: *mut UsdtProbe,
    next: *mut UsdtProbedef,
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
        module: *const c_char) -> *mut UsdtProvider;
    fn usdt_create_probe(
        func: *const c_char,
        name: *const c_char,
        argc: size_t,
        types: *const *const c_char) -> *mut UsdtProbedef;
    fn usdt_provider_add_probe(
        provider: *mut UsdtProvider,
        probedef: *mut UsdtProbedef) -> c_int;
    fn usdt_provider_enable(provider: *mut UsdtProvider) -> c_int;
    fn usdt_fire_probe(
        probe: *mut UsdtProbe,
        argc: size_t,
        argv: *const *const c_char);
    fn usdt_is_enabled(
        probe: *mut UsdtProbe) -> c_int;
}

fn main() {

    /* provider:mod:func:name */
    let provider = CString::new("rustprov").expect("CString::new failed");
    let module = CString::new("rustmod").expect("CString::new failed");
    let func = CString::new("rustfunc").expect("CString::new failed");
    let name = CString::new("rustname").expect("CString::new failed");

    /* probe argument types */
    let types = CString::new("char *").expect("CString::new failed");

    /* probe arguments */
    let args = CString::new("hello from rust!").expect("CString::new failed");

    unsafe {
        let prov = usdt_create_provider(provider.as_ptr(), module.as_ptr());
        let probedef = usdt_create_probe(func.as_ptr(), name.as_ptr(), 1,
            &types.as_ptr());
        usdt_provider_add_probe(prov, probedef);
        usdt_provider_enable(prov);

        loop {
            if usdt_is_enabled((*probedef).probe) == 1 {
                usdt_fire_probe((*probedef).probe, 1, &args.as_ptr());
            }
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

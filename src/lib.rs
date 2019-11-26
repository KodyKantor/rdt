#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod rdt_tests {
    /*
     * XXX need to look up a safe way to compare *const c_char and CStrings
     */
    use super::*;
    use std::ffi::CString;

    fn create_provider() -> *mut usdt_provider_t {
        /*
         * This is the provider:mod predicate.
         *
         * Note that the provider string will get a PID appended
         * (e.g. rustprov1234), despite it not being visible here.
         */
        let provider = CString::new("rustprov").expect("CString::new failed");
        let module = CString::new("rustmod").expect("CString::new failed");
        unsafe {
            return usdt_create_provider(provider.as_ptr(), module.as_ptr());
        }
    }

    fn create_probe() -> *mut usdt_probedef_t {
        let func = CString::new("rustfunc").expect("CString::new failed");
        let name = CString::new("rustname").expect("CString::new failed");

        /* probe argument types */
        let types = CString::new("char *").expect("CString::new failed");

        unsafe {
            return usdt_create_probe(func.as_ptr(), name.as_ptr(), 1,
                &mut types.as_ptr());
        }
    }

    #[test]
    fn test_provider() {
        let prov = create_provider();
        unsafe {
            assert_eq!((*prov).enabled, 0);

            usdt_provider_free(prov);
        }
        /*
         * assert prov == prov
         * assert mod == mod
         */
    }

    #[test]
    fn test_probdef() {
        let probedef = create_probe();
        unsafe { assert_eq!((*probedef).argc, 1); }
        /*
         * assert types[0] == "char *"
         * assert name == name
         * assert func == func
         *
         * ideally we would explicitly free the probedef, but free_probedef
         * is not a public function (for whatever reason). free_probedef is
         * only called via usdt_provider_free().
         */
    }

    #[test]
    fn test_add_probe_to_provider_and_enable() {
        let prov = create_provider();
        let probedef = create_probe();

        unsafe {
            let err = usdt_provider_add_probe(prov, probedef);
            assert_eq!(err, 0);

            let err = usdt_provider_enable(prov);
            assert_eq!(err, 0);

            /* This also frees the probedef. */
            usdt_provider_free(prov);
        }
    }
}

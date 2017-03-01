#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate libc;

use libc::{pollfd, timeval};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use libc::{c_int, c_void};
    use super::*;
    use std::ptr;

    #[test]
    fn create_client() {
        unsafe {
            let mut err: c_int = 0;
            unsafe extern "C" fn callback(_client: *mut AvahiClient,
                                          _state: AvahiClientState,
                                          _userdata: *mut c_void) {
            }

            let poller = avahi_simple_poll_new();
            let client = avahi_client_new(avahi_simple_poll_get(poller),
                                          AvahiClientFlags(0),
                                          Some(callback),
                                          ptr::null_mut(),
                                          &mut err);
            if err == 0
            // TODO AVAHI_OK, avahi_strerror..
            {
                avahi_client_free(client);
            }
            avahi_simple_poll_free(poller);
        }
    }
}

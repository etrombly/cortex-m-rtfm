//! examples/singleton.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use alloc_singleton::stable::pool::{Box, Pool};
use cortex_m_semihosting::{debug, hprintln};
use lm3s6965::Interrupt;
use rtfm::app;

#[app(device = lm3s6965)]
const APP: () = {
    #[Singleton(Send)]
    static mut M: [u32; 2] = [0; 2];

    static mut P: Pool<M> = ();

    #[init(resources = [M])]
    fn init() -> init::LateResources {
        rtfm::pend(Interrupt::I2C0);

        init::LateResources {
            P: Pool::new(resources.M),
        }
    }

    #[interrupt(
        priority = 2,
        resources = [P],
        spawn = [foo, bar],
    )]
    fn I2C0() {
        spawn.foo(resources.P.alloc(1).unwrap()).unwrap();
        spawn.bar(resources.P.alloc(2).unwrap()).unwrap();
    }

    #[task(resources = [P])]
    fn foo(x: Box<M>) {
        hprintln!("foo({})", x).unwrap();

        resources.P.lock(|p| p.dealloc(x));

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(priority = 2, resources = [P])]
    fn bar(x: Box<M>) {
        hprintln!("bar({})", x).unwrap();

        resources.P.dealloc(x);
    }

    extern "C" {
        fn UART0();
        fn UART1();
    }
};

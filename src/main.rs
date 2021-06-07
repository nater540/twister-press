#![no_main]
#![no_std]

use nrf52832_hal as _;
use panic_semihosting as _;
use rtic::app;

#[app(device = nrf52832_hal)]
const APP: () = {};

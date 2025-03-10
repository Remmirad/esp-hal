//! Control LED on GPIO 1 by the systimer via ETM

#![no_std]
#![no_main]

use esp32h2_hal::{
    clock::ClockControl,
    etm::Etm,
    gpio::{etm::GpioEtmChannels, IO},
    peripherals::Peripherals,
    prelude::*,
    systimer::{etm::SysTimerEtmEvent, SystemTimer},
};
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let _clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let syst = SystemTimer::new(peripherals.SYSTIMER);
    let mut alarm0 = syst.alarm0.into_periodic();
    alarm0.set_period(1u32.secs());

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio1.into_push_pull_output();

    // setup ETM
    let gpio_ext = GpioEtmChannels::new(peripherals.GPIO_SD);
    let led_task = gpio_ext.channel0_task.toggle(&mut led);

    let timer_event = SysTimerEtmEvent::new(&mut alarm0);

    let etm = Etm::new(peripherals.SOC_ETM);
    let channel0 = etm.channel0;

    // make sure the configured channel doesn't get dropped - dropping it will
    // disable the channel
    let _configured_channel = channel0.setup(&timer_event, &led_task);

    // the LED is controlled by the timer without involving the CPU
    loop {}
}

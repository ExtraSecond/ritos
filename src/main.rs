#![feature(format_args_nl)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(exclusive_range_pattern)]

mod arch;
mod bsp;
mod interface;
mod memory;
mod panic;
mod print;
mod runtime_init;

unsafe fn kernel_init() -> ! {
    for i in bsp::device_drivers().iter_mut() {
        if let Err(()) = i.init() {
            panic!("Error loading driver: {}", i.compatible())
        }
    }
    bsp::post_driver_init();
    kernel_main()
}
/*
fn kernel_main() -> ! {
    use interface::console::ConsoleAll;
    use interface::gpio::GPIOAll;

    loop {
        if bsp::console().read_char() == '\n' {
            break;
        }
    }

    println!("[Info] Board name: {}", bsp::board_name());

    println!("[Info] Drivers loaded:");
    for (i, driver) in bsp::device_drivers().iter().enumerate() {
        println!("    {}. {}", i + 1, driver.compatible());
    }

    println!("[Info] {} chars written", bsp::console().chars_written());
    println!("[Info] Echoing input");

    bsp::gpio().setup(0, 1, interface::gpio::Pud::PudOff);
    bsp::gpio().output(0, 1);
    bsp::gpio().input(1);

    bsp::gpio().setup(1, 1, interface::gpio::Pud::PudUp);
    bsp::gpio().setup(2, 1, interface::gpio::Pud::PudDown);

    loop {
        let c = bsp::console().read_char();
        bsp::console().write_char(c);
    }
}
*/
fn kernel_main() -> ! {
    use core::time::Duration;
    use interface::time::Timer;
    use interface::gpio::GPIOAll;

    info!("Booting on: {}", bsp::board_name());
    info!(
        "Architectural timer resolution: {} ns",
        arch::timer().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    for (i, driver) in bsp::device_drivers().iter().enumerate() {
        info!("      {}. {}", i + 1, driver.compatible());
    }

    // Test a failing timer case.
    arch::timer().spin_for(Duration::from_nanos(1));

    bsp::gpio().setup(0, 1, interface::gpio::Pud::PudOff);
    bsp::gpio().output(0, 1);
    bsp::gpio().input(1);

    bsp::gpio().setup(1, 1, interface::gpio::Pud::PudUp);
    bsp::gpio().setup(2, 1, interface::gpio::Pud::PudDown);

    loop {
        info!("Spinning for 1 second");
        arch::timer().spin_for(Duration::from_secs(1));
    }
}

# assign-resources

This crate contains a macro to help assign and split up resources from a
struct, such as the `Peripherals` struct provided by embedded PACs and HALs,
into many smaller structs which can be passed to other tasks or functions.

It's best explained with the example below. Here we define new structs named
`UsbResources` and `LedResources`, each containing some IO pins and a
peripheral, and generate a new `split_resources!()` macro. When called,
the `split_resources!()` macro takes `PA12`, `PA11`, and `USB` out of
`p: Peripherals` and uses them to create the field `usb: UsbResources`, and
similarly creates the field `leds: LedResources` in the returned object. We can
then move these new structs into our tasks, which access the resources by name.

We can also label some resources with type aliases, so function signatures can
refer to that type as well.

```rust
use assign_resources::assign_resources;
use embassy_stm32::peripherals;

assign_resources! {
    usb: UsbResources {
        dp: PA12,
        dm: PA11,
        usb: USB,
    }
    leds: LedResources {
        r: PA2,
        g: PA3,
        b: PA4,
        tim2: TIM2 = PWMTimer,
    }
}

#[embassy_executor::task]
async fn usb_task(r: UsbResources) {
    // use r.dp, r.dm, r.usb
}

async fn setup_leds<'a>(r: LedResources) -> SimplePWM<'a, PWMTimer> {
    // setup three channel PWM (one for each color)
}

#[embassy_executor::task]
async fn led_task(rgb_pwm: SimplePWM<'a, PWMTimer>) {
    // use rgb_pwm
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_stm32::init(Default::default());
    let r = split_resources!(p);

    let rgb_pwm = setup_leds(r.leds);

    spawner.spawn(usb_task(r.usb)).unwrap();
    spawner.spawn(led_task(rgb_pwm)).unwrap();

    // can still use p.PA0, p.PA1, etc
}
```

This has a few advantages: you only need to write the specific pin names like
`PA12` in one place and can refer to them by name thereafter, you only have one
argument for each task instead of potentially very many, and you don't need
to write out lots of code to split the resources up. If you're targetting
multiple different hardware, you can use `#[cfg]` to change pin allocations
in just one place.


## Definition in a library
The following code would go in the library file, i.e., `lib.rs`
```rust
use assign_resources::assign_resources;
use embassy_stm32::peripherals;

assign_resources! {
    usb: UsbResources {
        dp: PA12,
        dm: PA11,
        usb: USB_OTG_FS,
    }
    leds: LedResources {
        r: PA2,
        g: PA3,
        b: PA4,
        tim2: TIM2,
    }
}
```

and the resources can be accessed from outside, e.g., in `my_bin.rs`:
```rust
// import `AssignedResources`, the `split_resources` macro and all custom resources structs from the library
use my_library::{AssignedResources, LedResources, UsbResources, split_resources};

#[embassy_executor::task]
async fn usb_task(r: UsbResources) {
    // use r.dp, r.dm, r.usb
}
#[embassy_executor::task]
async fn led_task(r: LedResources) {
    // use r.r, r.g, r.b, r.tim2
}
#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_stm32::init(Default::default());
    let r = split_resources!(p);
    spawner.spawn(usb_task(r.usb)).unwrap();
    spawner.spawn(led_task(r.leds)).unwrap();
    // can still use p.PA0, p.PA1, etc
}
``````

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

`SPDX-License-Identifier: Apache-2.0 OR MIT`

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

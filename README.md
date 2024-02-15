# assign-resources

This crate contains a macro to help assign and split up resources from a
struct, such as the `Peripherals` struct provided by embedded PACs and HALs,
into many smaller structs which can be passed to other tasks or functions.

It's best explained with the example below. Here we define new structs named
`UsbResources` and `LedResources`, each containing some IO pins and a
peripheral, and generate new `usb_resources!` and `led_resources!` macros. These macros will construct the respective types from the `Peripherals` instance. We can
then move these new structs into our tasks.

Resources type aliases may be generated, so function signatures can
refer to that type as well and any changes are propagated.

```rust
use embassy_stm32::peripherals::*;
use assign_resources::assigned_resources;

#[assigned_resources]
struct UsbResources {
    dp: PA12,
    dm: PA11,
    usb: USB,
}

#[assigned_resources]
struct LedResources {
    r: PA2,
    g: PA3,
    b: PA4,
    #[alias(PWMTimer)] // make an alias for this resource
    tim2: TIM2,
}

#[embassy_executor::task]
async fn usb_task(r: UsbResources) {
    // use r.dp, r.dm, r.usb
}

async fn setup_leds<'a>(r: LedResources) -> SimplePWM<'a, PWMTimer> {
    // setup three channel PWM (one for each color)       ^ alias used here
}

#[embassy_executor::task]
async fn led_task(rgb_pwm: SimplePWM<'a, PWMTimer>) {
    // use rgb_pwm                       ^ alias used here
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_stm32::init(Default::default());

    let rgb_pwm = setup_leds(led_resources!(p));

    spawner.spawn(usb_task(usb_resources!(p))).unwrap();
    spawner.spawn(led_task(rgb_pwm)).unwrap();

    // can still use p.PA0, p.PA1, etc
}
```

This has a few advantages: you only need to write the specific pin names like
`PA12` in one place and can refer to them by name thereafter, you only have one
argument for each task instead of potentially very many, and you don't need
to write out lots of code to split the resources up. If you're targetting
multiple different hardware versions, you can use `#[cfg]` to change pin allocations
in just one place.

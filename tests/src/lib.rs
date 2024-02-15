#[cfg(test)]
mod dummy;

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use crate::dummy::{Peripherals, PA11};

    use super::dummy::{self as peripherals, PA12, PA2, PA3, PA4, TIM2, USB_OTG_FS};
    use assign_resources::assigned_resources;

    #[assigned_resources]
    #[allow(non_snake_case)] // outer attribute
    struct UsbResources {
        DP: PA12,
        dm: peripherals::PA11, // user-provided type is flexible
        usb: USB_OTG_FS,
    }

    #[assigned_resources]
    struct LedResources {
        r: PA2,
        g: PA3,
        b: PA4,
        #[cfg(not(bogus_flag))] // inner attribute (with alias as well)
        #[alias = PWMTimer] // optional attribute to specify a type alias
        tim2: TIM2,
    }

    /// tests basic usage, type resolution, aliases, and attribute persistence
    #[test]
    fn basic() {
        let p = Peripherals::new();
        let leds = led_resources!(p);
        let usb = usb_resources!(p);

        assert_eq!(leds.r.type_id(), TypeId::of::<PA2>());
        assert_eq!(leds.g.type_id(), TypeId::of::<PA3>());
        assert_eq!(leds.b.type_id(), TypeId::of::<PA4>());
        assert_eq!(leds.tim2.type_id(), TypeId::of::<TIM2>());
        assert_eq!(leds.tim2.type_id(), TypeId::of::<PWMTimer>()); // verify type alias

        assert_eq!(usb.DP.type_id(), TypeId::of::<PA12>());
        assert_eq!(usb.dm.type_id(), TypeId::of::<PA11>());
        assert_eq!(usb.usb.type_id(), TypeId::of::<USB_OTG_FS>());
    }
}

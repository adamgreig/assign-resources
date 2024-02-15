#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub(crate) struct PA2;
pub(crate) struct PA3;
pub(crate) struct PA4;
pub(crate) struct PA11;
pub(crate) struct PA12;
pub(crate) struct TIM2;
pub(crate) struct USB_OTG_FS;

pub(crate) struct Peripherals {
    pub(crate) PA2: PA2,
    pub(crate) PA3: PA3,
    pub(crate) PA4: PA4,
    pub(crate) PA11: PA11,
    pub(crate) PA12: PA12,
    pub(crate) TIM2: TIM2,
    pub(crate) USB_OTG_FS: USB_OTG_FS,
}

impl Peripherals {
    pub(crate) const fn new() -> Self {
        Self {
            PA2,
            PA3,
            PA4,
            PA11,
            PA12,
            TIM2,
            USB_OTG_FS,
        }
    }
}

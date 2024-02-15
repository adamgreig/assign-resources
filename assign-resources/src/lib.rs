use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Attribute, Ident, ItemStruct, ItemType, Meta, Type};

/// Mark a struct as a resource for extraction from the `Peripherals` instance.
///
/// # Example
/// ```rust
/// use embassy_stm32::peripherals::*;
/// use assign_resources::assigned_resources;
///
/// #[assigned_resources]
/// struct UsbResources {
///     dp: PA12,
///     dm: PA11,
///     usb: USB,
/// }
///
/// #[assigned_resources]
/// struct LedResources {
///     r: PA2,
///     g: PA3,
///     b: PA4,
///     #[alias(PWMTimer)]
///     tim2: TIM2,
/// }
///
/// #[embassy_executor::task]
/// async fn usb_task(r: UsbResources) {
///     // use r.dp, r.dm, r.usb
/// }
///
/// async fn setup_leds<'a>(r: LedResources) -> SimplePWM<'a, PWMTimer> {
///     // setup three channel PWM (one for each color)
/// }
///
/// #[embassy_executor::task]
/// async fn led_task(rgb_pwm: SimplePWM<'a, PWMTimer>) {
///     // use rgb_pwm
/// }
///
/// #[embassy_executor::main]
/// async fn main(spawner: embassy_executor::Spawner) {
///     let p = embassy_stm32::init(Default::default());
///
///     let rgb_pwm = setup_leds(led_resources!(p));
///
///     spawner.spawn(usb_task(usb_resources!(p))).unwrap();
///     spawner.spawn(led_task(rgb_pwm)).unwrap();
///
///     // can still use p.PA0, p.PA1, etc
/// }
/// ```
#[proc_macro_attribute]
pub fn assigned_resources(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut s: ItemStruct = syn::parse2(item.into()).expect("Resource item must be a struct.");

    let mut aliases = Vec::new();

    // search for "alias" attribute and remove for rendering
    s.fields.iter_mut().for_each(|field| {
        field.attrs = field
            .attrs
            .iter()
            .cloned()
            .filter(|attr| {
                if let Meta::NameValue(alias) = &attr.meta {
                    if let Some(ident) = alias.path.get_ident() {
                        if ident.to_string().eq("alias") {
                            let alias_value = &alias.value;
                            let alias_type = &field.ty;
                            let alias_stmt: ItemType =
                                syn::parse2(quote! { type #alias_value = #alias_type; }).unwrap();
                            aliases.push(alias_stmt);
                            return false;
                        }
                    }
                }

                true
            })
            .collect();
    });

    let use_macro_ident = Ident::new(
        inflector::cases::snakecase::to_snake_case(s.ident.to_string().as_str()).as_str(),
        Span::call_site(),
    );

    let ident = &s.ident;
    let field_idents: Vec<Ident> = s
        .fields
        .iter()
        .cloned()
        .map(|field| field.ident.unwrap())
        .collect();
    let field_types: Vec<Type> = s
        .fields
        .iter()
        .cloned()
        .map(|field| {
            if let Type::Path(ty) = field.ty {
                let ident = &ty.path.segments.last().unwrap().ident;
                syn::parse2(quote! { #ident }).unwrap()
            } else {
                field.ty
            }
        })
        .collect();
    let field_attrs: Vec<Vec<Attribute>> =
        s.fields.iter().cloned().map(|field| field.attrs).collect();

    quote! {
        #(
            #aliases
        )*

        #s

        macro_rules! #use_macro_ident {
            ( $P:ident ) => {
                #ident {
                    #(
                        #(
                            #field_attrs
                        )*
                        #field_idents: $P.#field_types
                    ),*
                }
            };
        }
    }
    .into()
}

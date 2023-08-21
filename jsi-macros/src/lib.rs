use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

mod host_event;
mod host_object;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn host_object(
    _attr_input: proc_macro::TokenStream,
    attr_target: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let impl_block = parse_macro_input!(attr_target as host_object::HostObjectImpl);
    let s = proc_macro::TokenStream::from(impl_block.0);
    // println!("{}", s);
    s
}

#[proc_macro_error]
#[proc_macro_derive(HostEvent)]
pub fn host_event_emitter(target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let impl_block = parse_macro_input!(target as host_event::HostEventImpl);
    proc_macro::TokenStream::from(impl_block.0)
}

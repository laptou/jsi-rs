use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

mod host_event;
mod host_object;


/// A macro that makes it easy to define functions and properies on host
/// objects.
/// 
/// ```no_run
/// struct ExampleHostObject;
/// 
/// #[host_object]
/// impl ExampleHostObject {
///     pub fn time(&self, _rt: &mut RuntimeHandle) -> anyhow::Result<i64> {
///         Ok(3200)
///     }
/// }
/// ```
/// 
/// It is used on `impl` blocks and on the functions within the `impl` block.
/// 
/// - Add `#[host_object(getter)]` to designate an `fn` as a getter. This will
///   allow JavaScript code to read the member like a property instead of having
///   to call it like a function. Getters should be `fn(&self, &mut
///   RuntimeHandle) -> Result<T>` where `T: IntoValue`.
/// 
///   If the `fn` name in Rust starts with `get_`, this prefix will be removed.
///   
/// - Add `#[host_object(setter)]` to designate an `fn` as a setter. This will
///   allow JavaScript code to write the like a property instead of having to
///   call it like a function. Setters should be `fn(&self, &mut RuntimeHandle,
///   value: JsiValue) -> Result<()>`.
/// 
///   If the `fn` name in Rust starts with `set_`, this prefix will be removed.
///   
/// - All other methods will appear as methods on the host object in JavaScript.
///   The first two arguments should be `&self` and a `&mut Runtime`.
/// 
/// By default, all member names in a `host_object` block are converted from
/// `snake_case` to `camelCase` to give them idiomatic names in JavaScript. To
/// override this, you can set a name with `#[host_object(method as
/// custom_method_name)]` or `#[host_object(getter as custom_prop_name)]`.
/// 
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

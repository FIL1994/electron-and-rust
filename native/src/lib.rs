#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::vm::{Call, JsResult};
use neon::mem::Handle;
use neon::js::{JsString, JsNumber, JsUndefined, Variant, Object};

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node from rust").unwrap())
}

fn threading_hint(call: Call) -> JsResult<JsNumber> {
    Ok(JsNumber::new(call.scope, num_cpus::get() as f64))
}

fn my_func(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;

    let two:Handle<JsString> = try!(try!(call.arguments.require(scope,1)).check::<JsString>());

    let arg0 = call.arguments.get(scope, 0).unwrap();
    match arg0.variant() {
        Variant::String(val) => println!("the argument is a String"),
        Variant::Object(val) => {
            println!("the argument is an Object");
            let prop_names = val.get_own_property_names(scope)?;
        },
        _ => println!("the argument is something else")
    }

    Ok(JsUndefined::new())
}

register_module!(m, {
    try!(m.export("hello", hello));
    try!(m.export("threading_hint", threading_hint));
    try!(m.export("my_func", my_func));

    Ok(())
});

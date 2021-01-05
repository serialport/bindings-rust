#[macro_use]
extern crate napi_derive;

use napi::{self, CallContext, Env, JsNumber, JsObject, JsUndefined, Property, Result};
use std::convert::TryInto;
use serialport::{SerialPort};

mod list;
use list::list;

#[module_exports]
pub fn init(mut exports: JsObject, env: Env) -> Result<()> {
    exports.create_named_method("list", list)?;
    Ok(())
}

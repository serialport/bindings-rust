#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;
use napi::{CallContext, Env, JsNumber, JsObject, Result, Task};
use serialport::{available_ports};
use tokio::{self,task};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("list", list)?;
  println!("module loaded");
  Ok(())
}

#[js_function(1)]
pub fn list(ctx: CallContext) -> Result<JsObject> {

  let task = task::spawn_blocking(|| {
    available_ports()
  });

  ctx.env.execute_tokio_future(
    task,
    |&mut env, data| env.create_buffer_with_data(data).map(|v| v.into_raw()),
  )
}



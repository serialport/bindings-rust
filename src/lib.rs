#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{self, CallContext, JsObject, Result, Status, Env};
use serialport::{available_ports, SerialPortType};
use tokio::{self,task};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("list", list)?;
  Ok(())
}

async fn async_list() -> Result<Vec<serialport::SerialPortInfo>>{
  let handel = task::spawn_blocking(|| {
    available_ports()
  });
  handel.await.unwrap().map_err(|e|
    napi::Error::new(
      Status::GenericFailure,
      format!("failed to list ports, {}", e),
    )
  )
}

fn list_to_js_objects(env: &mut Env, data: Vec<serialport::SerialPortInfo>) -> Result<JsObject>{
  let mut arr = env.create_array_with_length(data.len())?;
  for (i, port_info) in data.iter().enumerate() {
    let mut js_port_info = env.create_object()?;
    let path = env.create_string(&port_info.port_name)?;
    js_port_info.set_named_property("path", path)?;
    js_port_info.set_named_property("manufacturer", env.get_undefined()?)?;
    js_port_info.set_named_property("serialNumber", env.get_undefined()?)?;
    js_port_info.set_named_property("productName", env.get_undefined()?)?;
    js_port_info.set_named_property("vendorId", env.get_undefined()?)?;
    js_port_info.set_named_property("productId", env.get_undefined()?)?;
    js_port_info.set_named_property("portType", env.create_string(&format!("{:?}", port_info.port_type))?)?;

    if let SerialPortType::UsbPort(usb_info) = port_info.port_type.clone() {
      let vid = format!("{:x}", usb_info.vid);
      js_port_info.set_named_property("vendorId", env.create_string(&vid)?)?;

      let pid = format!("{:x}", usb_info.pid);
      js_port_info.set_named_property("productId", env.create_string(&pid)?)?;

      if let Some(manufacturer) = usb_info.manufacturer {
        js_port_info.set_named_property("manufacturer", env.create_string(&manufacturer)?)?
      }

      if let Some(serial_number) = usb_info.serial_number {
        js_port_info.set_named_property("serialNumber", env.create_string(&serial_number)?)?
      }

      if let Some(product) = usb_info.product {
        js_port_info.set_named_property("productName", env.create_string(&product)?)?
      }
    }
    arr.set_element(i as u32, js_port_info)?;
  }

  Ok(arr)
}

#[js_function(1)]
pub fn list(ctx: CallContext) -> Result<JsObject> {
  ctx.env.execute_tokio_future(
    async_list(),
    list_to_js_objects
  )
}



use neon::prelude::*;
extern crate serialport;
use serialport::{available_ports, SerialPortInfo};

struct BackgroundTask;

impl Task for BackgroundTask {
    // If the computation does not error, it will return an i32.
    // Otherwise, it will return a String as an error
    type Output = Vec<SerialPortInfo>;
    type Error = String;
    type JsEvent = JsArray;

    // Perform expensive computation here. What runs in here
    // will not block the main thread. Will run in a background
    // thread
    fn perform(&self) -> Result<Vec<SerialPortInfo>, String> {
        match available_ports() {
            Ok(result) => return Ok(result),
            Err(err) => return Err(err.description),
        };
    }

    // When perform() is finished running, complete() will convert
    // the result of the task to a JS value. In this case we are
    // converting a Rust i32 to a JsNumber. This value will be passed
    // to the callback. perform() is executed on the main thread at
    // some point after the background task is completed.
    fn complete(self, mut cx: TaskContext, result: Result<Vec<SerialPortInfo>, String>) -> JsResult<JsArray> {
        let ports = result.unwrap();
        let js_array = JsArray::new(&mut cx, ports.len() as u32);

        // Iterate over the rust Vec and map each value in the Vec to the JS array
        for (i, port_info) in ports.iter().enumerate() {
            let port_name = cx.string(&port_info.port_name);
            let js_port_obj = JsObject::new(&mut cx);
            js_port_obj.set(&mut cx, "path", port_name).unwrap();
            js_array.set(&mut cx, i as u32, js_port_obj).unwrap();
        }

        Ok(js_array)
    }
}

pub fn perform_async_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // Take a function as an argument. This function should have the following
    // signature: `function callback(err, value) {}`. The JS value returned from
    // complete() is passed as the `value` and the error message "This will fail"
    // is passed as the `err`
    let f = cx.argument::<JsFunction>(0)?;
    BackgroundTask.schedule(f);
    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("list", perform_async_task)
});

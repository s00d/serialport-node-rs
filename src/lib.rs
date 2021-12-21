#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serialport::{available_ports, SerialPort, SerialPortType};
use std::collections::HashMap;
use std::str;
use std::time::Duration;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi]
pub struct Port {
  #[napi(readonly)]
  pub path: String,

  port: Box<dyn SerialPort>,
}

#[napi]
impl Port {
  /// This is the constructor
  #[napi(constructor)]
  pub fn new(path: String) -> Result<Port> {
    let baud_rate = 115_200;
    let timeout = 10;
    let builder = serialport::new(path.clone(), baud_rate).timeout(Duration::from_millis(timeout));

    let port = builder.open();

    match port {
      Ok(port) => Ok(Port { path, port }),
      Err(e) => Err(Error::new(
        Status::GenericFailure,
        format!("Failed to open \"{}\". Error: {}", path, e),
      )),
    }
  }

  #[napi(getter)]
  pub fn get_used_path(&self) -> &str {
    self.path.as_str()
  }

  #[napi]
  pub fn write(&mut self, data: String) {
    self.port.write_all(data.as_bytes()).unwrap();
  }

  #[napi]
  pub fn read(&mut self) -> Result<String> {
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    match self.port.read(serial_buf.as_mut_slice()) {
      Ok(_t) => Ok(format!("{:?}", &serial_buf)),
      Err(e) => Err(Error::new(
        Status::GenericFailure,
        format!("Failed to read \"{}\". Error: {}", &self.path, e),
      )),
    }
  }
}

#[napi]
fn ports_list(env: Env) -> HashMap<String, Object> {
  let mut map = HashMap::new();

  let ports = available_ports().expect("No ports found!");
  for p in ports {
    let mut obj = env.create_object().unwrap();
    obj.set("path", p.port_name.clone()).unwrap();
    obj.set("VID", "").unwrap();
    obj.set("PID", "").unwrap();
    obj.set("serial_number", "").unwrap();
    obj.set("manufacturer", "").unwrap();
    obj.set("product", "").unwrap();

    match p.port_type {
      SerialPortType::UsbPort(info) => {
        obj.set("type", "USB").unwrap();
        obj.set("VID", info.vid.to_string()).unwrap();
        obj.set("PID", info.pid.to_string()).unwrap();
        obj
          .set(
            "serial_number",
            info.serial_number.as_ref().map_or("", String::as_str),
          )
          .unwrap();
        obj
          .set(
            "manufacturer",
            info.manufacturer.as_ref().map_or("", String::as_str),
          )
          .unwrap();
        obj
          .set("product", info.product.as_ref().map_or("", String::as_str))
          .unwrap();
      }
      SerialPortType::BluetoothPort => {
        obj.set("type", "Bluetooth").unwrap();
      }
      SerialPortType::PciPort => {
        obj.set("type", "PCI").unwrap();
      }
      SerialPortType::Unknown => {
        obj.set("type", "Unknown").unwrap();
      }
    }
    map.insert(p.port_name.clone(), obj);
  }
  map
}


use std::net::{TcpStream, Shutdown, ToSocketAddrs, SocketAddr};
use std::time::Duration;
use serialport::{SerialPort, open_with_settings, ClearBuffer};

mod modbus_core;

pub struct Modbus{

}

impl Drop for Modbus{
  fn drop(&mut self){}
}

impl Modbus {
  pub fn new_tcp()->Modbus{
    Modbus{}
  }

  pub fn new_rtu()->Modbus{
    Modbus{}
  }

  pub fn connect(&mut self){}

  pub fn close(&mut self){}

  pub fn set_slave(&mut self){}

  pub fn read_bits(&self) {}

  pub fn read_input_bits(&self) {}

  pub fn read_registers(&self) {}

  pub fn read_input_registers(&self) {}

  pub fn report_slave_id(&self) {}

  pub fn write_bit(&self) {}

  pub fn write_register(&self) {}

  pub fn write_bits(&self) {}

  pub fn write_registers(&self) {}

  pub fn write_and_read_registers(&self) {}
}

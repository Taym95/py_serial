use cpython::{py_fn, py_module_initializer, PyResult, Python};
use serial2::SerialPort;

pub fn add(_py: Python, left: i64, right: i64) -> PyResult<i64> {
    let port = SerialPort::open("/dev/ttyUSB0", 115200).unwrap();
    let mut buffer = [0; 256];

    let s = String::from_utf8_lossy(&buffer);

    println!("result: {}", s);

    Ok(left + right)
}

py_module_initializer!(py_serial, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "sum_int", py_fn!(py,add(a: i64, b: i64)))?;
    Ok(())
});

extern crate byteorder;
extern crate i2cdev;

pub mod vl53l0x;

use vl53l0x::VL53L0X;

fn try_open_tof(filename: &'static str) -> Option<VL53L0X> {
    let front = match VL53L0X::new(filename) {
        Ok(front) => front,
        Err(e) => {
            println!("Failed to open TOF {:?} ", e);
            return None;
        }
    };
    println!("Success {:?}", filename);
    return Some(front);
}

fn get_distance(tof: &mut Option<VL53L0X>) -> u16 {
    let dist: u16;
    match tof {
        None => dist = 0,
        Some(ref mut tof) => {
            dist = tof.read();
        }
    }
    return dist;
}

fn application() -> () {
    let mut tof = try_open_tof("/dev/i2c-1");
    loop {
        let dist = get_distance(&mut tof);
        println!("Distance = {}", dist);
        if dist == 0 {
            break;
        }
    }
}

fn main() {
    application();
}

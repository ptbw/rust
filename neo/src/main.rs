use neo::pixel::*;

use std::{thread, time};

fn test() {
    let mut pixel = build_pixel();
    loop {
        pixel.red();
        pixel.render();
        println!("Red");
        thread::sleep(time::Duration::from_millis(1000));
        pixel.green();
        pixel.render();
        println!("Green");
        thread::sleep(time::Duration::from_millis(1000));
        pixel.blue();
        pixel.render();
        println!("Blue");
        thread::sleep(time::Duration::from_millis(1000));
        pixel.yellow();
        pixel.render();
        println!("Yellow");
        thread::sleep(time::Duration::from_millis(1000));
        pixel.all_off();
        pixel.render();
        println!("All Off");
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    test();
}

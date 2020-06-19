use rppal::i2c::I2c;
//use async_std::task::sleep;
use std::{time::Duration, thread};
//use futures::executor::block_on;
//use pcalib_test::PCA9685;

fn main() {
    thread::sleep(Duration::from_secs(2));

    let osc = 25 * (10^6);

    let mut bus = I2c::new().expect("Failed to create i2c bus");
    bus.set_slave_address(0b1000000);

    //Set Sleep Value to 1
    let mut buf = vec![0];
    bus.write_read(&vec![0x00, 0b00010001], &mut buf).expect("Failed to read bytes");
    println!("Mode 1, (Go to sleep) {:#b}", buf.get(0).unwrap());


    let mut prescale_buf = vec![0];
    bus.write_read(&vec![0xfe], &mut prescale_buf);
    println!("Old Prescale Value {:?}", prescale_buf);

    //Calculate Prescale
    // The refresh rate in Hz
    let refresh = 50;

    let prescale = ((25_000_000.0) / (4096.0 * refresh as f32)) - 1.0;

    //Write Prescalar
    bus.write_read(&vec![0xfe, prescale as u8], &mut prescale_buf);
    println!("New Prescale Value {:?}", prescale_buf);

    // # Write to Servo Register 1
    //Split u16 to two u8 (Full 2)
    let max = (0x01, 0x9A);
    let mid = (0x01, 0x33);
    let min = (0x00, 0xCC);

    //Read Buffer
    let mut buf = vec![0];
    //Write to LED on High
    bus.write(&vec![0x6, 0]);
    bus.write(&vec![0x7, 0]);
    bus.write(&vec![0x8, max.1]);
    bus.write(&vec![0x9, max.0]);
    println!("Set Low & High");

    bus.write_read(&vec![0x6], &mut buf);
    println!("Low: {:#b} (Should be 0b0)", buf.get(0).unwrap());
    bus.write_read(&vec![0x7], &mut buf);
    println!("Low: {:#b} (Should be 0b0)", buf.get(0).unwrap());
    bus.write_read(&vec![0x8], &mut buf);
    println!("Low: {:#b} (Should be 0b11111111)", buf.get(0).unwrap());
    bus.write_read(&vec![0x9], &mut buf);
    println!("Low: {:#b} (Should be 0b1111)", buf.get(0).unwrap());

    // # Start Device
    //Read Mode 1
    let mut buf = vec![0];
    bus.write_read(&vec![0x00], &mut buf).expect("Failed to read bytes");
    let mode = buf.get(0).unwrap();
    //Clear Bit 4 (Turn On)
    bus.write(&vec![0x00, mode - 0x10]).expect("Failed to read bytes");
    //Wait fo rat least 500us, stabilize oscillator
    thread::sleep(Duration::from_micros(750));
    //Write a logic 1 to bit 7
    bus.write(&vec![0x00, 0x41]);

    bus.write_read(&vec![0x00], &mut buf);
    println!("Mode 1 Register (First Start): {:#b}", buf.get(0).unwrap());

    thread::sleep(Duration::from_secs(2));

    bus.write(&vec![0x6, 0]);
    bus.write(&vec![0x7, 0]);
    bus.write(&vec![0x8, min.1]);
    bus.write(&vec![0x9, min.0]);


    thread::sleep(Duration::from_secs(2));

    bus.write(&vec![0x6, 0]);
    bus.write(&vec![0x7, 0]);
    bus.write(&vec![0x8, max.1]);
    bus.write(&vec![0x9, max.0]);

    thread::sleep(Duration::from_secs(2));

}
fn restart(bus: &mut I2c) { 
    //Read Mode 1
    let mut buf = vec![0];
    bus.write_read(&vec![0x00], &mut buf).expect("Failed to read bytes");
    let mode = buf.get(0).unwrap();
    println!("Mode 1 Register (Sleeping): {:#b}", mode);
    //If bit 7 is on
    if buf.get(0).unwrap() & 0x40 == 0x40 {
        //Clear Bit 4 (Turn On)
        bus.write(&vec![0x00, mode - 0x10]).expect("Failed to read bytes");
        //Wait fo rat least 500us, stabilize oscillator
        thread::sleep(Duration::from_micros(750));
        //Write a logic 1 to bit 7
        bus.write(&vec![0x00, 0x41]);
    }
    bus.write_read(&vec![0x00], &mut buf);
    println!("Mode 1 Register (Restarted): {:#b}", buf.get(0).unwrap());
}
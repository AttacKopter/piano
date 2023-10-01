use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Channel, Pca9685};
use std::fs;
use crate::midi_parser::MotorCommand;

pub fn get_controllers() -> Vec<Pca9685<I2cdev>> { 
    let addresses: Vec<(bool, bool, bool, bool, bool, bool)> = vec!(
    (false, false, false, false, false, false),
    (false, false, false, false, false, true),
    (false, false, false, false, true, false),
    (false, false, false, false, true, true),
    (false, false, false, true, false, false),
    (false, false, false, true, false, true),
    (false, false, false, true, true, false),
    );
    let mut controllers: Vec<Pca9685<I2cdev>> = Vec::new();
    for address in addresses {
        let dev = I2cdev::new("/dev/i2c-1").unwrap();
        let pwm = Pca9685::new(dev, address).unwrap();
        controllers.push(pwm);
    }
    controllers
}

fn get_tuning() -> Vec<(u16, u16)> {
    let data = fs::read_to_string("/tuning").expect("Unable to read file");
    let tuning: Vec<(u16, u16)> = data.lines().map(|line| {
        (line.split(",").collect::<Vec<&str>>()[0].parse::<u16>().unwrap(), line.split(",").collect::<Vec<&str>>()[1].parse::<u16>().unwrap())
    }).collect();
    tuning
}

fn move_motor(motor: i16, percent: u16, tuning: &Vec<(u16, u16)>, controllers: &mut Vec<Pca9685<I2cdev>>) {

    let min = tuning.get(motor as usize).unwrap().0;
    let max = tuning.get(motor as usize).unwrap().1;
    let off: u16 = percent/100 * (max-min) + min;

    set_pwm(motor, controllers, off)
}

pub fn set_pwm(motor: i16, controllers: &mut Vec<Pca9685<I2cdev>>, off: u16) {   
    let pwm: &mut Pca9685<I2cdev> = &mut controllers[(motor/12) as usize];     
    let channel = 11 - motor % 12;
    match channel {
        0 => pwm.set_channel_on_off(Channel::C0, 0, off).unwrap(),
        1 => pwm.set_channel_on_off(Channel::C1, 0, off).unwrap(),            2 => pwm.set_channel_on_off(Channel::C2, 0, off).unwrap(),
        3 => pwm.set_channel_on_off(Channel::C3, 0, off).unwrap(),
        4 => pwm.set_channel_on_off(Channel::C4, 0, off).unwrap(),
        5 => pwm.set_channel_on_off(Channel::C5, 0, off).unwrap(),            6 => pwm.set_channel_on_off(Channel::C6, 0, off).unwrap(),
        7 => pwm.set_channel_on_off(Channel::C7, 0, off).unwrap(),
        8 => pwm.set_channel_on_off(Channel::C8, 0, off).unwrap(),
        9 => pwm.set_channel_on_off(Channel::C9, 0, off).unwrap(),
        10 => pwm.set_channel_on_off(Channel::C10, 0, off).unwrap(),
        11 => pwm.set_channel_on_off(Channel::C11, 0, off).unwrap(),
        _ => panic!()
    }
}

pub fn zero() {
    let mut controllers = get_controllers();
    let tuning = get_tuning();
    for i in 0..84 {
        move_motor(i, 0, &tuning , &mut controllers);
    }
}

pub fn play_song(motor_commands: Vec<MotorCommand>) {

}
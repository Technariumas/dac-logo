extern crate regex;
extern crate chan_signal;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::sync::mpsc::{self, TryRecvError, Receiver};
use std::thread;
use chan_signal::Signal;
use std::io::{BufRead, BufReader};

pub struct Config {
    pub input: String,
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct RPiHardwarePWM {
    channel: u32,
    period: u32,
    sysfs_root: String,
}

impl RPiHardwarePWM {
    fn new(channel: u32, period: u32) -> RPiHardwarePWM {
        let sysfs_root = format!("/sys/class/pwm/pwmchip0/pwm{}", channel);
        RPiHardwarePWM {
            channel,
            period,
            sysfs_root,
        }
    }

    fn start(&self) -> () {
        match self.export() {
            Ok(_) => (),
            Err(_) => {
                self.unexport().unwrap();
                self.export().unwrap();
            }
        };
        self.set_period().expect(&format!(
            "Cannot set period for {:?}",
            self
        ));
        self.disable().expect(&format!("Cannot disable {:?}", self));
        self.enable().expect(&format!("Cannot enable {:?}", self));
    }

    fn write_int(&self, name: &str, value: u32) -> Result<(), Box<Error>> {
        let path = format!("{}/{}", self.sysfs_root, name);
        let mut file = File::create(&path)?;
        let buf = format!("{}", value);
        // println!("{:?}<-{:?}", &path, &buf);
        file.write(buf.as_bytes())?;
        Ok(())
    }

    fn set_period(&self) -> Result<(), Box<Error>> {
        // self.set_duty_cycle(0)?;
        self.write_int("period", self.period)
    }

    fn set_duty_cycle(&self, duty_cycle: u32) -> Result<(), Box<Error>> {
        self.write_int("duty_cycle", duty_cycle)
    }

    fn enable(&self) -> Result<(), Box<Error>> {
        self.write_int("enable", 1)
    }

    fn disable(&self) -> Result<(), Box<Error>> {
        self.write_int("enable", 0)
    }

    fn export(&self) -> Result<(), Box<Error>> {
        let path = "/sys/class/pwm/pwmchip0/export";
        let mut file = File::create(path)?;
        file.write(format!("{}", self.channel).as_bytes())?;
        Ok(())
    }

    fn unexport(&self) -> Result<(), Box<Error>> {
        let path = "/sys/class/pwm/pwmchip0/unexport";
        let mut file = File::create(path)?;
        file.write(format!("{}", self.channel).as_bytes())?;
        Ok(())
    }
}

impl Drop for RPiHardwarePWM {
    fn drop(&mut self) {
        let _ = self.disable();
        let _ = self.unexport();
    }
}

fn parse_file(input: &String) -> Result<Vec<Point>, Box<Error>> {
    let re = Regex::new(r"^x(\d+)y(\d+)$").unwrap();
    let file = File::open(input).unwrap();
    let br = BufReader::new(&file);
    let mut points: Vec<Point> = Vec::new();
    for line in br.lines() {
        let l = line.unwrap();
        let cap = re.captures(&l).unwrap();
        let point = Point {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
        };
        points.push(point);
    }
    Ok(points)
}

const PWM_PERIOD: u32 = 1000;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (terminator_tx, terminator_rx) = mpsc::channel();

    let child = thread::spawn(move || worker(config, terminator_rx));

    println!("Press Ctrl-C to terminate");
    let s = signal.recv().unwrap();
    println!("received signal: {:?}", s);
    terminator_tx.send(())?;
    child.join().unwrap();
    Ok(())
}

fn worker(config: Config, terminator_rx: Receiver<()>) {
    let points = parse_file(&config.input).unwrap();

    let pwm0 = RPiHardwarePWM::new(0, PWM_PERIOD);
    let pwm1 = RPiHardwarePWM::new(1, PWM_PERIOD);
    pwm0.start();
    pwm1.start();

    let mut i = 0;
    loop {
        let point = &points[i];
        pwm0.set_duty_cycle(point.x).unwrap();
        pwm1.set_duty_cycle(point.y).unwrap();
        i += 1;
        if i >= points.len() {
            i = 0;
        }
        match terminator_rx.try_recv() {
            Ok(_) |
            Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
}

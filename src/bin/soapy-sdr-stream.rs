extern crate soapysdr;
extern crate num_complex;
extern crate byteorder;
extern crate getopts;
extern crate signalbool;

use std::env;
use std::cmp::min;
use std::fs::File;
use std::io::BufWriter;
use std::i64;
use std::process;
use byteorder::{ WriteBytesExt, LittleEndian };
use soapysdr::Direction::{Rx, Tx};
use getopts::Options;
use num_complex::Complex;

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();

    let mut opts = Options::new();
    opts.optopt("d", "device", "device filter", "DEVICE");
    opts.optopt("r", "receive", "receive data to file", "NAME");
    opts.optopt("t", "transmit", "transmit data from file", "NAME");
    opts.optopt("c", "channel", "channel of device (default 0)", "N");
    opts.optopt("f", "frequency", "center frequency", "HZ");
    opts.optopt("s", "rate", "sample rate", "HZ");
    opts.optopt("a", "antenna", "antenna name", "ANT");
    opts.optopt("b", "bandwidth", "baseband filter bandwidth", "HZ");
    opts.optopt("g", "gain", "gain in dB", "GAIN");
    opts.optopt("n", "samples", "with -r: number of samples (default unlimited)", "N");
    opts.optopt("n", "samples", "with -t: number of times to repeat file (default 1)", "N");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}\n Run `{} --help` for help.", e.to_string(), program);
            process::exit(2);
        }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} (-r FILE | -t FILE) [options]", program);
        eprint!("{}", opts.usage(&brief));
        return;
    }

    let (direction, fname) = match (matches.opt_str("r"), matches.opt_str("t")) {
        (Some(fname), None) => (Rx, fname),
        (None, Some(fname)) => (Tx, fname),
        _ => {
            eprintln!("Specify exactly one of --transmit FILE or --receive FILE");
            process::exit(2);
        }
    };

    let dev_filter = matches.opt_str("d").unwrap_or("".into());
    let devs = soapysdr::enumerate(&dev_filter[..]).expect("Error listing devices");

    let dev_args = match devs.len() {
        0 => {
            eprintln!("No matching devices found");
            process::exit(1);
        }
        1 => devs.into_iter().next().unwrap(),
        n => {
            eprintln!("{} devices found. Try one of:", n);
            for dev in devs {
                eprintln!("\t -d '{}'", dev);
            }
            process::exit(1);
        }
    };

    let dev = soapysdr::Device::new(dev_args).expect("Error opening device");

    let channel = matches.opt_str("c").map_or(0, |channel| {
        channel.parse::<usize>().expect("Invalid channel")
    });

    if let Some(freq) = matches.opt_str("f") {
        let freq = parse_num(&freq).expect("Invalid frequency");
        dev.set_frequency(direction, channel, freq, ()).expect("Failed to set frequency");
    }

    if let Some(rate) = matches.opt_str("s") {
        let rate = parse_num(&rate).expect("invalid sample rate");
        dev.set_sample_rate(direction, channel, rate).expect("failed to set sample rate");
    }

    if let Some(antenna) = matches.opt_str("a") {
        dev.set_antenna(direction, channel, antenna).expect("failed to set antenna");
    }

    if let Some(bw) = matches.opt_str("b") {
        let bw = parse_num(&bw).expect("invalid bandwidth");
        dev.set_bandwidth(direction, channel, bw).expect("failed to set sample rate");
    }

    if let Some(gain) = matches.opt_str("g") {
        let gain = gain.parse::<f64>().expect("invalid gain");
        dev.set_gain(direction, channel, gain).expect("failed to set gain");
    }

    let mut num = matches.opt_str("n").map_or(i64::MAX, |n| {
        parse_num(&n).expect("invalid number of samples") as i64
    });

    let sb = signalbool::SignalBool::new(
        &[signalbool::Signal::SIGINT], signalbool::Flag::Interrupt,
      ).unwrap();

    match direction {
        Rx => {
            let mut stream = dev.rx_stream::<Complex<f32>>(&[channel]).unwrap();
            let mut buf = vec![Complex::new(0.0, 0.0); stream.mtu().unwrap()];

            let mut outfile = BufWriter::new(File::create(fname).expect("error opening output file"));
            stream.activate(None).expect("failed to activate stream");

            while num > 0 && !sb.caught() {
                let read_size = min(num as usize, buf.len());
                let len = stream.read(&[&mut buf[..read_size]], 1_000_000).expect("read failed");

                for sample in &buf[..len] {
                    outfile.write_f32::<LittleEndian>(sample.re).unwrap();
                    outfile.write_f32::<LittleEndian>(sample.im).unwrap();
                }

                num -= len as i64;
            }
            stream.deactivate(None).expect("failed to deactivate");
        }
        Tx => {
            unimplemented!();
        }
    }
    println!("exiting");
}

fn parse_num(s: &str) -> Result<f64, std::num::ParseFloatError> {
         if s.ends_with("k") { s[..s.len()-1].parse::<f64>().map(|x| x * 1e3) }
    else if s.ends_with("M") { s[..s.len()-1].parse::<f64>().map(|x| x * 1e6) }
    else if s.ends_with("G") { s[..s.len()-1].parse::<f64>().map(|x| x * 1e9) }
    else { s.parse::<f64>() }
}

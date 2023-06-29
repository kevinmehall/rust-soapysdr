use std::env;
extern crate soapysdr;
use soapysdr::Direction::{Rx, Tx};
use std::fmt;

fn main() {
    let filter = env::args().nth(1).unwrap_or(String::new());

    for devargs in soapysdr::enumerate(&filter[..]).expect("Error listing devices") {
        println!("{}", devargs);

        let dev = soapysdr::Device::new(devargs).expect("Failed to open device");

        for channel in 0..(dev.num_channels(Rx).unwrap_or(0)) {
            print_channel_info(&dev, Rx, channel).expect("Failed to get channel info");
        }

        for channel in 0..(dev.num_channels(Tx).unwrap_or(0)) {
            print_channel_info(&dev, Tx, channel).expect("Failed to get channel info");
        }
    }
}

struct DisplayRange(Vec<soapysdr::Range>);

impl fmt::Display for DisplayRange {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        for (i, range) in self.0.iter().enumerate() {
            if i != 0 {
                write!(w, ", ")?
            }
            if range.minimum == range.maximum {
                write!(w, "{} MHz", range.maximum / 1e6)?
            } else {
                write!(w, "{} to {} MHz", range.minimum / 1e6, range.maximum / 1e6)?
            }
        }
        Ok(())
    }
}

fn print_channel_info(
    dev: &soapysdr::Device,
    dir: soapysdr::Direction,
    channel: usize,
) -> Result<(), soapysdr::Error> {
    let dir_s = match dir {
        Rx => "RX",
        Tx => "Tx",
    };
    println!("\t{} Channel {}", dir_s, channel);

    let freq_range = dev.frequency_range(dir, channel)?;
    println!("\t\tFreq range: {}", DisplayRange(freq_range));

    let sample_rates = dev.get_sample_rate_range(dir, channel)?;
    println!("\t\tSample rates: {}", DisplayRange(sample_rates));

    println!("\t\tAntennas: ");
    for antenna in dev.antennas(dir, channel)? {
        println!("\t\t\t{}", antenna);
    }

    Ok(())
}

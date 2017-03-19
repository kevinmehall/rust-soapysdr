use std::env;
extern crate soapysdr;
pub use soapysdr::Direction::{Rx, Tx};

fn main() {
    let filter = env::args().nth(1).unwrap_or(String::new());

    for devargs in soapysdr::enumerate(&(filter[..]).into()).expect("Error listing devices") {
        println!("{}", devargs);

        let dev = soapysdr::Device::new(&devargs).expect("Failed to open device");

        for channel in 0..(dev.num_channels(Rx).unwrap_or(0)) {
            print_channel_info(&dev, Rx, channel).expect("Failed to get channel info");
        }

        for channel in 0..(dev.num_channels(Tx).unwrap_or(0)) {
            print_channel_info(&dev, Tx, channel).expect("Failed to get channel info");
        }
    }
}

fn print_channel_info(dev: &soapysdr::Device, dir: soapysdr::Direction, channel: usize) -> Result<(), soapysdr::Error> {
    let dir_s = match dir { Rx => "RX", Tx => "Tx"};
    println!("\t{} Channel {}", dir_s, channel);

    let freq_range = dev.frequency_range(dir, channel)?[0];
    println!("\t\tFreq range: {} to {} MHz", freq_range.minimum / 1e6, freq_range.maximum / 1e6);

    let sample_rates = dev.list_sample_rates(dir, channel)?;
    println!("\t\tSample rates: {:?}", sample_rates);

    println!("\t\tAntennas: ");
    for antenna in dev.antennas(dir, channel)? {
        println!("\t\t\t{}", antenna);
    }

    Ok(())
}

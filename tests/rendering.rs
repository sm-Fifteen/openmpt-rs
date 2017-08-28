#![cfg_attr(test, allow(unused_must_use))]

extern crate openmpt;
extern crate hound;

use std::fs::File;
use openmpt::module::Module;
use openmpt::module::Logger;

//#[test]
fn render_unatco() {
	render_file_to_wav("UNATCO.it");
}

fn render_file_to_wav(file_path : &str) {
	let mut stream = File::open(file_path).expect("unable to open file");

	let mut module = Module::create(&mut stream, Logger::None, &[]).unwrap();

	let spec = hound::WavSpec {
		channels: 2,
		sample_rate: 44100,
		bits_per_sample: 32, // c_float is equivalent to f32
		sample_format: hound::SampleFormat::Float,
	};

	let out_file = String::from(file_path) + ".wav";
	
	let mut writer = hound::WavWriter::create(out_file, spec).unwrap();
	let mut buffer = vec![0f32; 44100]; // 1 second at a time

	loop {
		let avail_samples = module.read_interleaved_float_stereo(
				44100, &mut buffer) << 1; // We're in interleaved stereo
		if avail_samples <= 0 { break; }

		for sample in &buffer[..avail_samples] {
			writer.write_sample(*sample);
		}
	}
}
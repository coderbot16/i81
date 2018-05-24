extern crate serde;
#[macro_use]
extern crate serde_derive;

mod image_ops;
mod structure;
mod rng;

use image_ops::Image;

fn display_image(map: &Image<bool>) {
	for z in (0..map.z_size()).rev() {
		for x in 0..map.x_size() {
			if x == map.x_size() / 2 {
				print!("|");
			}

			print!("{}", if *map.get(x, z) {'#'} else {'.'});
		}
		println!();

		if z == map.z_size() / 2 {
			println!("======== ========");
		}
	}
}

fn main() {
    println!("Hello, world!");

    use image_ops::i80::Continents;
	use image_ops::filter::{Chain, Source, Filter};
	use image_ops::zoom::{Zoom, BestCandidate, RandomCandidate};
	use image_ops::blur::{Blur, XSpill, BoolMix};

	use rng::NotchRng;

	let continents = Continents {
		chance: 10,
		rng: NotchRng::new(1, 100)
	};

	let mut chain = Chain::new();
	chain.0.push(Box::new(Zoom::new(NotchRng::new(2000, 100), RandomCandidate)));
	chain.0.push(Box::new(Blur::new(NotchRng::new(   1, 100), XSpill::new(BoolMix { true_chance: 4, false_chance: 2 }))));
	chain.0.push(Box::new(Zoom::new(NotchRng::new(2001, 100), BestCandidate)));
	chain.0.push(Box::new(Blur::new(NotchRng::new(   2, 100), XSpill::new(BoolMix { true_chance: 4, false_chance: 2 }))));
	chain.0.push(Box::new(Zoom::new(NotchRng::new(2002, 100), BestCandidate)));
	chain.0.push(Box::new(Blur::new(NotchRng::new(   3, 100), XSpill::new(BoolMix { true_chance: 4, false_chance: 2 }))));
	chain.0.push(Box::new(Zoom::new(NotchRng::new(2003, 100), BestCandidate)));
	chain.0.push(Box::new(Blur::new(NotchRng::new(   3, 100), XSpill::new(BoolMix { true_chance: 4, false_chance: 2 }))));
	chain.0.push(Box::new(Zoom::new(NotchRng::new(2004, 100), BestCandidate)));
	chain.0.push(Box::new(Blur::new(NotchRng::new(   3, 100), XSpill::new(BoolMix { true_chance: 4, false_chance: 2 }))));

	println!("{:?} {:?}", chain.input_position((-8, -8)), chain.input_size((16, 16)));

	let sample = chain.input_size((16, 16));
	let mut continents_data = Image::new(false, sample.0, sample.1);
	continents.fill(chain.input_position((-8, -8)), &mut continents_data);

	let mut out = Image::new(false, 16, 16);
	chain.filter((-8, -8), &continents_data, &mut out);

	println!("Out:");
	display_image(&out);
}

use clap::Parser;
use rand::prelude::*;

fn choose_structure_bonus(mut rng: &mut impl Rng) {
	print!("Structure Bonus: ");
	let mut structure_bonuses = vec![
		"Adjacent Lakes",
		"Adjacent Tunnels",
		"Adjacent Encounters",
		"Tunnels",
		"Straight Line",
		"Tundras & Farms",
	];
	structure_bonuses.shuffle(&mut rng);
	println!("{}", structure_bonuses[0]);
}

fn init_factions(mut rng: &mut impl Rng, invaders_from_afar: bool) -> Vec<&'static str>{
	let mut factions = vec![
		"Nordic",
		"Rusviet",
		"Crimea",
		"Saxony",
		"Polania",
	];

	if invaders_from_afar {
		factions.push("Albion");
		factions.push("Togawa");
	}

	factions.shuffle(&mut rng);
	factions
}

fn init_player_mats(mut rng: &mut impl Rng, invaders_from_afar: bool) -> Vec<&'static str> {
	let mut player_mats = vec![
		"Industrial",
		"Engineering",
		"Patriotic",
		"Mechanical",
		"Agricultural",
	];

	if invaders_from_afar {
		player_mats.push("Militant");
		player_mats.push("Innovative");
	}

	player_mats.shuffle(&mut rng);
	player_mats
}

#[derive(Parser)]
struct Args {
	#[arg(default_value_t = 5)]
	player_count: u8,
	#[arg(short, long)]
	invaders_from_afar: bool,
}

fn main() {
	let args = Args::parse();
	let max_player_count = if args.invaders_from_afar { 7 } else { 5 };
	if args.player_count < 1 || args.player_count > max_player_count {
		println!("Player count must be from 1 to {}", max_player_count);
		return;
	}

	let mut rng = rand::thread_rng();
	println!("Scythe Setup:");
	choose_structure_bonus(&mut rng);
	let factions = init_factions(&mut rng, args.invaders_from_afar);
	let player_mats = init_player_mats(&mut rng, args.invaders_from_afar);

	for i in 0..args.player_count {
		println!("Player {}: {} {}",
			i + 1,
			factions[i as usize],
			player_mats[i as usize]);
	}
}

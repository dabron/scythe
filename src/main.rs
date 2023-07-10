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

fn init_factions(mut rng: &mut impl Rng) -> Vec<&'static str>{
	let mut factions = vec![
		"Nordic",
		"Rusviet",
		"Togawa",
		"Crimea",
		"Saxony",
		"Polania",
		"Albion",
	];
	factions.shuffle(&mut rng);
	factions
}

fn init_player_mats(mut rng: &mut impl Rng) -> Vec<&'static str> {
	let mut player_mats = vec![
		"Industrial",
		"Engineering",
		"Militant",
		"Patriotic",
		"Innovative",
		"Mechanical",
		"Agricultural",
	];
	player_mats.shuffle(&mut rng);
	player_mats
}

fn main() {
	let mut rng = rand::thread_rng();
	println!("Scythe Setup:");
	choose_structure_bonus(&mut rng);
	let factions = init_factions(&mut rng);
	let player_mats = init_player_mats(&mut rng);
	let player_count = 5;

	for i in 0..player_count {
		println!("Player {}: {} {}",
			i + 1,
			factions[i],
			player_mats[i]);
	}
}

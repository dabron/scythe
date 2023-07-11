use clap::Parser;
use rand::prelude::*;

#[derive(Parser)]
struct Args {
	#[arg(default_value_t = 5)]
	player_count: u8,
	#[arg(short, long)]
	invaders_from_afar: bool,
	#[arg(short, long)]
	wind_gambit: bool,
}

struct Player {
	id: u8,
	faction: &'static str,
	player_mat: &'static str,
}

fn choose_resolution_tile(mut rng: &mut impl Rng) {
	print!("     Resolution: ");
	let mut resolution_tiles = vec![
		"Standard",
		"Land Rush",
		"Factory Explosion",
		"Spoils of War",
		"King of the Hill",
		"Déjà Vu",
		"Mission Possible",
		"Doomsday Clock",
		"Backup Plan",
	];
	resolution_tiles.shuffle(&mut rng);
	println!("{}", resolution_tiles[0]);
}

fn choose_airship_tiles(mut rng: &mut impl Rng) {
	print!("        Airship: ");
	let mut aggressive_airship_tile = vec![
		"Blitzkrieg",
		"Bombard",
		"Bounty",
		"Distract",
		"Espionage",
		"Siege Engine",
		"Toll",
		"War Correspondent",
	];
	let mut passive_airship_tile = vec![
		"Boost",
		"Craft",
		"Drill",
		"Ferry",
		"Hero",
		"Negotiate",
		"Reap",
		"Safe Haven",
	];
	aggressive_airship_tile.shuffle(&mut rng);
	passive_airship_tile.shuffle(&mut rng);
	println!("{} - {}", aggressive_airship_tile[0], passive_airship_tile[0]);
}

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

fn is_banned(faction: &str, player_mat: &str) -> bool {
	(faction == "Rusviet" && player_mat == "Industrial") ||
	(faction == "Crimea"  && player_mat == "Patriotic")
}

fn main() {
	let args = Args::parse();
	const MIN_PLAYER_COUNT: u8 = 1;
	let max_player_count = if args.invaders_from_afar { 7 } else { 5 };
	if args.player_count < MIN_PLAYER_COUNT || args.player_count > max_player_count {
		println!("Player count must be from {} to {}", MIN_PLAYER_COUNT, max_player_count);
		return;
	}
	
	let mut rng = rand::thread_rng();
	println!();
	choose_structure_bonus(&mut rng);
	
	if args.wind_gambit {
		choose_resolution_tile(&mut rng);
		choose_airship_tiles(&mut rng);
	}

	println!();

	let mut factions = init_factions(&mut rng, args.invaders_from_afar);
	let mut player_mats = init_player_mats(&mut rng, args.invaders_from_afar);
	let mut players: Vec<Player> = Vec::new();
	
	for i in 0..args.player_count {
		let faction = factions.remove(0);
		let mut player_mat = player_mats.remove(0);
		if is_banned(faction, player_mat) {
			if player_mats.len() > 0 {
				let player_mat2 = player_mats.remove(0);
				player_mats.push(player_mat);
				player_mats.shuffle(&mut rng);
				player_mat = player_mat2;
			} else {
				let index = rng.gen_range(0..i) as usize;
				let player_mat2 = players[index].player_mat;
				players[index].player_mat = player_mat;
				player_mat = player_mat2;
			}
		}
		players.push(Player {
			id: i + 1,
			faction: faction,
			player_mat: player_mat,
		});
	}

	for player in players {
		println!("       Player {}: {} - {}",
			player.id,
			player.faction,
			player.player_mat);
	}

	println!();
}

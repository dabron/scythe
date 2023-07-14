use clap::Parser;
use colored::*;
use rand::prelude::*;

#[derive(Parser)]
struct Args {
	#[arg(default_value_t = 5)]
	player_count: u8,
	#[arg(short, long)]
	invaders_from_afar: bool,
	#[arg(short, long)]
	wind_gambit: bool,
	#[arg(short, long)]
	rise_of_fenris: bool,
}

struct Player {
	id: u8,
	faction: Faction,
	player_mat: PlayerMat,
}

struct Faction {
	name: &'static str,
	color: Color,
}

struct PlayerMat {
	name: &'static str,
	number: &'static str,
	value: f32,
}

fn choose_resolution_tile(mut rng: &mut impl Rng) {
	print!("     Resolution: ");
	let mut resolution_tiles = vec![
		"Standard",
		"Land Rush [1]",
		"Factory Explosion [2]",
		"Spoils of War [3]",
		"King of the Hill [4]",
		"Déjà Vu [5]",
		"Mission Possible [6]",
		"Doomsday Clock [7]",
		"Backup Plan [8]",
	];
	resolution_tiles.shuffle(&mut rng);
	println!("{}", resolution_tiles[0]);
}

fn choose_airship_tiles(mut rng: &mut impl Rng) {
	print!("        Airship: ");
	let mut aggressive_airship_tile = vec![
		"Blitzkrieg [1]",
		"Bombard [2]",
		"Bounty [3]",
		"Distract [4]",
		"Espionage [5]",
		"Siege Engine [6]",
		"Toll [7]",
		"War Correspondent [8]",
	];
	let mut passive_airship_tile = vec![
		"Boost [9]",
		"Craft [10]",
		"Drill [11]",
		"Ferry [12]",
		"Hero [13]",
		"Negotiate [14]",
		"Reap [15]",
		"Safe Haven [16]",
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

fn add_invaders(factions: &mut Vec<Faction>) {
	factions.push(Faction{ name: "Albion", color: Color::BrightGreen });
	factions.push(Faction{ name: "Togawa", color: Color::BrightMagenta });
}

fn init_factions(mut rng: &mut impl Rng, invaders_from_afar: bool, rise_of_fenris: bool) -> Vec<Faction>{
	let mut factions = vec![
		Faction{ name: "Nordic",  color: Color::BrightBlue },
		Faction{ name: "Rusviet", color: Color::BrightRed },
		Faction{ name: "Crimea",  color: Color::BrightYellow },
		Faction{ name: "Saxony",  color: Color::BrightBlack },
		Faction{ name: "Polania", color: Color::BrightWhite },
	];

	if invaders_from_afar {
		add_invaders(&mut factions);
	}

	if rise_of_fenris {
		factions.push(Faction{ name: "Tesla", color: Color::BrightCyan });
	}

	factions.shuffle(&mut rng);
	factions
}

fn init_player_mats(mut rng: &mut impl Rng, invaders_from_afar: bool) -> Vec<PlayerMat> {
	let mut player_mats = vec![
		PlayerMat{ name: "Industrial",   number: "1", value: 1. },
		PlayerMat{ name: "Engineering",  number: "2", value: 2. },
		PlayerMat{ name: "Patriotic",    number: "3", value: 3. },
		PlayerMat{ name: "Mechanical",   number: "4", value: 4. },
		PlayerMat{ name: "Agricultural", number: "5", value: 5. },
	];

	if invaders_from_afar {
		player_mats.push(PlayerMat{ name: "Militant",   number: "2A", value: 2.5 });
		player_mats.push(PlayerMat{ name: "Innovative", number: "3A", value: 3.5 });
	}

	player_mats.shuffle(&mut rng);
	player_mats
}

fn is_banned(faction: &Faction, player_mat: &PlayerMat) -> bool {
	(faction.name == "Rusviet" && player_mat.name == "Industrial") ||
	(faction.name == "Crimea"  && player_mat.name == "Patriotic")
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

	let mut factions = init_factions(&mut rng, args.invaders_from_afar, args.rise_of_fenris);
	let mut player_mats = init_player_mats(&mut rng, args.invaders_from_afar);
	let mut players: Vec<Player> = Vec::new();
	let mut lowest_value = f32::MAX;
	
	for i in 0..args.player_count {
		let faction = factions.remove(0);
		let mut player_mat = player_mats.remove(0);
		if is_banned(&faction, &player_mat) {
			if player_mats.len() > 0 {
				let player_mat2 = player_mats.remove(0);
				player_mats.push(player_mat);
				player_mats.shuffle(&mut rng);
				player_mat = player_mat2;
			} else {
				let index = rng.gen_range(0..i) as usize;
				std::mem::swap(&mut player_mat, &mut players[index].player_mat)
			}
		}
		if player_mat.value < lowest_value {
			lowest_value = player_mat.value;
		}
		players.push(Player {
			id: i + 1,
			faction: faction,
			player_mat: player_mat,
		});
	}

	for player in players {
		let value_color = if player.player_mat.value == lowest_value
		{
			Color::TrueColor{ r: 0x99, g: 0xff, b: 0x99}
		} else {
			Color::TrueColor{ r: 0x33, g: 0xcc, b: 0x33}
		};
		let base_str = if player.faction.name == "Tesla" {
			if !args.invaders_from_afar {
				add_invaders(&mut factions);
				factions.shuffle(&mut rng);
			}
			let base = factions.remove(0);
			format!(" [{}]", base.name.color(base.color))
		} else {
			String::new()
		};
		println!("       Player {}: {}{} - {} [{}]",
			player.id,
			player.faction.name.color(player.faction.color),
			base_str,
			player.player_mat.name.truecolor(0x99, 0x99, 0x99),
			player.player_mat.number.color(value_color));
	}

	println!();
}

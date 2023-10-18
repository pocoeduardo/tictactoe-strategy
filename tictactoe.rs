use Symbol::*;
use std::fs::File;
use std::io::Write;
use std::io;


#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
	X,
	O,
	Empty,
	Unknown,
}

#[derive(Debug)]
struct Tab {
	positions: [[Symbol; 3]; 3],
	winner: Symbol,
	has_winning_strategy: Symbol,
	next_tab_start: Option<usize>,
	next_tab_end: Option<usize>,
}

impl Tab {
	fn new() -> Tab {
		Tab {
			positions: [[Empty; 3]; 3],
			winner: Unknown,
			has_winning_strategy: Unknown,
			next_tab_start: None,
			next_tab_end: None,
		}
	}

	fn from(&self) -> Tab {
		Tab {
			positions: self.positions.clone(),
			winner: Unknown,
			has_winning_strategy: Unknown,
			next_tab_start: None,
			next_tab_end: None,
		}
	}

	fn is_trio_equal(&self, symbol: Symbol, pos1: [usize; 2], pos2: [usize; 2]) -> bool {
		self.positions[pos1[0]][pos1[1]] == symbol && self.positions[pos2[0]][pos2[1]] == symbol
	}

	fn set(&mut self, i: usize, j: usize, symbol: Symbol) {
		self.positions[i][j] = symbol;
		self.check_winner(i, j);
	}

	fn check_winner(&mut self, i: usize, j: usize) {
		let symbol = self.positions[i][j];

		//for each position of the last assigned symbol, the possible alignments are limited, so no need to check every row/column/diagonal after each move!
		if i == 0 && j == 0 {
			if self.is_trio_equal(symbol, [0, 1], [0, 2]) || self.is_trio_equal(symbol, [1, 0], [2, 0]) || self.is_trio_equal(symbol, [1, 1], [2, 2]) {
				self.winner = symbol;
			}
		} else if i == 0 && j == 2 {
			if self.is_trio_equal(symbol, [0, 0], [0, 1]) || self.is_trio_equal(symbol, [1, 2], [2, 2]) || self.is_trio_equal(symbol, [1, 1], [2, 0]) {
				self.winner = symbol;
			}
		} else if i == 2 && j == 0 {
			if self.is_trio_equal(symbol, [0, 0], [1, 0]) || self.is_trio_equal(symbol, [2, 1], [2, 2]) || self.is_trio_equal(symbol, [1, 1], [0, 2]) {
				self.winner = symbol;
			}
		} else if i == 2 && j == 2 {
			if self.is_trio_equal(symbol, [2, 0], [2, 1]) || self.is_trio_equal(symbol, [0, 2], [1, 2]) || self.is_trio_equal(symbol, [0, 0], [1, 1]) {
				self.winner = symbol;
			}
		} else if i == 0 && j == 1 {
			if self.is_trio_equal(symbol, [0, 0], [0, 2]) || self.is_trio_equal(symbol, [1, 1], [2, 1]) {
				self.winner = symbol;
			}
		} else if i == 2 && j == 1 {
			if self.is_trio_equal(symbol, [0, 1], [1, 1]) || self.is_trio_equal(symbol, [2, 0], [2, 2]) {
				self.winner = symbol;
			}
		} else if i == 1 && j == 0 {
			if self.is_trio_equal(symbol, [1, 1], [1, 2]) || self.is_trio_equal(symbol, [0, 0], [2, 0]) {
				self.winner = symbol;
			}
		} else if i == 1 && j == 2 {
			if self.is_trio_equal(symbol, [1, 0], [1, 1]) || self.is_trio_equal(symbol, [0, 2], [2, 2]) {
				self.winner = symbol;
			}
		} else { //cell do meio
			if self.is_trio_equal(symbol, [0, 1], [2, 1]) || self.is_trio_equal(symbol, [1, 0], [1, 2]) || self.is_trio_equal(symbol, [0, 0], [2, 2]) || self.is_trio_equal(symbol, [0, 2], [2, 0]) {
				self.winner = symbol;
			}
		}
	}
}




fn main() {
	let mut curr_rodada = 0;
	let mut list: Vec<Vec<Tab>> = Vec::new(); //[rodada][n-esima lista de tabuleiros]
	list.push(Vec::new());
	let tab_vazio = Tab::new();
	list[0].push(tab_vazio);

	for r in 0..=9 {
		let next_tabs = next_step(&mut curr_rodada, &mut list[r]);
		list.push(next_tabs);
	}

	println!("moves: configurations");
	for r in 0..=9 {
		println!("{}: {}", r, list[r].len());
	}

	find_winning_strategy(&mut list);

	let _ = dump_to_file(&list, true);
}

fn next_step(curr_rodada: &mut i32, curr_tabs: &mut Vec<Tab>) -> Vec<Tab> {
	let next_symbol = if *curr_rodada % 2 == 0 {X} else {O};

	let mut next_tabs: Vec<Tab> = Vec::new();
	let mut next_size = 0;

	for tab in curr_tabs.iter_mut() {
		if tab.winner == Unknown {
			if *curr_rodada == 9 { //SE chegou no final sem vencedor: poe vazio
				tab.winner = Empty;
			} else {
				for i in 0..3 {
					for j in 0..3 {
						if tab.positions[i][j] == Empty {
							let mut tab_next = tab.from();
							tab_next.set(i, j, next_symbol);
	
							if tab.next_tab_start == None {
								tab.next_tab_start = Some(next_size);
							}
							tab.next_tab_end = Some(next_size);
	
							next_tabs.push(tab_next);
							next_size += 1;
						}
					}
				}
			}
		}
	}
	
	*curr_rodada += 1;

	return next_tabs;
}

fn find_winning_strategy(list: &mut Vec<Vec<Tab>>) { //marca quem tem estrategia vencedora, de tras para frente
	for r in (0..=9).rev() {
		let (player, outro_player) = if r % 2 == 0 {(X, O)} else {(O, X)};

		for i in 0..list[r].len() {
			let mut player_with_winning_strategy = Empty;
			let tab = &list[r][i];
			
			if tab.winner == X || tab.winner == O {
				player_with_winning_strategy = tab.winner;
			} else {
				if r < 9 {
					//procura jogada que cai em estado com estrategia vencedora:
					let start_idx = tab.next_tab_start.unwrap();
					let end_idx = tab.next_tab_end.unwrap();

					let mut player_has_strategy = false;
					let mut existe_empate = false;
					let mut outro_can_win = false;

					for idx in start_idx..=end_idx {
						let next_tab = &list[r + 1][idx];
		
						if next_tab.has_winning_strategy == player {
							player_has_strategy = true;
							break;
						} else if next_tab.has_winning_strategy == Unknown || next_tab.has_winning_strategy == Empty {
							existe_empate = true;
						} else if next_tab.has_winning_strategy == outro_player {
							outro_can_win = true;
						}
					}

					if player_has_strategy {
						player_with_winning_strategy = player;
					} else if outro_can_win && !existe_empate { //SE soh existe vitoria do outro: ele tem estrategia vencedora
						player_with_winning_strategy = outro_player;
					}

				}
			}

			//nao posso dar borrow em mutable antes, pois estou lendo os itens de list:
			// let tab = &mut list[r][i];
			// tab.has_winning_strategy = player_with_winning_strategy;

			list[r][i].has_winning_strategy = player_with_winning_strategy;

		}
	}
}

fn dump_to_file(list: &Vec<Vec<Tab>>, with_strat_only: bool) -> io::Result<()> {
	let mut file_to_write = File::create("tictactoe strat.txt")?;

	println!("Writting file...");
	
	for (r, rodada_tabs) in list.iter().enumerate() {
		let player = if r % 2 == 0 {X} else {O};
		
		for tab in rodada_tabs {
			if !with_strat_only || (tab.has_winning_strategy == X || tab.has_winning_strategy == O) { //SE eh para mostrar tabuleiro
				for i in 0..3 {
					for j in 0..3 {
						let symbol = tab.positions[i][j];
						if symbol == Empty {
							write!(file_to_write, "_")?;
						} else {
							write!(file_to_write, "{:?}", symbol)?;
						}
						write!(file_to_write, " ")?;
					}
					
					if i == 0 && r < 9 && !(tab.winner == X || tab.winner == O) {
						write!(file_to_write, "   {:?} plays next", player)?;
					} else if i == 1 && (tab.has_winning_strategy == X || tab.has_winning_strategy == O) && !(tab.winner == X || tab.winner == O) {
						write!(file_to_write, "   {:?} has a winning strategy!", tab.has_winning_strategy)?;
					} else if i == 2 && (tab.winner == X || tab.winner == O) {
						write!(file_to_write, "   {:?} just won!", tab.winner)?;
					}
					
					writeln!(file_to_write, "")?;
				}
	
				writeln!(file_to_write, "\n")?;
			}
		}
	}

	println!("Done!");

	Ok(())
}
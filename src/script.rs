//! # Module de scripting 
//! 
//! Ce module est en cours d'implémentation. Il supportera à l'avenir Cerise, un langage de script (DSL) et Tomate, son implémentation. 
//! 
//! __TRAVAUX__ : susceptible d'évoluer fortement 
//! 

use std::net::TcpStream; 

use crate::client::Informer; 
use crate::valeur::Valeurs; 
use crate::grammaire::ArgumentsLocaux; 

#[derive(Debug)] 
struct Expression  { 
	etat: String, 
	fct: String, 
	args: Vec<String> 
} 

#[derive(Debug)] 
enum CommandesFiltre { 
	Positif, 
	Negatif, 
	Inconditionnel 
} 

#[derive(Debug)] 
enum Commandes { 
	Commande(CommandesFiltre, String, Vec<Valeurs>), 
	Groupe(CommandesFiltre, Vec<Commandes>) 
}

struct Expressions { 
	liste: Vec<Expression> 
} 

impl Expressions { 
	fn traduire( &self, arguments: Vec<String>) -> Vec<Valeurs> { 
		let mut r: Vec<Valeurs> = Vec::with_capacity( arguments.len() ); 
		for argument in arguments { 
			if let Some( p ) = argument.find( ':' ) { 
				match &argument[..p] { 
					"booléen" => { 
						match &argument[p+1..] { 
							"vrai" | "1" => r.push( 
								Valeurs::Booleen( true ) 
							), 
							"faux" | "0" => r.push( 
								Valeurs::Booleen( false ) 
							), 
							_ => panic!("valeur booléenne incorrecte") 
						} 
					} 
					"relatif" => { 
						if let Ok( n ) = &argument[p+1..].parse::<i32>() { 
							r.push( 
								Valeurs::Relatif( *n ) 
							); 
						} else { 
							panic!("valeur relative incorrecte") 
						} 
					} 
					"flottant" => { 
						if let Ok( n ) = &argument[p+1..].parse::<f32>() { 
							r.push( 
								Valeurs::Flottant( *n ) 
							); 
						} else { 
							panic!("valeur flottante incorrecte") 
						} 
					} 
					"texte" => { 
						r.push( 
							Valeurs::Texte( argument[p+1..].to_string() ) 
						); 
					} 
					_ => panic!("type inconnu") 
				} 
				; 
			} else {
				r.push( 
					Valeurs::Texte( argument.to_string() ) 
				); 
			}
		} 
		r 
	} 
	fn parcourir( &mut self, parent: &mut Commandes ) { 
		match parent { 
			Commandes::Groupe( _, groupe ) => { 
				while self.liste.len() > 0 { 
					let commande = self.liste.remove( 0 ); 
					match &commande.fct[..] { 
						"diverger" => { 
							let mut enfants = Commandes::Groupe( 
								match &commande.etat[..] { 
									"+" => CommandesFiltre::Positif, 
									"-" => CommandesFiltre::Negatif, 
									"ø" => CommandesFiltre::Inconditionnel, 
									_ => panic!("erreur 3") 
								}, 
								Vec::new() 
							); 
							self.parcourir( &mut enfants ); 
							groupe.push( enfants ); 
						} 
						"converger" => return, 
						_ => groupe.push( 
							Commandes::Commande( 
								match &commande.etat[..] { 
									"+" => CommandesFiltre::Positif, 
									"-" => CommandesFiltre::Negatif, 
									"ø" => CommandesFiltre::Inconditionnel, 
									_ => panic!("erreur 3") 
								}, 
								commande.fct, 
								self.traduire( commande.args ) 
							) 
						) 
					} 
				} 
			} 
			_ => () 
		} 
	} 
} 

struct ContexteExecution<'v> { 
	client: TcpStream, 
	etat: bool, 
	etat_sav: bool, 
	visee: &'v mut Valeurs 
} 

impl ContexteExecution<'_> { 
	fn etat( &self ) -> bool { 
		self.etat
	} 
	fn modifier( &mut self, etat: bool ) { 
		self.etat = etat; 
	} 
} 

fn afficher( contexte: &mut ContexteExecution, arguments: &Vec<Valeurs> ) { 
	contexte.client.message( 	
		&(if arguments.len() == 0 { 
			format!( "{:?}", contexte.visee ) 
		} else { 
			format!( "{:?}", arguments ) 
		}) 
	); 
	contexte.modifier( true ); 
} 

fn type_texte( contexte: &mut ContexteExecution, _arguments: &Vec<Valeurs> ) { 
	match contexte.visee {
		Valeurs::Texte( _ ) => contexte.modifier( true ), 
		_ => contexte.modifier( false ) 
	}
} 

fn etat_sauvegarder( contexte: &mut ContexteExecution ) { 
	contexte.etat_sav = contexte.etat; 
} 

fn etat_rendre( contexte: &mut ContexteExecution ) { 
	contexte.etat = contexte.etat_sav; 
}

fn executer_commande( contexte: &mut ContexteExecution, appel: &str, args: &Vec<Valeurs> ) { 
	match appel { 
		"état:sauvegarder" => etat_sauvegarder( contexte ), 
		"état:recharger" => etat_rendre( contexte ), 
		"ràz" => contexte.modifier( true ), 
		"afficher" => afficher( contexte, args ), 
		"type:texte" => type_texte( contexte, args ), 
		_ => contexte.modifier( false ) 
	} 
} 

fn filtrer_commande( contexte: &mut ContexteExecution, commande: &mut Commandes ) { 
	match commande { 
		Commandes::Commande( filtre, appel, args ) => match filtre { 
			CommandesFiltre::Positif if contexte.etat() => executer_commande( contexte, &appel[..], &args ), 
			CommandesFiltre::Negatif if !contexte.etat() => executer_commande( contexte, &appel[..], &args ), 
			CommandesFiltre::Inconditionnel => executer_commande( contexte, &appel[..], &args ), 
			_ => () 
		} 
		_ => panic!("n'est pas une commande") 
	} 
} 

fn executer_groupe( contexte: &mut ContexteExecution, commandes: &mut Commandes ) { 
	match commandes { 
		Commandes::Groupe( filtre, groupe )  => { 
			match filtre { 
				CommandesFiltre::Positif if !contexte.etat() => return, 
				CommandesFiltre::Negatif if contexte.etat() => return, 
				_ => () 
			} 
			for item in groupe { 
				match item { 
					Commandes::Commande(_,_,_) => filtrer_commande( contexte, item ), 
					Commandes::Groupe(_,_) => executer_groupe( contexte, item ) 
				} 
			} 
		} 
		_ => panic!("n'est pas un groupe") 
 	} 
} 

pub fn tester( contexte_client: TcpStream, script: &str, valeur_visee: &mut Valeurs ) -> bool { 

	// (1) 
	let mut items: Vec<Expression> = Vec::new(); 
	{
	   for ligne in script.split( '\n' ).collect::<Vec<&str>>() { 
			if let Ok( args ) = (ArgumentsLocaux { 
				source: ligne.trim().chars().collect::<Vec<char>>(), 
				position: 0 
			}).tous() { 
				match args.len() { 
					0 => continue, 
					1 => panic!("erreur 1"), 
					_ => match &args[0][..] { 
						"+" | "-" | "ø" => items.push( 
							Expression { 
								etat: args[0].to_string(), 
								fct: args[1].to_string(), 
								args: args[2..].to_vec() 
							} 
						), 
						_ => panic!("erreur 2") 
					} 
				} 
			} 
		} 
	} 

	// (2) 
	let mut expressions = Expressions { 
		liste: items 
	}; 
	let mut racine = Commandes::Groupe( 
		CommandesFiltre::Positif, 
		Vec::new() 
	); 
	expressions.parcourir( 
		&mut racine 
	); 
	
	// (3) 
	let mut contexte_execution = ContexteExecution { 
		client: contexte_client, 
		etat: true, 
		etat_sav: true, 
		visee: valeur_visee 
	}; 
	
	executer_groupe( &mut contexte_execution, &mut racine ); 

	contexte_execution.etat 

} 

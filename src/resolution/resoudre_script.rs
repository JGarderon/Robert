
use std::fs::File; 
use std::io::Read; 

use crate::resolution::{Contexte, Resolveur, Retour}; 
use crate::grammaire::{self, ArgumentsLocaux}; 

use crate::script; 

fn resoudre_lancer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 

	// totalement provisoire 

	let mut texte = "".to_string(); 
	match File::open( "./tmp.script" ) { 
		Ok( mut f ) => match f.read_to_string( &mut texte) { 
			Ok( n ) => contexte.message( 
				&format!( 
					"script chargé ({} octets)", 
					n 
				) 
			), 
			Err( _ ) => return Retour::creer_str( 
				false, 
				"interne interne (fichier de script unique impossible à charger)" 
			) 
		} , 
		Err( _ ) => return Retour::creer_str( 
			false, 
			"interne interne (fichier de script unique indisponible)" 
		) 
	}; 

	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let client_tcp = match contexte.stream.try_clone() { 
		Ok( c ) => c, 
		Err( _ ) => return Retour::creer_str( 
			false, 
			"interne interne (clonage du client impossible)" 
		) 
	}; 

	let mut canal = acces_canal!( contexte ); 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => canal.resoudre( 
			&chemin, 
			| valeur_visee | { 
				Retour::creer_str( 
					if script::tester( 
						client_tcp, 
						&texte, 
						valeur_visee 
					) { 
						true 
					} else { 
						false 
					}, 
					"fin du script" 
				) 
			} 	 
		), 
		Err( e ) => return Retour::creer_str( false, e ) 
	} 
} 

pub fn resoudre( appel: &str ) -> Result<Resolveur,Retour> { 
	match appel { 
		"lancer" => Ok( resoudre_lancer as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module 'script' : fonction inconnue" ) ) 
	} 
} 



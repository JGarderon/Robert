

use crate::base::Valeurs; 
use crate::resolution::Contexte; 
use crate::grammaire::ArgumentsLocaux; 

// ---------------------------------------------------- 

use crate::resolution::Resolveur; 
use crate::resolution::Retour; 

// ---------------------------------------------------- 

fn resoudre_incrementer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let cle = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "vous devez spécifier une clé existante" ); 
	}; 
	let incr_option = arguments.extraire(); 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	if let Some( v ) = valeurs.get_mut( &cle ) { 
		match v { 
			Valeurs::Reel( n ) => { 
				if let Some( m ) = incr_option { 
					if let Ok( m ) = m.parse::<i32>() { 
						if let Some( r ) = n.checked_add( m ) { 
							*n = r; 
							Retour::creer_str( true, "incrémentation arbitraire effectuée" ) 
						} else { 
							Retour::creer_str( true, "incrémentation arbitraire impossible" ) 
						} 
					} else { 
						Retour::creer_str( false, "l'argument est invalide dans ce type" ) 
					} 
				} else { 
					*n += 1i32; 
					Retour::creer_str( true, "incrémentation par défaut (+1) effectuée" ) 
				} 
			} 
			Valeurs::Flottant( n ) => { 
				if let Some( m ) = incr_option { 
					if let Ok( m ) = m.parse::<f32>() { 
						*n += m; 
						Retour::creer_str( true, "incrémentation arbitraire effectuée" ) 
					} else { 
						Retour::creer_str( false, "l'argument est invalide dans ce type" ) 
					} 
				} else { 
					*n += 1.0f32; 
					Retour::creer_str( true, "incrémentation par défaut (+1.0) effectuée" ) 
				} 
			} 
			_ => Retour::creer_str( false, "incrémentation impossible, le type ne le supporte pas" ) 
		} 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

fn resoudre_maj( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let cle = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "vous devez spécifier une clé existante" ); 
	}; 
	let valeur = if let Some( v ) = arguments.extraire() { 
		v 
	} else { 
		return Retour::creer_str( false, "vous devez spécifier une valeur" ); 
	}; 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	if let Some( v ) = valeurs.get_mut( &cle ) { 
		match v { 
			Valeurs::Reel( n ) => { 
				if let Ok( m ) = valeur.parse::<i32>() { 
					*n = m; 
					Retour::creer_str( true, "màj effectuée" ) 
				} else { 
					Retour::creer_str( false, "l'argument est invalide dans ce type" ) 
				} 
			} 
			Valeurs::Flottant( n ) => { 
				if let Ok( m ) = valeur.parse::<f32>() { 
					*n = m; 
					Retour::creer_str( true, "màj effectuée" ) 
				} else { 
					Retour::creer_str( false, "l'argument est invalide dans ce type" ) 
				} 
			} 
			_ => Retour::creer_str( false, "màj numérique impossible, le type ne le supporte pas" ) 
		} 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

pub fn resoudre( appel: &str ) -> Result<Resolveur,Retour> { 
	match appel { 
		"incrémenter" => Ok( resoudre_incrementer as Resolveur ), 
		"maj" => Ok( resoudre_maj as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module numérique : fonction inconnue" ) ) 
	} 
}


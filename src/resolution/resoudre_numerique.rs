
use crate::base::Valeurs; 
use crate::resolution::Contexte; 
use crate::grammaire; 

// ---------------------------------------------------- 

use crate::resolution::Resolveur; 
use crate::resolution::Retour; 

// ---------------------------------------------------- 

/// # Fonction de résolution locale "incrémenter une valeur numérique" 
///
/// La fonction d'incrémentation est plus large que suppose son nom : elle ajoute par défaut "1" à la valeur numérique associée à la clé ou au chemin. Cependant elle peut aussi en argument optionnel, prendre une valeur qui sera parsée dans le format de destination et "ajouter". 
///
/// Si l'argument représente un relatif ou un flottant négatif, l'ajout réduira donc la valeur. 
/// 
/// Ainsi il n'est pas nécessaire d'ajouter une autre fonction de décrémentation ou de gestion des additions pour le projet Robert dans sa version par défaut car toutes les situations sont déjà couvertes. 
///
fn resoudre_incrementer ( contexte: &mut Contexte, mut arguments: grammaire::ArgumentsLocaux ) -> Retour { 
	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let i = match arguments.extraire() { 
		Some( i ) => i, 
		None => "1".to_string() 
	}; 
	let mut canal = Canal!( contexte ); 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => canal.resoudre( 
			&chemin, 
			| valeur | { 
				match valeur { 
					Valeurs::Relatif( v ) => { 
						match i.parse::<i32>() { 
							Ok( n ) => { 
								*v += n; 
								Retour::creer_str( true, "incrémentation effectuée" ) 
							} 
							Err( _ ) => Retour::creer_str( false, "argument d'incrément incorrect" ) 
						} 
					} 
					Valeurs::Flottant( v ) => { 
						match i.parse::<f32>() { 
							Ok( n ) => { 
								*v += n; 
								Retour::creer_str( true, "incrémentation effectuée" ) 
							} 
							Err( _ ) => Retour::creer_str( false, "argument d'incrément incorrect" ) 
						} 
					} 
					_ => Retour::creer_str( false, "incrémentation impossible sur une valeur non-numérique" ) 
				} 
			} 
		), 
		Err( _ ) => Retour::creer_str( true, "ce chemin n'existe pas" ) 
	} 
} 

pub fn resoudre( appel: &str ) -> Result<Resolveur,Retour> { 
	match appel { 
		"incrémenter" => Ok( resoudre_incrementer as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module numérique : fonction inconnue" ) ) 
	} 
} 


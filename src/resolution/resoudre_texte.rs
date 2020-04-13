
use crate::base::Valeurs; 
use crate::resolution::Contexte; 
use crate::grammaire; 
use crate::grammaire::ArgumentsLocaux; 

// ---------------------------------------------------- 

use crate::resolution::Resolveur; 
use crate::resolution::Retour; 

// ---------------------------------------------------- 

use crate::TAILLE_TEXTE_MAX; 

// ---------------------------------------------------- 

/// # Fonction de résolution locale "ajouter du texte" 
///
fn resoudre_ajouter( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let ajout = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "aucun texte supplémentaire fourni" ); 
	}; 
	let mut canal = Canal!( contexte ); 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => canal.resoudre( 
			&chemin, 
			| valeur | { 
				match valeur { 
					Valeurs::Texte( t ) => { 
						if t.len()+ajout.len() < TAILLE_TEXTE_MAX { 
							t.push_str( &ajout ); 
							Retour::creer_str( true, "texte ajouté" ) 
						} else { 
							Retour::creer_str( true, "texte final trop long" ) 
						}
					} 
					_ => Retour::creer_str( false, "le type de valeur cible n'est pas conforme" ) 
				} 
			} 
		), 
		Err( _ ) => Retour::creer_str( false, "ce chemin n'existe pas" ) 
	} 
} 

/// # Fonction de résolution locale "compter le texte (caractères)" 
///
fn resoudre_compter( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let mut canal = Canal!( contexte ); 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => canal.resoudre( 
			&chemin, 
			| valeur | { 
				match valeur { 
					Valeurs::Texte( t ) => { 
						Retour::creer( true, format!( "{} caractères", t.chars().count() ) ) 
					} 
					_ => Retour::creer_str( false, "le type de valeur cible n'est pas conforme" ) 
				} 
			} 
		), 
		Err( _ ) => Retour::creer_str( false, "ce chemin n'existe pas" ) 
	} 
} 

/// # Fonction de résolution locale "découper du texte (caractères)" 
///
fn resoudre_decouper( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let debut = match {
		if let Some( d ) = arguments.extraire() { 
			d 
		} else { 
			return Retour::creer_str( false, "l'origine est impérative" ); 
		} 
	}.parse::<usize>() { 
		Ok( n ) => n, 
		Err( _ ) => return Retour::creer_str( false, "origine =/= entier positif " ) 
	}; 
	let fin = arguments.extraire(); 
	let mut canal = Canal!( contexte ); 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => canal.resoudre( 
			&chemin, 
			| valeur | { 
				match valeur { 
					Valeurs::Texte( t ) => { 
						if debut >= t.len() { 
							Retour::creer_str( false, "origine hors borne" ) 
						} else { 
							let mut position = 0; 
							match fin { 
								Some( f ) => match f.parse::<usize>() { 
									Ok( n ) => { 
										if n > t.len() { 
											Retour::creer_str( false, "fin hors borne" ) 
										} else { 
											t.retain( 
												| _ | { 
													position += 1; 
													if position-1 < debut || position-1 > n { 
														false 
													} else { 
														true 
													} 
												} 
											); 
											Retour::creer_str( true, "texte découpé" ) 
										} 
									} 
									Err( _ ) => Retour::creer_str( false, "fin =/= entier positif " ) 
								} 
								None => { 
									t.retain( 
										| _ | { 
											position += 1; 
											if position-1 < debut { 
												false 
											} else { 
												true 
											} 
										} 
									); 
									Retour::creer_str( true, "texte découpé" ) 
								} 
							} 
						} 
					} 
					_ => Retour::creer_str( false, "le type de valeur cible n'est pas conforme" ) 
				} 
			} 
		), 
		Err( _ ) => Retour::creer_str( false, "ce chemin n'existe pas" ) 
	} 
} 

pub fn resoudre( appel: &str ) -> Result<Resolveur,Retour> { 
	match appel { 
		"ajouter" => Ok( resoudre_ajouter as Resolveur ), 
		"compter" => Ok( resoudre_compter as Resolveur ), 
		"découper" => Ok( resoudre_decouper as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module numérique : fonction inconnue" ) ) 
	} 
} 









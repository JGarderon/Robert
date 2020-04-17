
use std::io::Write; 

// ---------------------------------------------------- 

use crate::grammaire; 
use crate::grammaire::ArgumentsLocaux; 
use crate::base::Valeurs; 
use crate::contexte::Contexte; 

// ---------------------------------------------------- 

mod resoudre_numerique; 
mod resoudre_texte; 
mod resoudre_canal; 
mod resoudre_administration; 

// ---------------------------------------------------- 

/// Un type spécifique au projet : le type 'Résolveur' est la signature d'une fonction de résolution, quelque soit le module de résolution. 
/// Elle prend deux paramètres : le contexte du socket ainsi qu'un objet permettant de récupèrer à la demande les arguments dits 'locaux' (propre à une requête). La fonction renvoie un objet "retour", qui sera transmis au client via une série d'octets écrite sur le socket. 
/// La définition de cette signature a pour principal but de soulager les signatures dans d'autres fonctions de résolution. 
type Resolveur = fn ( &mut Contexte, ArgumentsLocaux ) -> Retour; 

// ----------------------------------------------------  

/// Les retours peuvent être soit un texte statique (_&'static str_) - c'est-à-dire invariable et intégré au directement dans le code source du programme (efficacité), soit un texte généré par la fonction de résolution (_String_) - c'est-à-dire variable. 
pub enum RetourType { 

	/// Est de type _&'static str_ 
	Statique(&'static str), 
	
	/// Est de type _String_ 
	Dynamique(String) 

} 

/// Structure définissant un 'Retour', afin d'uniformiser les messages à destination du client et l'état de résolution. 
pub struct Retour { 

	/// Permet de signaler au thread maître, lors du renvoi vers le client, si la fonction se déclare aboutie correctement ou en erreur. 
	/// Elle n'arrête pas la boucle principale de réception/traitement du thread. 
	pub etat: bool, 

	/// Contient le message qui doit être renvoyé au client. 
	pub message: RetourType 

} 

impl Retour { 

	/// Créer un retour "dynamique", c'est-à-dire un _String_. 
	pub fn creer( etat: bool, m: String ) -> Self { 
		Retour { 
			etat: etat, 
			message: RetourType::Dynamique( m )
		} 
	} 

	/// Créer un retour "statique", c'est-à-dire un _&'static str_. 
	pub fn creer_str( etat: bool, m: &'static str ) -> Self { 
		Retour { 
			etat: etat, 
			message: RetourType::Statique( m )
		} 
	} 

} 

// ---------------------------------------------------- 

/// Fonction de résolution : arrête la boucle principale du thread du client. 
/// Ne prend aucun argument (obligatoire). 
fn resoudre_stop ( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "aucun argument autorisé" ); 
	} 
	contexte.poursuivre = false; 
	Retour::creer_str( true, "au revoir" ) 
} 

/// # Fonction de résolution "définir une nouvelle valeur" 
/// Elle définit une nouvelle valeur stockée dans le canal (sans la diffuser). 
/// Au moins deux arguments doivent être fournis : la clé (ou un chemin comprenant la clé) ainsi qu'une valeur quelconque. Cette valeur peut être altérer dans un format particulier grâce à un troisième argument optionnelle qui représente son type. Si l'altération est impossible, l'ajout n'est pas effectuée. 
/// Si aucun type de valeur n'est fourni, c'est le texte qui est le type par défaut. 
fn resoudre_definir ( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let valeur = if let Some( v ) = arguments.extraire() { 
		v 
	} else { 
		return Retour::creer_str( false, "aucune valeur fournie ou séparateur clé/valeur non-respecté" ); 
	}; 
	let valeur_type = arguments.extraire(); 
	let mut canal = Canal!( contexte ); 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => { 
			if chemin.len() == 1 { 
				return canal.liste.creer_valeur( 
					chemin[0].to_string(), 
					valeur, 
					valeur_type 
				); 
			} else { 
				let cle = chemin[chemin.len()-1].to_string(); 
				return canal.resoudre( 
					&chemin[..chemin.len()-1], 
					move | parent | { 
 						parent.creer_valeur( 
 							cle, 
 							valeur, 
 							valeur_type 
 						) 
 					} 
				); 
			} 
		} 
		Err( e ) => Retour::creer_str( false, e ) 
	} 
} 

/// # Fonction de résolution "obtenir une valeur existante" 
fn resoudre_obtenir ( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
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
					Valeurs::Boolean( b ) => Retour::creer( true, format!( "(booléen) {}", b ) ), 
					Valeurs::Texte( t ) => Retour::creer( true, format!( "(texte) \"{}\"", t ) ), 
					Valeurs::Relatif( n ) => Retour::creer( true, format!( "(réel) {}", n ) ), 
					Valeurs::Flottant( n ) => Retour::creer( true, format!( "(flottant) {}", n ) ), 
					Valeurs::Objet( o ) => Retour::creer( true, format!( "(objet) {:?}", o ) ) 
				} 
			} 
		), 
		Err( e ) => Retour::creer_str( false, e ) 
	} 
} 

/// # Fonction de résolution "supprimer une valeur existante" 
fn resoudre_supprimer ( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let mut canal = Canal!( contexte ); 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => canal.resoudre( 
			&chemin[..chemin.len()-1], 
			| parent | { 
				match parent { 
					Valeurs::Objet( h ) => if let Some( _ ) = h.remove( &chemin[chemin.len()-1].to_string() ) { 
						Retour::creer_str( true, "la paire clé/valeur a été retirée" ) 
					} else { 
						Retour::creer_str( false, "cette clé n'existe pas dans l'objet" ) 
					} 
					_ => Retour::creer_str( false, "ce chemin n'amène pas à un objet" ) 
				} 
			} 
		), 
		Err( e ) => Retour::creer_str( false, e ) 
	} 
} 

/// # Fonction de résolution "tester l'existence d'un chemin" 
fn resoudre_tester ( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let mut canal = Canal!( contexte ); 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => canal.resoudre( 
			&chemin, 
			| _ | { 
				Retour::creer_str( true, "ce chemin existe" ) 
			} 
		), 
		Err( _ ) => Retour::creer_str( true, "ce chemin n'existe pas" ) 
	} 
} 

/// # Fonction de résolution "lister toutes les valeurs d'un canal" 
fn resoudre_lister ( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let mut canal = Canal!( contexte ); 
	let mut stream_copie = match contexte.stream.try_clone() { 
		Ok( s ) => s, 
		Err( _ ) => return Retour::creer_str( false, "erreur interne ; copie du stream impossible" ) 
	}; 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => canal.resoudre( 
			&chemin, 
			| valeur | { 
				match valeur { 
					Valeurs::Objet( h ) => { 
						for (cle, valeur) in h.iter() { 
							if let Err(_) = stream_copie.write( 
								format!( 
									"\t{} : {:?}\n", 
									cle, 
									valeur 
								).as_bytes() 
							) { 
								stream_copie.flush().unwrap(); 
								return Retour::creer_str( false, "erreur lors de l'envoi" ); 
							} 
						} 
						stream_copie.flush().unwrap(); 
						Retour::creer( true, format!( "stop ({})", h.len() ) ) 
					} 
					_ => Retour::creer_str( false, "cette paire clé/valeur n'amène pas à un objet" ) 
				} 
			} 
		), 
		Err( _ ) => Retour::creer_str( true, "ce chemin n'existe pas" ) 
	} 
} 

/// # Fonction de résolution "altérer une valeur existante" 
fn resoudre_alterer ( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let arg_chemin = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "un chemin vide n'est pas acceptable" ); 
	}; 
	let valeur_type = if let Some( vt ) = arguments.extraire() { 
		vt 
	} else { 
		return Retour::creer_str( false, "vous devez spécifier un type correct pour l'altération" ); 
	}; 
	let mut canal = Canal!( contexte ); 
	match grammaire::chemin_extraire( &arg_chemin ) { 
		Ok( chemin ) => canal.resoudre( 
			&chemin, 
			| valeur | { 
				if valeur.alterer( &valeur_type ) { 
					Retour::creer_str( true, "altération effectuée" ) 
				} else { 
					Retour::creer_str( true, "altération impossible" ) 
				} 
			} 
		), 
		Err( _ ) => Retour::creer_str( true, "ce chemin n'existe pas" ) 
	} 
} 

/// # Fonction de résolution centrale 
/// Cette fonction est appelée par le thread du client, et redirigera l'appel vers la bonne fonction de résolution, en fonction du module souhaité. 
/// Les fonctions génériques, définies dans ce présent module, y sont directement incorporées. Les fonctions spéficiques, qui se retrouvent dans les sous-modules, ont une fonction de résolution secondaire : cette fonction est principale car elle appelera la fonction de résolution secondaire, celle du sous-module. 
pub fn resoudre( contexte: &mut Contexte, appel: &str, arguments: &str ) -> Retour { 
	(if let Some( n ) = appel.find( ':' ) { 
		match &appel[..n] { 
			"numérique" => match resoudre_numerique::resoudre( &appel[n+1..] ) { 
				Ok( fct ) => fct, 
				Err( r ) => return r 
			}, 
			"texte" => match resoudre_texte::resoudre( &appel[n+1..] ) { 
				Ok( fct ) => fct, 
				Err( r ) => return r 
			}, 
			"canal" => match resoudre_canal::resoudre( &appel[n+1..] ) { 
				Ok( fct ) => fct, 
				Err( r ) => return r 
			}, 
			"administration" => match resoudre_administration::resoudre( &appel[n+1..] ) { 
				Ok( fct ) => fct, 
				Err( r ) => return r 
			}, 
			_ => return Retour::creer_str( false, "module inconnu" ) 
		}
	} else { 
		match appel { 
			"stop" => resoudre_stop as Resolveur, 
			"définir" => resoudre_definir as Resolveur, 
			"obtenir" => resoudre_obtenir as Resolveur, 
			"supprimer" => resoudre_supprimer as Resolveur, 
			"tester" => resoudre_tester as Resolveur, 
			"lister" => resoudre_lister as Resolveur, 
			"altérer" => resoudre_alterer as Resolveur, 

			_ => return Retour::creer_str( false, "module général : fonction inconnue" ) 
		} 
	})( 
		contexte, 
		ArgumentsLocaux { 
	        source: arguments.chars().collect::<Vec<char>>(), 
	        position: 0 
	    } 
	) 
} 




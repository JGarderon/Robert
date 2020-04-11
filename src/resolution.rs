
use std::io::Write; 
use std::sync::Arc; 
use std::sync::Mutex; 
use std::net::TcpStream; 
use std::collections::HashMap; 

// ---------------------------------------------------- 

use crate::grammaire; 
use crate::grammaire::ArgumentsLocaux; 
use crate::base::CanalThread; 
use crate::base::CanauxThread; 
use crate::base::Valeurs; 

// ---------------------------------------------------- 

// mod resoudre_numerique; 
// mod resoudre_texte; 
// mod resoudre_canal; 
// mod resoudre_administration; 

// ---------------------------------------------------- 

use crate::NBRE_MAX_VALEURS; 

// ---------------------------------------------------- 

/// Un type spécifique au projet : le type 'Résolveur' est la signature d'une fonction de résolution, quelque soit le module de résolution. 
/// Elle prend deux paramètres : le contexte du socket ainsi qu'un objet permettant de récupèrer à la demande les arguments dits 'locaux' (propre à une requête). La fonction renvoie un objet "retour", qui sera transmis au client via une série d'octets écrite sur le socket. 
/// La définition de cette signature a pour principal but de soulager les signatures dans d'autres fonctions de résolution. 
type Resolveur = fn ( &mut Contexte, ArgumentsLocaux ) -> Retour; 

// ---------------------------------------------------- 

/// La structure 'Contexte' permet de rassembler dans un objet unique, l'ensemble des éléments propres à un socket quelque soit la fonction de résolution qui sera appelée. Elle référence aussi le dictionnaire (canal) en cours, ainsi que le dictionnaire des canaux. 
/// Dans une fonction de résolution, elle se présentera toujours dans la forme d'une référence mutable. 
pub struct Contexte { 
	
	/// Ce champ lorsqu'il est à "faux", permet d'interrompre la boucle locale du thead gérant le socket, dès la fin de la fonction de résolution actuelle. 
	pub poursuivre: bool, 
	
	/// Ce champ contient le nécessaire pour accéder au dictionnaire représentant le canal actuel. 
	pub canalthread: CanalThread, 
	
	/// Ce champ contient le nécessaire pour accéder au dictionnaires des canaux. 
	pub canauxthread: CanauxThread, 

	/// Ce champ contient l'objet socket. 
	pub stream: TcpStream 
} 

// ----------------------------------------------------  

/// Les retours peuvent être soit un texte statique (&'static str) - c'est-à-dire invariable et intégré au directement dans le code source du programme (efficacité), soit un texte généré par la fonction de résolution (String) - c'est-à-dire variable. 
pub enum RetourType { 
	Statique(&'static str), 
	Dynamique(String) 
} 

/// L'implémentation permet ici de rassembler les valeurs vers l'équivalent d'un slice de Bytes, qui sera utilisé pour le retour écrit sur le socket du client. 
impl RetourType { 
	pub fn vers_bytes( &self ) -> &[u8] { 
		match self { 
			RetourType::Statique( m ) => m.as_bytes(), 
			RetourType::Dynamique( m ) => m.as_bytes() 
		} 
	} 
} 

pub struct Retour { 
	pub etat: bool, 
	pub message: RetourType 
} 

impl Retour { 
	pub fn creer( etat: bool, m: String ) -> Self { 
		Retour { 
			etat: etat, 
			message: RetourType::Dynamique( m )
		} 
	} 
	pub fn creer_str( etat: bool, m: &'static str ) -> Self { 
		Retour { 
			etat: etat, 
			message: RetourType::Statique( m )
		} 
	} 
} 

// ---------------------------------------------------- 

fn resoudre_stop( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "aucun argument autorisé" ); 
	} 
	contexte.poursuivre = false; 
	Retour::creer_str( true, "au revoir" ) 
} 

// fn resoudre_vider( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	if !arguments.est_stop() { 
// 		return Retour::creer_str( false, "aucun argument autorisé" ); 
// 	} 
// 	let mut dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &mut dico.liste; 
// 	valeurs.clear(); 
// 	Retour::creer_str( true, "base vidée" ) 
// } 

// // pub fn chemin_acceder( &mut self, vecteur: &mut Vec<&str>, fct: AccesseurObjet ) -> Retour { 
// // 	if let Some( motif ) = vecteur.pop() { 
// // 		match self { 
// // 			Valeurs::Objet( h ) if vecteur.len() == 0 => { 
// // 				if let Some( valeur ) = h.get_mut( motif ) {
// // 					fct( valeur ) 
// // 				} else { 
// // 					Retour::creer_str( false, "chemin incorrect (dernier élément)" ) 
// // 				} 
// // 			} 
// // 			Valeurs::Objet( h ) if vecteur.len() > 0 => { 
// // 				if let Some( valeur ) = h.get_mut( motif ) {
// // 					valeur.chemin_acceder( vecteur, fct ) 
// // 				} else { 
// // 					Retour::creer_str( false, "chemin incorrect" ) 
// // 				} 
// // 			} 
// // 			_ => Retour::creer_str( false, "valeur inaccessible par un chemin" ) 
// // 		}  
// // 	} else { 
// // 		Retour::creer_str( false, "erreur interne : le vecteur du chemin est vide" ) 
// // 	} 
// // } 

// fn resoudre_definir( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let mut cle = if let Some( c ) = arguments.extraire() { 
// 		c 
// 	} else { 
// 		return Retour::creer_str( false, "une clé vide n'est pas une clé acceptable" ); 
// 	}; 
// 	let valeur = if let Some( v ) = arguments.extraire() { 
// 		v 
// 	} else { 
// 		return Retour::creer_str( false, "aucune valeur fournie ou séparateur clé/valeur non-respecté" ); 
// 	}; 
// 	let valeur_type = arguments.extraire(); 
// 	if !arguments.est_stop() { 
// 		return Retour::creer_str( false, "trop d'arguments fournis (max. 2-3)" ); 
// 	} 
	



// 	let mut dico = { 
// 		match contexte.dico.lock() { 
// 			Ok( d ) => d, 
// 			Err(_) => return Retour::creer_str( false, "erreur interne; dictionnaire inaccessible" ) 
// 		} 
// 	}; 
// 	if let Ok( chemin ) = grammaire::chemin_extraire( &cle ) { 
// 		if chemin.len() == 1 { 
// 			if dico.creer_valeur( 
// 				chemin[0], 
// 				&valeur, 
// 				valeur_type 
// 			) { 
// 				Retour::creer_str( true, "valeur ajoutée" ) 
// 			} else { 
// 				Retour::creer_str( true, "impossible d'ajouter cette valeur avec ces paramètres" ) 
// 			} 
// 		} else { 
// 			Retour::creer_str( true, "non implémenté" ) 
// 		} 
// 	} else { 
// 		Retour::creer_str( false, "la clé (ou le chemin) ne semble pas correcte" ) 
// 	} 




// 	// 	let point = valeurs; //{ 
// 	// 	// 	if chemin.len() == 1 { 
// 	// 	// 		valeurs 
// 	// 	// 	} else { 
// 	// 	// 		if let Some( valeur ) = valeurs.get_mut( chemin[0] ) { 
// 	// 	// 			valeur.acceder( &chemin[1..chemin.len()-1] ) 
// 	// 	// 		} else { 
// 	// 	// 			return Retour::creer_str( false, "la racine vers la valeur créé doit exister" ); 
// 	// 	// 		} 
// 	// 	// 	} 
// 	// 	// }; 
// 	// 	let cle_point = chemin[chemin.len()].to_string(); 
// 	// 	println!("{:?}", point); 
// 	// 	println!("{:?}", cle_point); 
// 	// 	if point.len() < NBRE_MAX_VALEURS { 
// 	// 		match arguments.extraire() { 
// 	// 			None => { 
// 	// 				point.insert( 
// 	// 					cle_point, 
// 	// 					Valeurs::Texte( valeur ) 
// 	// 				); 
// 	// 				Retour::creer_str( true, "paire clé/valeur ajoutée (type par défaut : texte)" ) 
// 	// 			} 
// 	// 			Some( t ) => { 
// 	// 				if !arguments.est_stop() { 
// 	// 					return Retour::creer_str( false, "trop d'arguments fournis (max. 2-3)" ); 
// 	// 				} 
// 	// 				if &t == "objet" { 
// 	// 					if valeur == "~" { 
// 	// 						point.insert( 
// 	// 							cle_point, 
// 	// 							Valeurs::Objet( HashMap::new() ) 
// 	// 						);  
// 	// 						Retour::creer_str( true, "l'objet a été créé au point désiré" )  
// 	// 					} else { 
// 	// 						Retour::creer_str( false, "attention, seule la valeur '~' est autorisée en argument de valeur pour la création d'un objet" ) 
// 	// 					}
// 	// 				} else { 
// 	// 					let mut v = Valeurs::Texte( valeur ); 
// 	// 					if v.alterer( &t ) { 
// 	// 						point.insert( 
// 	// 							cle_point, 
// 	// 							v 
// 	// 						);  
// 	// 						Retour::creer( true, format!( 
// 	// 							"paire clé/valeur ajoutée (type {})", 
// 	// 							&t
// 	// 						) ) 
// 	// 					} else { 
// 	// 						Retour::creer( false, format!( 
// 	// 							"le type '{}' n'est pas un type conforme", 
// 	// 							&t
// 	// 						) ) 
// 	// 					} 
// 	// 				} 
// 	// 			} 
// 	// 		} 
// 	// 	} else { 
// 	// 		Retour::creer_str( false, "nbre max. de valeurs atteint" ) 
// 	// 	} 
// 	// } else { 
// 	// 	Retour::creer_str( false, "la clé ne correspond pas à une clé ou un chemin valide" ) 
// 	// } 
// } 

// fn resoudre_obtenir( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let cle = if let Some( c ) = arguments.extraire() { 
// 		c 
// 	} else { 
// 		return Retour::creer_str( false, "vous devez spécifier une clé existante" ); 
// 	}; 
// 	if !arguments.est_stop() { 
// 		return Retour::creer_str( false, "trop d'arguments fournis (maximum 1)" ); 
// 	} 
// 	let dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &dico.liste; 




// 	if valeurs.contains_key( &cle ) { 
// 		match &valeurs[&cle] { 
// 			Valeurs::Boolean( b ) => Retour::creer( true, format!( "(booléen) {}", b ) ), 
// 			Valeurs::Texte( t ) => Retour::creer( true, format!( "(texte) \"{}\"", t ) ), 
// 			Valeurs::Relatif( n ) => Retour::creer( true, format!( "(réel) {}", n ) ), 
// 			Valeurs::Flottant( n ) => Retour::creer( true, format!( "(flottant) {}", n ) ), 
// 			Valeurs::Objet( o ) => Retour::creer( true, format!( "(objet ; non-encore implémenté) {:?}", o ) ), 
// 		} 
// 	} else { 
// 		Retour::creer_str( false, "clé inconnue" ) 
// 	} 
// } 

// fn resoudre_supprimer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let cle = if let Some( c ) = arguments.extraire() { 
// 		c 
// 	} else { 
// 		return Retour::creer_str( false, "vous devez spécifier une clé existante" ); 
// 	}; 
// 	if !arguments.est_stop() { 
// 		return Retour::creer_str( false, "trop d'arguments fournis (maximum 1)" ); 
// 	} 
// 	let mut dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &mut dico.liste; 
// 	if let Some( _ ) = valeurs.remove( &cle ) { 
// 		Retour::creer_str( true, "clé supprimée" ) 
// 	} else { 
// 		Retour::creer_str( false, "clé inconnue" ) 
// 	} 
// } 

// fn resoudre_lister( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	if let Some( _ ) = arguments.extraire() { 
// 		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
// 	} 
// 	let dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &dico.liste; 
// 	for (cle, valeur) in valeurs.iter() { 
// 		if let Err(_) = contexte.stream.write( 
// 			format!( 
// 				"\t{} : {:?}\n", 
// 				cle, 
// 				valeur 
// 			).as_bytes() 
// 		) { 
// 			contexte.stream.flush().unwrap(); 
// 			return Retour::creer_str( false, "erreur lors de l'envoi" ); 
// 		} 
// 	} 
// 	contexte.stream.flush().unwrap(); 
// 	Retour::creer( true, format!( "stop ({})", valeurs.len() ) ) 
// } 

// fn resoudre_tester( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour {
// 	let cle = if let Some( c ) = arguments.extraire() { 
// 		c 
// 	} else { 
// 		return Retour::creer_str( false, "vous devez spécifier une clé à tester" ); 
// 	}; 
// 	if !arguments.est_stop() { 
// 		return Retour::creer_str( false, "trop d'arguments fournis (maximum 1)" ); 
// 	} 
// 	let dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &dico.liste; 
// 	if valeurs.contains_key( &cle ) { 
// 		Retour::creer_str( true, "clé existante" ) 
// 	} else { 
// 		Retour::creer_str( true, "clé inexistante" ) 
// 	} 
// } 

// fn resoudre_ajouter( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let cle = if let Some( c ) = arguments.extraire() { 
// 		c 
// 	} else { 
// 		return Retour::creer_str( false, "une clé vide n'est pas une clé acceptable" ); 
// 	}; 
// 	let ajout = if let Some( v ) = arguments.extraire() { 
// 		v 
// 	} else { 
// 		return Retour::creer_str( false, "aucune valeur fournie ou séparateur clé/valeur non-respecté (espace simple)" ); 
// 	}; 
// 	if !arguments.est_stop() { 
// 		return Retour::creer_str( false, "trop d'arguments fournis (maximum 2)" ); 
// 	} 
// 	let mut dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &mut dico.liste; 
// 	if let Some( v ) = valeurs.get_mut( &cle ) { 
// 		if v.ajouter_texte( &ajout ) { 
// 			Retour::creer_str( true, "valeur modifée" ) 
// 		} else { 
// 			Retour::creer_str( false, "ce format n'est pas supporté ou le texte est trop long" ) 
// 		} 
// 	} else { 
// 		Retour::creer_str( false, "clé inconnue" ) 
// 	} 
// } 

// fn resoudre_alterer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let cle = if let Some( c ) = arguments.extraire() { 
// 		c 
// 	} else { 
// 		return Retour::creer_str( false, "vous devez spécifier une clé existante" ); 
// 	}; 
// 	let valeur_type = if let Some( t ) = arguments.extraire() { 
// 		t 
// 	} else { 
// 		return Retour::creer_str( false, "vous devez spécifier un type connu" ); 
// 	}; 
// 	if !arguments.est_stop() { 
// 		return Retour::creer_str( false, "trop d'arguments fournis (maximum 2)" ); 
// 	} 
// 	let mut dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &mut dico.liste; 
// 	if let Some( v ) = valeurs.get_mut( &cle ) { 
// 		if v.alterer( &valeur_type ) { 
// 			Retour::creer_str( true, "altération effectuée" ) 
// 		} else { 
// 			Retour::creer( false, format!( 
// 				"altération impossible avec ce type '{}'", 
// 				valeur_type 
// 			) ) 
// 		} 
// 	} else { 
// 		Retour::creer_str( false, "clé inconnue" ) 
// 	} 
// } 

// fn resoudre_resumer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	if !arguments.est_stop() { 
// 		return Retour::creer_str( false, "aucun argument autorisé" ); 
// 	} 
// 	let dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &dico.liste; 
// 	Retour::creer(  
// 		true, 
// 		format!( 
// 			"canal \"{}\" ({})", 
// 			dico.nom, 
// 			valeurs.len() 
// 		) 
// 	) 
// } 

pub fn resoudre( contexte: &mut Contexte, appel: &str, arguments: &str ) -> Retour { 
	(if let Some( n ) = appel.find( ':' ) { 
		match &appel[..n] { 
			// "numérique" => match resoudre_numerique::resoudre( &appel[n+1..] ) { 
			// 	Ok( fct ) => fct, 
			// 	Err( r ) => return r 
			// }, 
			// "texte" => match resoudre_texte::resoudre( &appel[n+1..] ) { 
			// 	Ok( fct ) => fct, 
			// 	Err( r ) => return r 
			// }, 
			// "canal" => match resoudre_canal::resoudre( &appel[n+1..] ) { 
			// 	Ok( fct ) => fct, 
			// 	Err( r ) => return r 
			// }, 
			// "administration" => match resoudre_administration::resoudre( &appel[n+1..] ) { 
			// 	Ok( fct ) => fct, 
			// 	Err( r ) => return r 
			// }, 
			_ => return Retour::creer_str( false, "module inconnu" ) 
		}
	} else { 
		match appel { 
			// actions génériques 
			"stop" => resoudre_stop as Resolveur, 
			// "vider" => resoudre_vider as Resolveur, 
			// "définir" => resoudre_definir as Resolveur, 
			// "obtenir" => resoudre_obtenir as Resolveur, 
			// "supprimer" => resoudre_supprimer as Resolveur, 
			// "lister" => resoudre_lister as Resolveur, 
			// "tester" => resoudre_tester as Resolveur, 
			// "ajouter" => resoudre_ajouter as Resolveur, 
			// "altérer" => resoudre_alterer as Resolveur, 
			// "résumer" => resoudre_resumer as Resolveur, 

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

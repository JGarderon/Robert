
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
use crate::base::AccesseurCanalV; 

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

/// Les retours peuvent être soit un texte statique (_&'static str_) - c'est-à-dire invariable et intégré au directement dans le code source du programme (efficacité), soit un texte généré par la fonction de résolution (String) - c'est-à-dire variable. 
pub enum RetourType { 

	/// Est de type _&'static str_ 
	Statique(&'static str), 
	
	/// Est de type _String_ 
	Dynamique(String) 

} 

impl RetourType { 
	
	/// Rassemble la valeur vers l'équivalent d'un slice de _Bytes_, qui sera utilisé pour le retour écrit sur le socket du client. 
	pub fn vers_bytes( &self ) -> &[u8] { 
		match self { 
			RetourType::Statique( m ) => m.as_bytes(), 
			RetourType::Dynamique( m ) => m.as_bytes() 
		} 
	} 

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

fn resoudre_stop( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "aucun argument autorisé" ); 
	} 
	contexte.poursuivre = false; 
	Retour::creer_str( true, "au revoir" ) 
} 

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
			"définir" => resoudre_definir as Resolveur, 
			"obtenir" => resoudre_obtenir as Resolveur, 
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



















macro_rules! Canal {
    ( $contexte:ident ) => {
        { 
        	match $contexte.canalthread.lock() { 
        		Ok( c )	=> c, 
        		Err( empoisonne ) => empoisonne.into_inner() 
        	} 
        } 
    };
} 

fn resoudre_definir ( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let mut arg_chemin = if let Some( c ) = arguments.extraire() { 
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

fn resoudre_obtenir ( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let mut arg_chemin = if let Some( c ) = arguments.extraire() { 
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




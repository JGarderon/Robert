
use std::io::Write; 
use std::sync::mpsc::{Sender, Receiver}; 
use std::sync::mpsc; 
use std::collections::HashMap; 
use std::sync::Arc; 
use std::sync::Mutex; 
use std::net::TcpStream; 

// ---------------------------------------------------- 

use crate::grammaire::ArgumentsLocaux; 
use crate::base::DictionnaireThread; 
use crate::base::Dictionnaires; 
use crate::base::Dictionnaire; 
use crate::base::Valeurs; 

// ---------------------------------------------------- 

use crate::DEBUG; 
use crate::DICO_NOM_DEFAUT; 

// ---------------------------------------------------- 

mod resoudre_numerique; 
mod resoudre_texte; 

// ---------------------------------------------------- 

type Resolveur = fn ( &mut Contexte, ArgumentsLocaux ) -> Retour; 

// ---------------------------------------------------- 

pub struct Contexte { 
	pub poursuivre: bool, 
	pub dico: DictionnaireThread, 
	pub dicos: Arc<Mutex<Dictionnaires>>, 
	pub resoudre: fn(&mut Contexte, &str, &str) -> Retour, 
	pub stream: TcpStream 
} 

// ---------------------------------------------------- 

pub enum RetourType { 
	Statique(&'static str), 
	Dynamique(String) 
} 

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

fn resoudre_vider( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "aucun argument autorisé" ); 
	} 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	valeurs.clear(); 
	Retour::creer_str( true, "base vidée" ) 
} 

fn resoudre_definir( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let cle = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "une clé vide n'est pas une clé acceptable" ); 
	}; 
	let valeur = if let Some( v ) = arguments.extraire() { 
		v 
	} else { 
		return Retour::creer_str( false, "aucune valeur fournie ou séparateur clé/valeur non-respecté (espace simple)" ); 
	}; 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	match arguments.extraire() { 
		None => { 
			valeurs.insert( 
				cle, 
				Valeurs::Texte( valeur ) 
			); 
			Retour::creer_str( true, "paire clé/valeur ajoutée (type par défaut : texte)" ) 
		} 
		Some( t ) => { 
			if !arguments.est_stop() { 
				return Retour::creer_str( false, "trop d'arguments fournis (max. 2-3)" ); 
			} 
			let mut v = Valeurs::Texte( valeur ); 
			if v.alterer( &t ) { 
				valeurs.insert( 
					cle, 
					v 
				);  
				Retour::creer( true, format!( 
					"paire clé/valeur ajoutée (type {})", 
					&t
				) ) 
			} else { 
				Retour::creer( false, format!( 
					"le type '{}' n'est pas un type conforme", 
					&t
				) ) 
			} 
		} 
	} 
} 

fn resoudre_obtenir( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let cle = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "vous devez spécifier une clé existante" ); 
	}; 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "trop d'arguments fournis (maximum 1)" ); 
	} 
	let dico = contexte.dico.lock().unwrap(); 
	let valeurs = &dico.liste; 
	if valeurs.contains_key( &cle ) { 
		match &valeurs[&cle] { 
			Valeurs::Boolean( b ) => Retour::creer( true, format!( "(booléen) {}", b ) ), 
			Valeurs::Texte( t ) => Retour::creer( true, format!( "(texte) \"{}\"", t ) ), 
			Valeurs::Reel( n ) => Retour::creer( true, format!( "(réel) {}", n ) ), 
			Valeurs::Flottant( n ) => Retour::creer( true, format!( "(flottant) {}", n ) ), 
		} 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

fn resoudre_supprimer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let cle = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "vous devez spécifier une clé existante" ); 
	}; 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "trop d'arguments fournis (maximum 1)" ); 
	} 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	if let Some( _ ) = valeurs.remove( &cle ) { 
		Retour::creer_str( true, "clé supprimée" ) 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

fn resoudre_lister( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if let Some( _ ) = arguments.extraire() { 
		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
	} 
	let dico = contexte.dico.lock().unwrap(); 
	let valeurs = &dico.liste; 
	for (cle, valeur) in valeurs.iter() { 
		if let Err(_) = contexte.stream.write( 
			format!( 
				"\t{} : {:?}\n", 
				cle, 
				valeur 
			).as_bytes() 
		) { 
			contexte.stream.flush().unwrap(); 
			return Retour::creer_str( false, "erreur lors de l'envoi" ); 
		} 
	} 
	contexte.stream.flush().unwrap(); 
	Retour::creer( true, format!( "stop ({})", valeurs.len() ) ) 
} 

fn resoudre_tester( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour {
	let cle = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "vous devez spécifier une clé à tester" ); 
	}; 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "trop d'arguments fournis (maximum 1)" ); 
	} 
	let dico = contexte.dico.lock().unwrap(); 
	let valeurs = &dico.liste; 
	if valeurs.contains_key( &cle ) { 
		Retour::creer_str( true, "clé existante" ) 
	} else { 
		Retour::creer_str( true, "clé inexistante" ) 
	} 
} 

fn resoudre_ajouter( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let cle = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "une clé vide n'est pas une clé acceptable" ); 
	}; 
	let ajout = if let Some( v ) = arguments.extraire() { 
		v 
	} else { 
		return Retour::creer_str( false, "aucune valeur fournie ou séparateur clé/valeur non-respecté (espace simple)" ); 
	}; 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "trop d'arguments fournis (maximum 2)" ); 
	} 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	if let Some( v ) = valeurs.get_mut( &cle ) { 
		if v.ajouter_texte( &ajout ) { 
			Retour::creer_str( true, "valeur modifée" ) 
		} else { 
			Retour::creer_str( false, "ce format n'est pas supporté ou le texte est trop long" ) 
		} 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

fn resoudre_alterer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let cle = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "vous devez spécifier une clé existante" ); 
	}; 
	let valeur_type = if let Some( t ) = arguments.extraire() { 
		t 
	} else { 
		return Retour::creer_str( false, "vous devez spécifier un type connu" ); 
	}; 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "trop d'arguments fournis (maximum 2)" ); 
	} 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	if let Some( v ) = valeurs.get_mut( &cle ) { 
		if v.alterer( &valeur_type ) { 
			Retour::creer_str( true, "altération effectuée" ) 
		} else { 
			Retour::creer( false, format!( 
				"altération impossible avec ce type '{}'", 
				valeur_type 
			) ) 
		} 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

fn resoudre_resumer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "aucun argument autorisé" ); 
	} 
	let dico = contexte.dico.lock().unwrap(); 
	let valeurs = &dico.liste; 
	Retour::creer(  
		true, 
		format!( 
			"canal \"{}\" ({})", 
			dico.nom, 
			valeurs.len() 
		) 
	) 
} 

// ### --- ### 

fn resoudre_canal_creer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let nom = if let Some( n ) = arguments.extraire() { 
		n 
	} else { 
		return Retour::creer_str( false, "nom de canal obligatoire" ); 
	}; 
	if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
	} else { 
		let mut dicos = contexte.dicos.lock().unwrap(); 
		if dicos.liste.contains_key( &nom ) { 
			Retour::creer_str( false, "canal existant" ) 
		} else { 
			dicos.liste.insert( 
				nom.to_string(), 
				Arc::new( Mutex::new( Dictionnaire { 
					nom: nom, 
					liste: HashMap::new(), 
					souscripteurs: Vec::<Sender<String>>::new() 
				} ) ) as DictionnaireThread  
			); 
			Retour::creer_str( true, "canal créé" ) 
		} 
	} 
} 

fn resoudre_canal_supprimer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let nom = if let Some( n ) = arguments.extraire() { 
		n 
	} else { 
		return Retour::creer_str( false, "nom de canal obligatoire" ); 
	}; 
	if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
	} else if &nom == DICO_NOM_DEFAUT { 
		Retour::creer_str( false, "impossible de supprimer le canal par défaut" ) 
	} else { 
		let mut dicos = contexte.dicos.lock().unwrap(); 
		if dicos.liste.contains_key( &nom ) { 
			{ 
				let message = "canal supprimé".to_string(); 
				let mut dico = dicos.liste[&nom].lock().unwrap(); 
				dico.souscripteurs.retain( 
					| souscripteur | { 
						souscripteur.send( message.clone() ).unwrap(); 
						false 
					} 
				); 
			} 
			if let Some(_) = dicos.liste.remove( &nom ) { 
				Retour::creer_str( true, "canal supprimé" ) 
			} else { 
				Retour::creer_str( false, "impossible de supprimer le canal" ) 
			} 
		} else { 
			Retour::creer_str( false, "canal inexistant" ) 
		} 
	} 
} 

fn resoudre_canal_tester( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let nom = if let Some( n ) = arguments.extraire() { 
		n 
	} else { 
		return Retour::creer_str( false, "nom de canal obligatoire" ); 
	}; 
	if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
	} else { 
		let dicos = contexte.dicos.lock().unwrap(); 
		if dicos.liste.contains_key( &nom ) { 
			Retour::creer_str( true, "canal existant" ) 
		} else { 
			Retour::creer_str( true, "canal inexistant" ) 
		} 
	} 
} 

fn resoudre_canal_lister( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if let Some( _ ) = arguments.extraire() { 
		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
	} 
	let dicos = contexte.dicos.lock().unwrap(); 
	for (nom, d) in dicos.liste.iter() { 
		let dico = d.lock().unwrap(); 
		if let Err(_) = contexte.stream.write( 
			format!( 
				"\tcanal \"{}\" ({:?})\n", 
				nom, 
				dico.liste.len() 
			).as_bytes() 
		) { 
			contexte.stream.flush().unwrap(); 
			return Retour::creer_str( false, "erreur lors de l'envoi" ); 
		} 
	} 
	contexte.stream.flush().unwrap(); 
	Retour::creer( true, format!( "stop ({})", dicos.liste.len() ) ) 
} 

fn resoudre_canal_changer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let nom = if let Some( n ) = arguments.extraire() { 
		n 
	} else { 
		return Retour::creer_str( false, "nom de canal obligatoire" ); 
	}; 
	if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
	} else { 
		let dicos = contexte.dicos.lock().unwrap(); 
		if dicos.liste.contains_key( &nom ) { 
			contexte.dico = dicos.liste[&nom].clone(); 
			Retour::creer_str( true, "canal modifié" ) 
		} else { 
			Retour::creer_str( false, "canal inexistant" ) 
		} 
	} 
} 

fn resoudre_canal_capturer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if let Some( _ ) = arguments.extraire() { 
		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
	} 
	contexte.dico = Arc::new( Mutex::new( Dictionnaire { 
		nom: "".to_string(), 
		liste: HashMap::new(), 
		souscripteurs: Vec::<Sender<String>>::new() 
	} ) ) as DictionnaireThread; 
	Retour::creer_str( true, "canal privé actif" ) 
} 

fn resoudre_canal_souscrire( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if let Some( _ ) = arguments.extraire() { 
		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
	} 
	let (expediteur, destinaire): ( Sender<String>, Receiver<String> ) = mpsc::channel(); 
	{ 
		let mut dico = contexte.dico.lock().unwrap(); 
		dico.souscripteurs.push( expediteur ); 
	} 
	while let Ok( m ) = destinaire.recv() { 
		if let Ok( _ ) = contexte.stream.write( 
			format!( "\t>>> {}\n", m ).as_bytes() 
		) { 
			if let Ok( _ ) = contexte.stream.flush() { 
			} else { 
				break; 
			} 
		} else { 
			break; 
		}  
	} 
	Retour::creer_str( true, "diffusion terminée" ) 
} 

fn resoudre_canal_emettre( contexte: &mut Contexte, arguments: ArgumentsLocaux ) -> Retour { 
	let mut dico = contexte.dico.lock().unwrap(); 
	let message = arguments.source.iter().collect::<String>(); 
	dico.souscripteurs.retain( 
		| souscripteur | { 
			if let Ok( _ ) = souscripteur.send( message.clone() ) { 
				true 
			} else { 
				false 
			} 
		} 
	); 
	Retour::creer( 
		true, 
		format!( 
			"diffusion émise aux souscripteurs ({})", 
			dico.souscripteurs.len() 
		) 
	) 
} 

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
			_ => return Retour::creer_str( false, "module inconnu" ) 
		}
	} else { 
		match appel { 
			// actions génériques 
			"stop" => resoudre_stop as Resolveur, 
			"vider" => resoudre_vider as Resolveur, 
			"définir" => resoudre_definir as Resolveur, 
			"obtenir" => resoudre_obtenir as Resolveur, 
			"supprimer" => resoudre_supprimer as Resolveur, 
			"lister" => resoudre_lister as Resolveur, 
			"tester" => resoudre_tester as Resolveur, 
			"ajouter" => resoudre_ajouter as Resolveur, 
			"altérer" => resoudre_alterer as Resolveur, 
			"résumer" => resoudre_resumer as Resolveur, 
			
			// actions sur les canaux 
			"canal:créer" => resoudre_canal_creer as Resolveur, 
			"canal:capturer" => resoudre_canal_capturer as Resolveur, 
			"canal:supprimer" => resoudre_canal_supprimer as Resolveur, 
			"canal:tester" => resoudre_canal_tester as Resolveur as Resolveur, 
			"canal:lister" if DEBUG => resoudre_canal_lister as Resolveur, 
			"canal:changer" => resoudre_canal_changer as Resolveur, 
			"canal:souscrire" => resoudre_canal_souscrire as Resolveur, 
			"canal:émettre" => resoudre_canal_emettre as Resolveur, 

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

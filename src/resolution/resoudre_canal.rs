
use std::io::Write; 
use std::sync::mpsc::{Sender, Receiver}; 
use std::sync::mpsc; 
use std::sync::Arc; 
use std::sync::Mutex; 
use std::collections::HashMap; 

// ---------------------------------------------------- 

use crate::base::DictionnaireThread; 
use crate::base::Dictionnaire; 
use crate::resolution::Contexte; 
use crate::grammaire::ArgumentsLocaux; 

// ---------------------------------------------------- 

use crate::resolution::Resolveur; 
use crate::resolution::Retour; 

// ---------------------------------------------------- 

use crate::DEBUG; 
use crate::DICO_NOM_DEFAUT; 
use crate::NBRE_MAX_CANAUX; 

// ---------------------------------------------------- 

fn resoudre_creer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let nom = if let Some( n ) = arguments.extraire() { 
		n 
	} else { 
		return Retour::creer_str( false, "nom de canal obligatoire" ); 
	}; 
	if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
	} else { 
		let mut dicos = contexte.dicos.lock().unwrap(); 
		if dicos.liste.len() < NBRE_MAX_CANAUX { 
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
		} else { 
			Retour::creer_str( false, "nbre max. de canaux atteint" ) 
		} 
	} 
} 

fn resoudre_supprimer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
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

fn resoudre_tester( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
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

fn resoudre_lister( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
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

fn resoudre_changer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
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

fn resoudre_capturer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
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

fn resoudre_souscrire( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
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

fn resoudre_emettre( contexte: &mut Contexte, arguments: ArgumentsLocaux ) -> Retour { 
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

pub fn resoudre( appel: &str ) -> Result<Resolveur,Retour> { 
	match appel { 
		"créer" => Ok( resoudre_creer as Resolveur ), 
		"capturer" => Ok( resoudre_capturer as Resolveur ), 
		"supprimer" => Ok( resoudre_supprimer as Resolveur ), 
		"tester" => Ok( resoudre_tester as Resolveur ), 
		"lister" if DEBUG => Ok( resoudre_lister as Resolveur ), 
		"changer" => Ok( resoudre_changer as Resolveur ), 
		"souscrire" => Ok( resoudre_souscrire as Resolveur ), 
		"emettre" => Ok( resoudre_emettre as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module texte : fonction inconnue" ) ) 
	} 
} 








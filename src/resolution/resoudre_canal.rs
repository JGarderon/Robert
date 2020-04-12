
use std::io::Write; 
use std::sync::mpsc::{Sender, Receiver}; 
use std::sync::mpsc; 
use std::sync::Arc; 
use std::sync::Mutex; 
use std::collections::HashMap; 

// ---------------------------------------------------- 

use crate::base::CanalThread; 
use crate::base::Canal; 
use crate::base::Valeurs; 
use crate::resolution::Contexte; 
use crate::grammaire::ArgumentsLocaux; 

// ---------------------------------------------------- 

use crate::resolution::Resolveur; 
use crate::resolution::Retour; 

// ---------------------------------------------------- 

use crate::DEBUG; 
use crate::CANAL_NOM_DEFAUT; 
use crate::NBRE_MAX_CANAUX; 

// ---------------------------------------------------- 

/// # Fonction de résolution locale "créer un nouveau canal" 
///
fn resoudre_creer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let nom = if let Some( n ) = arguments.extraire() { 
		n 
	} else { 
		return Retour::creer_str( false, "nom de canal obligatoire" ); 
	}; 
	if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max. 32)" ) 
	} else { 
		let mut canaux = match contexte.canauxthread.lock() { 
			Ok( c ) => c, 
			Err( e ) => e.into_inner() 
		}; 
		if canaux.liste.len() < NBRE_MAX_CANAUX { 
			if canaux.liste.contains_key( &nom ) { 
				Retour::creer_str( false, "canal existant" ) 
			} else { 
				canaux.liste.insert( 
					nom.to_string(), 
					Arc::new( Mutex::new( Canal { 
						nom: nom, 
						liste: Valeurs::Objet( HashMap::new() ), 
						souscripteurs: Vec::<Sender<String>>::new() 
					} ) ) as CanalThread 
				); 
				Retour::creer_str( true, "canal créé" ) 
			}  
		} else { 
			Retour::creer_str( false, "nbre max. de canaux atteint" ) 
		} 
	} 
} 

/// # Fonction de résolution locale "supprimer un canal existant" 
///
/// Cette fonction tentera de supprimer un canal existant mais auparavant, elle avertira tous les éventuels souscripteurs présents de cette action, et les retira de la liste des souscripteurs. 
///
fn resoudre_supprimer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let nom = if let Some( n ) = arguments.extraire() { 
		n 
	} else { 
		return Retour::creer_str( false, "nom de canal obligatoire" ); 
	}; 
	if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max. 32)" ) 
	} else { 
		let mut canaux = match contexte.canauxthread.lock() { 
			Ok( c ) => c, 
			Err( e ) => e.into_inner() 
		}; 
		if canaux.liste.contains_key( &nom ) { 
			{ 
				let message = "canal supprimé".to_string(); 
				match canaux.liste[&nom].lock() { 
					Ok( c ) => c, 
					Err( e ) => e.into_inner() 
				}.souscripteurs.retain( 
					| souscripteur | { 
						souscripteur.send( message.clone() ).unwrap(); 
						false 
					} 
				); 
			} 
			if let Some(_) = canaux.liste.remove( &nom ) { 
				Retour::creer_str( true, "canal supprimé" ) 
			} else { 
				Retour::creer_str( false, "impossible de supprimer le canal" ) 
			} 
		} else { 
			Retour::creer_str( false, "canal inexistant" ) 
		} 
	}
} 

/// # Fonction de résolution locale "tester l'existence d'un canal" 
///
/// Ai-je vraiment besoin de décrire en détail son intérêt ? :) 
///
fn resoudre_tester( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let nom = if let Some( n ) = arguments.extraire() { 
		n 
	} else { 
		return Retour::creer_str( false, "nom de canal obligatoire" ); 
	}; 
	if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max. 32)" ) 
	} else { 
		if (match contexte.canauxthread.lock() { 
			Ok( c ) => c, 
			Err( e ) => e.into_inner() 
		}).liste.contains_key( &nom ) { 
			Retour::creer_str( true, "canal existant" ) 
		} else { 
			Retour::creer_str( true, "canal inexistant" ) 
		} 
	} 
} 

/// # Fonction de résolution locale "capturer un nouveau canal" 
///
/// Cette fonction est un peu particulière, car elle crééra un canal spéficique à un thread. 
///
/// Le client disposera donc de son propre stockage de valeurs, ce qui est son intérêt. La perte de connexion provoque aussi la perte des informations contenus dans ce canal "artificiel". Il n'est référencé nul part ailleurs : aucune fonction classique sur les canaux n'aura de prise sur lui. 
///
/// Aucun souscripteur ne pourra donc s'y abonner (à part le client lui-même... mais ce dernier étant représenté par un socket, il ne peut pas émettre et recevoir en même temps). 
///
/// Le nom d'un canal "capturé" (privé), doit toujours être nul (c'est un signe distinctif, un nom de canal ne doit jamais être nul). 
///
/// En l'état, il peut donc avoir un écart sur la mesure de la taille de la mémoire si la mesure passe seulement par la structure Canaux et qu'acune recherche des canaux "capturés" n'est réalisée. 
///
fn resoudre_capturer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if let Some( _ ) = arguments.extraire() { 
		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
	} 
	contexte.canalthread = Arc::new( Mutex::new( Canal { 
		nom: "".to_string(), 
		liste: Valeurs::Objet( HashMap::new() ), 
		souscripteurs: Vec::<Sender<String>>::new() 
	} ) ) as CanalThread ; 
	Retour::creer_str( true, "canal privé actif" ) 
} 

// fn resoudre_changer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let nom = if let Some( n ) = arguments.extraire() { 
// 		n 
// 	} else { 
// 		return Retour::creer_str( false, "nom de canal obligatoire" ); 
// 	}; 
// 	if nom.len() > 32 { 
// 		Retour::creer_str( false, "nom de canal trop long (max. 32)" ) 
// 	} else { 
// 		let dicos = contexte.dicos.lock().unwrap(); 
// 		if dicos.liste.contains_key( &nom ) { 
// 			contexte.dico = dicos.liste[&nom].clone(); 
// 			Retour::creer_str( true, "canal modifié" ) 
// 		} else { 
// 			Retour::creer_str( false, "canal inexistant" ) 
// 		} 
// 	} 
// } 

// fn resoudre_souscrire( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	if let Some( _ ) = arguments.extraire() { 
// 		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
// 	} 
// 	let (expediteur, destinaire): ( Sender<String>, Receiver<String> ) = mpsc::channel(); 
// 	{ 
// 		let mut dico = contexte.dico.lock().unwrap(); 
// 		dico.souscripteurs.push( expediteur ); 
// 	} 
// 	while let Ok( m ) = destinaire.recv() { 
// 		if let Ok( _ ) = contexte.stream.write( 
// 			format!( "\t>>> {}\n", m ).as_bytes() 
// 		) { 
// 			if let Ok( _ ) = contexte.stream.flush() { 
// 			} else { 
// 				break; 
// 			} 
// 		} else { 
// 			break; 
// 		}  
// 	} 
// 	Retour::creer_str( true, "diffusion terminée" ) 
// } 

// fn resoudre_emettre( contexte: &mut Contexte, arguments: ArgumentsLocaux ) -> Retour { 
// 	let mut dico = contexte.dico.lock().unwrap(); 
// 	let message = arguments.source.iter().collect::<String>(); 
// 	dico.souscripteurs.retain( 
// 		| souscripteur | { 
// 			if let Ok( _ ) = souscripteur.send( message.clone() ) { 
// 				true 
// 			} else { 
// 				false 
// 			} 
// 		} 
// 	); 
// 	Retour::creer( 
// 		true, 
// 		format!( 
// 			"diffusion émise aux souscripteurs ({})", 
// 			dico.souscripteurs.len() 
// 		) 
// 	) 
// } 

pub fn resoudre( appel: &str ) -> Result<Resolveur,Retour> { 
	match appel { 
		"créer" => Ok( resoudre_creer as Resolveur ), 
		"supprimer" => Ok( resoudre_supprimer as Resolveur ), 
		"tester" => Ok( resoudre_tester as Resolveur ), 
		"capturer" => Ok( resoudre_capturer as Resolveur ), 
		// "changer" => Ok( resoudre_changer as Resolveur ), 
		// "souscrire" => Ok( resoudre_souscrire as Resolveur ), 
		// "emettre" => Ok( resoudre_emettre as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module texte : fonction inconnue" ) ) 
	} 
} 








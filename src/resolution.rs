
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

fn resoudre_numerique_incrementer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
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

fn resoudre_numerique_maj( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
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

// ### --- ### 

fn resoudre_texte_contenir( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let motif = if let Some( m ) = arguments.extraire() { 
		m 
	} else { 
		return Retour::creer_str( false, "motif de recherche obligatoire" ); 
	}; 
	let dico = contexte.dico.lock().unwrap(); 
	let mut i = 0; 
	for (cle, valeur) in dico.liste.iter() { 
		match valeur { 
			Valeurs::Texte( t ) => { 
				if t.contains( &motif ) { 
					if let Err(_) = contexte.stream.write( 
						format!( 
							"\t{}\n", 
							cle 
						).as_bytes() 
					) { 
						contexte.stream.flush().unwrap(); 
						return Retour::creer_str( false, "erreur lors de l'envoi" ); 
					} 
					i += 1; 
				} 
			} 
			_ => () 
		} 
	} 
	Retour::creer( true, format!( "stop ({})", i ) ) 
} 

fn resoudre_texte_debuter( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let motif = if let Some( m ) = arguments.extraire() { 
		m 
	} else { 
		return Retour::creer_str( false, "motif de recherche obligatoire" ); 
	}; 
	let dico = contexte.dico.lock().unwrap(); 
	let mut i = 0; 
	for (cle, valeur) in dico.liste.iter() { 
		match valeur { 
			Valeurs::Texte( t ) => { 
				if t.starts_with( &motif ) { 
					if let Err(_) = contexte.stream.write( 
						format!( 
							"\t{}\n", 
							cle 
						).as_bytes() 
					) { 
						contexte.stream.flush().unwrap(); 
						return Retour::creer_str( false, "erreur lors de l'envoi" ); 
					} 
					i += 1; 
				} 
			} 
			_ => () 
		} 
	} 
	Retour::creer( true, format!( "stop ({})", i ) ) 
} 

fn resoudre_texte_terminer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let motif = if let Some( m ) = arguments.extraire() { 
		m 
	} else { 
		return Retour::creer_str( false, "motif de recherche obligatoire" ); 
	}; 
	let dico = contexte.dico.lock().unwrap(); 
	let mut i = 0; 
	for (cle, valeur) in dico.liste.iter() { 
		match valeur { 
			Valeurs::Texte( t ) => { 
				if t.ends_with( &motif ) { 
					if let Err(_) = contexte.stream.write( 
						format!( 
							"\t{}\n", 
							cle 
						).as_bytes() 
					) { 
						contexte.stream.flush().unwrap(); 
						return Retour::creer_str( false, "erreur lors de l'envoi" ); 
					} 
					i += 1; 
				} 
			} 
			_ => () 
		} 
	} 
	Retour::creer( true, format!( "stop ({})", i ) ) 
} 

fn resoudre_texte_remplacer_un( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	let cle = if let Some( c ) = arguments.extraire() { 
		c 
	} else { 
		return Retour::creer_str( false, "clé obligatoire" ); 
	}; 
	let recherche = if let Some( m ) = arguments.extraire() { 
		m 
	} else { 
		return Retour::creer_str( false, "motif de recherche obligatoire" ); 
	}; 
	let remplacement = if let Some( r ) = arguments.extraire() { 
		r 
	} else { 
		return Retour::creer_str( false, "motif de remplacement obligatoire" ); 
	}; 
	let nbre_max =  arguments.extraire(); 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	if let Some( v ) = valeurs.get_mut( &cle ) { 
		match v { 
			Valeurs::Texte( t ) => { 
				if let Some( t_n ) = nbre_max { 
					if let Ok( n ) = t_n.parse() { 
						*t = t.replacen( &recherche, &remplacement, n ); 
						Retour::creer_str( true, "remplacement(s) effectué(s)" ) 
					} else { 
						Retour::creer_str( true, "nbre de remplacements maximum invalide" ) 
					} 
				} else { 
					*t = t.replace( &recherche, &remplacement ); 
					Retour::creer_str( true, "remplacement(s) effectué(s)" ) 
				} 
			} 
			_ => Retour::creer_str( false, "la valeur n'est pas un texte" ) 
		} 
	} else { 
		Retour::creer_str( false, "erreur lors de l'envoi" ) 
	} 
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
	(match appel { 
			// actions génériques 
			"stop" => resoudre_stop, 
			"vider" => resoudre_vider, 
			"définir" => resoudre_definir, 
			"obtenir" => resoudre_obtenir, 
			"supprimer" => resoudre_supprimer, 
			"lister" => resoudre_lister, 
			"tester" => resoudre_tester, 
			"ajouter" => resoudre_ajouter, 
			"altérer" => resoudre_alterer, 
			"résumer" => resoudre_resumer, 

			// actions sur les valeurs numériques 
			"numérique:incrémenter" => resoudre_numerique_incrementer, 
			"numérique:màj" => resoudre_numerique_maj, 

			// actions sur les valeurs textuelles 
			"texte:contenir" => resoudre_texte_contenir, 
			"texte:débuter" => resoudre_texte_debuter, 
			"texte:terminer" => resoudre_texte_terminer, 
			"texte:remplacerun" => resoudre_texte_remplacer_un, 
			
			// actions sur les canaux 
			"canal:créer" => resoudre_canal_creer, 
			"canal:capturer" => resoudre_canal_capturer, 
			"canal:supprimer" => resoudre_canal_supprimer, 
			"canal:tester" => resoudre_canal_tester, 
			"canal:lister" if DEBUG => resoudre_canal_lister, 
			"canal:changer" => resoudre_canal_changer, 
			"canal:souscrire" => resoudre_canal_souscrire, 
			"canal:émettre" => resoudre_canal_emettre, 

			_ => return Retour::creer_str( false, "fonction inconnue" ) 
		})( 
			contexte, 
			ArgumentsLocaux { 
		        source: arguments.chars().collect::<Vec<char>>(), 
		        position: 0 
		    } 
		) 
} 


use std::iter::FromIterator; 
use std::net::{TcpListener, TcpStream}; 
use std::io::Read; 
use std::io::Write; 
use std::io::Bytes; 
use std::iter::Enumerate; 
use std::thread; 
use std::thread::JoinHandle; 
use std::collections::HashMap; 
use std::sync::Arc; 
use std::sync::Mutex; 
use std::sync::mpsc; 
use std::sync::mpsc::{Sender, Receiver}; 

// ---------------------------------------------------- 

const DEBUG: bool = true; 

const DICO_NOM_DEFAUT: &'static str = "défaut";

const TAILLE_LIGNE_MAX: usize = 1024; 
const TAILLE_TEXTE_MAX: usize = TAILLE_LIGNE_MAX*5; 

// ---------------------------------------------------- 

#[derive(Debug)] 
enum ArgumentsLocauxEtat { 
    Suivant(usize, usize), 
    Stop, 
    Erreur(&'static str) 
} 

#[derive(Debug)] 
struct ArgumentsLocaux { 
    source: Vec<char>, 
    position: usize 
} 

impl ArgumentsLocaux { 
    fn trim( &self, texte: &[char] ) -> Option<usize> { 
        for (i, signe) in texte.iter().enumerate() { 
            match signe { 
                ' ' | '\t'| '\r' | '\n'  => (), 
                _ => return Some( i ) 
            } 
        } 
        None 
    } 
    fn suivant( &mut self ) -> ArgumentsLocauxEtat { 
        let texte = &self.source[self.position..]; 
        if texte.len() == 0 { 
            return ArgumentsLocauxEtat::Stop; 
        } 
        let debut = match self.trim( texte ) { 
            Some( i ) => i, 
            None => return ArgumentsLocauxEtat::Stop 
        }; 
        let mut ouvert = false; 
        for (i, signe) in texte[debut..].iter().enumerate() { 
            match signe { 
                ' ' if !ouvert => return ArgumentsLocauxEtat::Suivant( debut, debut+i ), 
                '"' => { 
                    ouvert = !ouvert; 
                    if !ouvert { 
                        return ArgumentsLocauxEtat::Suivant( debut+1, debut+i ); 
                    } 
                } 
                _ => () 
            } 
        } 
        if ouvert { 
            ArgumentsLocauxEtat::Erreur( "guillemet non-fermé" ) 
        } else { 
            ArgumentsLocauxEtat::Suivant( debut, texte.len() ) 
        } 
    } 
    fn extraire( &mut self ) -> Option<String> { 
        if let ArgumentsLocauxEtat::Suivant( depart, stop ) = self.suivant() { 
            let r = &self.source[self.position+depart..self.position+stop]; 
            self.position += stop; 
            Some( String::from_iter( r ) ) 
        } else { 
            None 
        } 
    } 
    fn est_stop( &mut self ) -> bool { 
        if let None = self.extraire() { 
            true 
        } else { 
            false 
        } 
    } 
} 

// ---------------------------------------------------- 

enum RetourType { 
	Statique(&'static str), 
	Dynamique(String) 
} 

impl RetourType { 
	fn vers_bytes( &self ) -> &[u8] { 
		match self { 
			RetourType::Statique( m ) => m.as_bytes(), 
			RetourType::Dynamique( m ) => m.as_bytes() 
		} 
	} 
} 

struct Retour { 
	etat: bool, 
	message: RetourType 
} 

impl Retour { 
	fn creer( etat: bool, m: String ) -> Self { 
		Retour { 
			etat: etat, 
			message: RetourType::Dynamique( m )
		} 
	} 
	fn creer_str( etat: bool, m: &'static str ) -> Self { 
		Retour { 
			etat: etat, 
			message: RetourType::Statique( m )
		} 
	} 
}

// ---------------------------------------------------- 

#[derive(Debug)] 
enum Valeurs { 
	Boolean(bool), 
	Reel(i32), 
	Flottant(f32), 
	Texte(String) 
} 

impl Drop for Valeurs { 
    fn drop(&mut self) { 
    	if DEBUG { 
        	println!( "! suppression 'Valeurs' : {:?}", self); 
    	}
    } 
} 

impl Valeurs { 
	fn alterer( &mut self, r#type: &str ) -> bool { 
		match r#type { 
			"booléen" => match self { 
				Valeurs::Boolean( _ ) => true, 
				Valeurs::Reel( n ) => {
					*self = Valeurs::Boolean( if *n > 0i32 { true } else { false } ); 
					true 
				} 
				Valeurs::Flottant( n ) => { 
					*self = Valeurs::Boolean( if *n > 0f32 { true } else { false } ); 
					true 
				} 
				Valeurs::Texte( t ) => { 
					match &t[..] { 
						"vrai" => { 
							*self = Valeurs::Boolean( true ); 
							true 
						} 
						"false" => { 
							*self = Valeurs::Boolean( false ); 
							true 
						} 
						_ => false 
					} 
				} 
			} 
			"réel" => match self { 
				Valeurs::Reel( _ ) => true, 
				Valeurs::Boolean( b ) => { 
					*self = Valeurs::Reel( if *b { 1i32 } else { 0i32 } ); 
					true 
				} 
				Valeurs::Flottant( n ) => { 
					*self = Valeurs::Reel( n.round() as i32 ); 
					true 
				} 
				Valeurs::Texte( t ) => { 
					if let Ok( n ) = t.parse::<i32>() { 
						*self = Valeurs::Reel( n ); 
						true 
					} else { 
						false 
					} 
				} 
			} 
			"flottant" => match self { 
				Valeurs::Flottant( _ ) => true, 
				Valeurs::Reel( n ) => { 
					*self = Valeurs::Flottant( *n as f32 ); 
					true 
				}, 
				Valeurs::Boolean( b ) => { 
					*self = Valeurs::Flottant( if *b { 1f32 } else { 0f32 } ); 
					true 
				}, 
				Valeurs::Texte( t ) => { 
					if let Ok( n ) = t.parse::<f32>() { 
						*self = Valeurs::Flottant( n ); 
						true 
					} else { 
						false 
					} 
				} 
			} 
			"texte" => match self { 
				Valeurs::Texte( _ ) => true, 
				Valeurs::Boolean( b ) => { 
					*self = Valeurs::Texte( if *b { "vrai".to_string() } else { "faux".to_string() } ); 
					true 
				}, 
				Valeurs::Reel( n ) => { 
					*self = Valeurs::Texte( n.to_string() ); 
					true 
				} 
				Valeurs::Flottant( n ) => { 
					*self = Valeurs::Texte( n.to_string() ); 
					true 
				} 
			} 
			_ => false 
		} 
	} 
    fn ajouter_texte( &mut self, v: &str ) -> bool { 
        match self { 
            Valeurs::Texte( t ) => { 
            	if t.len() + v.len() < TAILLE_TEXTE_MAX { 
	            	t.push_str( v ); 
	            	true 
            	} else { 
            		false 
            	} 
            } 
            _ => return false 
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

// fn resoudre_canal_creer( contexte: &mut Contexte, arguments: ArgumentsLocaux ) -> Retour { 
// 	let nom = arguments.trim(); 
// 	if nom == "" { 
// 		Retour::creer_str( false, "nom de canal obligatoire" ) 
// 	} else if nom.len() > 32 { 
// 		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
// 	} else { 
// 		let mut dicos = contexte.dicos.lock().unwrap(); 
// 		if dicos.liste.contains_key( arguments ) { 
// 			Retour::creer_str( false, "canal existant" ) 
// 		} else { 
// 			dicos.liste.insert( 
// 				arguments.to_string(), 
// 				Arc::new( Mutex::new( Dictionnaire { 
// 					nom: nom.to_string(), 
// 					liste: HashMap::new(), 
// 					souscripteurs: Vec::new() 
// 				} ) ) as DictionnaireThread  
// 			); 
// 			Retour::creer_str( true, "canal créé" ) 
// 		} 
// 	} 
// } 

// fn resoudre_canal_supprimer( contexte: &mut Contexte, arguments: ArgumentsLocaux ) -> Retour { 
// 	let nom = arguments.trim(); 
// 	if nom == "" { 
// 		Retour::creer_str( false, "nom de canal obligatoire" ) 
// 	} else if nom.len() > 32 { 
// 		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
// 	} else if nom == DICO_NOM_DEFAUT { 
// 		Retour::creer_str( false, "impossible de supprimer le canal par défaut" ) 
// 	} else { 
// 		let mut dicos = contexte.dicos.lock().unwrap(); 
// 		if dicos.liste.contains_key( nom ) { 
// 			{ 
// 				let message = "canal supprimé".to_string(); 
// 				let mut dico = dicos.liste[nom].lock().unwrap(); 
// 				dico.souscripteurs.retain( 
// 					| souscripteur | { 
// 						souscripteur.send( message.clone() ).unwrap(); 
// 						false 
// 					} 
// 				); 
// 			} 
// 			if let Some(_) = dicos.liste.remove( nom ) { 
// 				Retour::creer_str( true, "canal supprimé" ) 
// 			} else { 
// 				Retour::creer_str( false, "impossible de supprimer le canal" ) 
// 			} 
// 		} else { 
// 			Retour::creer_str( false, "canal inexistant" ) 
// 		} 
// 	} 
// } 

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

fn resoudre_canal_capture( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if let Some( _ ) = arguments.extraire() { 
		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
	} 
	contexte.dico = Arc::new( Mutex::new( Dictionnaire { 
		nom: "".to_string(), 
		liste: HashMap::new(), 
		souscripteurs: Vec::new() 
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

fn resoudre( contexte: &mut Contexte, appel: &str, arguments: &str ) -> Retour { 
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
			
			// actions sur les canaux 
			// "canal:créer" => resoudre_canal_creer, 
			// "canal:capturer" => resoudre_canal_capture, 
			// "canal:supprimer" => resoudre_canal_supprimer, 
			"canal:tester" => resoudre_canal_tester, 
			"canal:lister" if DEBUG => resoudre_canal_lister, 
			"canal:changer" => resoudre_canal_changer, 
			"canal:souscrire" => resoudre_canal_souscrire, 
			"canal:émettre" => resoudre_canal_emettre, 
			
			// "chercher" => resoudre_chercher, -> https://doc.rust-lang.org/std/string/struct.String.html#method.contains  
			_ => return Retour::creer_str( false, "fonction inconnue" ) 
		})( 
			contexte, 
			ArgumentsLocaux { 
		        source: arguments.chars().collect::<Vec<char>>(), 
		        position: 0 
		    } 
		) 
} 

// ---------------------------------------------------- 

struct Contexte { 
	poursuivre: bool, 
	dico: DictionnaireThread, 
	dicos: Arc<Mutex<Dictionnaires>>, 
	resoudre: fn(&mut Contexte, &str, &str) -> Retour, 
	stream: TcpStream 
} 

enum ExtractionLigne { 
	Commande(String), 
	Erreur(Retour), 
	Stop 
} 

fn extraire_ligne( iterateur: &mut Bytes<TcpStream> ) -> ExtractionLigne { 
	let mut a: [u8; TAILLE_LIGNE_MAX] = [0; TAILLE_LIGNE_MAX]; 
	let mut position: usize = 0; 
	loop { 
		match iterateur.next() { 
			Some( Ok( 13u8 ) ) if position < TAILLE_LIGNE_MAX => { 
				if let Ok( s ) = String::from_utf8( a[..position].to_vec() ) { 
					return ExtractionLigne::Commande( s ); 
				} else { 
					return ExtractionLigne::Erreur( 
						Retour::creer_str( false, "chaîne invalide" ) 
					); 
				} 
			} 
			Some( Ok( n ) ) if position < TAILLE_LIGNE_MAX => a[position] = n, 
			Some( Ok( _ ) ) if position >= TAILLE_LIGNE_MAX => { 
				loop { 
					match iterateur.next() { 
						Some( Ok( 13u8 ) ) => break, 
						_ => () 
					} 
				} 
				return ExtractionLigne::Erreur( 
					Retour::creer_str( false, "ligne trop longue" ) 
				); 
			} 
			_ => break 
		} 
		position += 1; 
	} 
	if position == 0 { 
		return ExtractionLigne::Stop; 
	} 
	if let Ok( s ) = String::from_utf8( a[..position].to_vec() ) { 
		return ExtractionLigne::Commande( s ); 
	} else { 
		return ExtractionLigne::Erreur( 
			Retour::creer_str( false, "caractère(s) invalide(s)" ) 
		); 
	} 
} 

fn extraction_commande( commande: &str ) -> (&str, &str) { 
	if let Some( position ) = commande.find( ' ' ) { 
		( &commande[0..position], &commande[position+1..] ) 
	} else { 
		( &commande, "" ) 
	} 
} 

fn handle_client( mut contexte: Contexte ) { 
	let fct_resolution = contexte.resoudre; 
	let mut iterateur = match contexte.stream.try_clone() { 
		Ok( s ) => s, 
		Err(_) => return 
	}.bytes(); 
	while contexte.poursuivre { 
		let r = match extraire_ligne( &mut iterateur ) { 
			ExtractionLigne::Commande( s ) => { 
				let appel = extraction_commande( s.trim() ); 
				fct_resolution( 
					&mut contexte, 
					appel.0, 
					appel.1 
				) 
			} 
			ExtractionLigne::Erreur( m ) => m, 
			ExtractionLigne::Stop => break 
		}; 
		if let Ok(_) = contexte.stream.write( 
			if r.etat { "[+] ".as_bytes() } else { "[-] ".as_bytes() } 
		) { 
			if let Ok(_) = contexte.stream.write( r.message.vers_bytes() ) { 
				if let Err(_) = contexte.stream.flush() { 
					break; 
				} else { 
					if let Err(_) = contexte.stream.write( "\n".as_bytes() ) { 
						break; 
					} 
				} 
			} else { 
				break; 
			} 
		} else { 
			break; 
		} 
	} 
	if DEBUG { 
		match contexte.stream.peer_addr() { 
			Ok( adresse ) => println!( "! fin de connexion: {:?}", adresse ), 
			_ => () 
		} 
	} 
} 

// ---------------------------------------------------- 

#[derive(Debug)] 
struct Dictionnaire { 
	nom: String, 
	liste: HashMap<String,Valeurs>, 
	souscripteurs: Vec<Sender<String>>  
} 

impl Drop for Dictionnaire { 
    fn drop(&mut self) { 
    	if DEBUG { 
        	println!( "! suppression 'Dictionnaire' : {:?}", self ); 
    	}
    } 
} 

type DictionnaireThread = Arc<Mutex<Dictionnaire>>; 

struct Dictionnaires { 
	liste: HashMap<String,DictionnaireThread> 
} 

fn creer_racine( nom_defaut: &str ) -> (DictionnaireThread, Arc<Mutex<Dictionnaires>>) { 
	let mut tmp = Dictionnaires { 
		liste: HashMap::new() 
	}; 
	let nom = nom_defaut.to_string(); 
	let dico_thread = Arc::new( 
		Mutex::new( 
			Dictionnaire { 
				nom: nom.clone(), 
				liste: HashMap::new(), 
				souscripteurs: Vec::new() 
			} 
		) 
	) as DictionnaireThread; 
	tmp.liste.insert( 
		nom, 
		dico_thread.clone() 
	); 
	let dicos = Arc::new( 
		Mutex::new( 
			tmp 
		) 
	); 
	( 
		dico_thread, 
		dicos 
	) 
} 

// fn lancement_service( ipport: &str ) -> Result<JoinHandle> { 

// } 

// ---------------------------------------------------- 

fn main() -> std::io::Result<()> { 

	let (dico_thread, dicos) = creer_racine( DICO_NOM_DEFAUT ); 

    let listener = TcpListener::bind("127.0.0.1:8080")?; 
    let mut fils: Vec<JoinHandle<_>> = Vec::new(); 
    for resultat in listener.incoming() { 
    	match resultat { 
    		Ok( stream ) => { 
    			if DEBUG { 
	    			match &stream.peer_addr() { 
	    				Ok( adresse ) => println!( "! nouvelle connexion: {:?}", adresse ), 
	    				_ => continue 
	    			} 
    			} 
    			let contexte = Contexte { 
    				poursuivre: true, 
    				dico: dico_thread.clone(), 
    				dicos: dicos.clone(), 
    				resoudre: resoudre, 
    				stream: stream, 
    			}; 
    			fils.push( 
    				thread::spawn( 
			        	move || { 
			        		handle_client( contexte ); 
			        	} 
			        ) 
			    ); 
			} 
	        _ => () 
    	} 
    } 
    for enfant in fils { 
    	enfant.join().unwrap(); 
    } 
    Ok(()) 
} 


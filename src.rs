
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
	Entier(u32), 
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
				Valeurs::Entier( n ) => {
					*self = Valeurs::Boolean( if *n > 0u32 { true } else { false } ); 
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
			"entier" => match self { 
				Valeurs::Entier( _ ) => true, 
				Valeurs::Boolean( b ) => { 
					*self = Valeurs::Entier( if *b { 1 } else { 0 } ); 
					true 
				} 
				Valeurs::Flottant( n ) => { 
					*self = Valeurs::Entier( n.round() as u32 ); 
					true 
				} 
				Valeurs::Texte( t ) => { 
					if let Ok( n ) = t.parse::<u32>() { 
						*self = Valeurs::Entier( n ); 
						true 
					} else { 
						false 
					} 
				} 
			} 
			"flottant" => match self { 
				Valeurs::Flottant( _ ) => true, 
				Valeurs::Entier( n ) => { 
					*self = Valeurs::Flottant( f32::from_bits( *n ) ); 
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
				Valeurs::Entier( n ) => { 
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

fn resoudre_stop( contexte: &mut Contexte, _arguments: &str ) -> Retour { 
	contexte.poursuivre = false; 
	Retour::creer_str( true, "au revoir" ) 
} 

fn resoudre_vider( contexte: &mut Contexte, _arguments: &str ) -> Retour { 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	valeurs.clear(); 
	Retour::creer_str( true, "base vidée" ) 
} 

fn resoudre_definir( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	if let Some( position ) = arguments.find( ' ' ) { 
		let mut dico = contexte.dico.lock().unwrap(); 
		let valeurs = &mut dico.liste; 
		let cle = arguments[0..position].trim(); 
		if cle != "" { 
			valeurs.insert( 
				cle.to_string(), 
				Valeurs::Texte( arguments[position..].trim().to_string() ) 
			); 
			Retour::creer_str( true, "ajouté" ) 
		} else { 
			Retour::creer_str( false, "une clé vide n'est pas une clé acceptable" ) 
		} 
	} else { 
		Retour::creer_str( false, "séparateur clé/valeur non-respecté (espace simple)" ) 
	} 
} 

fn resoudre_obtenir( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	let dico = contexte.dico.lock().unwrap(); 
	let valeurs = &dico.liste; 
	if valeurs.contains_key( arguments.trim() ) { 
		match &valeurs[arguments] { 
			Valeurs::Boolean( b ) => Retour::creer( true, format!( "(booléen) {}", b ) ), 
			Valeurs::Texte( t ) => Retour::creer( true, format!( "(texte) \"{}\"", t ) ), 
			Valeurs::Entier( n ) => Retour::creer( true, format!( "(entier) {}", n ) ), 
			Valeurs::Flottant( n ) => Retour::creer( true, format!( "(flottant) {}", n ) ), 
		} 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

fn resoudre_supprimer( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	if let Some( _ ) = valeurs.remove( arguments.trim() ) { 
		Retour::creer_str( true, "clé supprimée" ) 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

fn resoudre_ajouter( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	if let Some( position ) = arguments.find( ' ' ) { 
		let mut dico = contexte.dico.lock().unwrap(); 
		let valeurs = &mut dico.liste; 
		let cle = arguments[0..position].trim(); 
		if let Some( v ) = valeurs.get_mut( cle ) { 
			if v.ajouter_texte( &arguments[position+1..] ) { 
				Retour::creer_str( true, "valeur modifée" ) 
			} else { 
				Retour::creer_str( false, "ce format n'est pas supporté ou le texte est trop long" ) 
			} 
		} else { 
			Retour::creer_str( false, "clé inconnue" ) 
		} 
	} else { 
		Retour::creer_str( false, "séparateur clé/valeur non-respecté (espace simple)" ) 
	} 
} 

fn resoudre_lister( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	if arguments != "" { 
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

fn resoudre_tester( contexte: &mut Contexte, arguments: &str ) -> Retour {
	let dico = contexte.dico.lock().unwrap(); 
	let valeurs = &dico.liste; 
	if valeurs.contains_key( arguments.trim() ) { 
		Retour::creer_str( true, "clé existante" ) 
	} else { 
		Retour::creer_str( true, "clé inexistante" ) 
	} 
} 

fn resoudre_alterer( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	if let Some( position ) = arguments.find( ' ' ) { 
		let mut dico = contexte.dico.lock().unwrap(); 
		let valeurs = &mut dico.liste; 
		let cle = arguments[0..position].trim(); 
		let r#type: &str = &arguments[position+1..]; 
		if cle != "" { 
			return match r#type { 
				"booléen" | "texte" | "entier" | "flottant" => { 
					if let Some( v ) = valeurs.get_mut( cle ) { 
						if v.alterer( r#type ) { 
							Retour::creer_str( true, "altération effectuée" ) 
						} else { 
							Retour::creer_str( false, "altération impossible" ) 
						} 
					} else { 
						Retour::creer_str( false, "clé inconnue" ) 
					} 
				} 
				_ => Retour::creer_str( false, "altération de type inconnu" ) 
			} 
		} else { 
			Retour::creer_str( false, "une clé vide n'est pas une clé acceptable" ) 
		} 
	} else { 
		Retour::creer_str( false, "séparateur clé/valeur non-respecté (espace simple)" ) 
	} 
} 

fn resoudre_incrementer( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	if let Some( v ) = valeurs.get_mut( arguments.trim() ) { 
		match v { 
			Valeurs::Entier( n ) => { 
				// if arguments.trim() == "" { 
					*n += 1; 
					Retour::creer_str( true, "incrémentation (+1) effectuée" ) 
				// } else if let Ok( m ) = arguments.trim().parse::<u32>() { 
				// 	*n += m; 
				// 	Retour::creer_str( true, "incrémentation arbitraire effectuée" ) 	
				// } else { 
				// 	Retour::creer_str( false, "incrémentation impossible, l'argument est invalide dans ce type" ) 
				// } 
			} 
			Valeurs::Flottant( n ) => { 
				// if arguments.trim() == "" { 
					*n += 1f32; 
					Retour::creer_str( true, "incrémentation (+1) effectuée" ) 
				// } else if let Ok( m ) = arguments.trim().parse::<f32>() { 
				// 	*n += m; 
				// 	Retour::creer_str( true, "incrémentation arbitraire effectuée" ) 	
				// } else { 
				// 	Retour::creer_str( false, "incrémentation impossible, l'argument est invalide dans ce type" ) 
				// } 
			} 
			_ => Retour::creer_str( false, "incrémentation impossible" ) 
		} 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

fn resoudre_decrementer( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	let mut dico = contexte.dico.lock().unwrap(); 
	let valeurs = &mut dico.liste; 
	if let Some( v ) = valeurs.get_mut( arguments.trim() ) { 
		match v { 
			Valeurs::Entier( n ) => { 
				*n -= 1; 
				Retour::creer_str( true, "incrémentation effectuée" ) 	
			} 
			_ => Retour::creer_str( false, "incrémentation impossible" ) 
		} 
	} else { 
		Retour::creer_str( false, "clé inconnue" ) 
	} 
} 

fn resoudre_resumer( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	if arguments != "" { 
		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
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

fn resoudre_canal_creer( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	let nom = arguments.trim(); 
	if nom == "" { 
		Retour::creer_str( false, "nom de canal obligatoire" ) 
	} else if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
	} else { 
		let mut dicos = contexte.dicos.lock().unwrap(); 
		if dicos.liste.contains_key( arguments ) { 
			Retour::creer_str( false, "canal existant" ) 
		} else { 
			dicos.liste.insert( 
				arguments.to_string(), 
				Arc::new( Mutex::new( Dictionnaire { 
					nom: nom.to_string(), 
					liste: HashMap::new(), 
					souscripteurs: Vec::new() 
				} ) ) as DictionnaireThread  
			); 
			Retour::creer_str( true, "canal créé" ) 
		} 
	} 
} 

fn resoudre_canal_supprimer( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	let nom = arguments.trim(); 
	if nom == "" { 
		Retour::creer_str( false, "nom de canal obligatoire" ) 
	} else if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
	} else if nom == DICO_NOM_DEFAUT { 
		Retour::creer_str( false, "impossible de supprimer le canal par défaut" ) 
	} else { 
		let mut dicos = contexte.dicos.lock().unwrap(); 
		if dicos.liste.contains_key( nom ) { 
			{ 
				let message = "canal supprimé".to_string(); 
				let mut dico = dicos.liste[nom].lock().unwrap(); 
				dico.souscripteurs.retain( 
					| souscripteur | { 
						souscripteur.send( message.clone() ).unwrap(); 
						false 
					} 
				); 
			} 
			if let Some(_) = dicos.liste.remove( nom ) { 
				Retour::creer_str( true, "canal supprimé" ) 
			} else { 
				Retour::creer_str( false, "impossible de supprimer le canal" ) 
			} 
		} else { 
			Retour::creer_str( false, "canal inexistant" ) 
		} 
	} 
} 

fn resoudre_canal_tester( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	let nom = arguments.trim(); 
	if nom == "" { 
		Retour::creer_str( false, "nom de canal obligatoire" ) 
	} else if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
	} else { 
		let dicos = contexte.dicos.lock().unwrap(); 
		if dicos.liste.contains_key( arguments ) { 
			Retour::creer_str( true, "canal existant" ) 
		} else { 
			Retour::creer_str( true, "canal inexistant" ) 
		} 
	} 
} 

fn resoudre_canal_lister( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	if arguments != "" { 
		return Retour::creer_str( false, "aucun argument autorisé pour cette fonction" ); 
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

fn resoudre_canal_changer( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	let nom = arguments.trim(); 
	if nom == "" { 
		Retour::creer_str( false, "nom de canal obligatoire" ) 
	} else if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max 32)" ) 
	} else { 
		let dicos = contexte.dicos.lock().unwrap(); 
		if dicos.liste.contains_key( arguments ) { 
			contexte.dico = dicos.liste[nom].clone(); 
			Retour::creer_str( true, "canal modifié" ) 
		} else { 
			Retour::creer_str( false, "canal inexistant" ) 
		} 
	} 
} 

fn resoudre_canal_capture( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	if arguments.trim() != "" { 
		Retour::creer_str( false, "aucun argument autorisé" ) 
	} else { 
		contexte.dico = Arc::new( Mutex::new( Dictionnaire { 
			nom: "".to_string(), 
			liste: HashMap::new(), 
			souscripteurs: Vec::new() 
		} ) ) as DictionnaireThread; 
		Retour::creer_str( true, "canal privé actif" ) 
	} 
} 

fn resoudre_canal_souscrire( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	if arguments.trim() != "" { 
		Retour::creer_str( false, "aucun argument autorisé" ) 
	} else { 
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
} 

fn resoudre_canal_emettre( contexte: &mut Contexte, arguments: &str ) -> Retour { 
	let mut dico = contexte.dico.lock().unwrap(); 
	let message = arguments.trim().to_string(); 
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
			"stop" => resoudre_stop, 
			"vider" => resoudre_vider, 
			"définir" => resoudre_definir, 
			"obtenir" => resoudre_obtenir, 
			"supprimer" => resoudre_supprimer, 
			"ajouter" => resoudre_ajouter, 
			"lister" => resoudre_lister, 
			"tester" => resoudre_tester, 
			"altérer" => resoudre_alterer, 
			"incrémenter" => resoudre_incrementer, 
			"décrémenter" => resoudre_decrementer, 
			"resumer" => resoudre_resumer, 
			"canal:créer" => resoudre_canal_creer, 
			"canal:capturer" => resoudre_canal_capture, 
			"canal:supprimer" => resoudre_canal_supprimer, 
			"canal:tester" => resoudre_canal_tester, 
			"canal:lister" if DEBUG => resoudre_canal_lister, 
			"canal:changer" => resoudre_canal_changer, 
			"canal:souscrire" => resoudre_canal_souscrire, 
			"canal:émettre" => resoudre_canal_emettre, 
			// "chercher" => resoudre_chercher, -> https://doc.rust-lang.org/std/string/struct.String.html#method.contains  
			_ => return Retour::creer_str( false, "fonction inconnue" ) 
		})( contexte, arguments ) 
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


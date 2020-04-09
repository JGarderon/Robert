
use std::net::{TcpListener}; 
use std::io::Read; 
use std::io::Write; 
use std::thread; 
use std::thread::JoinHandle; 

// ---------------------------------------------------- 

const DEBUG: bool = true; 

const DICO_NOM_DEFAUT: &'static str = "défaut";

const TAILLE_LIGNE_MAX: usize = 1024; 
const TAILLE_TEXTE_MAX: usize = TAILLE_LIGNE_MAX*5; 

// ---------------------------------------------------- 

mod resolution; 
use crate::resolution::{Contexte}; 

mod base; 

mod grammaire; 
use crate::grammaire::{ExtractionLigne}; 

// ---------------------------------------------------- 

fn handle_client( mut contexte: Contexte ) { 
	let fct_resolution = contexte.resoudre; 
	let mut iterateur = match contexte.stream.try_clone() { 
		Ok( s ) => s, 
		Err(_) => return 
	}.bytes(); 
	while contexte.poursuivre { 
		let r = match grammaire::extraire_ligne( &mut iterateur ) { 
			ExtractionLigne::Commande( s ) => { 
				let appel = grammaire::extraction_commande( s.trim() ); 
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

// fn lancement_service( ipport: &str ) -> Result<JoinHandle> { 

// } 

// ---------------------------------------------------- 

fn main() -> std::io::Result<()> { 

	let (dico_thread, dicos) = base::creer_racine( DICO_NOM_DEFAUT ); 

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
    				resoudre: resolution::resoudre, 
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


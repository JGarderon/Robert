
//! Robert est un logiciel type "Redis-Like" : un système de gestion de données haute performance, stockée en RAM, qui n'offre cependant comme son illustre grand frère, toutes les finesses d'une base de données SQL classique. 
//! 
//! Robert est donc à classer dans la famille des No-SQL. Les requêtes des utilisateurs ne sont pas à proprement parler un langage de programmation, mais un DSL - un langage spécifique. Avec cette spécificité : il est intégralement francophone (commentaires dans le code, la documentation, mais aussi les commandes elles-mêmes). 
//! 
//! Vous vous demandez d'où vient son nom ? Bonne question : comme il fonctionne sur un système "clé/valeur", stocké dans ce qu'il convient d'appeler des "dictionnaires", il semblait assez logique que ce petit logiciel sans prétention, qui se veut simple, facilement extensible et efficace s'appelle... le (petit) Robert. Comme un illustre ancêtre papier ! Et puis Redis et Rust commencent tous les deux par un 'R' alors... 
//! 
//! A ce jour, il gère des clés au format texte et des valeurs de plusieurs types (valeur textuelle, réelle, flottante, boolèenne). D'autres types de valeurs sont attendus et sans attendre (compte tenu du caractère ouvert et libre du code), n'hésitez pas à y ajouter votre grain de sel. 
//! 
//! La philosophie de Robert est d'offrir un logiciel appréhendable par le plus grand nombre, simple et rapide, qui ne soit pas un "jouet" de programmation sans être une usine à gaz que seuls une poignée de personnes est capable de développer et maintenir. Robert dans la lignée de la philosophie "KISS" de l'univers Unix : _Keep It Simple, Stupid !_ Ainsi il ne vous fera jamais le café... 
//! 
//! Par l'usage de Rust pour son développement, le logiciel est stable, sûr et son empreinte mémoire est très faible. Rust ne connaît (quasi-)pas les fuites de mémoire : Robert non plus (car il tente d'en suivre au plus près la philosophie). Le projet souhaite aussi s'assoir sur des ressources sûres, et éviter d'utiliser des adjonctions de code extérieur insondable. Aussi Robert n'a aucune autre dépendance à ce jour, que l'usage des modules internes au langage. 
//! 
//! __D'où sa devise : _copier, compiler, profiter !___
//! 

use std::net::{TcpListener}; 
use std::io::Read; 
use std::io::Write; 
use std::thread; 
use std::thread::JoinHandle; 

// --- --- --- --- --- --- --- --- --- 

/// Définit sur le mode "débug" est actif (renvoi sur la console par défaut). 
const DEBUG: bool = true; 

/// Nom du dictionnaire par défaut, créé par le programme et qui sert aussi de canal par défaut. Il ne peut et ne doit être jamais supprimé lors de l'exécution des requêtes des utilisateurs. 
const CANAL_NOM_DEFAUT: &'static str = "défaut";

/// Taille maximale admissible par ligne reçue sur un socket. Cette taille fournie donc la taille maximum admissible des requêtes pour le reste du programme. 
const TAILLE_LIGNE_MAX: usize = 1024; 

/// Taille maximale admissible pour le texte contenu dans les dictionnaires. 
const TAILLE_TEXTE_MAX: usize = TAILLE_LIGNE_MAX*5; 

// ///Nbre maximum admissible de valeurs pour chaque objet. 
// const NBRE_MAX_OBJETS: usize = 250; 

///Nbre maximum admissible de valeurs pour chaque canal (dictionnaire). 
const NBRE_MAX_VALEURS: usize = 500; 

///Nbre maximum admissible de canaux dans le processus en cours. 
const NBRE_MAX_CANAUX: usize = 8; 

// --- --- --- --- --- --- --- --- --- 

mod resolution; 
use crate::resolution::{Contexte}; 

mod base; 

mod grammaire; 
use crate::grammaire::{ExtractionLigne}; 

/// Fonction recevant un client et le traitant, par le biais d'un objet 'Contexte' déjà créé. Principalement une boucle qui reçoit sur texte dans un tampon, l'examine rapidement avec les outils du module "grammaire", et lancement la fonction de résolution de la requête. 
fn recevoir( mut contexte: Contexte ) { 
	let mut iterateur = match contexte.stream.try_clone() { 
		Ok( s ) => s, 
		Err(_) => return 
	}.bytes(); 
	while contexte.poursuivre { 
		let r = match grammaire::extraire_ligne( &mut iterateur ) { 
			ExtractionLigne::Commande( s ) => { 
				let appel = grammaire::extraction_commande( s.trim() ); 
				resolution::resoudre( 
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


/// Fonction permettant de lancer le service d'écoute (socket TCP). A l'avenir, cette fonction retournerait un objet JoinHandle permettant au service d'agir dans un thread dédié et ne pas boucler la fonction 'main'. 
/// Chaque nouveau client est envoyé dans un nouveau thread, avec un objet "Contexte", qui porte les informations essentielles liées au socket TCP en cours. Les requêtes sont gérées par le thread du client. 
fn lancement_service( ipport: &str ) -> Result<(), &'static str> { 
	let (canal_thread, canaux_thread) = base::creer_racine( CANAL_NOM_DEFAUT ); 
    if let Ok( listener ) = TcpListener::bind( ipport ) {
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
	    				canalthread: canal_thread.clone(), 
	    				canauxthread: canaux_thread.clone(), 
	    				stream: stream, 
	    			}; 
	    			fils.push( 
	    				thread::spawn( 
				        	move || { 
				        		recevoir( contexte ); 
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
	    Ok( () ) 
    } else { 
    	Err( "impossible d'ouvrir le port désiré sur l'interface voulue" ) 
    } 
} 

// --- --- --- --- --- --- --- --- --- 

/// Ai-je vraiment besoin de documenter à quoi sert cette fonction... ? 
fn main() { 

	if let Err( e ) = lancement_service( "127.0.0.1:8080" ) { 
		println!( "démarrage impossible : {:?}", e );
	} 

} 


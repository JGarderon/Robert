
//! Robert est un logiciel type "Redis-Like" : un système de gestion de données haute performance, stockée en RAM, qui n'offre cependant comme son illustre grand frère, toutes les finesses d'une base de données SQL classique. 
//! 
//! Robert est donc à classer dans la famille des No-SQL "naïfs". Les requêtes des utilisateurs ne sont pas à proprement parler un langage de programmation, mais un DSL - un langage spécifique (une API en réalité). Avec cette spécificité : il est intégralement francophone (commentaires dans le code, la documentation, mais aussi les commandes elles-mêmes). 
//! 
//! Vous vous demandez d'où vient son nom ? Bonne question : comme il fonctionne sur un système "clé/valeur", stocké dans ce qu'il convient d'appeler des "dictionnaires", il semblait assez logique que ce petit logiciel sans prétention, qui se veut simple, facilement extensible et efficace s'appelle... le (petit) Robert. Comme un illustre ancêtre papier ! Et puis Redis et Rust commencent tous les deux par un 'R' alors... 
//! 
//! A ce jour, il gère des clés au format texte et des valeurs de plusieurs types (valeur textuelle, réelle, flottante, boolèenne). D'autres types de valeurs sont attendus et sans attendre (compte tenu du caractère ouvert et libre du code), n'hésitez pas à y ajouter votre grain de sel. 
//! 
//! La philosophie de Robert est d'offrir un logiciel appréhendable par le plus grand nombre, simple et rapide, qui ne soit pas un "jouet" de programmation sans être une usine à gaz que seuls une poignée de personnes est capable de développer et maintenir. Robert dans la lignée de la philosophie "KISS" de l'univers Unix : _Keep It Simple, Stupid !_ Ainsi il ne vous fera jamais le café... 
//! 
//! Par l'usage de Rust pour son développement, le logiciel est stable, sûr et son empreinte mémoire est très faible. Rust ne connaît (quasi-)pas les fuites de mémoire : Robert non plus (car il tente d'en suivre au plus près la philosophie). Le projet souhaite aussi s'assoir sur des ressources sûres, et éviter d'utiliser des adjonctions de code extérieur insondable. Aussi Robert n'a aucune autre dépendance à ce jour, que l'usage des modules internes au langage. 
//! 
//! __D'où sa devise "CCP" : _copier, compiler, profiter !___
//! 

use std::net::{TcpListener}; 
use std::thread; 
use std::thread::JoinHandle; 

// --- --- --- --- --- --- --- --- --- 

mod configuration; 

mod contexte; 


#[macro_use] 
mod canal; 
#[macro_use] 
mod profil; 

mod resolution; 

mod grammaire; 

mod serie; 

mod client; 

mod valeur; 

// --- --- --- --- --- --- --- --- --- 

use crate::canal::creer_racine; 
use crate::profil::Profil; 
use crate::configuration::CANAL_NOM_DEFAUT; 
use crate::contexte::Contexte; 

/// Fonction permettant de lancer le service d'écoute (socket TCP). A l'avenir, cette fonction retournerait un objet JoinHandle permettant au service d'agir dans un thread dédié et ne pas boucler la fonction 'main'. 
/// Chaque nouveau client est envoyé dans un nouveau thread, avec un objet "Contexte", qui porte les informations essentielles liées au socket TCP en cours. Les requêtes sont gérées par le thread du client. 
fn lancement_service( ipport: &str ) -> Result<(), &'static str> { 
	static mut ETAT_GENERAL: bool = true; // /!\ UNSAFE / à retirer urgemment 
	let (canal_thread, canaux_thread) = creer_racine( CANAL_NOM_DEFAUT ); 
    if let Ok( listener ) = TcpListener::bind( ipport ) {
	    let mut fils: Vec<JoinHandle<_>> = Vec::new(); 
	    let mut iterateur_connexion = listener.incoming();  
	    while unsafe { ETAT_GENERAL } { // /!\ UNSAFE / à retirer urgemment 
	    	let stream = match iterateur_connexion.next() { 
	    		Some( Ok( s ) ) => s, 
	    		Some( Err( _ ) ) => continue, 
	    		None => { 
	    			println!("! l'écouteur a rencontré un problème ; le service va débuter son extinction"); 
	    			break; 
	    		} 
	    	}; 
	    	if configuration::DEBUG { 
    			match &stream.peer_addr() { 
    				Ok( adresse ) => println!( "! nouvelle connexion: {:?}", adresse ), 
    				_ => continue 
    			} 
			} 
			let contexte = Contexte { 
				service_ecoute: listener.try_clone().unwrap(), 
				service_poursuite: unsafe { &mut ETAT_GENERAL }, // /!\ UNSAFE / à retirer urgemment 
				poursuivre: true, 
				canalthread: canal_thread.clone(), 
				canauxthread: canaux_thread.clone(), 
				profil: Profil::creer(), 
				stream: stream, 
			}; 
			fils.push( 
				thread::spawn( 
		        	move || { 
		        		client::recevoir( contexte ); 
		        	} 
		        ) 
		    ); 
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

/// Fonction principal du programme 
/// 
/// Ai-je vraiment besoin de documenter à quoi sert cette fonction... ? 
/// 
fn main() { 

	if let Err( e ) = lancement_service( "127.0.0.1:8080" ) { 
		println!( "démarrage impossible : {:?}", e );
	} 

} 


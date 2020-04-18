//! # Module de lancement des services 
//! 

use std::net::{TcpListener}; 
use std::thread; 
use std::thread::JoinHandle; 

use crate::client; 
use crate::canal::creer_racine; 
use crate::profil::Profil; 
use crate::configuration::DEBUG; 
use crate::configuration::CANAL_NOM_DEFAUT; 
use crate::contexte::Contexte; 

/// Fonction permettant de lancer le service d'écoute (socket TCP). A l'avenir, cette fonction retournerait un objet JoinHandle permettant au service d'agir dans un thread dédié et ne pas boucler la fonction 'main'. 
/// Chaque nouveau client est envoyé dans un nouveau thread, avec un objet "Contexte", qui porte les informations essentielles liées au socket TCP en cours. Les requêtes sont gérées par le thread du client. 
pub fn lancement_service( ipport: &str ) -> Result<(), &'static str> { 
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
            if DEBUG { 
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


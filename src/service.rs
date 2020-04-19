//! # Module de lancement des services 
//! 
//! Ce module ne contient pour l'instant qu'une seule fonction, qui démarre "réellement" le programme. C'est le point d'entrée si Robert doit être ajouté dans un autre projet : vous pouvez intégrer cette fonction pour servir un port TCP. 
//! 

    // --- --- --- --- --- --- --- --- --- 
    // (1) Importation des modules internes 
    // --- --- --- --- --- --- --- --- --- 

use std::net::{TcpListener}; 
use std::thread::{self, JoinHandle}; 

    // --- --- --- --- --- --- --- --- --- 
    // (2) Importation des modules du projet 
    // --- --- --- --- --- --- --- --- --- 

use crate::client; 
use crate::canal::creer_racine; 
use crate::profil::Profil; 
use crate::configuration::DEBUG; 
use crate::configuration::CANAL_NOM_DEFAUT; 
use crate::contexte::Contexte; 

    // --- --- --- --- --- --- --- --- --- 
    // (3) Constantes du projet 
    // --- --- --- --- --- --- --- --- --- 

    // --- --- --- --- --- --- --- --- --- 
    // (4) Définition des structures, énumérations et leurs implémentations 
    // --- --- --- --- --- --- --- --- --- 

    // --- --- --- --- --- --- --- --- --- 
    // (5) Définition des fonctions 
    // --- --- --- --- --- --- --- --- --- 

/// # Fonction permettant de lancer le service d'écoute (socket TCP). 
/// Chaque nouveau client est envoyé dans un nouveau thread, avec un objet "Contexte", qui porte les informations essentielles liées au socket TCP en cours. Les requêtes sont gérées par le thread du client. 
/// 
/// A l'avenir, cette fonction devrait retourner un objet JoinHandle permettant au service d'agir dans un thread dédié et ne pas bloquer la fonction 'main'. Cependant tant qu'il n'y a pas d'autres besoins à couvrir, cette fonction reste en l'état. 
/// 
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


//! # Module des clients TCP
//!
//! Ce module gère la réception des clients TCP dans le service. Il dépend pour son fonctionnement, principalement de deux autres modules : celui grammatical ('grammaire') et de contextualisation ('contexte').
//!

// --- --- --- --- --- --- --- --- ---
// (1) Importation des modules internes
// --- --- --- --- --- --- --- --- ---

use std::io::Read;
use std::io::Write;

// --- --- --- --- --- --- --- --- ---
// (2) Importation des modules du projet
// --- --- --- --- --- --- --- --- ---

use crate::contexte::Contexte;
use crate::grammaire;
use crate::grammaire::ExtractionLigne;
use crate::resolution;
use crate::resolution::RetourType;

// --- --- --- --- --- --- --- --- ---
// (3) Constantes du projet
// --- --- --- --- --- --- --- --- ---

use crate::configuration::DEBUG;

// --- --- --- --- --- --- --- --- ---
// (4) Définition des structures, énumérations et leurs implémentations
// --- --- --- --- --- --- --- --- ---

pub trait Informer {
    fn ecrire(&mut self, texte: &str, flush: bool) -> bool;
    fn message(&mut self, message: &str) -> bool;
    fn erreur(&mut self, erreur: &str) -> bool;
}

impl Informer for std::net::TcpStream {
    fn ecrire(&mut self, texte: &str, flush: bool) -> bool {
        match self.write(texte.as_bytes()) {
            Ok(_) => {
                if flush {
                    match self.flush() {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                } else {
                    true
                }
            }
            Err(_) => false,
        }
    }

    fn message(&mut self, message: &str) -> bool {
        self.ecrire(&format!("[@] {}\n", message), false)
    }

    fn erreur(&mut self, erreur: &str) -> bool {
        self.ecrire(&format!("[!] {}\n", erreur), false)
    }
}

// --- --- --- --- --- --- --- --- ---
// (5) Définition des fonctions
// --- --- --- --- --- --- --- --- ---

pub fn nettoyer(contexte: &mut Contexte) {
    match contexte.existence.try_recv() {
        Ok(_) => (),
        Err(_) => (),
    };
}

/// Fonction recevant un client et le traitant, par le biais d'un objet 'Contexte' déjà créé. Principalement une boucle qui reçoit sur texte dans un tampon, l'examine rapidement avec les outils du module "grammaire", et lancement la fonction de résolution de la requête.
pub fn recevoir(mut contexte: Contexte) {
    let mut iterateur = match contexte.stream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    }
    .bytes();
    if !contexte.message("bonjour") {
        return;
    }
    while contexte.poursuivre {
        nettoyer(&mut contexte);
        if !*contexte.service_poursuite {
            contexte.ecrire(
				"[!] le service est en cours d'extinction ; vous allez être déconnecté immédiatement\n",
				true
			);
            break;
        }
        let r = match grammaire::extraire_ligne(&mut iterateur) {
            ExtractionLigne::Commande(s) => {
                let appel = grammaire::extraction_commande(s.trim());
                resolution::resoudre(&mut contexte, appel.0, appel.1)
            }
            ExtractionLigne::Erreur(m) => m,
            ExtractionLigne::Stop => break,
        };
        let mut e = contexte.ecrire(if r.etat { "[+] " } else { "[-] " }, false);
        e &= match r.message {
            RetourType::Statique(m) => contexte.ecrire(m, false),
            RetourType::Dynamique(m) => contexte.ecrire(&m, false),
        };
        e &= contexte.ecrire("\n", true);
        if !e {
            break;
        }
    }
    if DEBUG {
        match contexte.stream.peer_addr() {
            Ok(adresse) => println!("! fin de connexion: {:?}", adresse),
            _ => (),
        }
    }
}

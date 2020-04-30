//! # Module de lancement des services
//!
//! Ce module ne contient pour l'instant qu'une seule fonction, qui démarre "réellement" le programme. C'est le point d'entrée si Robert doit être ajouté dans un autre projet : vous pouvez intégrer cette fonction pour servir un port TCP.
//!

// --- --- --- --- --- --- --- --- ---
// (1) Importation des modules internes
// --- --- --- --- --- --- --- --- ---

use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

// --- --- --- --- --- --- --- --- ---
// (2) Importation des modules du projet
// --- --- --- --- --- --- --- --- ---

use crate::canal::creer_racine;
use crate::client;
use crate::configuration::CANAL_NOM_DEFAUT;
use crate::configuration::DEBUG;
use crate::contexte::Contexte;
use crate::profil::Profil;

// --- --- --- --- --- --- --- --- ---
// (3) Constantes du projet
// --- --- --- --- --- --- --- --- ---

// --- --- --- --- --- --- --- --- ---
// (4) Définition des structures, énumérations et leurs implémentations
// --- --- --- --- --- --- --- --- ---

/// Cette structure privée supporte l'ensemble des threads qui gèrent les clients connectés. A chaque passage de la boucle principale du service, un booléen est tenté d'être envoyé.
///
/// Si l'émission aboutit, c'est que le destinaire existe : le thread est donc considéré actif. Sinon, il est retiré de la liste des enfants.
///
/// A ce jour, il n'y a pas un moyen plus sûr en Rust pour ce type d'opération.
struct Enfants {
    /// Non-public, la liste de l'ensemble des threads pour les clients TCP
    liste: Vec<Enfant>,
}

/// Cette implémentation n'est pas publique et est réservée aux seuls services gérés par ce module.
impl Enfants {
    /// Créée une nouvelle liste d'enfants
    fn creer() -> Self {
        Enfants { liste: Vec::new() }
    }
    /// Assemble un test et sa cible à la liste
    fn ajouter(&mut self, test: Sender<bool>, cible: JoinHandle<()>) {
        self.liste.push(Enfant {
            test: test,
            cible: cible,
        });
    }
    /// Tente d'envoyer un booléen à la cible ; en cas de défaut l'enfant est retiré
    fn nettoyer_enfants(&mut self) {
        self.liste.retain(|enfant| enfant.tester());
    }
    /// Lorsque le service s'arrête, les enfants doivent être correctement arrêtés
    fn finaliser(self) {
        for enfant in self.liste {
            enfant.cible.join().unwrap();
        }
    }
}

/// Cette structure privée porte le test (un expéditeur de booléen sur un channel) et sa cible (l'objet porteur d'un thread)
struct Enfant {
    /// l'expéditeur, permettant de tester l'activité d'un thread
    test: Sender<bool>,

    /// l'objet porteur du thread
    cible: JoinHandle<()>,
}

/// Cette implémentation n'est pas publique et est réservée aux seuls services gérés par ce module.
impl Enfant {
    /// Teste si la cible est accessible par l'envoi d'un booléen
    fn tester(&self) -> bool {
        match self.test.send(true) {
            Ok(_) => true,
            Err(_) => {
                if DEBUG {
                    println!("! un enfant (thread) n'est pas accessible");
                }
                false
            }
        }
    }
}

// --- --- --- --- --- --- --- --- ---
// (5) Définition des fonctions
// --- --- --- --- --- --- --- --- ---

/// # Fonction permettant de lancer le service d'écoute (socket TCP).
/// Chaque nouveau client est envoyé dans un nouveau thread, avec un objet "Contexte", qui porte les informations essentielles liées au socket TCP en cours. Les requêtes sont gérées par le thread du client.
///
/// A l'avenir, cette fonction devrait retourner un objet JoinHandle permettant au service d'agir dans un thread dédié et ne pas bloquer la fonction 'main'. Cependant tant qu'il n'y a pas d'autres besoins à couvrir, cette fonction reste en l'état.
///
pub fn lancement_service<'max>(ipport: &str) -> Result<(), &'static str> {
    let etat_general: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let (canal_thread, canaux_thread) = creer_racine(CANAL_NOM_DEFAUT);
    if let Ok(listener) = TcpListener::bind(ipport) {
        let mut enfants = Enfants::creer();
        let mut iterateur_connexion = listener.incoming();

        while etat_general.load(Ordering::Relaxed) {
            enfants.nettoyer_enfants();

            let stream = match iterateur_connexion.next() {
                Some(Ok(s)) => s,
                Some(Err(_)) => continue,
                None => {
                    println!("! l'écouteur a rencontré un problème ; le service va débuter son extinction");
                    break;
                }
            };
            if DEBUG {
                match &stream.peer_addr() {
                    Ok(adresse) => println!("! nouvelle connexion: {:?}", adresse),
                    _ => continue,
                }
            }
            let (test_etat_expediteur, test_etat_destinataire) = channel();
            let contexte = Contexte {
                existence: test_etat_destinataire,
                service_ecoute: listener.try_clone().unwrap(),
                service_poursuite: etat_general.clone(),
                poursuivre: true,
                canalthread: canal_thread.clone(),
                canauxthread: canaux_thread.clone(),
                profil: Profil::creer(),
                stream: stream,
            };
            let cible_enfant = thread::spawn(move || {
                client::recevoir(contexte);
            });
            enfants.ajouter(test_etat_expediteur, cible_enfant);
        }
        enfants.finaliser();
        Ok(())
    } else {
        Err("impossible d'ouvrir le port désiré sur l'interface voulue")
    }
}

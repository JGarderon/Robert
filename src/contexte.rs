//! # Module de contexte client
//!

use std::net::TcpStream;
use std::sync::mpsc::Receiver; 

// ----------------------------------------------------

use crate::canal::{CanalThread, CanauxThread};
use crate::client::Informer;
use crate::profil::Profil;

// ----------------------------------------------------

/// La structure 'Contexte' permet de rassembler dans un objet unique, l'ensemble des éléments propres à un socket quelque soit la fonction de résolution qui sera appelée. Elle référence aussi le canal en cours d'usage par le client, ainsi que l'origine (Canaux).
/// Dans une fonction de résolution, elle se présentera toujours dans la forme d'une référence mutable.
pub struct Contexte<'a> {

    /// Ce champ permet de tester l'activité d'un enfant (thread), Rust n'offrant pas de solution définitive pour l'état d'un thread 
    pub existence: Receiver<bool>, 

    /// Ce champ permet de récupérer un clone de l'objet en écoute sur l'interface réseau.
    pub service_ecoute: std::net::TcpListener,

    /// Ce champ lorsqu'il est à "faux", permet d'interrompre la boucle globale du service.
    pub service_poursuite: &'a mut bool,

    /// Ce champ lorsqu'il est à "faux", permet d'interrompre la boucle locale du thead gérant le socket, dès la fin de la fonction de résolution actuelle.
    pub poursuivre: bool,

    /// Ce champ contient le nécessaire pour accéder au dictionnaire représentant le canal actuel.
    /// Il est d'un type Arc<Mutex<Canal>> : un CanalThread est un Canal avec sa protection d'usage pour les threads.
    pub canalthread: CanalThread,

    /// Ce champ contient le nécessaire pour accéder au dictionnaires des canaux.
    /// Il est d'un type Arc<Mutex<Canaux>> : un CanauxThread est l'origine de tous les canaux, avec sa protection d'usage pour les threads.
    pub canauxthread: CanauxThread,

    /// Ce champ contient la structure 'Profil', contenant le nécessaire à l'authenfication et aux droits du client.
    pub profil: Profil<'a>,

    /// Ce champ contient l'objet socket, librement clonable.
    pub stream: TcpStream,
}

impl Informer for Contexte<'_> {
    fn ecrire( &mut self, texte: &str, flush: bool ) -> bool {
        self.stream.ecrire( texte, flush )
    }

    fn message( &mut self, message: &str ) -> bool {
        self.stream.message( message )
    }

    fn erreur( &mut self, erreur: &str ) -> bool {
        self.stream.erreur( erreur )
    }
}

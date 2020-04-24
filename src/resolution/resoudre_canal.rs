//! # Sous-module de résolution "canal"
//!
//! Ce module gère les fonctions liées à la gestion des canaux du processus. Certaines de ces fonctions peuvent être restreintes aux seuls clients authentifiés.
//!

// --- --- --- --- --- --- --- --- ---
// (1) Importation des modules internes
// --- --- --- --- --- --- --- --- ---

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;

// --- --- --- --- --- --- --- --- ---
// (2) Importation des modules du projet
// --- --- --- --- --- --- --- --- ---

use crate::canal::{Canal, CanalThread, Souscripteur};
use crate::client::Informer;
use crate::grammaire::ArgumentsLocaux;
use crate::resolution::{Contexte, Resolveur, Retour};
use crate::valeur::Valeurs;

// --- --- --- --- --- --- --- --- ---
// (3) Constantes du projet
// --- --- --- --- --- --- --- --- ---

use crate::configuration::NBRE_MAX_CANAUX;

// --- --- --- --- --- --- --- --- ---
// (4) Définition des structures, énumérations et leurs implémentations
// --- --- --- --- --- --- --- --- ---

// --- --- --- --- --- --- --- --- ---
// (5) Définition des fonctions
// --- --- --- --- --- --- --- --- ---

/// # Fonction de résolution locale "créer un nouveau canal"
///
fn resoudre_creer(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    est_authentifie!(contexte);
    let nom = if let Some(n) = arguments.extraire() {
        n
    } else {
        return Retour::creer_str(false, "nom de canal obligatoire");
    };
    if nom.len() > 32 {
        Retour::creer_str(false, "nom de canal trop long (max. 32)")
    } else {
        let mut canaux = match contexte.canauxthread.lock() {
            Ok(c) => c,
            Err(e) => e.into_inner(),
        };
        if canaux.liste.len() < NBRE_MAX_CANAUX {
            if canaux.liste.contains_key(&nom) {
                Retour::creer_str(false, "canal existant")
            } else {
                canaux.liste.insert(
                    nom.to_string(),
                    Arc::new(Mutex::new(Canal {
                        nom: nom,
                        liste: Valeurs::Objet(HashMap::new()),
                        souscripteurs: Vec::<Souscripteur>::new(),
                    })) as CanalThread,
                );
                Retour::creer_str(true, "canal créé")
            }
        } else {
            Retour::creer_str(false, "nbre max. de canaux atteint")
        }
    }
}

/// # Fonction de résolution locale "supprimer un canal existant"
///
/// Cette fonction tentera de supprimer un canal existant mais auparavant, elle avertira tous les éventuels souscripteurs présents de cette action, et les retira de la liste des souscripteurs.
///
fn resoudre_supprimer(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    est_authentifie!(contexte);
    let nom = if let Some(n) = arguments.extraire() {
        n
    } else {
        return Retour::creer_str(false, "nom de canal obligatoire");
    };
    if nom.len() > 32 {
        Retour::creer_str(false, "nom de canal trop long (max. 32)")
    } else {
        let mut canaux = match contexte.canauxthread.lock() {
            Ok(c) => c,
            Err(e) => e.into_inner(),
        };
        if canaux.liste.contains_key(&nom) {
            {
                let message = "canal supprimé".to_string();
                match canaux.liste[&nom].lock() {
                    Ok(c) => c,
                    Err(e) => e.into_inner(),
                }
                .souscripteurs
                .retain(|souscripteur| {
                    souscripteur.pont.send(message.clone()).unwrap();
                    false
                });
            }
            if let Some(_) = canaux.liste.remove(&nom) {
                Retour::creer_str(true, "canal supprimé")
            } else {
                Retour::creer_str(false, "impossible de supprimer le canal")
            }
        } else {
            Retour::creer_str(false, "canal inexistant")
        }
    }
}

/// # Fonction de résolution locale "tester l'existence d'un canal"
///
/// Ai-je vraiment besoin de décrire en détail son intérêt ? :)
///
fn resoudre_tester(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    est_authentifie!(contexte);
    let nom = if let Some(n) = arguments.extraire() {
        n
    } else {
        return Retour::creer_str(false, "nom de canal obligatoire");
    };
    if nom.len() > 32 {
        Retour::creer_str(false, "nom de canal trop long (max. 32)")
    } else {
        if (match contexte.canauxthread.lock() {
            Ok(c) => c,
            Err(e) => e.into_inner(),
        })
        .liste
        .contains_key(&nom)
        {
            Retour::creer_str(true, "canal existant")
        } else {
            Retour::creer_str(true, "canal inexistant")
        }
    }
}

/// # Fonction de résolution locale "capturer un nouveau canal"
///
/// Cette fonction est un peu particulière, car elle crééra un canal spéficique à un thread 'client'.
///
/// Le client disposera donc de son propre stockage de valeurs, ce qui est son intérêt. La perte de connexion provoque aussi la perte des informations contenues dans ce canal "artificiel". Il n'est référencé nul part ailleurs : aucune fonction classique sur les canaux n'aura de prise sur lui (y compris, sauf cas particuliers, le module d'administration).
///
/// Aucun souscripteur ne pourra donc s'y abonner (à part le client lui-même... mais ce dernier étant représenté par un socket, il ne peut pas émettre et recevoir en même temps).
///
/// Le nom d'un canal "capturé" (privé), doit toujours être nul (c'est un signe distinctif, un nom de canal ne doit jamais être nul).
///
/// En l'état, il peut donc avoir un écart sur la mesure de la taille de la mémoire si la mesure passe seulement par la structure Canaux et qu'acune recherche des canaux "capturés" n'est réalisée.
///
fn resoudre_capturer(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    if let Some(_) = arguments.extraire() {
        return Retour::creer_str(false, "aucun argument accepté pour cette fonction");
    }
    contexte.canalthread = Arc::new(Mutex::new(Canal {
        nom: "".to_string(),
        liste: Valeurs::Objet(HashMap::new()),
        souscripteurs: Vec::<Souscripteur>::new(),
    })) as CanalThread;
    Retour::creer_str(true, "canal privé actif")
}

/// # Fonction de résolution locale "renommer un canal existant"
///
/// Cette fonction repose en grande partie sur sa soeur de suppression d'un canal existant. La raison repose sur le fonctionnement interne de Rust : "_It is a logic error for a key to be modified in such a way that the key's hash, as determined by the Hash trait, or its equality, as determined by the Eq trait, changes while it is in the map. This is normally only possible through Cell, RefCell, global state, I/O, or unsafe code._" (voir : https://doc.rust-lang.org/std/collections/struct.HashMap.html)
///
/// Robert évitant à tout prix le code non-sûr et vu la complexité introduite avec les souscripteurs, il est plus aisé qu'un renommage produise l'effet d'une suppression et la réintroduction du canal avec son nouveau nom.
///
fn resoudre_renommer(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    est_authentifie!(contexte);
    let ancien_nom = if let Some(n) = arguments.extraire() {
        n
    } else {
        return Retour::creer_str(false, "ancien nom de canal obligatoire");
    };
    let nouveau_nom = if let Some(n) = arguments.extraire() {
        n
    } else {
        return Retour::creer_str(false, "nouveau nom de canal obligatoire");
    };
    if ancien_nom.len() > 32 || ancien_nom.len() > 32 {
        Retour::creer_str(false, "l'un des deux noms est trop long (max. 32)")
    } else {
        let mut canaux = match contexte.canauxthread.lock() {
            Ok(c) => c,
            Err(e) => e.into_inner(),
        };
        if canaux.liste.contains_key(&nouveau_nom) {
            return Retour::creer_str(false, "nouveau nom de canal déjà existant");
        }
        if canaux.liste.contains_key(&ancien_nom) {
            {
                let message = "canal modifié".to_string();
                match canaux.liste[&ancien_nom].lock() {
                    Ok(c) => c,
                    Err(e) => e.into_inner(),
                }
                .souscripteurs
                .retain(|souscripteur| {
                    souscripteur.pont.send(message.clone()).unwrap();
                    false
                });
            }
            if let Some(canal) = canaux.liste.remove(&ancien_nom) {
                match canal.lock() {
                    Ok(c) => c,
                    Err(e) => e.into_inner(),
                }
                .nom = nouveau_nom.to_string();
                canaux.liste.insert(nouveau_nom, canal);
                Retour::creer_str(true, "canal supprimé")
            } else {
                Retour::creer_str(false, "impossible de supprimer le canal")
            }
        } else {
            Retour::creer_str(false, "canal inexistant")
        }
    }
}

fn resoudre_souscrire(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    let args = if let Ok(a) = arguments.tous() {
        if a.len() < 2 {
            return Retour::creer_str(false, "deux arguments obligatoires");
        }
        a
    } else {
        return Retour::creer_str(false, "les arguments fournis sont incorrects");
    };
    let (expediteur, destinaire): (Sender<String>, Receiver<String>) = mpsc::channel();
    {
        match contexte.canalthread.lock() {
            Ok(c) => c,
            Err(e) => e.into_inner(),
        }
        .souscripteurs
        .push(Souscripteur {
            pont: expediteur,
            messages: match &args[0][..] {
                "vrai" | "1" => true,
                "false" | "0" => false,
                _ => return Retour::creer_str(false, "arguments 'messages' invalide"),
            },
            valeurs: match &args[0][..] {
                "vrai" | "1" => true,
                "false" | "0" => false,
                _ => return Retour::creer_str(false, "arguments 'valeurs' invalide"),
            },
        });
    }
    while let Ok(m) = destinaire.recv() {
        if !contexte.message(&m) {
            break;
        }
    }
    Retour::creer_str(true, "diffusion terminée")
}

fn resoudre_emettre(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    est_authentifie!(contexte);
    if let Ok(messages) = arguments.tous() {
        if messages.len() == 0 {
            Retour::creer_str(false, "vous devez indiquer au moins un message en argument")
        } else {
            let mut canal = acces_canal!(contexte);
            for message in messages {
                canal.notifier(&contexte.profil, message)
            }
            Retour::creer(
                true,
                format!(
                    "diffusion émise aux souscripteurs ({})",
                    canal.souscripteurs.len()
                ),
            )
        }
    } else {
        Retour::creer_str(false, "les arguments fournis sont invalides")
    }
}

pub fn resoudre(appel: &str) -> Result<Resolveur, Retour> {
    match appel {
        "créer" => Ok(resoudre_creer as Resolveur),
        "supprimer" => Ok(resoudre_supprimer as Resolveur),
        "tester" => Ok(resoudre_tester as Resolveur),
        "capturer" => Ok(resoudre_capturer as Resolveur),
        "renommer" => Ok(resoudre_renommer as Resolveur),
        "souscrire" => Ok(resoudre_souscrire as Resolveur),
        "émettre" => Ok(resoudre_emettre as Resolveur),
        _ => Err(Retour::creer_str(
            false,
            "module 'canal' : fonction inconnue",
        )),
    }
}

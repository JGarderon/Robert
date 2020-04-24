use crate::grammaire;
use crate::grammaire::ArgumentsLocaux;
use crate::resolution::Contexte;
use crate::valeur::Valeurs;

// ----------------------------------------------------

use crate::resolution::Resolveur;
use crate::resolution::Retour;

// ----------------------------------------------------

use crate::configuration::TAILLE_TEXTE_MAX;

// ----------------------------------------------------

/// # Fonction de résolution locale "ajouter du texte"
///
/// Cette fonction ajoutera le texte fourni en argument, en respectant la taille maximale autorisée. Le texte doit être valide UTF-8.
///
/// La fonction notifie les souscripteurs qui demandent les opérations sur les valeurs du canal.
///
fn resoudre_ajouter(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    let arg_chemin = if let Some(c) = arguments.extraire() {
        c
    } else {
        return Retour::creer_str(false, "un chemin vide n'est pas acceptable");
    };
    let ajout = if let Some(c) = arguments.extraire() {
        c
    } else {
        return Retour::creer_str(false, "aucun texte supplémentaire fourni");
    };
    let mut canal = acces_canal!(contexte);
    let r = match grammaire::chemin_extraire(&arg_chemin) {
        Ok(chemin) => canal.resoudre(&chemin, |valeur| match valeur {
            Valeurs::Texte(t) => {
                if t.len() + ajout.len() < TAILLE_TEXTE_MAX {
                    t.push_str(&ajout);
                    Retour::creer_str(true, "texte ajouté")
                } else {
                    Retour::creer_str(true, "texte final trop long")
                }
            }
            _ => Retour::creer_str(false, "le type de valeur cible n'est pas conforme"),
        }),
        Err(_) => Retour::creer_str(false, "ce chemin n'existe pas"),
    };
    if r.etat {
        canal.notifier(
            &contexte.profil,
            format!("(module 'texte') ajout : {}", arg_chemin),
        );
    }
    r
}

/// # Fonction de résolution locale "compter le texte (octets + caractères)"
///
/// Cette fonction retournera sur deux lignes, le nombre de caractères (UTF-8) de la valeur textuelle puis du nombre d'octets correspondants. Les deux nombres ne doivent pas varier pour du texte qui se retreint à ASCII.
///
fn resoudre_compter(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    let arg_chemin = if let Some(c) = arguments.extraire() {
        c
    } else {
        return Retour::creer_str(false, "un chemin vide n'est pas acceptable");
    };
    let mut canal = acces_canal!(contexte);
    match grammaire::chemin_extraire(&arg_chemin) {
        Ok(chemin) => canal.resoudre(&chemin, |valeur| match valeur {
            Valeurs::Texte(t) => Retour::creer(
                true,
                format!(
                    "\t{} caractère(s)\n\t{} octet(s)",
                    t.chars().count(),
                    t.len()
                ),
            ),
            _ => Retour::creer_str(false, "le type de valeur cible n'est pas conforme"),
        }),
        Err(_) => Retour::creer_str(false, "ce chemin n'existe pas"),
    }
}

/// # Fonction de résolution locale "découper du texte (caractères)"
///
/// Cette fonction utilise une origine obligatoire (borne inférieure) et optionnellement une limite (borne supérieure), pour ne garder que la partie désirée.
///
/// Attention : la borne supérieure est l'indication de position d'un caractère valide UTF-8 (pas d'un octet), et ne représente __donc pas__ le nombre de caractères à garder depuis l'origine.
///  
/// La fonction notifie les souscripteurs qui demandent les opérations sur les valeurs du canal.
///
fn resoudre_decouper(contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    let arg_chemin = if let Some(c) = arguments.extraire() {
        c
    } else {
        return Retour::creer_str(false, "un chemin vide n'est pas acceptable");
    };
    let debut = match {
        if let Some(d) = arguments.extraire() {
            d
        } else {
            return Retour::creer_str(false, "l'origine est impérative");
        }
    }
    .parse::<usize>()
    {
        Ok(n) => n,
        Err(_) => return Retour::creer_str(false, "origine =/= entier positif "),
    };
    let fin = arguments.extraire();
    let mut canal = acces_canal!(contexte);
    let r = match grammaire::chemin_extraire(&arg_chemin) {
        Ok(chemin) => canal.resoudre(&chemin, |valeur| match valeur {
            Valeurs::Texte(t) => {
                if debut >= t.len() {
                    Retour::creer_str(false, "origine hors borne")
                } else {
                    let mut position = 0;
                    match fin {
                        Some(f) => match f.parse::<usize>() {
                            Ok(n) => {
                                if n > t.len() {
                                    Retour::creer_str(false, "fin hors borne")
                                } else {
                                    t.retain(|_| {
                                        position += 1;
                                        if position - 1 < debut || position - 1 > n {
                                            false
                                        } else {
                                            true
                                        }
                                    });
                                    Retour::creer_str(true, "texte découpé")
                                }
                            }
                            Err(_) => Retour::creer_str(false, "fin =/= entier positif "),
                        },
                        None => {
                            t.retain(|_| {
                                position += 1;
                                if position - 1 < debut {
                                    false
                                } else {
                                    true
                                }
                            });
                            Retour::creer_str(true, "texte découpé")
                        }
                    }
                }
            }
            _ => Retour::creer_str(false, "le type de valeur cible n'est pas conforme"),
        }),
        Err(_) => Retour::creer_str(false, "ce chemin n'existe pas"),
    };
    if r.etat {
        canal.notifier(
            &contexte.profil,
            format!("(module 'texte') ajout : {}", arg_chemin),
        );
    }
    r
}

pub fn resoudre(appel: &str) -> Result<Resolveur, Retour> {
    match appel {
        "ajouter" => Ok(resoudre_ajouter as Resolveur),
        "compter" => Ok(resoudre_compter as Resolveur),
        "découper" => Ok(resoudre_decouper as Resolveur),
        _ => Err(Retour::creer_str(
            false,
            "module numérique : fonction inconnue",
        )),
    }
}

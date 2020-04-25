//! # Module de définition des types de valeurs
//!
//! Ce module permet de définition une énumération importante pour le programme : les types de valeurs possibles. Ces types sont "indépassables" au sein du processus, mais on peut les étendre avant la compilation, en ajoutant de nouveaux
//!
//! Ces types peuvent être vus comme ceux fondamentaux dans un langage de programmation. Ils n'existe pas de notion d'alias cependant.
//!
//! Un type peut parfois être "altérer" vers un autre. C'est-à-dire que le programme autorise des passages d'un type de valeur à l'autre - aux risques et périls du client, car certaines de ces altération sont possibles en perdant ou modifiant des informations.
//!
//! L'altération depuis ou vers le type de valeur _Objet_ est toujours impossible et interdit. Il n'a en effet aucun sens.
//!
//! ## Dans le détail...
//!
//! Aujourd'hui Robert compte 5 types de valeurs :
//! 1. Les booléens (_Booleen_) qui ont un type fondamental interne _bool_.
//!    Les booléens peuvent représenter deux états ("vrai" / "1") ou ("faux" / "0").
//! 2. Les relatifs (_Relatif_) qui ont un type fondamental interne _i32_ (extensible avant compilation à i64).
//!    Les booléens peuvent représenter des nombres entiers positifs ou négatifs.
//! 3. Les flottants (_Flottant_) qui ont un type fondamental interne _f32_ (extensible avant compilation à f64).
//!    Les flottants peuvent représenter des nombres rationnels (avec la virgule), positifs ou négatifs. Attention, ils n'a aucune correction effectuée sur la précision des arrondis (voir 'cancellation' sur Wikipédia).
//! 4. Les textes (_Texte_) qui ont un type interne _String_ (référencables en _&str_ en Rust).
//!    Seul le texte valide UTF-8 est reconnu pour du texte, comme d'ailleurs pour toutes les commandes.
//! 5. Les objets (_Objets_) qui sont les regroupements de valeurs dans un dictionnaire (paire clé/valeurs, où la clé est utilisée sous forme de hash).
//!    Un objet est différent d'un canal, et sert à créer la récurcisité au sein des canaux ainsi que des représentations complexes d'informations.
//!
//! A ce jour il manque deux types de valeurs appréciables : une chaîne binaire de taille arbitraire fixe ainsi que la représentation d'une date (UTC, probablement sous le format d'un timestamp 64bits). Ces formats seront peut-être ajoutés prochainement.
//!

// --- --- --- --- --- --- --- --- ---
// (1) Importation des modules internes
// --- --- --- --- --- --- --- --- ---

use std::collections::HashMap;

// --- --- --- --- --- --- --- --- ---
// (2) Importation des modules du projet
// --- --- --- --- --- --- --- --- ---

use crate::resolution::Retour;
use crate::serie::{Serie, Source};

// --- --- --- --- --- --- --- --- ---
// (2 bis) Importation des sous-modules dédiés
// --- --- --- --- --- --- --- --- ---

pub mod soustype_rss;

use crate::valeur::soustype_rss::{RSSFlux, RSSItems};

// --- --- --- --- --- --- --- --- ---
// (3) Constantes du projet
// --- --- --- --- --- --- --- --- ---

use crate::configuration::DEBUG;
use crate::configuration::NBRE_MAX_VALEURS;

// --- --- --- --- --- --- --- --- ---
// (4) Définition des structures, énumérations et leurs implémentations
// --- --- --- --- --- --- --- --- ---

#[derive(Debug)]
pub enum Valeurs {
    /// Stocke un bit
    Booleen(bool),

    /// Stocke un entier positif ou négatif
    Relatif(i32),

    /// Stocke un nombre rationnel positif ou négatif
    Flottant(f32),

    /// Stocke une chaîne de caractères valide UTF-8
    Texte(String),

    /// Stocke un dictionnaire de paires clé/valeur (récurcivité)
    Objet(HashMap<String, Valeurs>),

    /// Début d'intégration de type de valeurs complexes (sous-types)
    Rss(RSSFlux, RSSItems),
}

/// Implémentation du trait 'Série', qui convertit la valeur vers une chaîne binaire standardisée (ex. vers un fichier de sauvegarde ou en masse vers le client)
impl Serie for Valeurs {
    /// Tente la sérialisation vers une variable générique supportant l'écriture de _`&[u8]`_
    fn serialiser<T: std::io::Write>(&self, source: &mut Source<T>) -> Option<usize> {
        match self {
            Valeurs::Booleen(b) => {
                let mut buffer: [u8; 1] = [0; 1];
                buffer[0] = if *b { 255u8 } else { 0u8 };
                source.ecrire(1, &buffer)
            }
            Valeurs::Relatif(n) => {
                let mut buffer: [u8; 4] = [0; 4];
                buffer.copy_from_slice(&n.to_be_bytes());
                source.ecrire(2, &buffer)
            }
            Valeurs::Flottant(n) => {
                let mut buffer: [u8; 4] = [0; 4];
                buffer.copy_from_slice(&n.to_be_bytes());
                source.ecrire(3, &buffer)
            }
            Valeurs::Texte(t) => source.ecrire(4, &t.as_bytes().to_vec()),
            Valeurs::Objet(o) => {
                let mut n = 0;
                let mut buffer: [u8; 4] = [0; 4];
                buffer.copy_from_slice(&(o.len() as u32).to_be_bytes());
                if let Some(t) = source.ecrire(5, &buffer[0..4]) {
                    n += t;
                    for (cle, valeur) in o {
                        if let Some(t) = source.ecrire(6, &cle.as_bytes().to_vec()) {
                            n += t;
                        } else {
                            return None;
                        }
                        if let Some(t) = valeur.serialiser(source) {
                            n += t;
                        } else {
                            return None;
                        }
                    }
                    Some(n)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Implémentation du trait interne à Rust 'Drop', lors de la suppression d'une variable (appel automatique)
impl Drop for Valeurs {
    /// Retourne un message sur STDIN si le débug est actif
    fn drop(&mut self) {
        if DEBUG {
            println!("! suppression 'Valeurs' : {:?}", self);
        }
    }
}

/// Implémentation standard
impl Valeurs {
    /// Création d'une valeur en fonction de son type (via l'altération). Si l'altération échoue ou s'il y a déjà trop d'éléments dans le dictionnaire ('Canal' ou 'Valeurs::Objet'), l'insertion échoue complètement (aucune information ajoutée)
    pub fn creer_valeur(
        &mut self,
        cle: String,
        valeur: String,
        valeur_type: Option<String>,
    ) -> Retour {
        {match self {
				Valeurs::Objet( h ) => {
					if h.len() >= NBRE_MAX_VALEURS {
						return Retour::creer_str( false, "objet plein (max. d'éléments atteints)" )
					}
					h
				}
				_ => return Retour::creer_str( false, "tentative de création sur autre chose qu'un objet" )
		}}.insert(
			cle,
			match valeur_type {
				None => Valeurs::Texte( valeur ),
				Some( t ) => match &t[..] {
					"objet" => {
						if valeur != "~" {
							return Retour::creer_str( false, "la valeur doit être à '~'" );
						} else {
							Valeurs::Objet( HashMap::new() )
						}
					}
					_ => {
						let mut v = Valeurs::Texte( valeur.to_string() );
						if !v.alterer( &t ) {
							return Retour::creer_str( false, "altération impossible, la valeur n'est pas conforme au type souhaité" );
						}
						v
					}
				}
			}
		);
        Retour::creer_str(true, "valeur créée et ajoutée au canal")
    }

    /// Résout les chemins fournis vers la valeur cible
    pub fn resoudre<F>(&mut self, chemin: &[&str], fct: F) -> Retour
    where
        F: FnOnce(&mut Valeurs) -> Retour,
    {
        match self {
            Valeurs::Objet(o) => {
                if let Some(v) = o.get_mut(chemin[0]) {
                    if chemin.len() == 1 {
                        fct(v)
                    } else {
                        v.resoudre(&chemin[1..], fct)
                    }
                } else {
                    Retour::creer_str(false, "chemin incorrect (clé inconnue)")
                }
            }
            _ => Retour::creer_str(false, "chemin incorrect (hors d'un objet)"),
        }
    }

    /// Altére "sur place" (sans ajout/suppression dans le dictionnaire) lorsque c'est possible
    pub fn alterer(&mut self, r#type: &str) -> bool {
        match r#type {
            "booléen" => match self {
                Valeurs::Objet(_) => false,
                Valeurs::Booleen(_) => true,
                Valeurs::Relatif(n) => {
                    *self = Valeurs::Booleen(if *n > 0i32 { true } else { false });
                    true
                }
                Valeurs::Flottant(n) => {
                    *self = Valeurs::Booleen(if *n > 0f32 { true } else { false });
                    true
                }
                Valeurs::Texte(t) => match &t[..] {
                    "vrai" => {
                        *self = Valeurs::Booleen(true);
                        true
                    }
                    "false" => {
                        *self = Valeurs::Booleen(false);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            "relatif" => match self {
                Valeurs::Objet(_) => false,
                Valeurs::Relatif(_) => true,
                Valeurs::Booleen(b) => {
                    *self = Valeurs::Relatif(if *b { 1i32 } else { 0i32 });
                    true
                }
                Valeurs::Flottant(n) => {
                    *self = Valeurs::Relatif(n.round() as i32);
                    true
                }
                Valeurs::Texte(t) => {
                    if let Ok(n) = t.parse::<i32>() {
                        *self = Valeurs::Relatif(n);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            "flottant" => match self {
                Valeurs::Objet(_) => false,
                Valeurs::Flottant(_) => true,
                Valeurs::Relatif(n) => {
                    *self = Valeurs::Flottant(*n as f32);
                    true
                }
                Valeurs::Booleen(b) => {
                    *self = Valeurs::Flottant(if *b { 1f32 } else { 0f32 });
                    true
                }
                Valeurs::Texte(t) => {
                    if let Ok(n) = t.parse::<f32>() {
                        *self = Valeurs::Flottant(n);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            "texte" => match self {
                Valeurs::Objet(_) => false,
                Valeurs::Texte(_) => true,
                Valeurs::Booleen(b) => {
                    *self = Valeurs::Texte(if *b {
                        "vrai".to_string()
                    } else {
                        "faux".to_string()
                    });
                    true
                }
                Valeurs::Relatif(n) => {
                    *self = Valeurs::Texte(n.to_string());
                    true
                }
                Valeurs::Flottant(n) => {
                    *self = Valeurs::Texte(n.to_string());
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}

// --- --- --- --- --- --- --- --- ---
// (5) Définition des fonctions
// --- --- --- --- --- --- --- --- ---

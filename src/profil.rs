//! # Module des profils
//!

use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// ----------------------------------------------------

use crate::configuration::PROFILS_PSEUDO_DEFAUT;
use crate::configuration::PROFILS_SOURCE;

// ----------------------------------------------------

/// # Macro "est authentifié (test)"
///
/// Cette macro ajoute un test d'authentification (avec retour de fonction si négative). Sert comme raccourci pour les modules de résolution qui souhaitent restreindre les droits aux seuls profils reconnus et autorisés.
///
macro_rules! est_authentifie {
    ( $contexte:ident ) => {
        if !$contexte.profil.est_authentifie() {
            return Retour::creer_str(false, "authentification obligatoire");
        }
    };
}

// ----------------------------------------------------

pub enum ProfilPseudo<'a> {
    Statique(&'a str),
    Dynamique(String),
}

impl fmt::Display for ProfilPseudo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProfilPseudo::Statique(m) => *m,
                ProfilPseudo::Dynamique(m) => &m[..],
            }
        )
    }
}

pub struct Profil<'a> {
    pub identifie: bool,
    pub pseudo: ProfilPseudo<'a>,
}

impl Profil<'_> {
    pub fn creer() -> Self {
        Profil {
            identifie: false,
            pseudo: ProfilPseudo::Statique(PROFILS_PSEUDO_DEFAUT),
        }
    }
    pub fn authentifier(&mut self, pseudo: &str, passe: &str) -> Result<bool, &'static str> {
        if let Ok(f) = File::open(PROFILS_SOURCE) {
            let b = BufReader::new(f);
            for l in b.lines() {
                match l {
                    Ok(ligne) => {
                        let v = ligne.split('\t').collect::<Vec<&str>>();
                        if v.len() < 2 {
                            return Err("fichier des profils incorrect");
                        }
                        if v[0].trim() == pseudo && v[1].trim() == passe {
                            self.identifie = true;
                            self.pseudo = ProfilPseudo::Dynamique(pseudo.to_string());
                            return Ok(true);
                        }
                    }
                    Err(_) => (),
                }
            }
            Ok(false)
        } else {
            Err("fichier des profils inaccessible")
        }
    }
    pub fn anonymiser(&mut self) {
        *self = Profil::creer();
    }
    pub fn est_authentifie(&self) -> bool {
        self.identifie
    }
}

impl fmt::Display for Profil<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (authentifié : {})",
            self.pseudo,
            if self.identifie { "oui" } else { "non" }
        )
    }
}

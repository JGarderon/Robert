use crate::grammaire::ArgumentsLocaux;
use crate::resolution::{Contexte, Resolveur, Retour};
use crate::valeur::Valeurs;

use crate::valeur::soustype_rss::{RSSFlux, RSSItems}; // , RSSItem

pub fn resoudre_tester(_contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    let args = match arguments.tous() {
        Ok(mut v) => match v.len() {
            2 => {
                v.push("aucune description".to_string());
                v.push("".to_string());
                v
            }
            3 => {
                v.push("".to_string());
                v
            }
            4 => v,
            _ => return Retour::creer_str(false, "nbre d'arguments invalides"),
        },
        Err(e) => return Retour::creer_str(false, e),
    };
    let v = Valeurs::Rss(
        RSSFlux::creer(
            &args[0][..], // titre
            &args[1][..], // lien
            &args[2][..], // description
            &args[3][..], // date recuperation
        ),
        RSSItems::creer(),
    );
    Retour::creer(true, format!("item RSS test : {:?}", v))
}

fn resoudre_aide(_contexte: &mut Contexte, _arguments: ArgumentsLocaux) -> Retour {
    Retour::creer_str(false, "non-implémenté")
}

pub fn resoudre(appel: &str) -> Result<Resolveur, Retour> {
    match appel {
        "tester" => Ok(resoudre_tester as Resolveur),

        "aide?" => Ok(resoudre_aide as Resolveur),
        _ => Err(Retour::creer_str(false, "module 'rss' : fonction inconnue")),
    }
}

use crate::grammaire::ArgumentsLocaux;
use crate::resolution::{Contexte, Resolveur, Retour};
use crate::valeur::Valeurs;

use crate::valeur::soustype_rss::{RSSFlux, RSSItems}; // , RSSItem

pub fn resoudre_flux_creer(_contexte: &mut Contexte, mut arguments: ArgumentsLocaux) -> Retour {
    let flux = match arguments.tous() {
        Ok(args) => match args.len() {
            2 => RSSFlux::creer(
                &args[0][..], // titre
                &args[1][..], // lien
                "",           // description
                "",           // date recuperation
            ),
            3 => RSSFlux::creer(
                &args[0][..], // titre
                &args[1][..], // lien
                &args[2][..], // description
                "",           // date recuperation
            ),
            4 => RSSFlux::creer(
                &args[0][..], // titre
                &args[1][..], // lien
                &args[2][..], // description
                &args[3][..], // date recuperation
            ),
            _ => return Retour::creer_str(false, "nbre d'arguments invalides"),
        },
        Err(e) => return Retour::creer_str(false, e),
    };
    let v = Valeurs::Rss(flux, RSSItems::creer());
    Retour::creer(true, format!("item RSS test : {:?}", v))
}

fn resoudre_aide(_contexte: &mut Contexte, _arguments: ArgumentsLocaux) -> Retour {
    Retour::creer_str(false, "non-implémenté")
}

pub fn resoudre(appel: &str) -> Result<Resolveur, Retour> {
    match appel {
        "flux:creer" => Ok(resoudre_flux_creer as Resolveur),

        "aide?" => Ok(resoudre_aide as Resolveur),
        _ => Err(Retour::creer_str(false, "module 'rss' : fonction inconnue")),
    }
}

use std::fmt;

#[derive(Debug)]
pub struct RSSFlux {
    /// Titre du flux
    titre: String,

    /// Lien vers le flux
    lien: String,

    /// Description du flux
    description: String,

    /// Date de la dernière récupération du flux (prochainement: SystemTime ?)
    date_recuperation: String,
}

impl RSSFlux {
    pub fn creer(titre: &str, lien: &str, description: &str, date_recuperation: &str) -> Self {
        RSSFlux {
            titre: titre.to_string(),
            description: description.to_string(),
            lien: lien.to_string(),
            date_recuperation: date_recuperation.to_string(),
        }
    }
}

impl fmt::Display for RSSFlux {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Flux RSS '{}' ({})", self.titre, self.lien,)
    }
}

#[derive(Debug)]
pub struct RSSItems {
    /// Liste de tous les items stockés
    liste: Vec<RSSItem>,
}

impl RSSItems {
    pub fn creer() -> Self {
        RSSItems { liste: Vec::new() }
    }
}
#[derive(Debug)]
pub struct RSSItem {
    /// Titre de l'item
    titre: String,

    /// Description de l'item
    description: String,

    /// Date de publication du contenu
    date_publication: String,

    /// Date de récupération de l'item
    date_recuperation: String,

    /// Lien vers le contenu original
    lien: String,
}

impl fmt::Display for RSSItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Item RSS '{}' ({})", self.titre, self.lien,)
    }
}

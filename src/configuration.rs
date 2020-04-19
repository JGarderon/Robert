//! # Module de configuration 
//! 
//! Tout ce qui permet de définir des paramêtres lors de sa compilation, se situe ici. 
//! 

/// Définit sur le mode "débug" est actif (renvoi sur la console par défaut). 
pub const DEBUG: bool = true; 

/// Nom du dictionnaire par défaut, créé par le programme et qui sert aussi de canal par défaut. Il ne peut et ne doit être jamais supprimé lors de l'exécution des requêtes des utilisateurs. 
pub const CANAL_NOM_DEFAUT: &'static str = "défaut";

/// Taille maximale admissible par ligne reçue sur un socket. Cette taille fournie donc la taille maximum admissible des requêtes pour le reste du programme. 
pub const TAILLE_LIGNE_MAX: usize = 1024; 

/// Taille maximale admissible pour le texte contenu dans les dictionnaires. 
pub const TAILLE_TEXTE_MAX: usize = TAILLE_LIGNE_MAX*5; 

// ///Nbre maximum admissible de valeurs pour chaque objet. 
// const NBRE_MAX_OBJETS: usize = 250; 

/// Nbre maximum admissible de valeurs pour chaque canal (dictionnaire). 
pub const NBRE_MAX_VALEURS: usize = 500; 

/// Nbre maximum admissible de canaux dans le processus en cours. 
pub const NBRE_MAX_CANAUX: usize = 8; 

/// Chemin vers le fichier des profils 
pub const PROFILS_SOURCE: &'static str = "./profils.csv"; 

/// Nom par défaut ('pseudo') d'un client TCP non-authentifié 
pub const PROFILS_PSEUDO_DEFAUT: &'static str = "visiteur anonyme"; 

/// Racine par défaut pour trouver les scripts accessibles 
pub const SCRIPTS_DOSSIER: &'static str = "./scripts"; 


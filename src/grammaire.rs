//! # Module grammatical 
//! 
//! Ce module permet la gestion de la partie grammaticale (syntaxique) et un partie sémantique des requêtes reçues. 
//! 
//! Ce module ne doit normalement dépendre d'aucun autre, car il reçoit du texte, et renvoie du texte. Cependant une tolérance doit être faite sur les retours, autorisant par exemple la structure Retour du module de résolution, afin d'optimiser le processus d'analyse. 
//! 
//! ## Principe de fonctionnement 
//! 
//! Une requête est composée d'une ligne (séparateur '\n'), dans laquelle on dispose d'argument (séparateur ' '). Ces arguments peuvent être de tous les ordres (clé, chemin, valeur quelconque) et dépendend d'un contexte de résolution. 
//! 
//! __nb :__ _Attention, une requête a une taille limite, qui est la taille limite acceptée d'une ligne lors de la reception par le socket (voir la configuration)._
//! 
//! Ce contexte est retrouvé par le premier argument qui est toujours, dans l'esprit des fonctions lambda, le chemin vers une fonction de résolution. Le format est le suivant : "_fonction_" ou "_module:fonction_". 
//! 
//! Un 'chemin' est une clé qui permet de résoudre la profondeur (un objet dans un objet). Une clé est un chemin qui n'a seul niveau. Cette clé est toujours en seconde position. Les autres arguments n'ont pas de signifcation propre. 
//! 
//! Certaines fonctions n'autorisent qu'un nombre limité d'arguments. 
//! 

use std::net::TcpStream; 
use std::iter::FromIterator; 
use std::io::Bytes; 

use crate::TAILLE_LIGNE_MAX; 

// ---------------------------------------------------- 

use crate::resolution::Retour; 

// ---------------------------------------------------- 

#[derive(Debug)] 
pub enum ArgumentsLocauxEtat { 
    Suivant(usize, usize), 
    Stop, 
    Erreur(&'static str) 
} 

#[derive(Debug)] 
pub struct ArgumentsLocaux { 
    pub source: Vec<char>, 
    pub position: usize 
} 

impl ArgumentsLocaux { 
    pub fn trim( &self, texte: &[char] ) -> Option<usize> { 
        for (i, signe) in texte.iter().enumerate() { 
            match signe { 
                ' ' | '\t'| '\r' | '\n'  => (), 
                _ => return Some( i ) 
            } 
        } 
        None 
    } 
    pub fn suivant( &mut self ) -> ArgumentsLocauxEtat { 
        if self.position >= self.source.len() { 
            return ArgumentsLocauxEtat::Stop; 
        } 
        let texte = &self.source[self.position..]; 
        if texte.len() == 0 { 
            return ArgumentsLocauxEtat::Stop; 
        } 
        let debut = match self.trim( texte ) { 
            Some( i ) => i, 
            None => return ArgumentsLocauxEtat::Stop 
        }; 
        let mut guillemet_ouvert = false; 
        for (i, signe) in texte[debut..].iter().enumerate() { 
            match signe { 
                ' ' if !guillemet_ouvert => return ArgumentsLocauxEtat::Suivant( debut, debut+i ), 
                '"' => { 
                    guillemet_ouvert = !guillemet_ouvert; 
                    if !guillemet_ouvert { 
                        return ArgumentsLocauxEtat::Suivant( debut+1, debut+i ); 
                    } 
                } 
                _ => () 
            } 
        } 
        if guillemet_ouvert { 
            ArgumentsLocauxEtat::Erreur( "guillemet non-fermé" ) 
        } else { 
            ArgumentsLocauxEtat::Suivant( debut, texte.len() ) 
        } 
    } 
    pub fn extraire( &mut self ) -> Option<String> { 
        if let ArgumentsLocauxEtat::Suivant( depart, stop ) = self.suivant() { 
            let r = &self.source[self.position+depart..self.position+stop]; 
            self.position += stop+1; 
            Some( String::from_iter( r ) ) 
        } else { 
            None 
        } 
    } 
    pub fn est_stop( &mut self ) -> bool { 
        if let None = self.extraire() { 
            true 
        } else { 
            false 
        } 
    } 
    pub fn tous( &mut self ) -> Result<Vec<String>, &'static str> { 
        let mut r: Vec<String> = Vec::new(); 
        loop { 
            match self.suivant() { 
                ArgumentsLocauxEtat::Suivant( depart, stop ) => { 
                    r.push( 
                        String::from_iter( 
                            &self.source[self.position+depart..self.position+stop] 
                        ) 
                    ); 
                    self.position += stop+1; 
                } 
                ArgumentsLocauxEtat::Stop => break, 
                ArgumentsLocauxEtat::Erreur( e ) => return Err( e ) 
            } 
        } 
        Ok( r ) 
    } 
} 

// ---------------------------------------------------- 

pub enum ExtractionLigne { 
	Commande(String), 
	Erreur(Retour), 
	Stop 
} 

pub fn extraire_ligne( iterateur: &mut Bytes<TcpStream> ) -> ExtractionLigne { 
	let mut a: [u8; TAILLE_LIGNE_MAX] = [0; TAILLE_LIGNE_MAX]; 
	let mut position: usize = 0; 
	loop { 
		match iterateur.next() { 
			Some( Ok( 13u8 ) ) if position < TAILLE_LIGNE_MAX => { 
				if let Ok( s ) = String::from_utf8( a[..position].to_vec() ) { 
					return ExtractionLigne::Commande( s ); 
				} else { 
					return ExtractionLigne::Erreur( 
						Retour::creer_str( false, "chaîne invalide" ) 
					); 
				} 
			} 
			Some( Ok( n ) ) if position < TAILLE_LIGNE_MAX => a[position] = n, 
			Some( Ok( _ ) ) if position >= TAILLE_LIGNE_MAX => { 
				loop { 
					match iterateur.next() { 
						Some( Ok( 13u8 ) ) => break, 
						_ => () 
					} 
				} 
				return ExtractionLigne::Erreur( 
					Retour::creer_str( false, "ligne trop longue" ) 
				); 
			} 
			_ => break 
		} 
		position += 1; 
	} 
	if position == 0 { 
		return ExtractionLigne::Stop; 
	} 
	if let Ok( s ) = String::from_utf8( a[..position].to_vec() ) { 
		return ExtractionLigne::Commande( s ); 
	} else { 
		return ExtractionLigne::Erreur( 
			Retour::creer_str( false, "caractère(s) invalide(s)" ) 
		); 
	} 
} 

pub fn extraction_commande( commande: &str ) -> (&str, &str) { 
	if let Some( position ) = commande.find( ' ' ) { 
		( &commande[0..position], &commande[position+1..] ) 
	} else { 
		( &commande, "" ) 
	} 
} 

// ---------------------------------------------------- 

pub fn chemin_extraire( chemin: &str ) -> Result<Vec<&str>,&'static str> { 
    let iterateur = chemin.split( '/' ); 
    let mut motifs: Vec<&str> = Vec::new(); 
    for motif in iterateur { 
        motifs.push( motif ); 
    } 
    if motifs.len() < 1 { 
        Err( "le chemin est vide" ) 
    } else { 
        Ok( motifs ) 
    } 
} 







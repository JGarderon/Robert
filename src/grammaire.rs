//! Module 'grammaire' 
//! Ce module permet la gestion de la partie grammaticale (syntaxique) et un partie sémantique des requêtes reçues. 


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
            self.position += stop; 
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


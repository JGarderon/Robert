//! # Module de sérialisation des canaux 
//! 
//! Ce module sert à implémenter les fonctions de sérialisation et désérialisation des valeurs, vers des données binaires. 
//! 
//! Ainsi au démarrage pour amorcer des canaux ou en cours de fonctionnement du logiciel vers des points de sauvegarde, les standards utilisés seront définis. 
//! 
//! Ce module est indépendant du module grammatical, qui est lisible l'humain. Le format supporté ici, est strictement binaire. 
//! 
//! Par défaut, le champ de taille de chaque objet stocké est représenté par un entier non-signé sur 32 bits, soit au maximum 4,3 Go. Ce point est indispensable à prendre en compte avant la compilation car si votre base se rapproche de cette taille, la valeur totale stockée pour être largement supérieure et la sérialisation être impossible (ou plus grave, l'erreur reste silencieuse mais le fichier est corrompu). 
//! 
//! La modification du format de taille de la source à un entier non-signé sur 64 bits (u64), résoud ce problème, mais le fichier généré sera bien plus gros. 
//! 

    // --- --- --- --- --- --- --- --- --- 
    // (1) Importation des modules internes 
    // --- --- --- --- --- --- --- --- --- 

use std::io::BufWriter; 
use std::io::Write; 

    // --- --- --- --- --- --- --- --- --- 
    // (2) Importation des modules du projet 
    // --- --- --- --- --- --- --- --- --- 

    // --- --- --- --- --- --- --- --- --- 
    // (3) Constantes du projet 
    // --- --- --- --- --- --- --- --- --- 

    // --- --- --- --- --- --- --- --- --- 
    // (4) Définition des structures, énumérations et leurs implémentations 
    // --- --- --- --- --- --- --- --- --- 

pub struct Source<T: std::io::Write> { 
	pub fichier: BufWriter<T>  
} 

impl<T: std::io::Write> Source<T> { 
	pub fn ecrire( &mut self, contenu_type: u8, contenu_valeur: &[u8] ) -> Option<usize> { 
		let mut n = 0; 
		if let Ok( t ) = self.fichier.write( &vec!( contenu_type ) ) { 
			n += t; 
		} else { 
			return None; 
		} 
		let mut tableau: [u8;4] = [0;4]; 
		tableau.copy_from_slice( &(contenu_valeur.len() as u32).to_be_bytes() ); 
		if let Ok( t ) = self.fichier.write( &tableau ) { 
			n += t; 
		} else { 
			return None; 
		} 
		if let Ok( t ) = self.fichier.write( &contenu_valeur ) { 
			n += t; 
		} else { 
			return None; 
		} 
		Some( n ) 
	} 
} 

pub trait Serie { 
	fn serialiser<T: std::io::Write>( &self, source: &mut Source<T> ) -> Option<usize>; 
} 

    // --- --- --- --- --- --- --- --- --- 
    // (5) Définition des fonctions 
    // --- --- --- --- --- --- --- --- --- 





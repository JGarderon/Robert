//! # Module "Série" 
//! 
//! Ce module sert à implémenter les fonctions de sérialisation et désérialisation des valeurs, vers des données binaires. 
//! 
//! Ainsi au démarrage pour amorcer des canaux ou en cours de fonctionnement du logiciel vers des points de sauvegarde, les standards utilisés seront définis. 
//! 
//! Ce module est indépendant du module grammatical, qui est lisible l'humain. Le format supporté ici, est strictement binaire. 
//! 

struct Source { 

}

trait Serie { 
	fn serialiser( &mut self, &mut Source ) -> bool; 
} 






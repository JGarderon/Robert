
use std::mem; 

// ---------------------------------------------------- 

use crate::base::Valeurs;
use crate::base::Dictionnaire; 
use crate::resolution::Contexte; 
use crate::grammaire::ArgumentsLocaux; 

// ---------------------------------------------------- 

use crate::resolution::Resolveur; 
use crate::resolution::Retour; 

// ---------------------------------------------------- 

trait Mesure { 
	fn mesurer( &self ) -> usize;  
} 

impl Mesure for Valeurs { 
	fn mesurer( &self ) -> usize { 
		mem::size_of_val( self )+match self { 
			Valeurs::Boolean( b ) => mem::size_of_val( b ), 
			Valeurs::Relatif( n ) => mem::size_of_val( n ), 
			Valeurs::Flottant( f ) => mem::size_of_val( f ), 
			Valeurs::Texte( t ) => mem::size_of_val( t )+t.as_bytes().len() 
		} 
	} 
} 

impl Mesure for Dictionnaire { 
	fn mesurer( &self ) -> usize { 
		let mut total = mem::size_of_val( &self ) 
			+mem::size_of_val( &self.nom ) 
			+self.nom.as_bytes().len() 
			+mem::size_of_val( &self.liste ); 
		for (cle, valeur) in self.liste.iter() { 
			total += mem::size_of_val( &cle )+cle.as_bytes().len(); 
			total += mem::size_of_val( &valeur )+valeur.mesurer(); 
		} 
		total 
	} 
} 

// ---------------------------------------------------- 

fn resoudre_calculer_taille( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if let Some( _ ) = arguments.extraire() { 
		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
	} 
	let mut total = 0; 
	let dicos = contexte.dicos.lock().unwrap(); 
	for (_, d) in dicos.liste.iter() { 
		let dico = d.lock().unwrap(); 
		total += dico.mesurer(); 
	} 
	Retour::creer( true, format!( "total : {}", total ) ) 
} 


pub fn resoudre( appel: &str ) -> Result<Resolveur,Retour> { 
	match appel { 
		"calculertaille" => Ok( resoudre_calculer_taille as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module texte : fonction inconnue" ) ) 
	} 
} 








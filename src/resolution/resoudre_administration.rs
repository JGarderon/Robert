
use std::mem; 

// ---------------------------------------------------- 

use crate::base::Valeurs;
use crate::base::Canal; 
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
			Valeurs::Texte( t ) => mem::size_of_val( t )+t.as_bytes().len(), 
			Valeurs::Objet( h ) => mem::size_of_val( h ) 
		} 
	} 
} 

impl Mesure for Canal { 
	fn mesurer( &self ) -> usize { 
		let mut total = mem::size_of_val( &self ) 
			+mem::size_of_val( &self.nom ) 
			+self.nom.as_bytes().len() 
			+mem::size_of_val( &self.liste ); 
		match &self.liste { 
			Valeurs::Objet( h ) => for (cle, valeur) in h.iter() { 
				total += mem::size_of_val( &cle )+cle.as_bytes().len(); 
				total += mem::size_of_val( &valeur )+valeur.mesurer(); 
			} 
			_ => panic!( "le canal '{}' est corrompu", self.nom ) 
		} 
		total 
	} 
} 

// ---------------------------------------------------- 

fn resoudre_mesurer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if let Some( _ ) = arguments.extraire() { 
		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
	} 
	let canaux = { 
		match contexte.canauxthread.lock() { 
			Ok( canaux ) => canaux, 
			Err( empoisonne ) => empoisonne.into_inner() 
		} 
	}; 
	let mut total = 0; 
	for (_, canalthread) in canaux.liste.iter() { 
		total += { 
			match canalthread.lock() { 
				Ok( canal ) => canal, 
				Err( empoisonne ) => empoisonne.into_inner() 
			} 
		}.mesurer(); 
	} 
	Retour::creer( true, format!( "total : {}", total ) ) 
} 

fn resoudre_vider( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "aucun argument autorisé" ); 
	} 
	let mut canal = Canal!( contexte ); 
	match &mut canal.liste { 
		Valeurs::Objet( h ) => { 
			h.clear(); 
			Retour::creer_str( true, "base vidée" ) 
		} 
		_ => Retour::creer_str( false, "objet racine incorrect ; le canal semble corrompu" ) 
	} 
} 

// fn resoudre_lister( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	if let Some( _ ) = arguments.extraire() { 
// 		return Retour::creer_str( false, "aucun argument accepté pour cette fonction" ); 
// 	} 
// 	let dicos = contexte.dicos.lock().unwrap(); 
// 	for (nom, d) in dicos.liste.iter() { 
// 		let dico = d.lock().unwrap(); 
// 		if let Err(_) = contexte.stream.write( 
// 			format!( 
// 				"\tcanal \"{}\" ({:?})\n", 
// 				nom, 
// 				dico.liste.len() 
// 			).as_bytes() 
// 		) { 
// 			contexte.stream.flush().unwrap(); 
// 			return Retour::creer_str( false, "erreur lors de l'envoi" ); 
// 		} 
// 	} 
// 	contexte.stream.flush().unwrap(); 
// 	Retour::creer( true, format!( "stop ({})", dicos.liste.len() ) ) 
// } 

// fn resoudre_resumer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	if !arguments.est_stop() { 
// 		return Retour::creer_str( false, "aucun argument autorisé" ); 
// 	} 
// 	let dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &dico.liste; 
// 	Retour::creer(  
// 		true, 
// 		format!( 
// 			"canal \"{}\" ({})", 
// 			dico.nom, 
// 			valeurs.len() 
// 		) 
// 	) 
// } 


pub fn resoudre( appel: &str ) -> Result<Resolveur,Retour> { 
	match appel { 
		"mesurer" => Ok( resoudre_mesurer as Resolveur ), 
		"vider" => Ok( resoudre_vider as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module texte : fonction inconnue" ) ) 
	} 
} 








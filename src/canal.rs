//! # Module des canaux 
//! 

use std::sync::mpsc::Sender; 
use std::collections::HashMap; 
use std::sync::Arc; 
use std::sync::Mutex; 

// ---------------------------------------------------- 

use crate::resolution::Retour; 
use crate::valeur::Valeurs; 

// ---------------------------------------------------- 

use crate::configuration::DEBUG; 

// ---------------------------------------------------- 

macro_rules! acces_canal {
    ( $contexte:ident ) => {
        { 
        	match $contexte.canalthread.lock() { 
        		Ok( c )	=> c, 
        		Err( empoisonne ) => empoisonne.into_inner() 
        	} 
        } 
    };
} 

#[derive(Debug)] 
pub struct Souscripteur { 
	pub pont: Sender<String>, 
	pub messages: bool, 
	pub valeurs: bool 
} 

/// Un canal se constitue de trois principaux éléments : son nom, sa liste de valeurs (qui est stockée dans un Objet, un élément de l'énumération des Valeurs) ainsi qu'un vecteur de souscripteurs. 
/// A partir de la version 1.1, dans l'idéal, la compatibilité devrait être toujours maintenue avec ce minimum. 
#[derive(Debug)] 
pub struct Canal { 
	pub nom: String, 
	pub liste: Valeurs, 
	pub souscripteurs: Vec<Souscripteur>  
} 

impl Canal { 
	pub fn resoudre<F>( &mut self, chemin: &[&str], fct: F )  -> Retour 
		where F: FnOnce( &mut Valeurs ) -> Retour
	{ 
		self.liste.resoudre( chemin, fct ) 
	} 
} 

impl Drop for Canal { 
    fn drop(&mut self) { 
    	if DEBUG { 
        	println!( "! suppression 'Canal' : {:?}", self ); 
    	}
    } 
} 

pub type CanalThread = Arc<Mutex<Canal>>; 

pub struct Canaux { 
	pub liste: HashMap<String,CanalThread> 
} 

pub type CanauxThread = Arc<Mutex<Canaux>>; 

pub fn creer_racine( nom_defaut: &str ) -> (CanalThread, CanauxThread) { 
	let mut tmp = Canaux { 
		liste: HashMap::new() 
	}; 
	let nom = nom_defaut.to_string(); 
	let canal = Arc::new( 
		Mutex::new( 
			Canal { 
				nom: nom.clone(), 
				liste: Valeurs::Objet( HashMap::new() ), 
				souscripteurs: Vec::<Souscripteur>::new() 
			} 
		) 
	) as CanalThread; 
	tmp.liste.insert( 
		nom, 
		canal.clone() 
	); 
	let canaux = Arc::new( 
		Mutex::new( 
			tmp 
		) 
	) as CanauxThread; 
	( 
		canal, 
		canaux 
	) 
} 



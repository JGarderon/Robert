
use std::sync::mpsc::Sender; 
use std::collections::HashMap; 
use std::sync::Arc; 
use std::sync::Mutex; 

// ---------------------------------------------------- 

use crate::resolution::Retour; 

// ---------------------------------------------------- 

use crate::DEBUG; 
use crate::NBRE_MAX_VALEURS; 

// ---------------------------------------------------- 

/// Un canal se constitue de trois principaux éléments : son nom, sa liste de valeurs (qui est stockée dans un Objet, un élément de l'énumération des Valeurs) ainsi qu'un vecteur de souscripteurs. 
/// A partir de la version 1.1, dans l'idéal, la compatibilité devrait être toujours maintenue avec ce minimum. 
#[derive(Debug)] 
pub struct Canal { 
	pub nom: String, 
	pub liste: Valeurs, 
	pub souscripteurs: Vec<Sender<String>>  
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
				souscripteurs: Vec::<Sender<String>>::new() 
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

// ---------------------------------------------------- 

#[derive(Debug)] 
pub enum Valeurs { 
	Boolean(bool), 
	Relatif(i32), 
	Flottant(f32), 
	Texte(String), 
	Objet(HashMap<String,Valeurs>) 
} 

impl Drop for Valeurs { 
    fn drop(&mut self) { 
    	if DEBUG { 
        	println!( "! suppression 'Valeurs' : {:?}", self); 
    	}
    } 
} 

impl Valeurs { 

	pub fn creer_valeur( &mut self, cle: String, valeur: String, valeur_type: Option<String> ) -> Retour { 
		{
			match self { 
				Valeurs::Objet( h ) => { 
					if h.len() >= NBRE_MAX_VALEURS { 
						return Retour::creer_str( false, "objet plein (max. d'éléments atteints)" ) 
					} 
					h 
				} 
				_ => return Retour::creer_str( false, "tentative de création sur autre chose qu'un objet" ) 
			} 
		}.insert( 
			cle, 
			match valeur_type { 
				None => Valeurs::Texte( valeur ), 
				Some( t ) => match &t[..] { 
					"objet" => { 
						if valeur != "~" { 
							return Retour::creer_str( false, "la valeur doit être à '~'" ); 
						} else { 
							Valeurs::Objet( HashMap::new() ) 
						}
					} 
					_ => {
						let mut v = Valeurs::Texte( valeur.to_string() ); 
						if !v.alterer( &t ) { 
							return Retour::creer_str( false, "altération impossible, la valeur n'est pas conforme au type souhaité" ); 
						} 
						v 
					} 
				} 
			} 
		); 
		Retour::creer_str( true, "valeur créée et ajoutée au canal" ) 
	} 

	pub fn resoudre<F>( &mut self, chemin: &[&str], fct: F ) -> Retour 
		where F: FnOnce( &mut Valeurs ) -> Retour 
	{ 
		match self { 
			Valeurs::Objet( o ) => if let Some( v ) = o.get_mut( chemin[0] ) { 
				if chemin.len() == 1 { 
					fct( v ) 
				} else { 
					v.resoudre( &chemin[1..], fct ) 
				} 
			} else { 
				Retour::creer_str( false, "chemin incorrect (clé inconnue)" ) 
			} 
			_ => Retour::creer_str( false, "chemin incorrect (hors d'un objet)" ) 
		} 
	}

	pub fn alterer( &mut self, r#type: &str ) -> bool { 
		match r#type { 
			"booléen" => match self { 
				Valeurs::Objet( _ ) => false, 
				Valeurs::Boolean( _ ) => true, 
				Valeurs::Relatif( n ) => {
					*self = Valeurs::Boolean( if *n > 0i32 { true } else { false } ); 
					true 
				} 
				Valeurs::Flottant( n ) => { 
					*self = Valeurs::Boolean( if *n > 0f32 { true } else { false } ); 
					true 
				} 
				Valeurs::Texte( t ) => { 
					match &t[..] { 
						"vrai" => { 
							*self = Valeurs::Boolean( true ); 
							true 
						} 
						"false" => { 
							*self = Valeurs::Boolean( false ); 
							true 
						} 
						_ => false 
					} 
				} 
			} 
			"relatif" => match self { 
				Valeurs::Objet( _ ) => false, 
				Valeurs::Relatif( _ ) => true, 
				Valeurs::Boolean( b ) => { 
					*self = Valeurs::Relatif( if *b { 1i32 } else { 0i32 } ); 
					true 
				} 
				Valeurs::Flottant( n ) => { 
					*self = Valeurs::Relatif( n.round() as i32 ); 
					true 
				} 
				Valeurs::Texte( t ) => { 
					if let Ok( n ) = t.parse::<i32>() { 
						*self = Valeurs::Relatif( n ); 
						true 
					} else { 
						false 
					} 
				} 
			} 
			"flottant" => match self { 
				Valeurs::Objet( _ ) => false, 
				Valeurs::Flottant( _ ) => true, 
				Valeurs::Relatif( n ) => { 
					*self = Valeurs::Flottant( *n as f32 ); 
					true 
				}, 
				Valeurs::Boolean( b ) => { 
					*self = Valeurs::Flottant( if *b { 1f32 } else { 0f32 } ); 
					true 
				}, 
				Valeurs::Texte( t ) => { 
					if let Ok( n ) = t.parse::<f32>() { 
						*self = Valeurs::Flottant( n ); 
						true 
					} else { 
						false 
					} 
				} 
			} 
			"texte" => match self { 
				Valeurs::Objet( _ ) => false, 
				Valeurs::Texte( _ ) => true, 
				Valeurs::Boolean( b ) => { 
					*self = Valeurs::Texte( if *b { "vrai".to_string() } else { "faux".to_string() } ); 
					true 
				}, 
				Valeurs::Relatif( n ) => { 
					*self = Valeurs::Texte( n.to_string() ); 
					true 
				} 
				Valeurs::Flottant( n ) => { 
					*self = Valeurs::Texte( n.to_string() ); 
					true 
				} 
			} 
			_ => false 
		} 
	} 
} 




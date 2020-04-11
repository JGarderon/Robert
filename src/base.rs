
use std::sync::mpsc::Sender; 
use std::collections::HashMap; 
use std::sync::Arc; 
use std::sync::Mutex; 

// ---------------------------------------------------- 

use crate::resolution::Retour; 

// ---------------------------------------------------- 

use crate::DEBUG; 
use crate::TAILLE_TEXTE_MAX; 

// ---------------------------------------------------- 

#[derive(Debug)] 
pub struct Dictionnaire { 
	pub nom: String, 
	pub liste: HashMap<String,Valeurs>, 
	pub souscripteurs: Vec<Sender<String>>  
} 

impl Dictionnaire { 
	pub fn creer_valeur( &mut self, cle: &str, valeur: &str, valeur_type: Option<String> ) -> bool { 
		self.liste.insert( 
			cle.to_string(), 
			match valeur_type { 
				None => Valeurs::Texte( valeur.to_string() ), 
				Some( t ) => match &t[..] { 
					"objet" => { 
						if valeur != "~" { 
							return false; 
						} else { 
							Valeurs::Objet( HashMap::new() ) 
						}
					} 
					_ => {
						let mut v = Valeurs::Texte( valeur.to_string() ); 
						if !v.alterer( &t ) { 
							return false 
						} 
						v 
					} 
				} 
			} 
		); 
		true 
	} 
} 

impl Drop for Dictionnaire { 
    fn drop(&mut self) { 
    	if DEBUG { 
        	println!( "! suppression 'Dictionnaire' : {:?}", self ); 
    	}
    } 
} 

pub type DictionnaireThread = Arc<Mutex<Dictionnaire>>; 

pub struct Dictionnaires { 
	pub liste: HashMap<String,DictionnaireThread> 
} 

pub fn creer_racine( nom_defaut: &str ) -> (DictionnaireThread, Arc<Mutex<Dictionnaires>>) { 
	let mut tmp = Dictionnaires { 
		liste: HashMap::new() 
	}; 
	let nom = nom_defaut.to_string(); 
	let dico_thread = Arc::new( 
		Mutex::new( 
			Dictionnaire { 
				nom: nom.clone(), 
				liste: HashMap::new(), 
				souscripteurs: Vec::<Sender<String>>::new() 
			} 
		) 
	) as DictionnaireThread; 
	tmp.liste.insert( 
		nom, 
		dico_thread.clone() 
	); 
	let dicos = Arc::new( 
		Mutex::new( 
			tmp 
		) 
	); 
	( 
		dico_thread, 
		dicos 
	) 
} 

// ---------------------------------------------------- 

pub type AccesseurValeur = fn( &mut Valeurs ) -> Retour; 

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
	pub fn excuter( &mut self, vecteur: &[&str], fct: AccesseurValeur ) -> Retour { 
		if let Some( v ) = self.acceder( vecteur ) { 
			fct( v ) 
		} else { 
			Retour::creer_str( false, "le chemin n'est pas correct" ) 
		} 
	} 
	pub fn acceder( &mut self, vecteur: &[&str] ) -> Option<&mut Self> { 
		if vecteur.len() == 0 { 
			Some( self ) 
		} else { 
			match self { 
				Valeurs::Objet( h ) => { 
					if let Some( valeur ) = h.get_mut( vecteur[0] ) { 
						valeur.acceder( &vecteur[1..] ) 
					} else { 
						None 
					} 
				} 
				_ => None 
			} 
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
    pub fn ajouter_texte( &mut self, v: &str ) -> bool { 
        match self { 
            Valeurs::Texte( t ) => { 
            	if t.len() + v.len() < TAILLE_TEXTE_MAX { 
	            	t.push_str( v ); 
	            	true 
            	} else { 
            		false 
            	} 
            } 
            _ => return false 
        } 
    } 
} 





use std::collections::HashMap; 

use crate::resolution::Retour; 
use crate::serie::Serie; 
use crate::serie::Source; 

use crate::configuration::DEBUG; 
use crate::configuration::NBRE_MAX_VALEURS; 

#[derive(Debug)] 
pub enum Valeurs { 
	Boolean(bool), 
	Relatif(i32), 
	Flottant(f32), 
	Texte(String), 
	Objet(HashMap<String,Valeurs>) 
} 

impl Serie for Valeurs { 
	fn serialiser<T: std::io::Write>( &self, source: &mut Source<T> ) -> Option<usize> { 
		match self { 
			Valeurs::Boolean( b ) => { 
				let mut buffer: [u8;1] = [0;1]; 
				buffer[0] = if *b { 255u8 } else { 0u8 }; 
				source.ecrire( 1, &buffer ) 
			} 
			Valeurs::Relatif( n ) => { 
				let mut buffer: [u8;4] = [0;4]; 
				buffer.copy_from_slice( &n.to_be_bytes() ); 
				source.ecrire( 2, &buffer ) 
			} 
			Valeurs::Flottant( n ) => { 
				let mut buffer: [u8;4] = [0;4]; 
				buffer.copy_from_slice( &n.to_be_bytes() ); 
				source.ecrire( 3, &buffer ) 
			} 
			Valeurs::Texte( t ) => { 
				source.ecrire( 4, &t.as_bytes().to_vec() ) 
			} 
			Valeurs::Objet( o ) => { 
				let mut n = 0; 
				let mut buffer: [u8;4] = [0;4]; 
				buffer.copy_from_slice( &(o.len() as u32).to_be_bytes() ); 
				if let Some( t ) = source.ecrire( 5, &buffer[0..4] ) { 
					n += t; 
					for (cle, valeur) in o { 
						if let Some( t ) = source.ecrire( 6, &cle.as_bytes().to_vec() ) { 
							n += t; 
						} else { 
							return None; 
						} 
						if let Some( t ) = valeur.serialiser( source ) { 
							n += t; 
						} else { 
							return None; 
						} 
					} 
					Some( n ) 
				} else {
					None 
				} 
			} 
		} 
	} 
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


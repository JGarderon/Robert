
use std::net::TcpStream; 
use std::io::Write; 

// ---------------------------------------------------- 

use crate::base::CanalThread; 
use crate::base::CanauxThread; 
use crate::profil::Profil; 

// ---------------------------------------------------- 

/// La structure 'Contexte' permet de rassembler dans un objet unique, l'ensemble des éléments propres à un socket quelque soit la fonction de résolution qui sera appelée. Elle référence aussi le canal en cours d'usage par le client, ainsi que l'origine (Canaux). 
/// Dans une fonction de résolution, elle se présentera toujours dans la forme d'une référence mutable. 
pub struct Contexte<'a> { 
	
	/// Ce champ permet de récupérer un clone de l'objet en écoute sur l'interface réseau. 
	pub service_ecoute: std::net::TcpListener, 

	/// Ce champ lorsqu'il est à "faux", permet d'interrompre la boucle globale du service. 
	pub service_poursuite: &'a mut bool, 

	/// Ce champ lorsqu'il est à "faux", permet d'interrompre la boucle locale du thead gérant le socket, dès la fin de la fonction de résolution actuelle. 
	pub poursuivre: bool, 
	
	/// Ce champ contient le nécessaire pour accéder au dictionnaire représentant le canal actuel. 
	/// Il est d'un type Arc<Mutex<Canal>> : un CanalThread est un Canal avec sa protection d'usage pour les threads. 
	pub canalthread: CanalThread, 
	
	/// Ce champ contient le nécessaire pour accéder au dictionnaires des canaux. 
	/// Il est d'un type Arc<Mutex<Canaux>> : un CanauxThread est l'origine de tous les canaux, avec sa protection d'usage pour les threads. 
	pub canauxthread: CanauxThread, 

	/// 
	pub profil: Profil<'a>,  

	/// Ce champ contient l'objet socket, librement clonable. 
	pub stream: TcpStream 

} 

impl Contexte<'_> { 

	pub fn ecrire( &mut self, texte: &str, flush: bool ) -> bool { 
		match self.stream.write( texte.as_bytes() ) { 
			Ok( _ ) => if flush { match self.stream.flush() { 
				Ok( _ ) => true, 
				Err( _ ) => false 
			} } else { true } 
			Err( _ ) => false 
		} 
	} 

	pub fn message( &mut self, message: &str ) -> bool { 
		self.ecrire( 
			&format!( "[@] {}\n", message ),  
			false 
		) 
	} 

	pub fn erreur( &mut self, message: &str ) -> bool { 
		self.ecrire( 
			&format!( "[!] {}\n", message ),  
			false 
		) 
	} 

} 






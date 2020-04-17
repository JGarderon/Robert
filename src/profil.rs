
macro_rules! est_authentifie {
	( $contexte:ident ) => {
        if !$contexte.profil.identifie { 
        	return Retour::creer_str( false, "authentification obligatoire" ); 
        } 
    }; 
} 

// ---------------------------------------------------- 

pub struct Profil { 
	pub identifie: bool 
} 

impl Profil { 
	pub fn authentifier( &mut self, _pseudo: &str, _passe: &str ) -> bool { 
		self.identifie = true; 
		true 
	} 
} 




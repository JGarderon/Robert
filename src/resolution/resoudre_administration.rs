
use std::mem; 
use std::collections::HashMap;
use std::io::BufWriter; 
use std::fs::File; 

// ---------------------------------------------------- 

use crate::base::Valeurs;
use crate::base::Canal; 
use crate::resolution::Contexte; 
use crate::grammaire::ArgumentsLocaux; 

// ---------------------------------------------------- 

use crate::resolution::Resolveur; 
use crate::resolution::Retour; 
// ---------------------------------------------------- 

use crate::serie::Source; 

// ---------------------------------------------------- 

use crate::serie::Serie; 

// ---------------------------------------------------- 

trait Mesure { 
	fn mesurer( &self ) -> usize;  
} 

impl Mesure for HashMap<String,Valeurs> { 
	fn mesurer( &self ) -> usize { 
		let mut total = 0; 
		for (cle, valeur) in self.iter() { 
			total += mem::size_of_val( cle )+cle.as_bytes().len(); 
			total += valeur.mesurer(); 
		} 
		total 
	} 
} 

impl Mesure for Valeurs { 
	fn mesurer( &self ) -> usize { 
		mem::size_of_val( self )+match self { 
			Valeurs::Boolean( b ) => mem::size_of_val( b ), 
			Valeurs::Relatif( n ) => mem::size_of_val( n ), 
			Valeurs::Flottant( f ) => mem::size_of_val( f ), 
			Valeurs::Texte( t ) => mem::size_of_val( t )+t.as_bytes().len(), 
			Valeurs::Objet( h ) => mem::size_of_val( h )+h.mesurer()  
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

/// # Fonction de résolution locale "authentifier son profil" 
/// 
/// Permet de s'authentifier, avec un couple "pseudo / mot de passe". 
/// 
fn resoudre_authentifier( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	match arguments.tous() { 
		Ok( v ) => { 
			if v.len() == 2 { 
				match contexte.profil.authentifier( &v[0], &v[1] ) { 
					Ok( true ) => { 
						contexte.message( &format!( "bonjour {}", contexte.profil ) ); 
						Retour::creer_str( 
							true, 
							"authentification réussie" 
						) 
					} 
					Ok( false ) => { 
						contexte.poursuivre = false; 
						Retour::creer_str( 
							false, 
							"authentification échouée ; vous allez être déconnecté" 
						) 
					} 
					Err( e ) => { 
						contexte.erreur( &format!( "erreur interne : {}", e ) ); 
						Retour::creer_str( 
							false, 
							"authentification échouée ; merci de rééssayer ultérieurement" 
						) 
					} 
				} 
			} else { 
				Retour::creer_str( 
					false, 
					"deux arguments obligatoires : 'pseudo' et 'passe'" 
				) 
			} 
		} 
		Err( _ ) => return Retour::creer_str( 
			false, 
			"arguments invalides" 
		) 
	} 
} 

/// # Fonction de résolution locale "anonymiser son profil" 
/// 
/// Recréer un profil vierge, sans aucun droit associé. 
/// 
fn resoudre_anonymiser( contexte: &mut Contexte, _: ArgumentsLocaux ) -> Retour { 
	contexte.message( 
		if contexte.profil.est_authentifie() { 
			"vous étiez authentifié" 
		} else { 
			"vous n'étiez pas authentifié" 
		} 
	); 
	contexte.profil.anonymiser();  
	Retour::creer_str( 
		true, 
		"anonymisation réussie" 
	) 
} 

/// # Fonction de résolution locale "consulter son profil" 
/// 
/// Retourne l'état résumé du profil en message. 
/// 
fn resoudre_profiler( contexte: &mut Contexte, _: ArgumentsLocaux ) -> Retour { 
	contexte.message( &format!( "profil : {}", contexte.profil ) ); 
	Retour::creer_str( 
		true, 
		"profilage réussi" 
	) 
} 

/// # Fonction de résolution locale "éteindre le programme" 
/// 
/// Cette fonction doit être considérée comme avec un effet "prioritaire" : elle va annuler la boucle principale du service, ce qui provoquera l'arrêt de l'écoupe réseau ainsi que du programme. De plus, l'arrêt du service provoquera la fin de toutes les clients TCP actifs. 
/// 
/// Une fois l'arrêt enclenché, il n'y a plus aucun moyen de revenir à l'état antérieur (perte des informations non-sauvegardées). 
/// 
fn resoudre_eteindre( contexte: &mut Contexte, _: ArgumentsLocaux ) -> Retour { 
	est_authentifie!( contexte ); 
	*contexte.service_poursuite = false; // /!\ UNSAFE / à retirer urgemment 
	match std::net::TcpStream::connect( contexte.service_ecoute.local_addr().unwrap() ) { 
		Ok( _ ) => Retour::creer_str( true, "extinction enclenchée ; les fils vont être progressivement arrêtés" ), 
		Err( _ ) => Retour::creer_str( false, "extinction enclenchée ; attention, la nouvelle connexion nécessaire n'a pas pu être enclenchée (l'écoute d'un nouveau client est bloquante)" ) 
	} 
	
} 

fn resoudre_serialiser( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	est_authentifie!( contexte ); 
	let fid = if let Some( arg ) = arguments.extraire() { 
		arg 
	} else { 
		return Retour::creer_str( false, "identifiant de dump obligatoire" ); 
	}; 
	if !fid.is_ascii() { 
		return Retour::creer_str( false, "seuls les caractères ASCII sont autorisés" ); 
	} 
	if fid.len() > 32 { 
		return Retour::creer_str( false, "l'identifiant de dump doit faire 32 caractères maximum" ); 
	} 
	let canal = Canal!( contexte ); 
	let f = if let Ok( f ) = File::create( format!( "./{}.dump", fid ) ) { 
		f 
	} else { 
		return Retour::creer_str( false, "impossible de créer le fichier de dump" ); 
	}; 
	let mut s = Source { 
		fichier: BufWriter::new( f ) 
	}; 
	if let Some( n ) = canal.liste.serialiser( 
		&mut s 
	) { 
		Retour::creer( true, format!( "sérialisation terminée : {} octets", n ) ) 
	} else { 
		Retour::creer_str( true, "sérialisation en erreur" ) 
	} 
} 

fn resoudre_mesurer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	est_authentifie!( contexte ); 
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
	est_authentifie!( contexte ); 
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
		"authentifier" => Ok( resoudre_authentifier as Resolveur ), 
		"anonymiser" => Ok( resoudre_anonymiser as Resolveur ), 
		"profiler" => Ok( resoudre_profiler as Resolveur ), 
		"éteindre" => Ok( resoudre_eteindre as Resolveur ), 
		"mesurer" => Ok( resoudre_mesurer as Resolveur ), 
		"vider" => Ok( resoudre_vider as Resolveur ), 
		"sérialiser" => Ok( resoudre_serialiser as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module texte : fonction inconnue" ) ) 
	} 
} 








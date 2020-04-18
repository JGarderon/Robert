//! # Sous-module de résolution "administration"
//! 
//! Ce module gère les fonctions liées à l'administration du processus. Certaines de ces fonctions peuvent être restreintes aux seuls clients authentifiés. 
//! 

	// --- --- --- --- --- --- --- --- --- 
	// (1) Importation des modules internes 
	// --- --- --- --- --- --- --- --- --- 

use std::mem; 
use std::collections::HashMap;
use std::io::BufWriter; 
use std::fs::File; 

	// --- --- --- --- --- --- --- --- --- 
	// (2) Importation des modules du projet 
	// --- --- --- --- --- --- --- --- --- 

use crate::base::{Canal, Valeurs}; 
use crate::resolution::{Contexte, Resolveur, Retour}; 
use crate::grammaire::ArgumentsLocaux; 
use crate::serie::{Serie, Source}; 

	// --- --- --- --- --- --- --- --- --- 
	// (3) Constantes du projet 
	// --- --- --- --- --- --- --- --- --- 

	// --- --- --- --- --- --- --- --- --- 
	// (4) Définition des structures, énumérations et leurs implémentations 
	// --- --- --- --- --- --- --- --- --- 

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

	// --- --- --- --- --- --- --- --- --- 
	// (5) Définition des fonctions 
	// --- --- --- --- --- --- --- --- --- 

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

/// # Fonction de résolution locale "sérialiser les valeurs d'un canal" 
/// 
/// La sérialisation est stockée dans un fichier dédié. La désérialisation est donc possible, notamment en vue d'un arrêt programmé du processus puis de sa relance. 
/// 
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
	let canal = acces_canal!( contexte ); 
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

/// # Fonction de résolution locale "vider un canal" 
/// 
/// Vide de toutes ses valeurs, un canal donné. Si aucune sérialisation n'a été précédemment faite, les données sont perdues. 
/// 
fn resoudre_vider( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	est_authentifie!( contexte ); 
	let nom = if let Some( n ) = arguments.extraire() { 
		n 
	} else { 
		return Retour::creer_str( false, "nom de canal obligatoire" ); 
	}; 
	if nom.len() > 32 { 
		Retour::creer_str( false, "nom de canal trop long (max. 32)" ) 
	} else { 
		let mut canaux = { 
			match contexte.canauxthread.lock() { 
				Ok( canaux ) => canaux, 
				Err( empoisonne ) => empoisonne.into_inner() 
			} 
		}; 
		if let Some( c ) = canaux.liste.get_mut( &nom ) { 
			let mut canal = match c.lock() { 
				Ok( c ) => c, 
				Err( e ) => e.into_inner() 
			}; 
			match &mut canal.liste { 
				Valeurs::Objet( h ) => { 
					h.clear(); 
					Retour::creer_str( true, "base vidée" ) 
				} 
				_ => Retour::creer_str( false, "objet racine incorrect ; le canal semble corrompu" ) 
			} 
		} else { 
			Retour::creer_str( false, "nom de canal inconnu" ) 
		} 
	} 
} 

/// # Fonction de résolution locale "résumer l'ensemble des canaux" 
/// 
/// Donne pour chaque canal, une paire d'informations : son nom, 
/// 
fn resoudre_resumer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
	est_authentifie!( contexte ); 
	if !arguments.est_stop() { 
		return Retour::creer_str( false, "aucun argument autorisé" ); 
	} 
	let canauxthread_local = contexte.canauxthread.clone(); 
	let canaux = { 
		match canauxthread_local.lock() { 
			Ok( canaux ) => canaux, 
			Err( empoisonne ) => empoisonne.into_inner() 
		} 
	}; 
	let nbre = canaux.liste.len(); 
	let mut total = 0; 
	for (nom, canalthread) in canaux.liste.iter() { 
		let canal = match canalthread.lock() { 
			Ok( c ) => c, 
			Err( e ) => e.into_inner() 
		}; 
		let t = canal.mesurer(); 
		contexte.message( 
			&format!( 
				"canal \"{}\" (nbre : {}, taille : {})", 
				nom, 
				match &canal.liste { 
					Valeurs::Objet( o ) => o.len().to_string(), 
					_ => "?".to_string() 
				}, 
				t 
			) 
		); 
		total += t; 
	} 
	Retour::creer(  
		true, 
		format!( 
			"nbre total : {} (taille totale : {})", 
			nbre, 
			total 
		) 
	) 
} 

/// # Fonction de résolution locale - sous-module "administration" 
/// 
/// Permet de retourner la fonction désirée en fonction de l'appel. 
/// 
pub fn resoudre( appel: &str ) -> Result<Resolveur,Retour> { 
	match appel { 
		"authentifier" => Ok( resoudre_authentifier as Resolveur ), 
		"anonymiser" => Ok( resoudre_anonymiser as Resolveur ), 
		"profiler" => Ok( resoudre_profiler as Resolveur ), 
		"éteindre" => Ok( resoudre_eteindre as Resolveur ), 
		"vider" => Ok( resoudre_vider as Resolveur ), 
		"sérialiser" => Ok( resoudre_serialiser as Resolveur ), 
		"résumer" => Ok( resoudre_resumer as Resolveur ), 
		_ => Err( Retour::creer_str( false, "module 'administration' : fonction inconnue" ) ) 
	} 
} 








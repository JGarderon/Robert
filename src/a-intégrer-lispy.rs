

// 			// "ajouter" => resoudre_ajouter as Resolveur, -> vers texte 

// fn resoudre_contenir( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let motif = if let Some( m ) = arguments.extraire() { 
// 		m 
// 	} else { 
// 		return Retour::creer_str( false, "motif de recherche obligatoire" ); 
// 	}; 
// 	let dico = contexte.dico.lock().unwrap(); 
// 	let mut i = 0; 
// 	for (cle, valeur) in dico.liste.iter() { 
// 		match valeur { 
// 			Valeurs::Texte( t ) => { 
// 				if t.contains( &motif ) { 
// 					if let Err(_) = contexte.stream.write( 
// 						format!( 
// 							"\t{}\n", 
// 							cle 
// 						).as_bytes() 
// 					) { 
// 						contexte.stream.flush().unwrap(); 
// 						return Retour::creer_str( false, "erreur lors de l'envoi" ); 
// 					} 
// 					i += 1; 
// 				} 
// 			} 
// 			_ => () 
// 		} 
// 	} 
// 	Retour::creer( true, format!( "stop ({})", i ) ) 
// } 

// fn resoudre_debuter( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let motif = if let Some( m ) = arguments.extraire() { 
// 		m 
// 	} else { 
// 		return Retour::creer_str( false, "motif de recherche obligatoire" ); 
// 	}; 
// 	let dico = contexte.dico.lock().unwrap(); 
// 	let mut i = 0; 
// 	for (cle, valeur) in dico.liste.iter() { 
// 		match valeur { 
// 			Valeurs::Texte( t ) => { 
// 				if t.starts_with( &motif ) { 
// 					if let Err(_) = contexte.stream.write( 
// 						format!( 
// 							"\t{}\n", 
// 							cle 
// 						).as_bytes() 
// 					) { 
// 						contexte.stream.flush().unwrap(); 
// 						return Retour::creer_str( false, "erreur lors de l'envoi" ); 
// 					} 
// 					i += 1; 
// 				} 
// 			} 
// 			_ => () 
// 		} 
// 	} 
// 	Retour::creer( true, format!( "stop ({})", i ) ) 
// } 

// fn resoudre_terminer( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let motif = if let Some( m ) = arguments.extraire() { 
// 		m 
// 	} else { 
// 		return Retour::creer_str( false, "motif de recherche obligatoire" ); 
// 	}; 
// 	let dico = contexte.dico.lock().unwrap(); 
// 	let mut i = 0; 
// 	for (cle, valeur) in dico.liste.iter() { 
// 		match valeur { 
// 			Valeurs::Texte( t ) => { 
// 				if t.ends_with( &motif ) { 
// 					if let Err(_) = contexte.stream.write( 
// 						format!( 
// 							"\t{}\n", 
// 							cle 
// 						).as_bytes() 
// 					) { 
// 						contexte.stream.flush().unwrap(); 
// 						return Retour::creer_str( false, "erreur lors de l'envoi" ); 
// 					} 
// 					i += 1; 
// 				} 
// 			} 
// 			_ => () 
// 		} 
// 	} 
// 	Retour::creer( true, format!( "stop ({})", i ) ) 
// } 

// fn resoudre_remplacer_un( contexte: &mut Contexte, mut arguments: ArgumentsLocaux ) -> Retour { 
// 	let cle = if let Some( c ) = arguments.extraire() { 
// 		c 
// 	} else { 
// 		return Retour::creer_str( false, "clé obligatoire" ); 
// 	}; 
// 	let recherche = if let Some( m ) = arguments.extraire() { 
// 		m 
// 	} else { 
// 		return Retour::creer_str( false, "motif de recherche obligatoire" ); 
// 	}; 
// 	let remplacement = if let Some( r ) = arguments.extraire() { 
// 		r 
// 	} else { 
// 		return Retour::creer_str( false, "motif de remplacement obligatoire" ); 
// 	}; 
// 	let nbre_max =  arguments.extraire(); 
// 	let mut dico = contexte.dico.lock().unwrap(); 
// 	let valeurs = &mut dico.liste; 
// 	if let Some( v ) = valeurs.get_mut( &cle ) { 
// 		match v { 
// 			Valeurs::Texte( t ) => { 
// 				if let Some( t_n ) = nbre_max { 
// 					if let Ok( n ) = t_n.parse() { 
// 						*t = t.replacen( &recherche, &remplacement, n ); 
// 						Retour::creer_str( true, "remplacement(s) effectué(s)" ) 
// 					} else { 
// 						Retour::creer_str( true, "nbre de remplacements maximum invalide" ) 
// 					} 
// 				} else { 
// 					*t = t.replace( &recherche, &remplacement ); 
// 					Retour::creer_str( true, "remplacement(s) effectué(s)" ) 
// 				} 
// 			} 
// 			_ => Retour::creer_str( false, "la valeur n'est pas un texte" ) 
// 		} 
// 	} else { 
// 		Retour::creer_str( false, "erreur lors de l'envoi" ) 
// 	} 
// } 
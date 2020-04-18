var searchIndex={};
searchIndex["projet_robert"] = {"doc":"Robert est un logiciel type \"Redis-Like\" : un système de…","i":[[5,"lancement_service","projet_robert","Fonction permettant de lancer le service d'écoute (socket…",null,[[["str"]],[["str"],["result",["str"]]]]],[5,"main","","Fonction principal du programme ",null,[[]]],[0,"configuration","","Module de configuration",null,null],[17,"DEBUG","projet_robert::configuration","Définit sur le mode \"débug\" est actif (renvoi sur la…",null,null],[17,"CANAL_NOM_DEFAUT","","Nom du dictionnaire par défaut, créé par le programme et…",null,null],[17,"TAILLE_LIGNE_MAX","","Taille maximale admissible par ligne reçue sur un socket.…",null,null],[17,"TAILLE_TEXTE_MAX","","Taille maximale admissible pour le texte contenu dans les…",null,null],[17,"NBRE_MAX_VALEURS","","Nbre maximum admissible de valeurs pour chaque canal…",null,null],[17,"NBRE_MAX_CANAUX","","Nbre maximum admissible de canaux dans le processus en…",null,null],[17,"PROFILS_SOURCE","","Chemin vers le fichier des profils ",null,null],[17,"PROFILS_PSEUDO_DEFAUT","","Nom par défaut ('pseudo') d'un client TCP non-authentifié ",null,null],[0,"contexte","projet_robert","",null,null],[3,"Contexte","projet_robert::contexte","La structure 'Contexte' permet de rassembler dans un objet…",null,null],[12,"service_ecoute","","Ce champ permet de récupérer un clone de l'objet en écoute…",0,null],[12,"service_poursuite","","Ce champ lorsqu'il est à \"faux\", permet d'interrompre la…",0,null],[12,"poursuivre","","Ce champ lorsqu'il est à \"faux\", permet d'interrompre la…",0,null],[12,"canalthread","","Ce champ contient le nécessaire pour accéder au…",0,null],[12,"canauxthread","","Ce champ contient le nécessaire pour accéder au…",0,null],[12,"profil","","",0,null],[12,"stream","","Ce champ contient l'objet socket, librement clonable. ",0,null],[11,"ecrire","","",0,[[["str"],["self"],["bool"]],["bool"]]],[11,"message","","",0,[[["str"],["self"]],["bool"]]],[11,"erreur","","",0,[[["str"],["self"]],["bool"]]],[0,"canal","projet_robert","",null,null],[3,"Canal","projet_robert::canal","Un canal se constitue de trois principaux éléments : son…",null,null],[12,"nom","","",1,null],[12,"liste","","",1,null],[12,"souscripteurs","","",1,null],[3,"Canaux","","",null,null],[12,"liste","","",2,null],[5,"creer_racine","","",null,[[["str"]]]],[6,"CanalThread","","",null,null],[6,"CanauxThread","","",null,null],[11,"resoudre","","",1,[[["self"],["f"]],["retour"]]],[0,"profil","projet_robert","",null,null],[3,"Profil","projet_robert::profil","",null,null],[12,"identifie","","",3,null],[12,"pseudo","","",3,null],[4,"ProfilPseudo","","",null,null],[13,"Statique","","",4,null],[13,"Dynamique","","",4,null],[11,"creer","","",3,[[],["self"]]],[11,"authentifier","","",3,[[["str"],["self"]],[["result",["bool","str"]],["str"],["bool"]]]],[11,"anonymiser","","",3,[[["self"]]]],[11,"est_authentifie","","",3,[[["self"]],["bool"]]],[0,"resolution","projet_robert","",null,null],[3,"Retour","projet_robert::resolution","Structure définissant un 'Retour', afin d'uniformiser les…",null,null],[12,"etat","","Permet de signaler au thread maître, lors du renvoi vers…",5,null],[12,"message","","Contient le message qui doit être renvoyé au client. ",5,null],[4,"RetourType","","Les retours peuvent être soit un texte statique (&'static…",null,null],[13,"Statique","","Est de type &'static str ",6,null],[13,"Dynamique","","Est de type String ",6,null],[5,"resoudre_stop","","Fonction de résolution : arrête la boucle principale du…",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_definir","","Fonction de résolution \"définir une nouvelle valeur\" Elle…",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_obtenir","","Fonction de résolution \"obtenir une valeur existante\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_supprimer","","Fonction de résolution \"supprimer une valeur existante\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_tester","","Fonction de résolution \"tester l'existence d'un chemin\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_lister","","Fonction de résolution \"lister toutes les valeurs d'un…",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_alterer","","Fonction de résolution \"altérer une valeur existante\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre","","Fonction de résolution centrale Cette fonction est appelée…",null,[[["contexte"],["str"]],["retour"]]],[0,"resoudre_numerique","","",null,null],[5,"resoudre_incrementer","projet_robert::resolution::resoudre_numerique","Fonction de résolution locale \"incrémenter une valeur…",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre","","",null,[[["str"]],[["retour"],["result",["retour"]]]]],[0,"resoudre_texte","projet_robert::resolution","",null,null],[5,"resoudre_ajouter","projet_robert::resolution::resoudre_texte","Fonction de résolution locale \"ajouter du texte\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_compter","","Fonction de résolution locale \"compter le texte (octets +…",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_decouper","","Fonction de résolution locale \"découper du texte…",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre","","",null,[[["str"]],[["retour"],["result",["retour"]]]]],[0,"resoudre_canal","projet_robert::resolution","",null,null],[5,"resoudre_creer","projet_robert::resolution::resoudre_canal","Fonction de résolution locale \"créer un nouveau canal\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_supprimer","","Fonction de résolution locale \"supprimer un canal existant\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_tester","","Fonction de résolution locale \"tester l'existence d'un…",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_capturer","","Fonction de résolution locale \"capturer un nouveau canal\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_renommer","","Fonction de résolution locale \"renommer un canal existant\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_souscrire","","",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_emettre","","",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre","","",null,[[["str"]],[["retour"],["result",["retour"]]]]],[0,"resoudre_administration","projet_robert::resolution","Sous-module de résolution \"administration\"",null,null],[5,"resoudre_authentifier","projet_robert::resolution::resoudre_administration","Fonction de résolution locale \"authentifier son profil\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_anonymiser","","Fonction de résolution locale \"anonymiser son profil\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_profiler","","Fonction de résolution locale \"consulter son profil\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_eteindre","","Fonction de résolution locale \"éteindre le programme\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_serialiser","","Fonction de résolution locale \"sérialiser les valeurs d'un…",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_vider","","Fonction de résolution locale \"vider un canal\"",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre_resumer","","Fonction de résolution locale \"résumer l'ensemble des…",null,[[["contexte"],["argumentslocaux"]],["retour"]]],[5,"resoudre","","Fonction de résolution locale - sous-module \"administration\"",null,[[["str"]],[["retour"],["result",["retour"]]]]],[8,"Mesure","","",null,null],[10,"mesurer","","",7,[[["self"]],["usize"]]],[6,"Resolveur","projet_robert::resolution","Un type spécifique au projet : le type 'Résolveur' est la…",null,null],[11,"creer","","Créer un retour \"dynamique\", c'est-à-dire un String. ",5,[[["string"],["bool"]],["self"]]],[11,"creer_str","","Créer un retour \"statique\", c'est-à-dire un &'static str. ",5,[[["str"],["bool"]],["self"]]],[0,"grammaire","projet_robert","Module grammatical",null,null],[3,"ArgumentsLocaux","projet_robert::grammaire","",null,null],[12,"source","","",8,null],[12,"position","","",8,null],[4,"ArgumentsLocauxEtat","","",null,null],[13,"Suivant","","",9,null],[13,"Stop","","",9,null],[13,"Erreur","","",9,null],[4,"ExtractionLigne","","",null,null],[13,"Commande","","",10,null],[13,"Erreur","","",10,null],[13,"Stop","","",10,null],[5,"extraire_ligne","","",null,[[["bytes"]],["extractionligne"]]],[5,"extraction_commande","","",null,[[["str"]]]],[5,"chemin_extraire","","",null,[[["str"]],[["str"],["result",["vec","str"]],["vec",["str"]]]]],[11,"trim","","",8,[[["self"]],[["option",["usize"]],["usize"]]]],[11,"suivant","","",8,[[["self"]],["argumentslocauxetat"]]],[11,"extraire","","",8,[[["self"]],[["string"],["option",["string"]]]]],[11,"est_stop","","",8,[[["self"]],["bool"]]],[11,"tous","","",8,[[["self"]],[["result",["vec","str"]],["vec",["string"]],["str"]]]],[0,"serie","projet_robert","Module \"Série\"",null,null],[3,"Source","projet_robert::serie","",null,null],[12,"fichier","","",11,null],[8,"Serie","","",null,null],[10,"serialiser","","",12,[[["source"],["self"]],[["option",["usize"]],["usize"]]]],[11,"ecrire","","",11,[[["self"],["u8"]],[["option",["usize"]],["usize"]]]],[0,"client","projet_robert","Module des clients TCP",null,null],[5,"recevoir","projet_robert::client","Fonction recevant un client et le traitant, par le biais…",null,[[["contexte"]]]],[0,"valeur","projet_robert","",null,null],[4,"Valeurs","projet_robert::valeur","",null,null],[13,"Boolean","","",13,null],[13,"Relatif","","",13,null],[13,"Flottant","","",13,null],[13,"Texte","","",13,null],[13,"Objet","","",13,null],[11,"creer_valeur","","",13,[[["self"],["string"],["option",["string"]]],["retour"]]],[11,"resoudre","","",13,[[["self"],["f"]],["retour"]]],[11,"alterer","","",13,[[["str"],["self"]],["bool"]]],[11,"from","projet_robert::contexte","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"from","projet_robert::canal","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[],["u"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"try_into","","",2,[[],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"type_id","","",2,[[["self"]],["typeid"]]],[11,"from","projet_robert::profil","",3,[[["t"]],["t"]]],[11,"into","","",3,[[],["u"]]],[11,"to_string","","",3,[[["self"]],["string"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"try_into","","",3,[[],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"type_id","","",3,[[["self"]],["typeid"]]],[11,"from","","",4,[[["t"]],["t"]]],[11,"into","","",4,[[],["u"]]],[11,"to_string","","",4,[[["self"]],["string"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"try_into","","",4,[[],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"type_id","","",4,[[["self"]],["typeid"]]],[11,"from","projet_robert::resolution","",5,[[["t"]],["t"]]],[11,"into","","",5,[[],["u"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"try_into","","",5,[[],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"type_id","","",5,[[["self"]],["typeid"]]],[11,"from","","",6,[[["t"]],["t"]]],[11,"into","","",6,[[],["u"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"try_into","","",6,[[],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"type_id","","",6,[[["self"]],["typeid"]]],[11,"from","projet_robert::grammaire","",8,[[["t"]],["t"]]],[11,"into","","",8,[[],["u"]]],[11,"try_from","","",8,[[["u"]],["result"]]],[11,"try_into","","",8,[[],["result"]]],[11,"borrow","","",8,[[["self"]],["t"]]],[11,"borrow_mut","","",8,[[["self"]],["t"]]],[11,"type_id","","",8,[[["self"]],["typeid"]]],[11,"from","","",9,[[["t"]],["t"]]],[11,"into","","",9,[[],["u"]]],[11,"try_from","","",9,[[["u"]],["result"]]],[11,"try_into","","",9,[[],["result"]]],[11,"borrow","","",9,[[["self"]],["t"]]],[11,"borrow_mut","","",9,[[["self"]],["t"]]],[11,"type_id","","",9,[[["self"]],["typeid"]]],[11,"from","","",10,[[["t"]],["t"]]],[11,"into","","",10,[[],["u"]]],[11,"try_from","","",10,[[["u"]],["result"]]],[11,"try_into","","",10,[[],["result"]]],[11,"borrow","","",10,[[["self"]],["t"]]],[11,"borrow_mut","","",10,[[["self"]],["t"]]],[11,"type_id","","",10,[[["self"]],["typeid"]]],[11,"from","projet_robert::serie","",11,[[["t"]],["t"]]],[11,"into","","",11,[[],["u"]]],[11,"try_from","","",11,[[["u"]],["result"]]],[11,"try_into","","",11,[[],["result"]]],[11,"borrow","","",11,[[["self"]],["t"]]],[11,"borrow_mut","","",11,[[["self"]],["t"]]],[11,"type_id","","",11,[[["self"]],["typeid"]]],[11,"from","projet_robert::valeur","",13,[[["t"]],["t"]]],[11,"into","","",13,[[],["u"]]],[11,"try_from","","",13,[[["u"]],["result"]]],[11,"try_into","","",13,[[],["result"]]],[11,"borrow","","",13,[[["self"]],["t"]]],[11,"borrow_mut","","",13,[[["self"]],["t"]]],[11,"type_id","","",13,[[["self"]],["typeid"]]],[11,"mesurer","","",13,[[["self"]],["usize"]]],[11,"mesurer","projet_robert::canal","",1,[[["self"]],["usize"]]],[11,"serialiser","projet_robert::valeur","",13,[[["source"],["self"]],[["option",["usize"]],["usize"]]]],[11,"drop","projet_robert::canal","",1,[[["self"]]]],[11,"drop","projet_robert::valeur","",13,[[["self"]]]],[11,"fmt","projet_robert::canal","",1,[[["formatter"],["self"]],["result"]]],[11,"fmt","projet_robert::grammaire","",9,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",8,[[["formatter"],["self"]],["result"]]],[11,"fmt","projet_robert::valeur","",13,[[["formatter"],["self"]],["result"]]],[11,"fmt","projet_robert::profil","",4,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",3,[[["formatter"],["self"]],["result"]]]],"p":[[3,"Contexte"],[3,"Canal"],[3,"Canaux"],[3,"Profil"],[4,"ProfilPseudo"],[3,"Retour"],[4,"RetourType"],[8,"Mesure"],[3,"ArgumentsLocaux"],[4,"ArgumentsLocauxEtat"],[4,"ExtractionLigne"],[3,"Source"],[8,"Serie"],[4,"Valeurs"]]};
addSearchOptions(searchIndex);initSearch(searchIndex);
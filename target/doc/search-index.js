var searchIndex={};
searchIndex["projet_robert"] = {"doc":"Robert est un logiciel type \"Redis-Like\" : un système de…","i":[[5,"recevoir","projet_robert","Fonction recevant un client et le traitant, par le biais…",null,[[["contexte"]]]],[5,"lancement_service","","Fonction permettant de lancer le service d'écoute (socket…",null,[[["str"]],[["str"],["result",["str"]]]]],[5,"main","","Ai-je vraiment besoin de documenter à quoi sert cette…",null,[[]]],[0,"resolution","","",null,null],[3,"Contexte","projet_robert::resolution","La structure 'Contexte' permet de rassembler dans un objet…",null,null],[12,"service","","Ce champ lorsqu'il est à \"faux\" (AtomicBool), permet…",0,null],[12,"poursuivre","","Ce champ lorsqu'il est à \"faux\", permet d'interrompre la…",0,null],[12,"canalthread","","Ce champ contient le nécessaire pour accéder au…",0,null],[12,"canauxthread","","Ce champ contient le nécessaire pour accéder au…",0,null],[12,"stream","","Ce champ contient l'objet socket, librement clonable. ",0,null],[3,"Retour","","Structure définissant un 'Retour', afin d'uniformiser les…",null,null],[12,"etat","","Permet de signaler au thread maître, lors du renvoi vers…",1,null],[12,"message","","Contient le message qui doit être renvoyé au client. ",1,null],[4,"RetourType","","Les retours peuvent être soit un texte statique (&'static…",null,null],[13,"Statique","","Est de type &'static str ",2,null],[13,"Dynamique","","Est de type String ",2,null],[5,"resoudre_stop","","Fonction de résolution : arrête la boucle principale du…",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_definir","","Fonction de résolution \"définir une nouvelle valeur\" Elle…",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_obtenir","","Fonction de résolution \"obtenir une valeur existante\"",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_supprimer","","Fonction de résolution \"supprimer une valeur existante\"",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_tester","","Fonction de résolution \"tester l'existence d'un chemin\"",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_lister","","Fonction de résolution \"lister toutes les valeurs d'un…",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_alterer","","Fonction de résolution \"altérer une valeur existante\"",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre","","Fonction de résolution centrale Cette fonction est appelée…",null,[[["str"],["contexte"]],["retour"]]],[0,"resoudre_numerique","","",null,null],[5,"resoudre_incrementer","projet_robert::resolution::resoudre_numerique","Fonction de résolution locale \"incrémenter une valeur…",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre","","",null,[[["str"]],[["result",["retour"]],["retour"]]]],[0,"resoudre_texte","projet_robert::resolution","",null,null],[5,"resoudre_ajouter","projet_robert::resolution::resoudre_texte","Fonction de résolution locale \"ajouter du texte\"",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_compter","","Fonction de résolution locale \"compter le texte (octets +…",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_decouper","","Fonction de résolution locale \"découper du texte…",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre","","",null,[[["str"]],[["result",["retour"]],["retour"]]]],[0,"resoudre_canal","projet_robert::resolution","",null,null],[5,"resoudre_creer","projet_robert::resolution::resoudre_canal","Fonction de résolution locale \"créer un nouveau canal\"",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_supprimer","","Fonction de résolution locale \"supprimer un canal existant\"",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_tester","","Fonction de résolution locale \"tester l'existence d'un…",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_capturer","","Fonction de résolution locale \"capturer un nouveau canal\"",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_renommer","","Fonction de résolution locale \"renommer un canal existant\"",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_souscrire","","",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_emettre","","",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre","","",null,[[["str"]],[["result",["retour"]],["retour"]]]],[0,"resoudre_administration","projet_robert::resolution","",null,null],[5,"resoudre_eteindre","projet_robert::resolution::resoudre_administration","",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_mesurer","","",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre_vider","","",null,[[["argumentslocaux"],["contexte"]],["retour"]]],[5,"resoudre","","",null,[[["str"]],[["result",["retour"]],["retour"]]]],[8,"Mesure","","",null,null],[10,"mesurer","","",3,[[["self"]],["usize"]]],[6,"Resolveur","projet_robert::resolution","Un type spécifique au projet : le type 'Résolveur' est la…",null,null],[11,"vers_bytes","","Rassemble la valeur vers l'équivalent d'un slice de Bytes,…",2,[[["self"]]]],[11,"creer","","Créer un retour \"dynamique\", c'est-à-dire un String. ",1,[[["string"],["bool"]],["self"]]],[11,"creer_str","","Créer un retour \"statique\", c'est-à-dire un &'static str. ",1,[[["str"],["bool"]],["self"]]],[0,"base","projet_robert","",null,null],[3,"Canal","projet_robert::base","Un canal se constitue de trois principaux éléments : son…",null,null],[12,"nom","","",4,null],[12,"liste","","",4,null],[12,"souscripteurs","","",4,null],[3,"Canaux","","",null,null],[12,"liste","","",5,null],[4,"Valeurs","","",null,null],[13,"Boolean","","",6,null],[13,"Relatif","","",6,null],[13,"Flottant","","",6,null],[13,"Texte","","",6,null],[13,"Objet","","",6,null],[5,"creer_racine","","",null,[[["str"]]]],[6,"CanalThread","","",null,null],[6,"CanauxThread","","",null,null],[11,"resoudre","","",4,[[["self"],["f"]],["retour"]]],[11,"creer_valeur","","",6,[[["self"],["string"],["option",["string"]]],["retour"]]],[11,"resoudre","","",6,[[["self"],["f"]],["retour"]]],[11,"alterer","","",6,[[["str"],["self"]],["bool"]]],[0,"grammaire","projet_robert","Module grammatical",null,null],[3,"ArgumentsLocaux","projet_robert::grammaire","",null,null],[12,"source","","",7,null],[12,"position","","",7,null],[4,"ArgumentsLocauxEtat","","",null,null],[13,"Suivant","","",8,null],[13,"Stop","","",8,null],[13,"Erreur","","",8,null],[4,"ExtractionLigne","","",null,null],[13,"Commande","","",9,null],[13,"Erreur","","",9,null],[13,"Stop","","",9,null],[5,"extraire_ligne","","",null,[[["bytes"]],["extractionligne"]]],[5,"extraction_commande","","",null,[[["str"]]]],[5,"chemin_extraire","","",null,[[["str"]],[["str"],["result",["vec","str"]],["vec",["str"]]]]],[11,"trim","","",7,[[["self"]],[["option",["usize"]],["usize"]]]],[11,"suivant","","",7,[[["self"]],["argumentslocauxetat"]]],[11,"extraire","","",7,[[["self"]],[["string"],["option",["string"]]]]],[11,"est_stop","","",7,[[["self"]],["bool"]]],[17,"DEBUG","projet_robert","Définit sur le mode \"débug\" est actif (renvoi sur la…",null,null],[17,"CANAL_NOM_DEFAUT","","Nom du dictionnaire par défaut, créé par le programme et…",null,null],[17,"TAILLE_LIGNE_MAX","","Taille maximale admissible par ligne reçue sur un socket.…",null,null],[17,"TAILLE_TEXTE_MAX","","Taille maximale admissible pour le texte contenu dans les…",null,null],[17,"NBRE_MAX_VALEURS","","Nbre maximum admissible de valeurs pour chaque canal…",null,null],[17,"NBRE_MAX_CANAUX","","Nbre maximum admissible de canaux dans le processus en…",null,null],[11,"from","projet_robert::resolution","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[],["u"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"try_into","","",2,[[],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"type_id","","",2,[[["self"]],["typeid"]]],[11,"from","projet_robert::base","",4,[[["t"]],["t"]]],[11,"into","","",4,[[],["u"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"try_into","","",4,[[],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"type_id","","",4,[[["self"]],["typeid"]]],[11,"from","","",5,[[["t"]],["t"]]],[11,"into","","",5,[[],["u"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"try_into","","",5,[[],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"type_id","","",5,[[["self"]],["typeid"]]],[11,"from","","",6,[[["t"]],["t"]]],[11,"into","","",6,[[],["u"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"try_into","","",6,[[],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"type_id","","",6,[[["self"]],["typeid"]]],[11,"from","projet_robert::grammaire","",7,[[["t"]],["t"]]],[11,"into","","",7,[[],["u"]]],[11,"try_from","","",7,[[["u"]],["result"]]],[11,"try_into","","",7,[[],["result"]]],[11,"borrow","","",7,[[["self"]],["t"]]],[11,"borrow_mut","","",7,[[["self"]],["t"]]],[11,"type_id","","",7,[[["self"]],["typeid"]]],[11,"from","","",8,[[["t"]],["t"]]],[11,"into","","",8,[[],["u"]]],[11,"try_from","","",8,[[["u"]],["result"]]],[11,"try_into","","",8,[[],["result"]]],[11,"borrow","","",8,[[["self"]],["t"]]],[11,"borrow_mut","","",8,[[["self"]],["t"]]],[11,"type_id","","",8,[[["self"]],["typeid"]]],[11,"from","","",9,[[["t"]],["t"]]],[11,"into","","",9,[[],["u"]]],[11,"try_from","","",9,[[["u"]],["result"]]],[11,"try_into","","",9,[[],["result"]]],[11,"borrow","","",9,[[["self"]],["t"]]],[11,"borrow_mut","","",9,[[["self"]],["t"]]],[11,"type_id","","",9,[[["self"]],["typeid"]]],[11,"mesurer","projet_robert::base","",6,[[["self"]],["usize"]]],[11,"mesurer","","",4,[[["self"]],["usize"]]],[11,"drop","","",4,[[["self"]]]],[11,"drop","","",6,[[["self"]]]],[11,"fmt","","",4,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",6,[[["formatter"],["self"]],["result"]]],[11,"fmt","projet_robert::grammaire","",8,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",7,[[["formatter"],["self"]],["result"]]]],"p":[[3,"Contexte"],[3,"Retour"],[4,"RetourType"],[8,"Mesure"],[3,"Canal"],[3,"Canaux"],[4,"Valeurs"],[3,"ArgumentsLocaux"],[4,"ArgumentsLocauxEtat"],[4,"ExtractionLigne"]]};
addSearchOptions(searchIndex);initSearch(searchIndex);
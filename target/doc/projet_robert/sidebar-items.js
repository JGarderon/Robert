initSidebarItems({"constant":[["DEBUG","Définit sur le mode \"débug\" est actif (renvoi sur la console par défaut). "],["DICO_NOM_DEFAUT","Nom du dictionnaire par défaut, créé par le programme et qui sert aussi de canal par défaut. Il ne peut et ne doit être jamais supprimé lors de l'exécution des requêtes des utilisateurs. "],["TAILLE_LIGNE_MAX","Taille maximale admissible par ligne reçue sur un socket. Cette taille fournie donc la taille maximum admissible des requêtes pour le reste du programme. "],["TAILLE_TEXTE_MAX","Taille maximale admissible pour le texte contenu dans les dictionnaires. "]],"fn":[["lancement_service","Fonction permettant de lancer le service d'écoute (socket TCP). A l'avenir, cette fonction retournerait un objet JoinHandle permettant au service d'agir dans un thread dédié et ne pas boucler la fonction 'main'.  Chaque nouveau client est envoyé dans un nouveau thread, avec un objet \"Contexte\", qui porte les informations essentielles liées au socket TCP en cours. Les requêtes sont gérées par le thread du client. "],["main",""],["recevoir","Fonction recevant un client et le traitant. Principalement une boucle qui reçoit sur texte dans un tampon, l'examine rapidement avec les outils du module \"grammaire\", et résoud de la requête. "]],"mod":[["base",""],["grammaire","Module 'grammaire'  Ce module permet la gestion de la partie grammaticale (syntaxique) et un partie sémantique des requêtes reçues. "],["resolution",""]]});
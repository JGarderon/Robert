initSidebarItems({"enum":[["RetourType","Les retours peuvent être soit un texte statique (&'static str) - c'est-à-dire invariable et intégré au directement dans le code source du programme (efficacité), soit un texte généré par la fonction de résolution (String) - c'est-à-dire variable."]],"fn":[["resoudre","Fonction de résolution centrale Cette fonction est appelée par le thread du client, et redirigera l'appel vers la bonne fonction de résolution, en fonction du module souhaité. Les fonctions génériques, définies dans ce présent module, y sont directement incorporées. Les fonctions spéficiques, qui se retrouvent dans les sous-modules, ont une fonction de résolution secondaire : cette fonction est principale car elle appelera la fonction de résolution secondaire, celle du sous-module."],["resoudre_alterer","Fonction de résolution \"altérer une valeur existante\""],["resoudre_definir","Fonction de résolution \"définir une nouvelle valeur\" Elle définit une nouvelle valeur stockée dans le canal (sans la diffuser). Au moins deux arguments doivent être fournis : la clé (ou un chemin comprenant la clé) ainsi qu'une valeur quelconque. Cette valeur peut être altérer dans un format particulier grâce à un troisième argument optionnelle qui représente son type. Si l'altération est impossible, l'ajout n'est pas effectuée. Si aucun type de valeur n'est fourni, c'est le texte qui est le type par défaut."],["resoudre_lister","Fonction de résolution \"lister toutes les valeurs d'un canal\""],["resoudre_obtenir","Fonction de résolution \"obtenir une valeur existante\""],["resoudre_stop","Fonction de résolution : arrête la boucle principale du thread du client. Ne prend aucun argument (obligatoire)."],["resoudre_supprimer","Fonction de résolution \"supprimer une valeur existante\""],["resoudre_tester","Fonction de résolution \"tester l'existence d'un chemin\""]],"mod":[["resoudre_administration","Sous-module de résolution \"administration\""],["resoudre_canal","Sous-module de résolution \"canal\""],["resoudre_numerique",""],["resoudre_script",""],["resoudre_texte",""]],"struct":[["Retour","Structure définissant un 'Retour', afin d'uniformiser les messages à destination du client et l'état de résolution."]],"type":[["Resolveur","Un type spécifique au projet : le type 'Résolveur' est la signature d'une fonction de résolution, quelque soit le module de résolution. Elle prend deux paramètres : le contexte du socket ainsi qu'un objet permettant de récupèrer à la demande les arguments dits 'locaux' (propre à une requête). La fonction renvoie un objet \"retour\", qui sera transmis au client via une série d'octets écrite sur le socket. La définition de cette signature a pour principal but de soulager les signatures dans d'autres fonctions de résolution."]]});
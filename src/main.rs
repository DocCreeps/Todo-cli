//Un programme pour gérer une liste de tâches à faire
//Il doit pouvoir ajouter, lister et marquer des tâches comme faites
use std::collections::HashMap;

fn main() {
    // Récupère l'action à effectuer depuis les arguments de la ligne de commande
    let action = std::env::args().nth(1).expect("Veuillez fournir une action");
    // Récupère l'élément associé à l'action depuis les arguments de la ligne de commande
    let item = std::env::args().nth(2).expect("Veuillez fournir un élément");

    // Initialise une instance de Todo et gestion des erreurs si l'initialisation échoue
    let mut todo = Todo::new().expect("L'initialisation de la base de données a échoué");

    // Vérifie l'action et effectue les opérations correspondantes
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Tâche enregistrée"),
            Err(why) => println!("Une erreur s'est produite : {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' n'est pas présent dans la liste", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Tâche enregistrée"),
                Err(why) => println!("Une erreur s'est produite : {}", why),
            },
        }
    }
}

struct Todo {
    // Utilise HashMap intégré à Rust pour stocker des paires clé-valeur
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        // Ouvre le fichier "db.json" en écriture, création et lecture
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        // Tente de désérialiser le contenu du fichier en HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            // Gère le cas où le fichier est vide en initialisant une nouvelle HashMap
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            // Gère les autres erreurs de désérialisation de manière paniquante
            Err(e) => panic!("Une erreur s'est produite : {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        // Insère un nouvel élément dans notre HashMap, l'état actif est défini par défaut à vrai
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // Ouvre le fichier "db.json" en écriture, création et troncature
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        // Sérialise et écrit joliment la HashMap dans le fichier
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        // Marque l'élément correspondant comme complet s'il existe dans la HashMap
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

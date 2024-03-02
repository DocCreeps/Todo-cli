//Un programme pour gérer une liste de tâches à faire
//Il doit pouvoir ajouter, lister et marquer des tâches comme faites

use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;


struct Todo {
    //On utilise une HashMap pour stocker les tâches
    map: HashMap<String, bool>,
}

impl Todo {
    //Une méthode pour ajouter une tâche à la liste
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    //Une méthode pour stocker les tâches dans un fichier
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }


    fn new() -> Result<Todo, std::io::Error> {
        // Ouvre le fichier "db.txt" en mode écriture, création et lecture
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        // Initialise une chaîne de caractères pour stocker le contenu du fichier
        let mut content = String::new();

        // Lit le contenu du fichier dans la chaîne de caractères
        f.read_to_string(&mut content)?;

        // Transforme le contenu en une HashMap<String, bool>
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();

        // Retourne un résultat contenant la structure Todo avec la HashMap
        Ok(Todo { map })
    }


}
fn main() {
    let mut todo = Todo::new().expect("Initialisation de la db échoué");
    let action = std::env::args().nth(1).expect("Renseignez une action");
    let item = std::env::args().nth(2).expect("Renseignez un item");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo sauvegarder"),
            Err(why) => println!("Une erreur à été rencontrer: {}", why),
        }
    }

}

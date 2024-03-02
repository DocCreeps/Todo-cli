//Un programme pour gérer une liste de tâches à faire
//Il doit pouvoir ajouter, lister et marquer des tâches comme faites

//On utilise une HashMap pour stocker les tâches
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;


struct Todo {
    //Un champ pour stocker les tâches
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
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
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

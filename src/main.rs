use std::{process::{Command, Output}, fs::File, io::Write};
use chrono::Local;
use tokio::task;

//définition d'un type pour raccourcir les signatures de fonction
//Result est un type de structure qui accepte deux élements. 
// - un résultat de n'importe quel type (d'où le T qui indique un type "générique")
// - une erreur
//Le type Result propose différentes fonctions pour traiter soit le résultat soit l'erreur
pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;


#[tokio::main] //tokio est le nom d'une librairie qui permet la programmation asynchrone
async fn main() -> Result<()> {
    let path = "../long_script";
    let script_name = "long_script";

    let commands = vec![
        (script_name, "1"),
        (script_name, "-1"),        
        (script_name, "3"),
    ];

    //lancement en parallèle des scripts
    let tasks = commands.into_iter()
        .map(|c| spawn_task(path, c))
        .collect::<Vec<_>>();

    //attente de la fin de toutes les tâches lancées
    for task in tasks {
        task.await.expect("Erreur lors de l'attente de la tâche");
    }

    println!("tout est terminé");

    Ok(())
}

fn spawn_task(path: &'static str, command: (&'static str, &'static str)) -> task::JoinHandle<()> {
    let (script, arg) = command;
    
    //lancement d'une tache sur un thread différent du thread principal
    //mots clé:
    //- async indique une méthode asynchrone
    //- move indique que l'on donne les droits d'accès mémoire exclusif à la fonction dans le spawn
    //cette notion d'ownership (possession ?) est l'une règle de gestion spécifique à Rust qui lui permet
    //de ne pas nécessiter de garbage collecteur, et pourtant s'assurer qu'il n'y a pas de fuite mémoire
    //cf https://jimskapt.github.io/rust-book-fr
    task::spawn(async move {
        let script_path = format!("{path}/{script}");

        let output = Command::new(script_path)
            .arg(arg)
            .output()
            .unwrap_or_else(|_| panic!("Erreur lors de l'appel de la commande {}", script));

        let _ = save_result(output, script);
    })
}

fn save_result(output: Output, script_name: &str) -> Result<bool> {
    let now = Local::now().timestamp_micros();
    let filename = format!("{script_name}-{now}.txt");    
    //le ? sur un Result permet de récupérer la structure résultat s'il n'y avait pas d'erreur
    //s'il y avait un erreur, l'exécution s'arrête là et on retourne à la méthode appelante
    //avec une Error.
    //Pour cela la fonction courante doit forcément retourner un objet Result
    let mut file = File::create(filename)?;    
dbg!(output.status);        
    if output.status.success() { 
        file.write_all(&output.stdout)?;
    } else {
        let erreur = format!("Erreur à l'exécution de la commande {script_name}");
        file.write_all(erreur.as_bytes())?;
    }
    //pas besoin de fermer le fichier, car en sortie de fonction,
    //la resource File est détruite automatiquement (fichier fermer...)
    //grace au principe de possession

    Ok(true)
}
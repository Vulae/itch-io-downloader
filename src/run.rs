use std::{path::PathBuf, error::Error};

use tokio::{fs, process::Command};




static SEARCH_BLACKLIST: &'static [&'static str] = &[
    "UnityCrashHandler64.exe"
];



fn search_exe(path: PathBuf) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let mut queue: Vec<PathBuf> = vec![ path ];

    while queue.len() > 0 {
        let path = queue.remove(0);

        if SEARCH_BLACKLIST.iter().any(|blacklisted| {
            path.to_str().unwrap().ends_with(blacklisted)
        }) {
            continue;
        }

        if path.is_dir() {
            for entry in path.read_dir()? {
                queue.push(entry?.path());
            }
        } else if path.is_file() {
            if path.extension().is_some() && path.extension().unwrap() == "exe" {
                return Ok(Some(path.to_owned()));
            }
        }
    }

    Ok(None)
}



pub async fn run_game(id: &i64, games_dir: &PathBuf) -> Result<(), Box<dyn Error>> {

    let mut game_dir = PathBuf::from(games_dir);
    game_dir.push(id.to_string());
    
    println!("Starting game {}.", id);

    let executable_path = search_exe(game_dir)?;

    if executable_path.is_some() {
        let full_path = fs::canonicalize(executable_path.unwrap()).await?;
        // I'm dumb and don't know how canonicalize works, it adds \\?\ to beginning?
        let (_, exec_path) = full_path.to_str().unwrap().split_at(4);
        println!("Executing game: \"{}\".", exec_path);
        let _program = Command::new(exec_path).spawn().unwrap();
    } else {
        println!("Failed to find executable.");
    }

    Ok(())

}



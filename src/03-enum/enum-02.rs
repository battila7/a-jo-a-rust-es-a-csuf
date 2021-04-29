use crate::State::*;
use std::collections::HashSet;

#[allow(dead_code)]
enum State {
    Lobby { party: HashSet<String> },
    InQueue { party: HashSet<String>, queued_since: u64 },
    MatchMade { this_party: HashSet<String>, that_party: HashSet<String> },
    InGame { game_state: String },
    GameOver { results: String }
}

fn main() {
    let mut state = Lobby {
        party: HashSet::new()
    };
    
    loop {
        state = match state {
            Lobby { party } => {
                println!("A gombra kattintva bekerültök a sorba!");
            
                if queue_btn_clicked() {
                    InQueue { party, queued_since: 0 }            
                } else {
                    Lobby { party }
                }
            },
            InQueue { party, queued_since } => {
                println!("Várakozás mérkőzésre {} óta", queued_since);
                
                MatchMade { this_party: party, that_party: HashSet::new() }
            }
            _ => { todo!(); }
        }    
    }
}

fn queue_btn_clicked() -> bool {
    true
}

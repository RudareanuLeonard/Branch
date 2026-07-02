use crate::AppState;

pub fn move_up_down(appState: &mut AppState, key: &String){
    
    if key == "s"{
        if appState.selected_index + 1 < appState.files.len().try_into().unwrap() {
            //doing the if with "+1" because if i did "-1" on the right side... the "conversion" to the same type would be more difficil;;;; this way is more simple
            println!("You can go down");
            appState.selected_index += 1;
            println!("Selected index = {}", appState.selected_index.to_string());
        }
        else{
            println!("you are already on the last file and can not go more down");
        }
    }
    else if key == "w"{
        if appState.selected_index > 0{
            println!("you can go up");
            appState.selected_index -= 1;
            println!("Selected index = {}", appState.selected_index.to_string());
        }
        else{
            println!("you can't go more up because you are on the first file");

        }
    }

}
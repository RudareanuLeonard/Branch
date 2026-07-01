//will read keyboard input here
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

pub fn keyboard_read(){
    
    let device_state = DeviceState::new();

    loop{
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            println!("Pressed key: {:?}", key);
    }
}

}

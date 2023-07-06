use std::net::{TcpStream};
use std::io::{Read, Write};
use std::os::raw;
use std::{str, string, thread};
use std::time::Duration;

use tetra::{graphics::{mesh::{Mesh, ShapeStyle}, self, Color}, input::{self, Key}, Context, ContextBuilder, State};
use tetra::math::Vec2;

// SUCCESS KEY = <:>

struct GameState {
    local_player_position: [f32; 2],  
    player_shape: Mesh,
    // Thread Communications
 
    
}
static mut server_id:i32 = 0;
static mut local_details:[f32; 2] = [0.0, 0.0];
static mut players_details:Vec<[f32;2]> = Vec::new();

// let main_thread_sender_ptr: Sender<[f32;2]> = None;  


const STARTING_POSITION:[f32; 2] = [0.0, 0.0];

const MOVEMENT_SPEED: f32 = 2.0;

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {  
            local_player_position: [0.0; 2],
            player_shape: Mesh::circle(ctx, ShapeStyle::Stroke(10.0), Vec2::zero(), 10.0)?,
          
        })   
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        // Draw Local Player
        self.player_shape.draw(ctx, Vec2::from(self.local_player_position));

        // Draw Other Players
        unsafe {
            for i in players_details.iter() {
                self.player_shape.draw(ctx, Vec2::from(i.clone()));
            }
        }

        Ok(())
    }  
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let mut speed:f32 = 0.0;
        if input::is_key_down(ctx, Key::LeftShift) {
            speed = 2.0;
        }
        if input::is_key_down(ctx, Key::W) {
            self.local_player_position[1] -= MOVEMENT_SPEED + speed;
        }
        else if input::is_key_down(ctx, Key::S) {
            self.local_player_position[1] += MOVEMENT_SPEED + speed;
        }
        if input::is_key_down(ctx, Key::A) {
            self.local_player_position[0] -= MOVEMENT_SPEED + speed;
        }
        else if input::is_key_down(ctx, Key::D) {
            self.local_player_position[0] += MOVEMENT_SPEED + speed;
        }

        // Setting global var to be sent to server_handle thread
        unsafe {
            local_details = self.local_player_position.clone();
        }


        Ok(())
    }
}



fn setup_window() -> tetra::Result {
    ContextBuilder::new("Online Squares!", 1280, 720)
        .build()?
        .run(GameState::new)    
 
}
fn main() {
    thread::spawn(server_handle);
    
    let _ = setup_window();
    
}


// Server Communications
fn server_handle() {
    let stream = TcpStream::connect("127.0.0.1:80");
    
    match stream {
        Ok(mut stream) => {
            let mut raw_receive_data:[u8; 128] = [0u8; 128];
            let mut server_id_str = "";
            // Receiving necessary items from server [Id]
            // loop {
            //     let _ = stream.read(&mut raw_receive_data);
            //     let data = std::str::from_utf8(&raw_receive_data);
            //     match data {
            //         Ok(r) => {
            //             server_id_str = r;

            //             let _ = stream.write("<:>".as_bytes());
            //             println!("Connected to server");
            //             break;
            //         }
            //                 //Err(e) => {println!("{}, Value got is: {}", e, r);}            
            //         Err(e) => {println!("{}", e);}
            //         }
            // }
            
            
            loop {
                /* RECEIVING DATA FROM SERVER */
            //    let peek = stream.peek()
                //let _ = stream.read(&mut raw_receive_data);                
                let _ = stream.read(&mut raw_receive_data);
                
                let data = std::str::from_utf8(&raw_receive_data);
                match data {
                    Ok(msg) => {
                        // NOTE: Convert str received from server to arr of all players details
                        let val = string_to_f32arrvec(String::from(msg));
                        unsafe {
                            players_details = val.clone();
                        }
                    }
                    Err(e) => {println!("ERROR: {}", e); break;}
                }

                /* SENDING LOCAL DATA TO SERVER */
                // Get local players details from main-thread
                let mut arr:[f32; 2] = [0.0, 0.0];
                unsafe {
                    arr = local_details.clone();
                }
                // Doesn't need ot be Vec of [f32;2], just convert local players [f32;2] to string
                let send_val = get_local_details_str(arr);
                //println!("sending: {}", send_val);
                let _ = stream.write(send_val.as_bytes());
                
            }
        }
        Err(e) => { panic!("{}", e) }
    }
}

fn f32vec_to_string(arr: Vec<[f32; 2]>) -> String
{
    let mut ret:String = "".to_string();
    
    for i in 0..arr.len() {
        let values = arr[i];
        ret.push_str(format!("{},{}; ", values[0], values[1]).as_str());
    };

    ret
}
fn get_local_details_str(arr: [f32;2]) -> String {
    let mut ret:String = "".to_string();
    ret = format!("{},{};~ ", arr[0], arr[1]);
    ret    
}


fn string_to_f32arrvec(data: String) -> Vec<[f32; 2]> {
    let mut ret:Vec<[f32; 2]> = Vec::new();

    ret.resize(data.matches(";").count(), [0.0; 2]);
    let mut player_index:usize = 0;

    let mut xy_index = 0;
    
    let mut xy_str:[String; 2] = ["".to_string(), "".to_string()];

    for (index, char) in data.chars().enumerate() {
        if char == ',' {
            // Seperation of 2 values per player
            xy_index = 1;
        }
        else if char == ';' {
            // Player Details Finished
            let mut xy_f32 = [0.0, 0.0];
            for i in 0..2 {
                let val = xy_str[i].parse::<f32>();
                xy_str[i] = "".to_string();                                          // Clear contents out after extracting to 'val'
                match val {
                    Ok(v) => { xy_f32[i] = v; }
                    Err(e) => {println!("{} - data: {}", e, data)}
                }
            }
            ret[player_index] = xy_f32;
            
            xy_index = 0; 
            player_index += 1;
            

        }
        else if char.is_numeric() || char == '.'{
             

            xy_str[xy_index].push(char);
        }
    }
    ret
}
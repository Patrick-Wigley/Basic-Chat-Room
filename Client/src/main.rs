use std::net::{TcpStream};
use std::io::{Read, Write};
use std::{str, string, thread};

use tetra::{graphics::{mesh::{Mesh, ShapeStyle}, self, Color}, input::{self, Key}, Context, ContextBuilder, State};
use tetra::math::Vec2;

struct GameState {
    local_player_position: [f32; 2],
    global_player_positions: Vec<[f32; 2]>,    
    player_shape: Mesh,
    // Thread Communications
 
    
}
static mut local_details:[f32; 2] = [0.0, 0.0];
static mut players_details:Vec<[f32;2]> = Vec::new();


// let main_thread_sender_ptr: Sender<[f32;2]> = None;  


const STARTING_POSITION:[f32; 2] = [0.0, 0.0];

const MOVEMENT_SPEED: f32 = 2.0;

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {  
            local_player_position: [0.0; 2],
            global_player_positions: Vec::new(),
            player_shape: Mesh::circle(ctx, ShapeStyle::Stroke(10.0), Vec2::zero(), 10.0)?,
          
        })   
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.player_shape.draw(ctx, Vec2::from(self.local_player_position));

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
            println!("Connected to server");

            let mut raw_receive_data:[u8; 128] = [0u8; 128];
            loop {
                /* RECEIVING DATA FROM SERVER */
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
                let mut vec:Vec<[f32; 2]> = Vec::new();
                unsafe {
                    vec.push(local_details.clone());
                }
                let send_val = f32vec_to_string(vec);
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
                    Err(e) => {panic!("{}", e)}
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
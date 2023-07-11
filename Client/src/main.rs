use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;

use tetra::{graphics::{mesh::{Mesh, ShapeStyle, GeometryBuilder}, self, text::{Font, Text}, Color, Rectangle},
            input::{self, Key}, Context, ContextBuilder, State};
use tetra::math::{Vec2};


#[derive(Clone, Debug)]
struct GlobalPlayerDetails {
    name: String,
    position: [f32; 2],
   // message: Vec<String>
}
#[derive(Clone, Debug)]
struct LocalPlayerDetails {
    name: String,
    position: [f32; 2],
    message: String
}

/* GLOBALS & CONSTS */
static mut LOCAL_DETAILS:LocalPlayerDetails = LocalPlayerDetails {name: (String::new()), position: ([0.0, 0.0]), message: (String::new())};
static mut PLAYERS_DETAILS:Vec<GlobalPlayerDetails> = Vec::new();
static mut PLAYERS_MESSAGES:Vec<String> = Vec::new();

const MOVEMENT_SPEED: f32 = 2.0;
const SCREEN_SIZE:[i32; 2] = [900, 720]; 
const TEXT_SIZE:f32 = 17.0;

static mut LOCAL_DESIRES_CONNECTED:bool = true;


/* TETRA */
struct GameState {
    // Meshes
    map: Mesh,
    map_rect: Rectangle<f32>,
    player_shape: Mesh,  
    chat_box: Mesh, 
    chat_box_rect: Rectangle<f32>,

    text: Text,
    chat_mode: bool,
    
    
    local_player_position: [f32; 2],  
    local_player_message: String,

    scroll: [f32; 2] 
}
impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let maps_rectangle = Rectangle::new(100.0, 100.0, 2000.0, 2000.0);
        let chat_box_rectangle = Rectangle::new(0.0, 0.0, SCREEN_SIZE[0] as f32, 300.0);
        
        Ok(GameState {  
            map: GeometryBuilder::new()
            .set_color(Color::rgb(0.4, 0.6, 0.4))
            .rectangle(ShapeStyle::Fill, maps_rectangle)?
            .build_mesh(ctx)?,        
            map_rect: maps_rectangle,
            player_shape: Mesh::circle(ctx, ShapeStyle::Stroke(10.0), Vec2::zero(), 10.0)?,
            chat_box: GeometryBuilder::new()
            .set_color(Color::rgba(0.3, 0.3, 0.3, 0.5))
            .rectangle(ShapeStyle::Fill, chat_box_rectangle)?
            .set_color(Color::rgb(0.0,0.0,0.0,))
            .rectangle(ShapeStyle::Stroke(5.0), Rectangle::new(0.0, 0.0, SCREEN_SIZE[0] as f32, 300.0))?
            .build_mesh(ctx)?,
            chat_box_rect: chat_box_rectangle,

            // Cant seem to find file
            text: Text::new("-", Font::vector(ctx, "./res/style1.ttf", TEXT_SIZE)?),
            chat_mode: false,

            local_player_position: [0.0; 2],
            local_player_message: String::new(),

            scroll: [0.0; 2]
        })   
    }
}
impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.43, 0.24, 0.51));
        
        // Draw Arena
        self.map.draw(ctx, Vec2::from([0.0 - self.scroll[0], 0.0 - self.scroll[1]]));

        // Draw Local Player
        self.player_shape.draw(ctx, Vec2::from([self.local_player_position[0] - self.scroll[0], self.local_player_position[1] - self.scroll[1]]));

        if self.chat_mode {
            // Draw Chat-box
            self.chat_box.draw(ctx, Vec2::new(0.0, 0.0));
            self.text.set_content(self.local_player_message.as_str());
            self.text.draw(ctx, Vec2::new(10.0, self.chat_box_rect.height - TEXT_SIZE - 3.0));
        }


        unsafe {
            // Draw Other Players
            for (index, player) in PLAYERS_DETAILS.clone().iter().enumerate() {
                let mut xy:Vec2<f32> = Vec2::from(player.position);
                xy[0] = xy[0] - self.scroll[0];
                xy[1] = xy[1] - self.scroll[1];

                self.player_shape.draw(ctx, Vec2::from([xy.x, xy.y]));
              
                
                self.text.set_content(player.name.clone());
                // self.text.set_max_width(Some(50.0));
                self.text.draw(ctx, Vec2::from([xy[0] - ((player.name.len() as f32 * TEXT_SIZE)/2.0), xy[1] - 28.0]));
            }
            
            
            // Draw Messages 
            for msg in PLAYERS_MESSAGES.clone() {
                println!("{}", msg);
            }
            PLAYERS_MESSAGES.clear();
         

        }
        Ok(())
    }  
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.scroll[0] += ((self.local_player_position[0] - self.scroll[0]) - (SCREEN_SIZE[0] as f32/2.0) as f32)/10.0;
        self.scroll[1] += ((self.local_player_position[1] - self.scroll[1]) - (SCREEN_SIZE[1] as f32/2.0))/10.0;
        
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

        if input::is_key_down(ctx, Key::Escape) {
            unsafe {
                //LOCAL_DESIRES_CONNECTED = false;
            }
        }
        if input::is_key_released(ctx, Key::T) {
            self.chat_mode = true;
        }

        // Arena Collision Detection
        if self.local_player_position[0] <= self.map_rect.x {
            self.local_player_position[0] = self.map_rect.x
        }
        if self.local_player_position[1] <= self.map_rect.y {
            self.local_player_position[1] = self.map_rect.y
        }
        if self.local_player_position[0] >= (self.map_rect.x + self.map_rect.width) {
            self.local_player_position[0] = (self.map_rect.x + self.map_rect.width)
        }
        if self.local_player_position[1] >= (self.map_rect.y + self.map_rect.height) {
            self.local_player_position[1] = (self.map_rect.y + self.map_rect.height)
        }


        // Typing in chat
        if self.chat_mode {
            // Player has chat open
            let wrapped_value = input::get_text_input(ctx);
            match wrapped_value {
               Some(val) => {self.local_player_message.push_str(val);}
               None => {}
            }
            if input::is_key_released(ctx, Key::Backspace) {
                self.local_player_message.pop();
            }
            if input::is_key_down(ctx, Key::Escape) {
                self.chat_mode = false;
            }
            if input::is_key_down(ctx, Key::Enter) {
                unsafe {
                    LOCAL_DETAILS.message = format!("'{}'", self.local_player_message.clone());
                }
                self.local_player_message = "".to_string();
            }
        }
        
        // Setting global var to be used in 'server_handle' thread
        unsafe {
            LOCAL_DETAILS.position = self.local_player_position.clone();
        }
        Ok(())
    }
}
/// Returns built tetra application
fn setup_window() -> tetra::Result {
    ContextBuilder::new("Online Squares!", SCREEN_SIZE[0], SCREEN_SIZE[1])
        .build()?
        .run(GameState::new)    
}

fn main() {
    unsafe {
        // Username cannot contain server key's e.g. ':'
        LOCAL_DETAILS.name = "user".to_string();
    }
    thread::spawn(server_handle);
    let _ = setup_window();    
}

/* SERVER COMMUNICATIONS */
/// TCP Stream for current client & server communications of data \ 
/// Handled by its own thread
fn server_handle() {
    let stream = TcpStream::connect("127.0.0.1:80");
    
    match stream {
        Ok(mut stream) => {
            let mut raw_receive_data:[u8; 550] = [0u8; 550];
            
            loop {
                /* RECEIVING DATA FROM SERVER */               
                let _ = stream.read(&mut raw_receive_data);
                let data = std::str::from_utf8(&raw_receive_data);
                match data {
                    Ok(msg) => {
                        unsafe {
                            let actual_data = msg[0..(msg.find("~").unwrap())].to_string();
                            PLAYERS_DETAILS = get_players_from_string(actual_data);
                        }
                    }
                    Err(e) => {println!("[ERROR]: {}", e);}
                }

                /* SENDING LOCAL DETAILS TO SERVER - (X,Y Position) */
                let send_val:String;
                unsafe {
                    if !LOCAL_DESIRES_CONNECTED {
                        // Leave server
                        let _ = stream.write("(DISCONNECT)".as_bytes());
                        break;
                    }
                    let local_details_copy = LOCAL_DETAILS.clone();
                    // Clear local message content
                    LOCAL_DETAILS.message = "".to_string();
                    send_val = get_string_from_local_details(local_details_copy);

                }
                let _ = stream.write(send_val.as_bytes());
                
            }
        }
        Err(e) => { 
            panic!("[ERROR]: {}", e)
        }
    }
}

/* SUB FUNCTIONS USED FOR 'server_handle' */
/// Prepares string which will be sent over to the server \
/// Returns prepared String
fn get_string_from_local_details(local_player_details: LocalPlayerDetails) -> String {
    format!("{}:{},{}:{};~ ",
    local_player_details.name,
    local_player_details.position[0], local_player_details.position[1], 
    local_player_details.message
    )  
}
/// Gathers each players details from string received & processes it to Vec<[f32; 2]> \
/// Returns processed Vector
fn get_players_from_string(data: String) -> Vec<GlobalPlayerDetails> {
    // EXAMPLE OF A MESSAGE - "12.1,51.2 : 'a message'; '61.9,21.0' : 'Hello World'; |;"
    #[derive(PartialEq)]
    enum ValueIndices {
        NAME=0,
        POSITION=1,
        MESSAGE=2, 
    }
    
    let mut ret:Vec<GlobalPlayerDetails> = Vec::new();

    let player_values = data.split(";");
    //println!("{:?}", player_values);
    
    for val in player_values.into_iter() {
        if !val.contains("|") && !val.is_empty() {
            // Player slot is active
            let mut player_details = GlobalPlayerDetails {name: ("[Unknown User]".to_string()), position: ([-1000.0, -1000.0]) };
            // println!("Player found: {}", val);
            let values = val.split(":");
            
            for (value_index, j) in values.into_iter().enumerate() {
                //println!("Val extracted: {}", j);
                match value_index {
                    // Handle each value here: 
                    0 => {player_details.name = j.to_string();}
                    1 => {player_details.position = extract_player_position(j.trim().to_string());}
                    2 => { 
                        if !j.is_empty() { 
                            if j.contains(":") { println!("[DEBUG]: message is messed up - '{}'", data); }
                            unsafe {
                                PLAYERS_MESSAGES.push(j.to_string()) 
                            } 
                        } 
                    }
                    _ => {panic!("Too many values during extraction of players data? - {}", data);}
                }   
            }
            ret.push(player_details);
        }
    }
    ret
}
/// EXTRACTS X,Y VALUES FROM STRING 
/// Returns: [x, y] as [f32; 2]
fn extract_player_position(data:String) -> [f32; 2] {
    let mut ret:[f32; 2] = [0.0; 2];
    let mut xy_str:[String; 2] = ["".to_string(), "".to_string()];
    let mut xy_index = 0;
    for char in data.chars() {
        if char == ',' {
            // is a seperator for x,y values
            xy_index = 1;
        }
        else if char.is_numeric() || char == '.' || char == '-' {
            xy_str[xy_index].push(char);
        }
    }
    for (index, val) in xy_str.iter().enumerate() {
        let parse_result = val.parse::<f32>();
        match parse_result {
            Ok(parse_val) => {ret[index] = parse_val;}
            Err(e) => {}//println!("{}", e)}
        }
    }
    ret  
}


// let mut ret:Vec<GlobalPlayerDetails> = Vec::new();


// let mut player_index:usize = 0;
// let mut xy_index:usize = 0;
// let mut xy_str:[String; 2] = ["".to_string(), "".to_string()];
// let mut player_invalid:bool = false;

// let mut msg:[String; 1] = ["".to_string()];

// /// Used to determine what data is being read when iterating through 'data: String'
// /// MAXIMUM OF 10 MODES AVAILABLE - 0-9
// #[derive(PartialEq)]
// enum DataMode {
//     POSITION=0,
//     MESSAGE=1, 
// }
// let mut mode:DataMode = DataMode::POSITION;
// let mut is_mode_setting:bool = false;


// ret.resize(data.matches(";").count(), GlobalPlayerDetails { position: ([0.0; 2]), message: ("".to_string()) });

// let all_players_data = data.split(";");
// for player_data in all_players_data.into_iter() {
//     let values = player_data.split(":");
// }


// // Iterates through each char in 'data: String' (all players details sent over from server). 
// // "(0/POSITION) 100, 50 (1/MESSAGE) "msg1", "nth msg""
// for char in data.chars() {
//     // Setting mode of what data is being read
//     if char == '(' {
//         is_mode_setting = true;
//     } else if char == ')' {
//         is_mode_setting = false;
//     } else if is_mode_setting {
//         let mode_val:usize = char.to_string().parse::<usize>().unwrap(); 
//         match mode_val {
//             0 => { mode = DataMode::POSITION }
//             1 => { mode = DataMode::MESSAGE }          
//             _ => { println!("[ERROR] message has corrupt")}      
//         } 
//     }
    
//     // ';' IS END OF A PLAYERS DETAILS 
//     if char == ';' {
//         // At this point, have collected the players details
//         let mut xy_f32 = [0.0, 0.0];
        
//         if !player_invalid {
//             for i in 0..2 {
//                 let val = xy_str[i].parse::<f32>();
//                 xy_str[i] = "".to_string();                                     // Clear contents out for next player iteration
//                 match val {
//                     Ok(v) => { xy_f32[i] = v; }
//                     Err(e) => { 
//                         // End of string seems to be corrupted - Returns the processed data
//                         println!("{} - {:?}", e, xy_str);
//                     }
//                 }
//             }
//            // println!("{}", msg[0]);
//             if msg[0].eq("") {
//                 // If 'msg' collected any data - (A msg has just been sent)
//                 ret[player_index].message = msg[0].clone();
//             }
//         }
//         ret[player_index].position = xy_f32;
        
//         xy_index = 0; 
//         player_index += 1;
//         player_invalid = false;
//     }
    
  

//     else if mode == DataMode::POSITION {
//         if char == ',' {
//             // Seperation of 2 values (X,Y)
//             xy_index = 1;
//         }
//         else if char.is_numeric() || char == '.' || char == '-'{
//             xy_str[xy_index].push(char);
//         }  
//         else if char == '|'{
//         player_invalid = true;
//     }
//     }
//     else if mode == DataMode::MESSAGE {
//         msg[0].push(char);
//     }

// }
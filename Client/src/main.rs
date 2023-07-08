use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;

use tetra::{graphics::{mesh::{Mesh, ShapeStyle, GeometryBuilder}, self, Color, Rectangle}, input::{self, Key}, Context, ContextBuilder, State};
use tetra::math::{Vec2};


/* GLOBALS & CONSTS */
static mut LOCAL_DETAILS:[f32; 2] = [0.0, 0.0];
static mut PLAYERS_DETAILS:Vec<[f32;2]> = Vec::new();
const MOVEMENT_SPEED: f32 = 2.0;
const SCREEN_SIZE:[i32; 2] = [900, 720]; 

static mut LOCAL_DESIRES_CONNECTED:bool = true;

/* TETRA */
struct GameState {
    map: Mesh,
    map_rect: Rectangle<f32>,
    local_player_position: [f32; 2],  
    player_shape: Mesh,  
    scroll: [f32; 2] 
}
impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let maps_rectangle = Rectangle::new(100.0, 100.0, 2000.0, 2000.0);
        
        
        Ok(GameState {  
            map: GeometryBuilder::new()
            .set_color(Color::rgb(0.4, 0.6, 0.4))
            .rectangle(ShapeStyle::Fill, maps_rectangle)?
            .build_mesh(ctx)?,
                    
            map_rect: maps_rectangle,
            local_player_position: [0.0; 2],
            player_shape: Mesh::circle(ctx, ShapeStyle::Stroke(10.0), Vec2::zero(), 10.0)?,
            scroll: [0.0; 2]
        })   
    }
}
impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.43, 0.24, 0.51));
        
        self.map.draw(ctx, Vec2::from([100.0 - self.scroll[0], 100.0 - self.scroll[1]]));

        // Draw Local Player
        self.player_shape.draw(ctx, Vec2::from([self.local_player_position[0] - self.scroll[0], self.local_player_position[1] - self.scroll[1]]));

        // Draw Other Players
        unsafe {
            for i in PLAYERS_DETAILS.iter() {
                let xy:Vec2<f32> = Vec2::from(i.clone());
                self.player_shape.draw(ctx, Vec2::from([xy.x - self.scroll[0], xy.y - self.scroll[1]]));
            }
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
                LOCAL_DESIRES_CONNECTED = false;
            }
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

        // Setting global var to be used in 'server_handle' thread
        unsafe {
            LOCAL_DETAILS = self.local_player_position.clone();
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
            let mut raw_receive_data:[u8; 128] = [0u8; 128];
            
            loop {
                /* RECEIVING DATA FROM SERVER */               
                let _ = stream.read(&mut raw_receive_data);
                let data = std::str::from_utf8(&raw_receive_data);
                match data {
                    Ok(msg) => {
                        unsafe {
                            PLAYERS_DETAILS = get_players_from_string(String::from(msg));
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
                    send_val = get_string_from_local_details(LOCAL_DETAILS.clone());
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
fn get_string_from_local_details(arr: [f32;2]) -> String {
    format!("{},{};~ ", arr[0], arr[1])      
}
/// Gathers each players details from string received & processes it to Vec<[f32; 2]> \
/// Returns processed Vector
fn get_players_from_string(data: String) -> Vec<[f32; 2]> {
    let mut ret:Vec<[f32; 2]> = Vec::new();

    ret.resize(data.matches(";").count(), [0.0; 2]);

    let mut player_index:usize = 0;

    let mut xy_index = 0;
    
    let mut xy_str:[String; 2] = ["".to_string(), "".to_string()];

    let mut player_invalid:bool = false;

    for char in data.chars() {
        if char == ',' {
            // Seperation of 2 values (X,Y)
            xy_index = 1;
        }
        else if char == ';' {
            // At this point, 'xy_str' has collected the players details
            let mut xy_f32 = [0.0, 0.0];
            
            if !player_invalid {
                for i in 0..2 {
                    let val = xy_str[i].parse::<f32>();
                    xy_str[i] = "".to_string();                                     // Clear contents out for next player iteration
                    match val {
                        Ok(v) => { xy_f32[i] = v; }
                        Err(e) => { 
                            // End of string seems to be corrupted - Returns the processed data
                            println!("{} - {:?}", e, xy_str);
                        }
                    }
                }
            }
            ret[player_index] = xy_f32;
            xy_index = 0; 
            player_index += 1;
            player_invalid = false;
        }
        else if char.is_numeric() || char == '.' || char == '-'{
            xy_str[xy_index].push(char);
        }
        else if char == '|' {
            player_invalid = true;
        }
    }
    ret
}
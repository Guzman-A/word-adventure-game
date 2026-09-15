// Simple Word Adventure
// By: Taco_Dev

////////////////////////////////////////////////////////////////[ Practice Topics ] ////////////////////////////////////////////////////////

// Structs
// Closures
// Flow Control
// Impl
// self
// Architecture
// While / For Loop
// Match Case
// 2 Dimensional Vec with custom data type
// Nested For loop

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::io;

// Character Data Type
#[derive(Copy,Clone, Debug)]
struct Character {
    user:       bool,
    health:     u8,
    attack:     u8,
    cord_x:     usize,
    cord_y:     usize,
}

// Functions that can call on character
impl Character {
    // Function that defines a new character ; called by Vec::new() ; self, &self, &mut self can method call like vec.len()
    fn new(user: bool, health:u8, attack:u8, cord_x: usize, cord_y:usize) -> Self {
        Self {user, cord_x, cord_y, health, attack}
    }

    // Displays Character type fields
    fn get_info(&self) -> () {
        println!("Health : {} , Attack : {} , Cord X : {} , Cord Y :{} " , &self.health, &self.attack, &self.cord_x , &self.cord_y);
    }

    fn move_player(&mut self, map:&Map, distance:isize, is_x_axis:bool) {

        // if distance is positive value
        if distance > 0 {
            println!("Distance is greater than 0.");

            if is_x_axis {
                //sets new variable new_coord_x to value of player coord_x PLUS the direction value cast to a usize.
                let new_coord_x = self.cord_x + usize::try_from(distance).ok().unwrap();
                //checks to see if map is passable (is possible move), then change player coordinates.
                if map.get_tile_at_coord(new_coord_x, self.cord_y).unwrap().is_passable{
                    self.cord_x = new_coord_x;
                }
                else {
                    println!("Ate a facefull of wall and called it puddin!");
                }
            }

            // if is y axis
            else {
                //sets new variable new_coord_y to value of player coord_y PLUS the direction value cast to a usize.
                let new_coord_y = self.cord_y + usize::try_from(distance).ok().unwrap();
                //checks to see if map is passable (is possible move), then change player coordinates.
                if map.get_tile_at_coord(self.cord_x, new_coord_y).unwrap().is_passable{
                    self.cord_y = new_coord_y;
                }
                else {
                    println!("Walked into a wall like a stupid Republican and blamed Joe Biden!");
                }
            }
        }

        //if distance is negative value
        else {
            if is_x_axis {
                //checks to make sure we won't end up with a negative value (off of board, not possible)
                if usize::try_from(distance.strict_abs()).ok().unwrap() <= self.cord_x {
                    //sets new variable new_coord_x to value of player coord_x PLUS the direction value cast to a usize.
                    let new_coord_x = self.cord_x - usize::try_from(distance.strict_abs()).ok().unwrap();
                    //checks to see if map is passable (is possible move), then change player coordinates.
                    if map.get_tile_at_coord(new_coord_x, self.cord_y).unwrap().is_passable{
                        self.cord_x = new_coord_x;
                    }
                    else {
                        println!("This here is a mighty nice wall with excellent craftsmanship");
                    }
                }
                else {
                    println!("Wall!");
                }
            }

            //if is y axis
            else {
                //checks to make sure we won't end up with a negative value (off of board, not possible)
                if usize::try_from(distance.strict_abs()).ok().unwrap() <= self.cord_y {
                    //sets new variable new_coord_y to value of player coord_y PLUS the direction value cast to a usize.

                    let new_coord_y = self.cord_y - usize::try_from(distance.strict_abs()).ok().unwrap();
                    //checks to see if map is passable (is possible move), then change player coordinates.
                    if map.get_tile_at_coord(self.cord_x, new_coord_y).unwrap().is_passable{
                        self.cord_y = new_coord_y;
                    }
                    else {
                        println!("You keep running it walls, ask someone for help putting on a helmet!");
                    }
                }
                else {
                    println!("Stupid meet wall, wall meet stupid!");
                }
            }
        }
    }
}

//Data type
#[derive(Copy, Clone, Debug)]
struct Tile {
    is_passable : bool,
    insta_death : bool,
    win : bool,
}

// Functions that can be called on Tile
impl Tile {
// Function that defines a new Tile
    fn new(insta_death:bool, is_passable:bool, win:bool) -> Self {
        Self { insta_death, is_passable, win }
    }
}

// Data type
#[derive(Debug)]
struct Map {
    // 2 Dimentional Vec with custom data type
    tile_set: Vec<Vec<Tile>>,
}

// Functions that can call on map data type
impl Map{
    // Function that can call on Map data type
    fn new() -> Self{
        // Self {tile_set:Vec::new(), player: Character{cord_x:1, cord_y:0, health:20, attack:30}}
        Self { tile_set:Vec::new() }
    }
    // gets coordinates of tile checks if out of bounds
    fn get_tile_at_coord(&self, x_coord:usize, y_coord:usize) -> Option<Tile>{
        //checks for out of bounds
        if y_coord < self.tile_set.len(){

            let row = &self.tile_set[y_coord];

            if x_coord < row.len(){
                return Some(row[x_coord]);
            }
            else {
                return None;
            }
        }
        else {
            return None;
        }
    }

    // Displays array
    fn print_map(&self, player:Character){
        // Loop through tile_set Vector(array) length y-axis
        for i in 0..self.tile_set.len(){

            // loops through position of Vector(array) length x-axis
            for y in 0..self.tile_set[i].len(){
                if player.cord_x == y && player.cord_y == i {
                    print!("H");
                }
                // if Vector(array) position = is_passable print "O"
                else if self.tile_set[i][y].is_passable{
                    print!("O");
                }
                else {
                    print!("X");
                }
            } // for y
            println!();
        } // for i
    } // fn print_map
} // Impl Map

// main entry
fn main() {

    // Is setting a new map
    let mut map: Map = Map::new();

    // 4 total
    // MainVec = [Vec0,Vec1,Vec2]
    // Vec0 = [tile,tile,tile]
    // Vec1 = [tile,tile,tile]
    // Vec2 = [tile,tile,tile]
    // Loops MainVec

    for x in 0..3{
        // Sets new row(Vec) of tiles
        let mut row: Vec<Tile> = Vec::new();
        // Loops within a row (Vec0) to add tiles
        for y in 0..3 {
            // Hand setting the tiles
            if x == 0 && y == 1 {
                row.push(Tile::new( false, true, false));
            }
            else if  x == 1 && y == 1 {
                row.push(Tile::new( false, true, false));

            }
            else if  x == 2 && y == 1 {
                row.push(Tile::new( false, true, false));

            }
            else if  x == 0 && y == 2 {
                row.push(Tile::new( false, true, false));
            }
            else {
                row.push(Tile::new( false, false, false));
            }
        } // for y

        // pushes row to map
        map.tile_set.push(row);

    } // for x

    let mut game_state:bool = true;

    // Sets player instance
    let mut player = Character {
        user : true,
        health : 10,
        attack : 2,
        cord_x : 1, // starting coord without rhyme or reason; may need to re-architect
        cord_y : 0, // starting coord without rhyme or reason
    };

    println!();
    println!("Simple Word Adventure");
    println!();
    println!("Map: ");
    println!();
    map.print_map(player);
    println!();
    println!("Your Stats : ");
    println!();
    println!("{:#?}", player);
    println!();
    println!("WASD : to Move ; Press Enter to confirm ");
    println!("i : Information, type 'quit' : quitters ");
    println!("z : Receive Damage , x : Receive Attack Power ");

    while game_state == true {

        let mut damage = | damage: u8 | -> u8 {
            player.health = player.health - 1;
            damage
        };

        let mut attack = | attack: u8 | -> u8 {
            player.attack = player.attack + 1;
            attack
        };

        // Setting variable to new function from String struct (data type)
        let mut input = String::new();
        // handle
        io::stdin().read_line(&mut input).expect("Failed to read line");

        println!();

        match input.trim(){

            "w" => {
                println!("Direction : Up");
                player.move_player(&map, -1, false);
            }

            "a" => {
                println!("Direction : Left , ... meh!");
                player.move_player(&map, -1, true);
            }

            "s" => {
                println!("Direction: Headed Down with great despair");
                player.move_player(&map, 1, false);
            }

            "d" => {
                println!("Direction: Right");
                player.move_player(&map, 1, true);
            }

            "z" => {
                damage(1);
            }

            "x" => {
                attack(1);
            }

            "i" => {
                Character::get_info(&player);
            }

            "quit" => {
                game_state = false;
            }

            _ => { // Wildcard match arm for user error since all options must be accounted for.

            }

        } // match input

        println!();
        println!("Map: ");
        map.print_map(player);

    } // game_state

} // fn main

/////////////////////////////////////////////////////////////////////////// [ TASKS ]/////////////////////////////////////////////////////////////

// [] Enable Raw mode terminal
// [] Add Logic for walls, hazards, win condition, key, locked door, open door, text story line, enemy, area of attack, damage

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

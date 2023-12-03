use array2d::Array2D;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*, reflect::Array,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod sudoku;

#[derive(Component)]
struct SingleNumberText;

#[derive(Component)]
struct PuzzleLocation {
    x : usize,
    y : usize
}


#[derive(Component)] 
struct Cursor;


#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    Menu,
    Paused,
    Running
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Menu
    }
}

#[derive(Resource, Default)]
struct Game {
    elapsed : f32
}

#[derive(Resource)]
struct Puzzle {
    data : Array2D<sudoku::Field>,
    entities: Array2D<Entity>,
    player_data : Array2D<Option<i32>>
}

#[derive(Resource)]
struct GridData {
    grid_size : f32,
    origin  : Vec2,
    color  : Color
}

impl Default for GridData {
    fn default() -> Self {
        Self {
            grid_size:  550.0 / 9.0,
            origin : Vec2::new(-800.0 / 2.0 + 25.0, 600.0 / 2.0 - 25.0),
            color : Color::WHITE
        }
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Sudoku Solver".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(EguiPlugin)
    .insert_resource(Puzzle{data: sudoku::read_puzzle("D:/experiments/rust/wavecollapse/data/puzzle1.txt"), 
                            entities: Array2D::filled_with(Entity::PLACEHOLDER, 9, 9),
                            player_data: Array2D::filled_with(None, 9, 9)})
    .insert_resource(Game::default())
    .insert_resource(GridData::default())
    .add_state::<GameState>()
    .add_systems(Startup, (setup_text, spawn_entities))
    .add_systems(Update, ui_system)
    .add_systems(Update, (
        draw_grid, process_input,
        update_text.after(process_input), 
        draw_cursor.after(process_input),
        ui_system).run_if(in_state(GameState::Running))
    )
    .run();
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>, 
                    mut puzzle: ResMut<Puzzle>,
                    grid_data: Res<GridData>) {
    
    commands.spawn(Camera2dBundle::default());
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;

    let grid_size =  grid_data.grid_size;
    let origin = Vec3::new(grid_data.origin.x + grid_size / 2.0, grid_data.origin.y + grid_size / 2.0, 0.0);


    for j in 0..9 {
        for i in 0..9 { 
            let entity = commands.spawn((
                Text2dBundle {
                    text: Text::from_section("0", text_style.clone())
                        .with_alignment(text_alignment),
                    transform: Transform::from_translation(
                         origin + Vec3::new(grid_size * i as f32, -grid_size * j as f32 -grid_size, 0.0)),
                    ..default()
                },
                SingleNumberText,
            )).id();
            puzzle.entities[(i,j)] = entity;
        }
    }   
}

fn spawn_entities(mut commands: Commands) {
    commands.spawn((
        PuzzleLocation{x: 0, y:0},
        Cursor
    ));
}

fn draw_grid(mut gizmos: Gizmos, grid_data: Res<GridData>) {
    let grid_size = grid_data.grid_size;
    let down = Vec2::new(0.0, -grid_size * 9.0);
    let over = Vec2::new(grid_size * 9.0, 0.0);
    let origin = grid_data.origin;
    let color = grid_data.color;

    for i in 0..=9  
    {   
        let mut vec = origin + Vec2::new( i as f32 * grid_size, 0.0);
        gizmos.ray_2d(vec, down, color);
        if i % 3 == 0 {
            vec.x = vec.x - 1.0;
            gizmos.ray_2d(vec, down, color);
            vec.x = vec.x + 2.0;
            gizmos.ray_2d(vec, down, color);    
        }
        vec = origin + Vec2::new( 0.0, -i as f32 * grid_size);
        gizmos.ray_2d(vec, over, color);
        if i % 3 == 0 {
            vec.y = vec.y - 1.0;
            gizmos.ray_2d(vec, over, color);
            vec.y = vec.y + 2.0;
            gizmos.ray_2d(vec, over, color);    
        }
    }
}

fn draw_cursor(mut gizmos: Gizmos, grid_data: Res<GridData>, query: Query<&PuzzleLocation, With<Cursor>>) {
    let location = query.single();
    let position = grid_data.origin + Vec2::new(grid_data.grid_size * location.x as f32 + grid_data.grid_size, -grid_data.grid_size * location.y as f32);
    let half_box = Vec2::new(grid_data.grid_size / 2.0, grid_data.grid_size / 2.0);
    let cursor_size = grid_data.grid_size - 3.0;
    gizmos.rect_2d(position - half_box, 0.0, Vec2::new(cursor_size, cursor_size) , Color::RED);
}

fn update_text(puzzle: Res<Puzzle>, mut query: Query<(&mut Visibility, &mut Text), With<SingleNumberText>>) {
    for i in 0..9 {
        for j in 0..9 {
            let entity = puzzle.entities[(i,j)];
            //let (mut vis, mut text) = query.get_mut(entity).unwrap();
            if let Ok( (mut vis, mut text) ) = query.get_mut(entity) {
                if let Some(value) = puzzle.player_data[(i,j)] {
                    *vis = Visibility::Visible;
                    let content = &mut text.sections[0].value;
                    *content = value.to_string();
                    let style  = &mut text.sections[0].style;
                    if let Some(puzzle_value) = puzzle.data[(i,j)].value {
                        if puzzle_value == value {
                            style.color = Color::WHITE;
                        } else { 
                            style.color = Color::RED;
                        }
                    }
                } else {
                    *vis = Visibility::Hidden;
                }    
            }
        }
    }
}

fn ui_system(mut contexts: EguiContexts, 
             game_state: Res<State<GameState>>, 
             mut next_state: ResMut<NextState<GameState>>,
             mut puzzle: ResMut<Puzzle>) {
    egui::Area::new("my_area")
    .fixed_pos(egui::pos2(32.0, 32.0))
    .show(contexts.ctx_mut(), |ui| {
        ui.label("Floating text!");

        if ui.button("Load Puzzle").clicked() {
            let mut data = sudoku::read_puzzle("D:/experiments/rust/wavecollapse/data/puzzle1.txt");
            let mut done = false;
            while !done {
                done = sudoku::update(&mut data);
            }
            
            // Copy problem to player data
            let mut player_data = Array2D::<Option<i32>>::filled_with(None,9 ,9 );
            for i in 0..9 {
                for j in 0..9 {
                    if data[(i,j)].is_original {
                        player_data[(i,j)] = data[(i,j)].value.clone()
                    }
                }
            }
            puzzle.data = data;
            puzzle.player_data = player_data;
        }
        let state = game_state.get();
        
        if (*state == GameState::Paused || *state == GameState::Menu) && ui.button("Start").clicked() {
            next_state.set(GameState::Running)
        } else if *state == GameState::Running && ui.button("Stop").clicked() {
            next_state.set(GameState::Menu);
        }
    });
}


fn process_input(mut puzzle: ResMut<Puzzle>, 
                 keys: Res<Input<KeyCode>>, 
                 mut query: Query<&mut PuzzleLocation, With<Cursor>>,
                 mut game: ResMut<Game>, 
                 time: Res<Time>) {

    if keys.just_pressed(KeyCode::Space) {
        sudoku::update(&mut puzzle.data);
    }

    let mut location = query.single_mut();
    if keys.just_pressed( KeyCode::Left) && location.x > 0 {
        location.x -= 1;
    }

    if keys.just_pressed( KeyCode::Right) && location.x < puzzle.data.row_len()-1 {
        location.x += 1;
    }

    if keys.just_pressed( KeyCode::Up) && location.y > 0 {
        location.y -= 1;
    }

    if keys.just_pressed( KeyCode::Down) && location.y < puzzle.data.column_len()-1 {
        location.y += 1;
    }

    for key_code in keys.get_just_pressed() {
        if *key_code >= KeyCode::Key1 && *key_code <= KeyCode::Key0  && 
        puzzle.data[(location.x, location.y)].is_original == false {
            let new_value = *key_code as i32;
            puzzle.player_data[(location.x, location.y)] = Some(new_value + 1);
        }
    }

    if keys.just_pressed(KeyCode::Delete) && 
        puzzle.data[(location.x, location.y)].is_original == false {
            puzzle.player_data[(location.x, location.y)] = None
    }

    game.elapsed += time.delta_seconds();
}
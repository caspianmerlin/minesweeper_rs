use std::error::Error;

use ini::Ini;
use winit::dpi::PhysicalPosition;






pub struct Config {
    graphics_type:      GraphicsType,
    legacy_rng:         bool,
    difficulty:         Difficulty,
    window_position:    PhysicalPosition<i32>,
    sound_enabled:      bool,
    marks_enabled:      bool,
    colour_enabled:     bool,
    beginner_score:     HighScore,
    intermediate_score: HighScore,
    expert_score:       HighScore,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            graphics_type:      GraphicsType::OpenGL,
            legacy_rng:         false,
            difficulty:         Difficulty::default(),
            window_position:    PhysicalPosition::new(80, 80),
            sound_enabled:      false,
            marks_enabled:      true,
            colour_enabled:     true,
            beginner_score:     HighScore::default(),
            intermediate_score: HighScore::default(),
            expert_score:       HighScore::default(),
        }
    }
}
impl Config {
    pub fn load() -> Config {
        todo!()
    }
    fn load_from_ini() -> Result<Config, ()> {
        let config_file = dirs::config_dir().ok_or(())?.join("minesweeper_rs").join("config.ini");
        let ini = Ini::load_from_file(config_file).or(Err(()))?;
        let general_section = ini.general_section();
        
        let config = Config {
            graphics_type: match general_section.get("graphics_type").ok_or(())? {
                "opengl" => GraphicsType::OpenGL,
                "vulkan" => GraphicsType::Vulkan,
                "metal" => GraphicsType::Metal,
                "direct3D" | "directx" => GraphicsType::Direct3D,
                _ => return Err(()),
            },
            legacy_rng: match general_section.get("graphics_type").ok_or(())? {
                "true" | "yes" | "y" => true,
                "false" | "no" | "n" => false,
                _ => return Err(()),
            },
            difficulty: match general_section.get("difficulty").ok_or(())? {
                "beginner" => Difficulty::BEGINNER,
                "intermediate" => Difficulty::INTERMEDIATE,
                "expert" => Difficulty::EXPERT,
                _ => {
                    let grid_width: u32 = general_section.get("num_columns").ok_or(())?.parse().or(Err(()))?;
                    let grid_height: u32 = general_section.get("num_rows").ok_or(())?.parse().or(Err(()))?;
                    let num_mines: u32 = general_section.get("num_mines").ok_or(())?.parse().or(Err(()))?;
                    Difficulty::new(grid_width, grid_height, num_mines)
                },
            },
            window_position: {
                let x: i32 = general_section.get("window_pos_x").ok_or(())?.parse().or(Err(()))?;
                let y: i32 = general_section.get("window_pos_y").ok_or(())?.parse().or(Err(()))?;
                PhysicalPosition::new(x, y)
            },
            sound_enabled: match general_section.get("sound_enabled").ok_or(())? {
                "true" | "yes" | "y" => true,
                "false" | "no" | "n" => false,
                _ => return Err(()),
            },
            marks_enabled: match general_section.get("marks_enabled").ok_or(())? {
                "true" | "yes" | "y" => true,
                "false" | "no" | "n" => false,
                _ => return Err(()),
            },
            colour_enabled: match general_section.get("colour_enabled").ok_or(())? {
                "true" | "yes" | "y" => true,
                "false" | "no" | "n" => false,
                _ => return Err(()),
            },
            beginner_score: {
                let beginner_name = general_section.get("beginner_name").ok_or(())?;
                let beginner_time = general_section.get("beginner_time").ok_or(())?.parse::<u32>().or(Err(()))?.min(999);
                HighScore { name: String::from(beginner_name), time: beginner_time }
            },
            intermediate_score: {
                let intermediate_name = general_section.get("intermediate_name").ok_or(())?;
                let intermediate_time = general_section.get("intermediate_time").ok_or(())?.parse::<u32>().or(Err(()))?.min(999);
                HighScore { name: String::from(intermediate_name), time: intermediate_time }
            },
            expert_score: {
                let expert_name = general_section.get("expert_name").ok_or(())?;
                let expert_time = general_section.get("expert_time").ok_or(())?.parse::<u32>().or(Err(()))?.min(999);
                HighScore { name: String::from(expert_name), time: expert_time }
            },
        };
        Ok(config)

    }
}





pub struct Difficulty {
    num_mines:   u32,
    grid_width:  u32,
    grid_height: u32,
}
impl Difficulty {
    const BEGINNER:     Difficulty = Difficulty { num_mines: 10, grid_width: 9,  grid_height: 9  };
    const INTERMEDIATE: Difficulty = Difficulty { num_mines: 40, grid_width: 16, grid_height: 16 };
    const EXPERT:       Difficulty = Difficulty { num_mines: 99, grid_width: 30, grid_height: 16 };

    pub fn num_mines(&self) -> u32 {
        self.num_mines
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.grid_width, self.grid_height)
    }

    pub fn new(grid_width: u32, grid_height: u32, num_mines: u32) -> Difficulty {
        let grid_width = grid_width.clamp(9, 30);
        let grid_height = grid_height.clamp(9, 24);
        let num_mines = num_mines.clamp(10, (grid_height - 1) * (grid_width - 1));
        Difficulty { num_mines, grid_width, grid_height }
    }
}
impl Default for Difficulty {
    fn default() -> Difficulty {
        Self::BEGINNER
    }
}





pub enum GraphicsType {
    OpenGL,
    Direct3D,
    Vulkan,
    Metal,
}

pub struct HighScore {
    name: String,
    time: u32,
}
impl Default for HighScore {
    fn default() -> HighScore {
        HighScore { name: String::from("Anonymous"), time: 999 }
    }
}
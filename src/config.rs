use std::{error::Error, fs::{File, self}, fmt::Display};

use ini::Ini;
use winit::dpi::PhysicalPosition;




#[derive(Debug)]
pub struct Config {
    pub graphics_type:      GraphicsType,
    pub legacy_rng:         bool,
    pub difficulty:         Difficulty,
    pub window_position:    PhysicalPosition<i32>,
    pub sound_enabled:      bool,
    pub marks_enabled:      bool,
    pub colour_enabled:     bool,
    pub beginner_score:     HighScore,
    pub intermediate_score: HighScore,
    pub expert_score:       HighScore,
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
        Config::load_from_ini().unwrap_or_default()
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
                "direct3d" | "directx" => GraphicsType::Direct3D,
                _ => return Err(()),
            },
            legacy_rng: match general_section.get("legacy_rng").ok_or(())? {
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

    pub fn save_to_ini(&self) -> Result<(), ()> {
        let root_config_dir = dirs::config_dir().ok_or(())?;
        let program_config_dir = root_config_dir.join("minesweeper_rs");
        if !program_config_dir.is_dir() {
            fs::create_dir(&program_config_dir).or(Err(()))?;
        }
        let config_file_path = program_config_dir.join("config.ini");
        if config_file_path.exists() {
            fs::remove_file(&config_file_path).or(Err(()))?;
        }
        
        
        
        
        let mut ini = Ini::new();
        ini.with_general_section()
        .set("graphics_type", self.graphics_type.to_string())
        .set("legacy_rng", self.legacy_rng.to_string())
        .set("difficulty", self.difficulty.difficulty_type.to_string())
        .set("num_rows", self.difficulty.grid_height.to_string())
        .set("num_columns", self.difficulty.grid_width.to_string())
        .set("num_mines", self.difficulty.num_mines.to_string())
        .set("window_pos_x", self.window_position.x.to_string())
        .set("window_pos_y", self.window_position.y.to_string())
        .set("sound_enabled", self.sound_enabled.to_string())
        .set("marks_enabled", self.marks_enabled.to_string())
        .set("colour_enabled", self.colour_enabled.to_string())
        .set("beginner_name", &self.beginner_score.name)
        .set("beginner_time", self.beginner_score.time.to_string())
        .set("intermediate_name", &self.intermediate_score.name)
        .set("intermediate_time", self.intermediate_score.time.to_string())
        .set("expert_name", &self.expert_score.name)
        .set("expert_time", self.expert_score.time.to_string());

        ini.write_to_file(&config_file_path).or(Err(()))

    }

}


#[derive(Debug)]
pub enum DifficultyType {
    Beginner,
    Intermediate,
    Expert,
    Custom,
}
impl Display for DifficultyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            &DifficultyType::Beginner       => "beginner",
            &DifficultyType::Intermediate   => "intermediate",
            &DifficultyType::Expert         => "expert",
            &DifficultyType::Custom         => "custom",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug)]
pub struct Difficulty {
    difficulty_type:    DifficultyType,
    num_mines:          u32,
    grid_width:         u32,
    grid_height:        u32,
}
impl Difficulty {
    const BEGINNER:     Difficulty = Difficulty { difficulty_type: DifficultyType::Beginner, num_mines: 10, grid_width: 9,  grid_height: 9  };
    const INTERMEDIATE: Difficulty = Difficulty { difficulty_type: DifficultyType::Intermediate, num_mines: 40, grid_width: 16, grid_height: 16 };
    const EXPERT:       Difficulty = Difficulty { difficulty_type: DifficultyType::Beginner, num_mines: 99, grid_width: 30, grid_height: 16 };

    pub fn dimensions(&self) -> (u32, u32) {
        (self.grid_width, self.grid_height)
    }

    pub fn num_mines(&self) -> u32 {
        self.num_mines
    }

    pub fn new(grid_width: u32, grid_height: u32, num_mines: u32) -> Difficulty {
        let grid_width = grid_width.clamp(9, 30);
        let grid_height = grid_height.clamp(9, 24);
        let num_mines = num_mines.clamp(10, (grid_height - 1) * (grid_width - 1));
        Difficulty { difficulty_type: DifficultyType::Custom, num_mines, grid_width, grid_height }
    }
}
impl Default for Difficulty {
    fn default() -> Difficulty {
        Self::BEGINNER
    }
}




#[derive(Debug)]
pub enum GraphicsType {
    OpenGL,
    Direct3D,
    Vulkan,
    Metal,
}
impl Display for GraphicsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            &Self::OpenGL   => "opengl",
            &Self::Direct3D => "direct3d",
            &Self::Vulkan   => "vulkan",
            &Self::Metal    => "metal",
        };
        write!(f, "{name}")
    }
}
#[derive(Debug)]
pub struct HighScore {
    name: String,
    time: u32,
}
impl Default for HighScore {
    fn default() -> HighScore {
        HighScore { name: String::from("Anonymous"), time: 999 }
    }
}
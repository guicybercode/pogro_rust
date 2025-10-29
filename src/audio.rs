pub struct AudioSystem {
    menu_playing: bool,
    game_playing: bool,
}

impl AudioSystem {
    pub fn new() -> Self {
        Self {
            menu_playing: false,
            game_playing: false,
        }
    }

    pub fn update_menu(&mut self, _dt: f32) {
        self.menu_playing = true;
    }

    pub fn update_game(&mut self, _dt: f32) {
        if !self.game_playing {
            self.game_playing = true;
            self.menu_playing = false;
        }
    }

    pub fn play_hit_sound(&mut self) {
    }

    pub fn play_select_sound(&mut self) {
    }

    pub fn play_score_sound(&mut self) {
    }

    pub fn stop(&mut self) {
        self.menu_playing = false;
        self.game_playing = false;
    }
}

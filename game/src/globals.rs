use chrono::{offset, DateTime, Utc};
use godot::engine::file_access::ModeFlags;
use godot::engine::FileAccess;
use godot::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, GodotClass)]
#[class(base=Object)]
pub(crate) struct Globals {
    base: Base<Object>,
    #[var]
    pub(crate) highscore: u32,
    pub(crate) when: Option<DateTime<Utc>>,
    last_score: u32,
}

const DATA_VERSION: u32 = 1;

#[godot_api]
impl IObject for Globals {

    fn init(base: Base<Object>) -> Self {
        godot_print!("INIT CALLED");
        let mut globals = Globals {
            base,
            highscore: 0,
            when: None,
            last_score: 0,
        };
        globals.retrieve_score();
        globals
    }
}

#[godot_api]
impl Globals {

    #[func]
    pub(crate) fn save(&mut self) {
        if let Err(err) = self.inner_store_score() {
            godot_error!("{}", err);
        } else {
            self.retrieve_score();
        }
    }

    #[func]
    fn retrieve_score(&mut self) {
        match self.inner_retrieve_score() {
            Ok(data @ SaveData {version: Some(DATA_VERSION), ..}) => {
                self.highscore = data.highscore;
                self.last_score = data.highscore;
                self.when = Some(data.when);
            }
            Ok(_) => {
                godot_warn!("old data, ignoring");
                self.highscore = 0;
                self.when = None;
            }
            Err(err) => {
                godot_error!("{}", err);
                self.highscore = 0;
                self.when = None;
            }
        }
    }

    fn inner_store_score(&self) -> Result<(), String> {
        if self.highscore <=self.last_score {
            return Ok(()) /* guard */
        }

        let mut rec = FileAccess::open("user://savegame.json".into(), ModeFlags::WRITE)
            .ok_or("couldn’t open savegame for writing")?;
        let highscore = self.highscore;
        let when = offset::Utc::now();
        let version = Some(DATA_VERSION);
        let resource = SaveData {when, highscore, version};
        rec.store_string(
            serde_json::to_string(&resource)
            .map_err(|err| err.to_string())?
            .into()
        );
        rec.close();
        Ok(())
    }

    fn inner_retrieve_score(&self) -> Result<SaveData, String> {
        let mut rec = FileAccess::open("user://savegame.json".into(), ModeFlags::READ)
            .ok_or("couldn’t open savegame for reading")?;
        let content = rec.get_as_text().to_string();
        let resource: SaveData = serde_json::from_str(&content)
            .map_err(|err| err.to_string())?;
        rec.close();
        godot_print!("retrieve high-score stored at {}", resource.when);
        Ok(resource)
    }
}


#[derive(Debug, Deserialize, Serialize)]
struct SaveData {
    when: DateTime<Utc>,
    highscore: u32,
    version: Option<u32>,
}

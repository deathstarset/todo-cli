use chrono::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use rustyline::DefaultEditor;
use std::{
    error::Error,
    time::{Duration, Instant, UNIX_EPOCH},
};

pub fn read_line(prompt: &str, initial: &String) -> Result<String, Box<dyn Error>> {
    let mut rl = DefaultEditor::new()?;
    let field = rl.readline_with_initial(prompt, (initial, ""))?;
    Ok(field)
}

pub fn gen_date() -> String {
    let instant = Instant::now();
    let instant_sec_epoch =
        instant.elapsed().as_secs() + UNIX_EPOCH.elapsed().expect("error in time").as_secs();
    let time_instant_created = UNIX_EPOCH + Duration::from_secs(instant_sec_epoch);
    let datetime: DateTime<Local> = time_instant_created.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn gen_random() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(10)
        .map(|i| char::from(i))
        .collect()
}

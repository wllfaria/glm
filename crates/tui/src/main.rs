use glm::{FileManager, ListState};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap_or(String::from("."));
    let fm = FileManager::<ListState>::new(path)?;
    let state = fm.get_state();

    for item in &state.items {
        println!("{item:?}");
    }

    Ok(())
}

pub fn open() -> anyhow::Result<()> {
    opener::open("spotify:")?;
    Ok(())
}

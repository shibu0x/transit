use keyring::Entry;

const SERVICE: &str = "transit";
const GOOGLE_ACCOUNT: &str = "google";

pub fn save_google_refresh_token(refresh_token: &str) -> anyhow::Result<()> {
    let entry = Entry::new(SERVICE, GOOGLE_ACCOUNT)?;

    entry.set_password(refresh_token)?;

    Ok(())
}

pub fn get_google_refresh_token() -> anyhow::Result<String> {
    let entry = Entry::new(SERVICE, GOOGLE_ACCOUNT)?;

    let refresh_token = entry.get_password()?;

    Ok(refresh_token)
}
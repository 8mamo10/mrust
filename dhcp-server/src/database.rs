use rusqlite::Connection;
use std::net::Ipv4Addr;

pub fn select_addresses(
    con: &Connection,
    deleted: Option<u8>,
) -> Result<Vec<Ipv4Addr>, failure::Error> {
    Ok(Vec::new())
}

use pnet::util::MacAddr;
use rusqlite::{params, Connection};
use std::net::Ipv4Addr;

pub fn select_addresses(
    con: &Connection,
    deleted: Option<u8>,
) -> Result<Vec<Ipv4Addr>, failure::Error> {
    Ok(Vec::new())
}

pub fn select_entry(
    con: &Connection,
    mac_addr: MacAddr,
) -> Result<Option<Ipv4Addr>, failure::Error> {
    let mut stmnt = con.prepare("SELECT ip_addr FROM lease_entries WHERE mac_addr = ?1")?;
    let mut row = stmnt.query(params![mac_addr.to_string()])?;
    if let Some(entry) = row.next()? {
        let ip = entry.get(0)?;
        let ip_string: String = ip;
        Ok(Some(ip_string.parse()?))
    } else {
        info!("specified MAC addr was not found");
        Ok(None)
    }
}

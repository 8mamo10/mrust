use pnet::util::MacAddr;
use rusqlite::{params, Connection, Transaction};
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

pub fn count_records_by_mac_addr(
    con: &Connection,
    mac_addr: MacAddr,
) -> Result<u8, failure::Error> {
    Ok(0)
}

pub fn insert_entry(
    tx: &Transaction,
    mac_addr: MacAddr,
    ip_addr: Ipv4Addr,
) -> Result<(), failure::Error> {
    Ok(())
}

pub fn update_entry(
    tx: &Transaction,
    mac_addr: MacAddr,
    ip_addr: Ipv4Addr,
    deleted: u8,
) -> Result<(), failure::Error> {
    Ok(())
}

pub fn delete_entry(tx: &Transaction, mac_addr: MacAddr) -> Result<(), failure::Error> {
    Ok(())
}

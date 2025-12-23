use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::utils::{
    record::{RecordKind, read_record, write_record},
    value::Value,
};

#[derive(Clone, Debug)]
pub struct Entry {
    /// the key of the entry
    key: String,
    /// the value of the entry
    value: Value,
}

#[derive(Debug)]
pub struct SsTable {
    /// the path to the sstable file
    path: PathBuf, 
    /// the entries in the sstable
    entries: Vec<Entry>, 
    /// the minimum key in the sstable
    min_key: String, 
    /// the maximum key in the sstable
    max_key: String, 
}

impl SsTable {
    pub fn create(path: impl AsRef<Path>, entries: Vec<(String, Value)>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = File::create(&path)?;
        let entry_count: u32 = entries
            .len()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "too many entries"))?;
        file.write_all(&entry_count.to_le_bytes())?;

        for (key, value) in &entries {
            match value {
                Value::Present(bytes) => {
                    write_record(&mut file, RecordKind::Set, key, bytes)?;
                }
                Value::Deleted => {
                    write_record(&mut file, RecordKind::Delete, key, &[])?;
                }
            }
        }
        let min_key = entries.first().map(|(key, _)| key.clone()).unwrap();
        let max_key = entries.last().map(|(key, _)| key.clone()).unwrap();

        // Write footer: [min_key_len:4][min_key:var][max_key_len:4][max_key:var][footer_offset:8]
        let footer_offset = file.stream_position()?;
        file.write_all(&(min_key.len() as u32).to_le_bytes())?;
        file.write_all(min_key.as_bytes())?;
        file.write_all(&(max_key.len() as u32).to_le_bytes())?;
        file.write_all(max_key.as_bytes())?;
        file.write_all(&footer_offset.to_le_bytes())?;  // 8 bytes, always last

        file.flush()?;
        file.sync_all()?;

        let stored_entries = entries
            .into_iter()
            .map(|(key, value)| Entry { key, value })
            .collect();

        Ok(Self {
            path,
            entries: stored_entries,
            min_key,
            max_key,
        })
    }

    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut file = File::open(&path)?;
        let entry_count = read_entry_count(&mut file)?;
        let mut entries = Vec::with_capacity(entry_count as usize);

        for _ in 0..entry_count {
            let record = read_record(&mut file)?
                .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "sstable truncated"))?;

            let value = match record.kind {
                RecordKind::Set => Value::from_bytes(record.value),
                RecordKind::Delete => Value::Deleted,
            };

            entries.push(Entry {
                key: record.key,
                value,
            });
        }

        let (min_key, max_key) = read_footer(&mut file)?;

        Ok(Self { path, entries, min_key, max_key })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.entries
            .binary_search_by(|entry| entry.key.as_str().cmp(key))
            .ok()
            .map(|idx| self.entries[idx].value.clone())
    }

    pub fn might_contain_key(&self, key: &str) -> bool {
        key >= self.min_key.as_str() && key <= self.max_key.as_str()
    }
}

fn read_entry_count<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_footer<R: Read + Seek>(reader: &mut R) -> io::Result<(String, String)> {
    // 1. Read footer_offset from the last 8 bytes
    reader.seek(SeekFrom::End(-8))?;
    let mut offset_buf = [0u8; 8];
    reader.read_exact(&mut offset_buf)?;
    let footer_offset = u64::from_le_bytes(offset_buf);

    // 2. Seek to footer start and read min_key
    reader.seek(SeekFrom::Start(footer_offset))?;
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let min_key_len = u32::from_le_bytes(len_buf) as usize;
    let mut min_key_bytes = vec![0u8; min_key_len];
    reader.read_exact(&mut min_key_bytes)?;
    let min_key = String::from_utf8(min_key_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("invalid min_key: {e}")))?;

    // 3. Read max_key
    reader.read_exact(&mut len_buf)?;
    let max_key_len = u32::from_le_bytes(len_buf) as usize;
    let mut max_key_bytes = vec![0u8; max_key_len];
    reader.read_exact(&mut max_key_bytes)?;
    let max_key = String::from_utf8(max_key_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("invalid max_key: {e}")))?;

    Ok((min_key, max_key))
}
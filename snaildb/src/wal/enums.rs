use crate::utils::record::RecordKind;

pub enum WriteCommand {
    WriteRecord {
        kind: RecordKind,
        key: String,
        value: Vec<u8>,
    },
    Flush,
    Reset,
    Shutdown,
}


use std::fmt;

pub struct SnapshotDescription {
    pub snapshot_start_time: String,
    pub qumulo_unique_id: String,
    pub snapshot_description: String,
}

impl fmt::Display for SnapshotDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {} - {}",
            self.snapshot_start_time, self.qumulo_unique_id, self.snapshot_description
        )
    }
}

impl fmt::Debug for SnapshotDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {} - {}",
            self.snapshot_start_time, self.qumulo_unique_id, self.snapshot_description
        )
    }
}
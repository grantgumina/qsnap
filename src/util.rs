use rusoto_ec2::{Tag, Volume, VolumeStatusItem};
use std::fmt;

pub fn check_all_volumes_for_status(volume_statuses: Vec<VolumeStatusItem>, status: &str) -> bool {
    let mut volumes_match_status = true;

    for vs in volume_statuses.into_iter() {
        if vs.volume_status.unwrap().status != Some(status.to_string()) {
            volumes_match_status = false;
            break;
        }
    }

    return volumes_match_status;
}

pub fn check_all_volumes_for_state(volumes: Vec<Volume>, state: &str) -> bool {
    let mut volumes_state_matches = true;

    for vs in volumes.into_iter() {
        if vs.state != Some(state.to_string()) {
            volumes_state_matches = false;
            break;
        }
    }

    return volumes_state_matches;
}

// Iterate through tags and find value for given key
pub fn get_tag_value(tags: &Vec<Tag>, key: &str) -> Option<String> {
    let tag = tags
        .into_iter()
        .find(|&tag| tag.key == Some(key.to_string()));
    let value = tag.unwrap().value.to_owned();

    return value;
}

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

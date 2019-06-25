pub const QSNAP_DEFAULT_HELP_MESSAGE: &str = "Use `qsnap -h` for help";
pub const EC2_INSTANCES_NOT_FOUND: &str =
    "No instances found. Make sure all IP addresses are valid.";
pub const EC2_DESCRIBE_INSTANCE_ERROR: &str = "Error with EC2 instance description request";
pub const EC2_DESCRIBE_SNAPSHOT_ERROR: &str = "Error with EC2 snapshot description request";
pub const EC2_SNAPSHOTS_NOT_FOUND_ERROR: &str =
    "No snapshots found. Make sure all IP addresses are valid.";
pub const EC2_CREATE_SNAPSHOT_ERROR: &str = "Error creating a snapshot";

pub const EC2_VOLUME_DELETE_ERROR: &str = "Error deleting volume";
pub const EC2_DESCRIBE_VOLUME_ERROR: &str = "Error getting volume information";

pub const EC2_CREATE_VOLUME_ERROR: &str = "Error creating volume from snapshot";
pub const EC2_ATTACH_VOLUME_ERROR: &str = "Error attaching volume to instance";
pub const EC2_DEATTACH_VOLUME_ERROR: &str = "Error detatching volume from instance";

pub const EC2_DESCRIBE_VOLUME_STATUS_ERROR: &str = "Error getting volume status";

// Snapshot tags
pub const EBS_SNAPSHOT_QUMULO_TAG_KEY: &str = "qumulo-cluster-snapshot";
pub const EBS_SNAPSHOT_UNIQUE_ID_TAG_KEY: &str = "qumulo-snapshot-unique-id";
pub const EBS_SNAPSHOT_DRIVE_MAPPING_TAG_KEY: &str = "qumulo-drive-mapping";
pub const EBS_SNAPSHOT_EC2_INSTANCE_ID_TAG_KEY: &str = "qumulo-ec2-instance-id";
pub const EBS_SNAPSHOT_DRIVE_TYPE_TAG_KEY: &str = "qumulo-drive-type";

// Volume tags
pub const EBS_RESTORE_VOLUME_TAG_KEY: &str = "qumulo-ebs-restore-volume";

// Operation messages
pub const VOLUMES_DETATCHED_MESSAGE: &str = "Volumes detached";
pub const VOLUMES_DELETED_MESSAGE: &str = "Volumes deleted";
pub const WAITING_FOR_VOLUMES_MESSAGE: &str = "Creating and attaching volumes...";
pub const CLUSTER_RESTORE_SUCCESS: &str = "Cluster restored";

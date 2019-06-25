extern crate clap;
extern crate haikunator;
extern crate rusoto_core;
extern crate rusoto_ec2;
extern crate spinners;

#[macro_use]
extern crate prettytable;

use clap::{App, Arg, ArgMatches, SubCommand};
use haikunator::Haikunator;
use if_chain::if_chain;
use prettytable::Table;
use rusoto_core::Region;
use rusoto_ec2::{
    AttachVolumeRequest, CreateSnapshotRequest, CreateVolumeRequest,
    DescribeSnapshotsResult, DescribeSnapshotsRequest, EbsInstanceBlockDevice,
    Ec2, Ec2Client, Filter, InstanceBlockDeviceMapping, Tag, TagSpecification,
};
use spinners::{Spinner, Spinners};
use std::process;
use std::{thread, time};

mod aws;
mod constants;
pub mod util;

// fn turn_off_cluster(ip_addresses: &Vec<String>, ec2_client: &Ec2Client) {
// Find instances for each ip address
// let ec2_instances = get_instances_by_ip_address(&ip_addresses_string_vector, &ec2_client);
// }

fn list_snapshots(matches: &ArgMatches) {
    if_chain! {

        if let Some(ip_addresses) = matches.values_of("node_ip_addresses");

        then {

            let ec2_client = Ec2Client::new(Region::UsWest2);
            let x: Vec<&str> = ip_addresses.collect();
            let ip_addresses_string_vector: Vec<String> =
                x.into_iter().map(|ipa| ipa.to_string()).collect();

            let cluster_block_device_mappings: Vec<InstanceBlockDeviceMapping> = aws::get_ebs_volume_mappings(&ip_addresses_string_vector, &ec2_client);
            let cluster_block_devices: Vec<Option<EbsInstanceBlockDevice>> = cluster_block_device_mappings.into_iter().map(|cbdm| cbdm.ebs).collect();
            let volume_ids: Vec<String> = cluster_block_devices.into_iter().filter_map(|cbd| cbd?.volume_id).collect();

            let snapshot_request_filters = Some(vec![
                Filter {
                    name: Some("volume-id".to_string()),
                    values: Some(volume_ids),
                },
                Filter {
                    name: Some("tag-key".to_string()),
                    values: Some(vec![constants::EBS_SNAPSHOT_QUMULO_TAG_KEY.to_string()]),
                }
            ]);

            let describe_snapshots_request = DescribeSnapshotsRequest {
                filters: snapshot_request_filters,
                ..Default::default()
            };

            let snapshot_descriptions: DescribeSnapshotsResult = ec2_client.describe_snapshots(describe_snapshots_request).sync().expect(constants::EC2_DESCRIBE_SNAPSHOT_ERROR);

            match snapshot_descriptions.snapshots {
                Some(snapshots) => {

                    let mut qumulo_snapshot_unique_ids: Vec<String> = vec![];
                    let mut qumulo_snapshot_descriptions: Vec<util::SnapshotDescription> = vec![];

                    for snapshot in snapshots.into_iter() {

                        let tags = snapshot.tags.unwrap();
                        let st = snapshot.start_time.unwrap().clone();
                        let sd = snapshot.description.unwrap().clone();

                        for tag in tags.into_iter() {

                            // TODO - clean this up
                            let tag_value = tag.value.unwrap().to_string();
                            let tag_key = tag.key.unwrap();
                            let tv = tag_value.to_string();

                            if tag_key == constants::EBS_SNAPSHOT_UNIQUE_ID_TAG_KEY  && !qumulo_snapshot_unique_ids.contains(&tag_value) {

                                let sd = util::SnapshotDescription {
                                    snapshot_start_time: st.to_owned(),
                                    qumulo_unique_id: tag_value,
                                    snapshot_description: sd.to_owned(),
                                };

                                qumulo_snapshot_descriptions.push(sd);
                                qumulo_snapshot_unique_ids.push(tv);

                            }

                        }

                    }

                    let mut table = Table::new();

                    table.add_row(row!["Created Time", "Unique Identifier Tag", "Description"]);

                    for qsd in &qumulo_snapshot_descriptions {
                        table.add_row(row![qsd.snapshot_start_time, qsd.qumulo_unique_id, qsd.snapshot_description]);
                    }

                    table.printstd();

                },
                None => {
                    println!("{:#?}", constants::EC2_SNAPSHOTS_NOT_FOUND_ERROR);
                    process::exit(1);
                }
            }

        }

    }
}

fn restore_cluster(matches: &ArgMatches) {
    if_chain! {

        if let ip_addresses = matches.values_of("node_ip_addresses").unwrap();
        if let snapshot_unique_id = matches.value_of("qumulo_snapshot_unique_id").unwrap();
        if let delete_attached_drives_flag = matches.is_present("delete_attached_drives_flag");

        then {

            // Get snapshots tagged with the unique id
            // Detach EBS volumes from cluster nodes
            // Destroy the detached EBS volumes
            // Create new drives from snapshots based on mappings
            // Attach new drives

            let ec2_client = Ec2Client::new(Region::UsWest2);

            let tag_key_snapshot_unique_id = format!("tag:{}", constants::EBS_SNAPSHOT_UNIQUE_ID_TAG_KEY);
            let x: Vec<&str> = ip_addresses.collect();
            let ip_addresses_string_vector: Vec<String> = x.into_iter().map(|ipa| ipa.to_string()).collect();

            // Get snapshots for given unique id
            let snapshot_request_filters = Some(vec![
                // Unique id filter
                Filter {
                    name: Some(tag_key_snapshot_unique_id.to_string()),
                    values: Some(vec![snapshot_unique_id.to_string()]),
                },
                // Qumulo snapshot metadata filter
                Filter {
                    name: Some("tag-key".to_string()),
                    values: Some(vec![constants::EBS_SNAPSHOT_QUMULO_TAG_KEY.to_string()]),
                }
            ]);

            let describe_snapshots_request = DescribeSnapshotsRequest {
                filters: snapshot_request_filters,
                ..Default::default()
            };

            let snapshot_descriptions: DescribeSnapshotsResult = ec2_client.describe_snapshots(describe_snapshots_request).sync().expect(constants::EC2_DESCRIBE_SNAPSHOT_ERROR);

            // Detach volumes
            let cluster_block_device_mappings: Vec<InstanceBlockDeviceMapping> = aws::get_ebs_volume_mappings(&ip_addresses_string_vector, &ec2_client);
            let cluster_block_devices: Vec<Option<EbsInstanceBlockDevice>> = cluster_block_device_mappings.into_iter().map(|cbdm| cbdm.ebs).collect();
            let volume_ids: Vec<String> = cluster_block_devices.into_iter().filter_map(|cbd| cbd?.volume_id).collect();

            aws::detach_ebs_volumes(&volume_ids, &ec2_client);

            // Wait for all volumes to detach
            loop {

                let volume_statuses = aws::get_volume_statuses(&volume_ids, &ec2_client).unwrap_or_else(Vec::new);

                if util::check_all_volumes_for_status(volume_statuses, "ok") {
                    break;
                }

                thread::sleep(time::Duration::from_secs(1));

            }

            println!("{}", constants::VOLUMES_DETATCHED_MESSAGE);

            // Delete volumes if user has specified
            if delete_attached_drives_flag {
                aws::delete_ebs_volumes(&volume_ids, &ec2_client);
                println!("{}", constants::VOLUMES_DELETED_MESSAGE);
            }

            let sp = Spinner::new(Spinners::Dots9, constants::WAITING_FOR_VOLUMES_MESSAGE.to_string());

            // Create a volume for each snapshot
            // Send an AttachVolumeRequest after the volume is created
            for snapshot in snapshot_descriptions.snapshots.unwrap_or_else(Vec::new).into_iter() {

                let tags = snapshot.tags.unwrap_or(Vec::new());

                let ec2_instance_id = util::get_tag_value(&tags, constants::EBS_SNAPSHOT_EC2_INSTANCE_ID_TAG_KEY);
                let device_name = util::get_tag_value(&tags, constants::EBS_SNAPSHOT_DRIVE_MAPPING_TAG_KEY);
                let volume_type = util::get_tag_value(&tags, constants::EBS_SNAPSHOT_DRIVE_TYPE_TAG_KEY);

                let tag_specs = Some(vec![
                    TagSpecification {
                        resource_type: Some("volume".to_string()),
                        tags: Some(vec![
                            Tag {
                                key: Some(constants::EBS_RESTORE_VOLUME_TAG_KEY.to_string()),
                                value:Some("true".to_string()),
                            },
                        ]),
                    },
                ]);

                let create_volume_request = CreateVolumeRequest {
                    availability_zone: "us-west-2b".to_string(),
                    tag_specifications: tag_specs,
                    snapshot_id: snapshot.snapshot_id,
                    volume_type: volume_type,
                    ..Default::default()
                };

                let new_volume = ec2_client.create_volume(create_volume_request).sync().expect(constants::EC2_CREATE_VOLUME_ERROR);

                let new_volume_id = new_volume.volume_id.unwrap();
                let v = vec![new_volume_id.to_owned()];

                // Wait for the volume to go online
                loop {

                    let volumes = aws::get_volumes(&v, &ec2_client).unwrap_or_else(Vec::new);

                    if util::check_all_volumes_for_state(volumes, "available") {
                        break;
                    }

                    thread::sleep(time::Duration::from_secs(1));

                }

                // Attach newly created volume to instance
                let attach_volume_request = AttachVolumeRequest {
                    device: device_name.unwrap(),
                    instance_id: ec2_instance_id.unwrap(),
                    volume_id: new_volume_id.to_owned(),
                    ..Default::default()
                };

                ec2_client.attach_volume(attach_volume_request).sync().expect(constants::EC2_ATTACH_VOLUME_ERROR);

            }

            sp.stop();

            println!("\n{}", constants::CLUSTER_RESTORE_SUCCESS);

        }
    }
}

fn snapshot_cluster(matches: &ArgMatches) {
    if_chain! {

        if let Some(ip_addresses) = matches.values_of("node_ip_addresses");
        if let Some(snapshot_description) = matches.value_of("description");

        then {

            let ec2_client = Ec2Client::new(Region::UsWest2);
            let x: Vec<&str> = ip_addresses.collect();
            let ip_addresses_string_vector: Vec<String> = x.into_iter().map(|ipa| ipa.to_string()).collect();
            let qumulo_snapshot_unique_id = Haikunator::default().haikunate();

            let ec2_instances = aws::get_instances_by_ip_address(&ip_addresses_string_vector, &ec2_client);

            for compute_instance in &ec2_instances {

                let mappings = compute_instance.block_device_mappings.to_owned();

                // Iterate through the nodes and snapshot each one
                for cbdm in mappings.unwrap_or_else(Vec::new).iter() {

                    // Tag the snapshot with all the information we'll need to recreate it later
                    let c = cbdm.to_owned();
                    let device_name = c.device_name;
                    let volume_id = c.ebs.unwrap().volume_id.unwrap();
                    let volume_type = aws::get_volume_type(&volume_id, &ec2_client);
                    let ec2_instance_id = compute_instance.to_owned().instance_id;

                    // Tag the snapshot as being created by this tool
                    let qumulo_tag = Tag {
                        key: Some(constants::EBS_SNAPSHOT_QUMULO_TAG_KEY.to_string()),
                        value: Some("true".to_string()),
                    };

                    // EC2 instance ID
                    let qumulo_ec2_instance_id_tag = Tag {
                        key: Some(constants::EBS_SNAPSHOT_EC2_INSTANCE_ID_TAG_KEY.to_string()),
                        value: ec2_instance_id,
                    };

                    // Drive letter
                    let qumulo_drive_mapping_tag = Tag {
                        key: Some(constants::EBS_SNAPSHOT_DRIVE_MAPPING_TAG_KEY.to_string()),
                        value: device_name,
                    };

                    // Unique Snapshot ID we create
                    let qumulo_snapshot_unique_id_tag = Tag {
                        key: Some(constants::EBS_SNAPSHOT_UNIQUE_ID_TAG_KEY.to_string()),
                        value: Some(qumulo_snapshot_unique_id.to_string()),
                    };

                    // Drive type
                    let qumulo_snapshot_drive_type_tag = Tag {
                        key: Some(constants::EBS_SNAPSHOT_DRIVE_TYPE_TAG_KEY.to_string()),
                        value: volume_type
                    };

                    let tag_spec = TagSpecification {
                        resource_type: Some("snapshot".to_string()),
                        tags: Some(vec![
                                qumulo_tag,
                                qumulo_snapshot_unique_id_tag,
                                qumulo_drive_mapping_tag,
                                qumulo_ec2_instance_id_tag,
                                qumulo_snapshot_drive_type_tag
                            ])
                    };

                    let create_snapshot_request = CreateSnapshotRequest {
                        description: Some(snapshot_description.to_owned()),
                        volume_id: volume_id,
                        tag_specifications: Some(vec![tag_spec]),
                        ..Default::default()
                    };

                    ec2_client.create_snapshot(create_snapshot_request).sync().expect(constants::EC2_CREATE_SNAPSHOT_ERROR);

                }

                let iid = compute_instance.to_owned().instance_id.unwrap();
                println!("Snapshotted node id: {}", iid);

            }

        }

    }
}

fn main() {
    let matches = App::new("qnsap")
                    .version("0.0.1")
                    .author("Grant J. Gumina")
                    .about("Automates snapshotting your cluster's EBS drives to S3")
                    .subcommand(SubCommand::with_name("snapshot")
                        .about("Snapshots EBS drives to S3")
                        .arg(Arg::with_name("node_ip_addresses")
                            .required(true)
                            .takes_value(true)
                            .short("i")
                            .min_values(1)
                            .help("Array of node private ip-addresses (ex: 127.0.0.1, 127.0.0.2,, 127.0.0.3, 127.0.0.4)")
                        )
                        .arg(Arg::with_name("description")
                            .takes_value(true)
                            .short("d")
                            .default_value("Snapshot of a Qumulo instance EBS volume")
                            .help("Notes for this snapshot")
                        )
                    )
                    .subcommand(SubCommand::with_name("restore")
                        .about("Restores EBS volumes from S3 snapshots")
                        .arg(Arg::with_name("node_ip_addresses")
                            .required(true)
                            .takes_value(true)
                            .short("i")
                            .min_values(1)
                            .help("")
                        )
                        .arg(Arg::with_name("delete_attached_drives_flag")
                            .required(false)
                            .takes_value(false)
                            .default_value("false")
                            .short("D")
                            .help("")
                        )
                        .arg(Arg::with_name("qumulo_snapshot_unique_id")
                            .required(true)
                            .takes_value(true)
                            .short("u")
                            .help("Unique snapshot ID created by this tool. Use qsnap list for more information.")
                        )
                    )
                    .subcommand(SubCommand::with_name("list")
                        .about("List all snapshots taken of a Qumulo cluster")
                        .arg(Arg::with_name("node_ip_addresses")
                            .required(true)
                            .takes_value(true)
                            .short("i")
                            .min_values(1)
                            .help("")
                        )
                    )
                    .get_matches();

    match matches.subcommand() {
        ("snapshot", Some(m)) => snapshot_cluster(m),
        ("restore", Some(m)) => restore_cluster(m),
        ("list", Some(m)) => list_snapshots(m),
        _ => println!("{}", constants::QSNAP_DEFAULT_HELP_MESSAGE),
    }
}

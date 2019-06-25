use rusoto_ec2::{
    DeleteVolumeRequest, DescribeInstancesRequest, DescribeInstancesResult,
    DescribeVolumeStatusRequest, DescribeVolumesRequest, DetachVolumeRequest, Ec2, Ec2Client,
    Filter, Instance, InstanceBlockDeviceMapping, Reservation, Volume, VolumeStatusItem,
};
use std::process;

use super::constants;

pub fn get_volume_type(volume_id: &str, ec2_client: &Ec2Client) -> Option<String> {
    let describe_volumes_request_filters = Some(vec![Filter {
        name: Some("volume-id".to_string()),
        values: Some(vec![volume_id.to_string()]),
    }]);

    let describe_volumes_request = DescribeVolumesRequest {
        filters: describe_volumes_request_filters,
        ..Default::default()
    };

    let volume_descriptions = ec2_client
        .describe_volumes(describe_volumes_request)
        .sync()
        .expect(constants::EC2_DESCRIBE_VOLUME_ERROR);
    let volumes = volume_descriptions.volumes.unwrap();
    let volume = volumes.first();
    let x = volume.unwrap().to_owned();
    let volume_type = x.volume_type;

    return volume_type;
}

pub fn get_instances_by_ip_address(
    ip_addresses: &Vec<String>,
    ec2_client: &Ec2Client,
) -> Vec<Instance> {
    // Find instances for each ip address
    let instance_description_filters = Some(vec![Filter {
        name: Some("private-ip-address".to_string()),
        values: Some(ip_addresses.to_owned()),
    }]);

    let instance_descriptions_request = DescribeInstancesRequest {
        filters: instance_description_filters,
        ..Default::default()
    };

    let instance_descriptions_result = ec2_client
        .describe_instances(instance_descriptions_request)
        .sync()
        .expect(constants::EC2_INSTANCES_NOT_FOUND);
    let reservations: Vec<Reservation> = instance_descriptions_result.reservations.unwrap();

    let mut ec2_instances: Vec<Instance> = vec![];

    for reservation in reservations {
        for instance in reservation.instances.unwrap() {
            ec2_instances.push(instance);
        }
    }

    return ec2_instances;
}

pub fn get_ebs_volume_mappings(
    ip_addresses: &Vec<String>,
    ec2_client: &Ec2Client,
) -> Vec<InstanceBlockDeviceMapping> {
    // IP address filter
    let ec2_instance_description_filers = Some(vec![Filter {
        name: Some("private-ip-address".to_string()),
        values: Some(ip_addresses.to_owned()),
    }]);

    // Find ec2 instances by issuing a DescribeInstanceRequest with IP address filters
    let describe_instance_request = DescribeInstancesRequest {
        filters: ec2_instance_description_filers,
        ..Default::default()
    };

    let instance_descriptions: DescribeInstancesResult = ec2_client
        .describe_instances(describe_instance_request)
        .sync()
        .expect(constants::EC2_DESCRIBE_INSTANCE_ERROR);
    let reservations: Vec<Reservation> = instance_descriptions.reservations.unwrap();
    let mut cluster_block_device_mappings: Vec<InstanceBlockDeviceMapping> = vec![];

    // If no instances are found, exit the program
    if reservations.len() == 0 {
        println!("{:#?}", constants::EC2_INSTANCES_NOT_FOUND);
        process::exit(1);
    }

    for reservation in reservations {
        for instance in reservation.instances.unwrap() {
            for bdm in instance.block_device_mappings.unwrap() {
                cluster_block_device_mappings.push(bdm);
            }
        }
    }

    return cluster_block_device_mappings;
}

pub fn get_volume_statuses(
    volume_ids: &Vec<String>,
    ec2_client: &Ec2Client,
) -> Option<Vec<VolumeStatusItem>> {
    let volume_status_request = DescribeVolumeStatusRequest {
        volume_ids: Some(volume_ids.to_owned()),
        ..Default::default()
    };

    let response = ec2_client
        .describe_volume_status(volume_status_request)
        .sync()
        .expect(constants::EC2_DESCRIBE_VOLUME_STATUS_ERROR);

    return response.volume_statuses;
}

pub fn get_volumes(volume_ids: &Vec<String>, ec2_client: &Ec2Client) -> Option<Vec<Volume>> {
    let volume_request = DescribeVolumesRequest {
        volume_ids: Some(volume_ids.to_owned()),
        ..Default::default()
    };

    let response = ec2_client
        .describe_volumes(volume_request)
        .sync()
        .expect(constants::EC2_DESCRIBE_VOLUME_ERROR);

    return response.volumes;
}

pub fn detach_ebs_volumes(volume_ids: &Vec<String>, ec2_client: &Ec2Client) {
    for vid in volume_ids {
        let detach_volume_request = DetachVolumeRequest {
            volume_id: vid.to_string(),
            force: Some(true),
            ..Default::default()
        };

        ec2_client
            .detach_volume(detach_volume_request)
            .sync()
            .expect(constants::EC2_DEATTACH_VOLUME_ERROR);
    }
}

pub fn delete_ebs_volumes(volume_ids: &Vec<String>, ec2_client: &Ec2Client) {
    for vid in volume_ids {
        let delete_volume_request = DeleteVolumeRequest {
            volume_id: vid.to_string(),
            ..Default::default()
        };

        ec2_client
            .delete_volume(delete_volume_request)
            .sync()
            .expect(constants::EC2_VOLUME_DELETE_ERROR);
    }
}

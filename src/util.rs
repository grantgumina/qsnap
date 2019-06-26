use rusoto_ec2::{Tag, Volume, VolumeStatusItem};
use rusoto_core::{Region};
use super::QsnapError as QsnapError;
use super::constants;

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

pub fn get_region_from_user_input(user_region: &str) -> Result<Region, QsnapError> {

    match user_region {

        "us-west-1" => Ok(Region::UsWest1),
        "us-west-2" => Ok(Region::UsWest2),
        "us-east-1" => Ok(Region::UsEast1),
        "us-east-2" => Ok(Region::UsEast2),
        "ap-east-1" => Ok(Region::ApEast1),
        "ap-south-1" => Ok(Region::ApSouth1),
        "ap-northeast-1" => Ok(Region::ApNortheast1),
        "ap-northeast-2" => Ok(Region::ApNortheast2),
        "ap-southeast-1" => Ok(Region::ApSoutheast1),
        "ap-southeast-2" => Ok(Region::ApSoutheast2),
        "ca-central-1" => Ok(Region::CaCentral1),
        "cn-north-1" => Ok(Region::CnNorth1),
        "cn-northwest-1" => Ok(Region::CnNorthwest1),
        "eu-central-1" => Ok(Region::EuCentral1),
        "eu-west-1" => Ok(Region::EuWest1),
        "eu-west-2" => Ok(Region::EuWest2),
        "eu-west-3" => Ok(Region::EuWest3),
        "eu-north-1" => Ok(Region::EuNorth1),
        "sa-east-1" => Ok(Region::SaEast1),
        "us-gov-east-1" => Ok(Region::UsGovEast1),
        "us-gov-west-1" => Ok(Region::UsGovWest1),
        _ => Err(QsnapError::new(constants::UNKNOWN_REGION_ERROR)),

    }

}

// Iterate through tags and find value for given key
pub fn get_tag_value(tags: &Vec<Tag>, key: &str) -> Option<String> {
    let tag = tags
        .into_iter()
        .find(|&tag| tag.key == Some(key.to_string()));
    let value = tag.unwrap().value.to_owned();

    return value;
}
use iokit::prelude::*;

#[test]
fn matches_ioresources_service() -> iokit::Result<()> {
    let Some(service) = matching_service("IOResources")? else {
        panic!("IOResources service was not found");
    };

    let class_name = service.class_name()?;
    assert_eq!(class_name, "IOResources");
    assert!(service.path(SERVICE_PLANE)?.contains("IOResources"));
    assert!(matching_service_entry_id(service.registry_entry_id()?).is_some());
    Ok(())
}

#[test]
fn matching_dictionaries_mirror_iokit_helpers() {
    use iokit::{CFValue, MatchingDictionary};
    use std::collections::BTreeMap;

    let expected =
        |key: &str, value: CFValue| CFValue::Dictionary(BTreeMap::from([(key.to_owned(), value)]));
    assert_eq!(
        MatchingDictionary::class("IOResources").to_value(),
        expected("IOProviderClass", CFValue::from("IOResources"))
    );
    assert_eq!(
        MatchingDictionary::name("IOResources").to_value(),
        expected("IONameMatch", CFValue::from("IOResources"))
    );
    assert_eq!(
        MatchingDictionary::bsd_name("disk0").to_value(),
        expected("BSD Name", CFValue::from("disk0"))
    );
    assert_eq!(
        MatchingDictionary::registry_entry_id(0x1234).to_value(),
        expected("IORegistryEntryID", CFValue::Integer(0x1234))
    );
}

#[test]
fn vendor_product_dictionaries_use_the_documented_keys() {
    use iokit::{CFValue, MatchingDictionary, PROPERTY_MATCH_KEY, PROVIDER_CLASS_KEY};

    let CFValue::Dictionary(usb) = MatchingDictionary::usb_device(0x05ac, 0x1234).to_value() else {
        panic!("USB matching dictionary is not a dictionary");
    };
    assert_eq!(usb[PROVIDER_CLASS_KEY], CFValue::from("IOUSBHostDevice"));
    assert_eq!(usb["idVendor"], CFValue::Integer(0x05ac));
    assert_eq!(usb["idProduct"], CFValue::Integer(0x1234));

    let CFValue::Dictionary(hid) = MatchingDictionary::hid_device(0x05ac, 0x0250)
        .with_property_match("PrimaryUsagePage", 1_u32)
        .to_value()
    else {
        panic!("HID matching dictionary is not a dictionary");
    };
    assert_eq!(hid[PROVIDER_CLASS_KEY], CFValue::from("IOHIDDevice"));
    let CFValue::Dictionary(properties) = &hid[PROPERTY_MATCH_KEY] else {
        panic!("IOPropertyMatch is not a dictionary");
    };
    assert_eq!(properties["VendorID"], CFValue::Integer(0x05ac));
    assert_eq!(properties["ProductID"], CFValue::Integer(0x0250));
    assert_eq!(properties["PrimaryUsagePage"], CFValue::Integer(1));
}

#[test]
fn property_match_replaces_a_non_dictionary_entry() {
    use iokit::{CFValue, MatchingDictionary, PROPERTY_MATCH_KEY};

    let CFValue::Dictionary(entries) = MatchingDictionary::new()
        .with_matching_key(PROPERTY_MATCH_KEY, "not a dictionary")
        .with_property_match("IOClass", "IOResources")
        .to_value()
    else {
        panic!("matching dictionary is not a dictionary");
    };
    let CFValue::Dictionary(properties) = &entries[PROPERTY_MATCH_KEY] else {
        panic!("IOPropertyMatch is not a dictionary");
    };
    assert_eq!(properties["IOClass"], CFValue::from("IOResources"));
}

#[test]
fn matching_dictionaries_find_services() -> iokit::Result<()> {
    use iokit::MatchingDictionary;

    let by_class = MatchingDictionary::class("IOResources")
        .first_service()?
        .expect("IOResources by class");
    assert_eq!(by_class.class_name()?, "IOResources");

    let by_name = MatchingDictionary::name("IOResources").services()?;
    assert!(by_name.iter().any(|service| service.is_equal_to(&by_class)));

    let by_id = MatchingDictionary::registry_entry_id(by_class.registry_entry_id()?)
        .first_service()?
        .expect("IOResources by registry entry ID");
    assert!(by_id.is_equal_to(&by_class));

    let platform = MatchingDictionary::class("IOPlatformExpertDevice")
        .first_service()?
        .expect("IOPlatformExpertDevice");
    let Some(iokit::CFValue::String(uuid)) = platform.property("IOPlatformUUID")? else {
        panic!("IOPlatformUUID is not a string");
    };
    let by_property = MatchingDictionary::class("IOPlatformExpertDevice")
        .with_property_match("IOPlatformUUID", uuid.as_str())
        .services()?;
    assert_eq!(by_property.len(), 1);
    assert!(by_property[0].is_equal_to(&platform));
    assert!(MatchingDictionary::class("IOPlatformExpertDevice")
        .with_property_match("IOPlatformUUID", "00000000-0000-0000-0000-000000000000")
        .services()?
        .is_empty());

    assert!(MatchingDictionary::class("DoesNotExist_______XYZ123")
        .services()?
        .is_empty());
    assert!(MatchingDictionary::class("DoesNotExist_______XYZ123")
        .first_service()?
        .is_none());
    Ok(())
}

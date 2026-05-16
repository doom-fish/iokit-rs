use iokit::prelude::*;

#[test]
fn opens_power_management_connection() -> iokit::Result<()> {
    let connect = find_power_management()?;
    let _clone = connect.clone();

    if let Some(service) = connect.get_service() {
        let class_name = service.class_name()?;
        assert!(!class_name.is_empty());
    }

    Ok(())
}

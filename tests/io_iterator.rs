use iokit::prelude::*;

#[test]
fn iterates_matching_services() -> iokit::Result<()> {
    let Some(mut iterator) = matching_services_iterator("IOResources")? else {
        panic!("IOResources iterator was not returned");
    };

    assert!(iterator.is_valid());
    let Some(first) = iterator.next_service() else {
        panic!("IOResources iterator did not produce a service");
    };

    assert_eq!(first.class_name()?, "IOResources");
    iterator.reset();
    assert!(iterator.next_service().is_some());
    Ok(())
}

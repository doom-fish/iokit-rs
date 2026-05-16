use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    if let Some(mut iterator) = matching_services_iterator("IOResources")? {
        let valid = iterator.is_valid();
        println!("iterator valid = {valid}");
        if let Some(service) = iterator.next_service() {
            let class_name = service.class_name()?;
            println!("first iterator match = {class_name}");
        } else {
            println!("iterator produced no services");
        }
    } else {
        println!("no IOResources iterator was returned");
    }

    Ok(())
}

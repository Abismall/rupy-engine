#[cfg(test)]
use rupy::graphics::global::{
    get_adapter, get_device, get_instance, get_queue, initialize_instance,
};
use std::sync::Arc;

#[tokio::test]
async fn test_initialize_instance() {
    initialize_instance()
        .await
        .expect("Failed to initialize GPU resources");

    let device = get_device().expect("Failed to retrieve device from cache");
    let queue = get_queue().expect("Failed to retrieve queue from cache");

    assert!(
        Arc::strong_count(&device) > 1,
        "Device was not cached properly"
    );
    assert!(
        Arc::strong_count(&queue) > 1,
        "Queue was not cached properly"
    );

    let adapter = get_adapter().expect("Failed to retrieve adapter from cache");
    let instance = get_instance().expect("Failed to retrieve instance from cache");
    assert!(
        Arc::strong_count(&adapter) > 1,
        "Adapter was not cached properly"
    );
    assert!(
        Arc::strong_count(&instance) > 1,
        "Instance was not cached properly"
    );
}

#[tokio::test]
async fn test_reinitialize_instance() {
    initialize_instance()
        .await
        .expect("Failed to initialize GPU resources");

    initialize_instance()
        .await
        .expect("Failed to initialize GPU resources");

    let device_count = Arc::strong_count(&get_device().unwrap());
    let queue_count = Arc::strong_count(&get_queue().unwrap());

    assert!(device_count > 1, "Device should still be cached");
    assert!(queue_count > 1, "Queue should still be cached");
}

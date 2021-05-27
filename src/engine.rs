use maligog::vk;
pub struct Engine {
    device: maligog::device::Device,
}

impl Engine {
    pub fn new() -> Self {
        let entry = maligog::entry::Entry::new().unwrap();
        let instance =
            entry.create_instance(&[], &[maligog::name::instance::Extension::ExtDebugUtils]);
        dbg!(entry.vulkan_loader_version());

        let pdevices = instance.enumerate_physical_device();
        let pdevice = pdevices
            .iter()
            .find(|pd| pd.device_type() == vk::PhysicalDeviceType::DISCRETE_GPU)
            .unwrap();

        let (device, queue_familes) =
            pdevice.create_device(&[(&pdevice.queue_families()[0], &[1.0])]);
        let queue = queue_familes
            .first()
            .unwrap()
            .queues
            .first()
            .unwrap()
            .clone();
        let buffer = device.create_buffer(
            Some("fuck"),
            2345,
            vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR,
            maligog::MemoryLocation::GpuOnly,
        );

        let lock = buffer.lock_memory().unwrap();
        assert!(lock.mapped_slice().is_none());

        Self { device }
    }
}

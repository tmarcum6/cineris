use std::sync::Arc;

use vulkano::VulkanLibrary;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo};
use vulkano::sync::{self, GpuFuture};

pub struct VulkanContext {
    pub instance: Option<Instance>,
    pub device: Option<Device>,
    pub queue: Option<vulkano::device::Queue>,
}

impl VulkanContext {
    pub fn new() -> Self {
        Self::config();
        Self {
            instance: None,
            device: None,
            queue: None,
        }
    }

    fn config() {
        let library = VulkanLibrary::new().expect("failed to initialize Vulkan Library");
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        )
        .expect("failed to create instance");

        let physical_device = instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available");

        for family in physical_device.queue_family_properties() {
            println!(
                "Found a queue family with {:?} queue(s)",
                family.queue_count
            );
        }

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family")
            as u32;

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");

        let queue = queues.next().unwrap();

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        let source_content: Vec<i32> = (0..64).collect();
        let source = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            source_content,
        )
        .expect("failed to create source buffer");

        let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();
        let destination = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                ..Default::default()
            },
            destination_content,
        )
        .expect("failed to create destination buffer");

        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(device.clone(), Default::default());

        let mut builder = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue_family_index,
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .copy_buffer(CopyBufferInfo::buffers(source.clone(), destination.clone()))
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = sync::now(device)
            .then_execute(queue, command_buffer)
            .unwrap()
            .then_signal_fence_and_flush() // same as signal fence, and then flush
            .unwrap();
        future.wait(None).unwrap();

        let src_content = source.read().unwrap();
        let destination_content = destination.read().unwrap();
        assert_eq!(&*src_content, &*destination_content);

        println!("Everything succeeded!");
    }
}

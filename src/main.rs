#![allow(
dead_code,
unused_variables,
clippy::too_many_arguments,
clippy::unnecessary_wraps
)]

use anyhow::{anyhow, Result};
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use vulkanalia::Version;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() -> Result<()> {
	pretty_env_logger::init();

	//Window
	let event_loop=EventLoop::new();
	let window=WindowBuilder::new()
		.with_title("Vulkan Tutorial (Rust)")
		.with_inner_size(LogicalSize::new(1024,768))
		.build(&event_loop)?;

	//App
	let mut app=unsafe{App::create(&window)?};
	let mut destroying=false;
	event_loop.run(move |event, _, control_flow|{
		*control_flow=ControlFlow::Poll;
		match event{
			//Render a frame if our vulkan app is not being destroyed
			Event::MainEventsCleared if !destroying =>
				unsafe {app.render(&window)}.unwrap(),

			//destroy our vulkan app
			Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
				destroying=true;
				*control_flow=ControlFlow::Exit;
				unsafe{app.destroy();}
			},

			_ => {}
		}
	});
}


/// Our Vulkan app.
#[derive(Clone, Debug)]
struct App {
	entry: Entry,
	instance: Instance,
}

impl App {
	/// Creates our Vulkan app.
	unsafe fn create(window: &Window) -> Result<Self> {
		let loader=LibloadingLoader::new(LIBRARY)?;
		let entry=Entry::new(loader).map_err(|b| anyhow!("{}",b))?;
		let instance=create_instance(window,&entry)?;
		Ok(Self {entry,instance})
	}

	/// Renders a frame for our Vulkan app.
	unsafe fn render(&mut self, window: &Window) -> Result<()> {
		Ok(())
	}

	/// Destroys our Vulkan app.
	unsafe fn destroy(&mut self) {
		self.instance.destroy_instance(None);
	}
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}

unsafe fn create_instance(window: &Window, entry: &Entry)->Result<Instance>{
	let application_info=vk::ApplicationInfo::builder()
		.application_name(b"Vulkan Tutorial\0")
		.application_version(vk::make_version(1,0,0))
		.engine_name(b"No Engine\0")
		.engine_version(vk::make_version(1,0,0))
		.api_version(vk::make_version(1,0,0));

	let extensions=vk_window::get_required_instance_extensions(window)
		.iter()
		.map(|e| e.as_ptr())
		.collect::<Vec<_>>();

	let info=vk::InstanceCreateInfo::builder()
		.application_info(&application_info)
		.enabled_extension_names(&extensions);

	Ok(entry.create_instance(&info,None)?)
}
use crate::savedata::{
	TotalData,
	ACHIEVEMENTS_TOTAL,
	ITEMS_TOTAL
};
use crate::*;
use slint::*;

const ACHIEVEMENTS_NAMES: &str = include_str!("Achievements.txt");
const ITEMS_NAMES: &str = include_str!("Items.txt");
use include_dir::*;

const IMAGES: Dir = include_dir!("./images");
fn imbed_achivement_images() -> Vec<(slint::Image, String)>{
	use slint::*;
	let mut files: Vec<&File> = IMAGES.get_dir("achievements").unwrap().files().collect();
	files.sort_by(|a, b| {

		a.path().to_str().unwrap()
		.strip_prefix("achievements/").unwrap()
		.strip_suffix(".png")
		.unwrap().parse::<i16>().unwrap()
		.cmp(&b.path().to_str().unwrap()
		.strip_prefix("achievements/").unwrap()
		.strip_suffix(".png")
		.unwrap().parse::<i16>().unwrap())
	});
	let mut images: Vec<(slint::Image, String)> = vec![];
	for (file, name) in files.iter().zip(ACHIEVEMENTS_NAMES.lines()) {
		let image = image::load_from_memory(file.contents()).unwrap().into_rgb8();
		let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(image.as_raw(), image.width(), image.height());
		let i = Image::from_rgb8(buffer);
		images.push((i, name.to_string()));
	}
	images
}

fn imbed_items_images() -> Vec<(slint::Image, String)> {
	use slint::*;
	let mut files: Vec<&File> = IMAGES.get_dir("items").unwrap().files().collect();
	files.sort_by(|a, b| {

		a.path().to_str().unwrap()
		.strip_prefix("items/").unwrap()
		.strip_suffix(".png")
		.unwrap().parse::<i16>().unwrap()
		.cmp(&b.path().to_str().unwrap()
		.strip_prefix("items/").unwrap()
		.strip_suffix(".png")
		.unwrap().parse::<i16>().unwrap())
	});
	let mut images: Vec<(slint::Image, String)> = vec![];
	for (file, name) in files.iter().zip(ITEMS_NAMES.lines()) {
		let image = image::load_from_memory(file.contents()).unwrap().into_rgb8();
		let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(image.as_raw(), image.width(), image.height());
		let i = Image::from_rgb8(buffer);
		images.push((i, name.to_string()));
	}
	images
}


pub struct Unlocker {
	data: Rc<TotalData>,
	app: App,
}

impl Unlocker {
	pub fn new() -> Self {
		Self { data: Rc::new(TotalData::new()), app: App::new().unwrap() }
	}

	pub fn init(&self) {
		let icons = {
			let images = imbed_achivement_images();
			let mut arr = vec![];
			for (im, s) in images.iter(){
				arr.push(IsaacIcon {image: im.clone(), name: s.into()});
			}
			arr
		};
	
		let item_icons = {
			let images = imbed_items_images();
			images.iter().fold( Vec::<IsaacIcon>::new(), |mut acc, (im, s)| {
				acc.push(IsaacIcon {image: im.clone(), name: s.into()} );
				acc
			})
		};
		{
			// Add only existing saves
			let mut v = vec![];
			for (i,s) in self.data.saves.iter().enumerate() {
				if s.is_some() {
					let s = SharedString::from((i + 1).to_string());
					v.push(s);
				}
			}
			let v = std::rc::Rc::new(VecModel::from(v));
			self.app.global::<Search>().set_saves(v.clone().into());
		}
		let savefile = self.app.global::<Search>().get_Savefile() as usize;

		let items = self.data.saves[savefile - 1].unwrap()
			.items.iter()
			.enumerate()
			.fold( Vec::<Achievement>::new(), |mut acc, (x, i)| {
				acc.push(Achievement {id: x as i32, unlocked: *i});
				acc
		} );
		let achievements = self.data.saves[savefile - 1].unwrap().
			achievements.iter()
			.enumerate()
			.fold( Vec::<Achievement>::new(), |mut acc, (x, i)| {
				acc.push(Achievement {id: x as i32, unlocked: *i});
				acc
		});

		let icons = std::rc::Rc::new(VecModel::from(icons));
		let item_icons = std::rc::Rc::new(VecModel::from(item_icons));

		let achievements = std::rc::Rc::new(VecModel::from(achievements));
		let items = std::rc::Rc::new(VecModel::from(items));

		self.app.global::<Search>().set_items_icons(item_icons.clone().into());
		self.app.global::<Search>().set_items(items.clone().into());
		self.app.global::<Search>().set_icons(icons.clone().into());
		self.app.global::<Search>().set_achievements(achievements.clone().into());
	
		self.app.global::<Search>().set_indexes(Rc::new(slint::VecModel::from((0..ACHIEVEMENTS_TOTAL as i32).collect::<Vec<i32>>())).into());
		self.app.global::<Search>().set_items_indexes(Rc::new(slint::VecModel::from((0..(ITEMS_TOTAL - 1) as i32).collect::<Vec<i32>>())).into());
		self.change_save_callback(&achievements, &items);
		self.range_change_callback();
		self.search_callback();
		self.unlock_callback(&achievements);
	}

	fn change_save_callback(&self, achievements: &Rc<VecModel<Achievement>>, items: &Rc<VecModel<Achievement>>) {
		let weak_achievements = Rc::downgrade(achievements);
		let weak_items = Rc::downgrade(items);
		let weak_saves = Rc::downgrade(&self.data);
		self.app.global::<Search>().on_select_save(move |save| { 
			let achievements = weak_achievements.upgrade().unwrap();
			let items = weak_items.upgrade().unwrap();
			let saves = weak_saves.upgrade().unwrap();
			let save_data = saves.saves[(save - 1) as usize].unwrap();
			achievements.set_vec(save_data.achievements.iter().enumerate().fold( Vec::<Achievement>::new(), |mut acc, (x, i)| {
				acc.push(Achievement {id: x as i32, unlocked: *i});
				acc
			}));
			items.set_vec(save_data.items.iter().enumerate().fold( Vec::<Achievement>::new(), |mut acc, (x, i)| {
				acc.push(Achievement {id: x as i32, unlocked: *i});
				acc
			}));
			
		});

	}

	fn range_change_callback(&self) {
		let weak_app = self.app.as_weak();

		self.app.global::<Search>().on_range_change(move |x: i32, y| {
			let app = weak_app.upgrade().unwrap();
			
			app.global::<Search>().set_indexes(Rc::new(slint::VecModel::from(((x-1).max(0).min(ACHIEVEMENTS_TOTAL as i32)..y.min(ACHIEVEMENTS_TOTAL as i32)).collect::<Vec<i32>>())).into());
			app.global::<Search>().set_items_indexes(Rc::new(slint::VecModel::from(((x-1).max(0).min(ITEMS_TOTAL as i32 - 1)..y.min(ITEMS_TOTAL as i32 - 1)).collect::<Vec<i32>>())).into());
		});

	}

	fn search_callback(&self) {
		let weak_app = self.app.as_weak();
		
		self.app.global::<Search>().on_search_change(move |s| {
			let app = weak_app.upgrade().unwrap();
			let sa: String = s.into();
			let anames = ACHIEVEMENTS_NAMES.lines().into_iter().find_all( |st| st.to_lowercase().contains(sa.to_lowercase().as_str()));
			let inames = ITEMS_NAMES.lines().into_iter().find_all( |st| st.to_lowercase().contains(sa.to_lowercase().as_str()));
			if let Some(ns) = anames {
				app.global::<Search>().set_indexes(Rc::new(slint::VecModel::from(ns.iter().map(|x| *x as i32).collect::<Vec<i32>>())).into());
			}
			if let Some(ns) = inames {
				app.global::<Search>()
				.set_items_indexes(Rc::new(slint::VecModel::from(ns.iter().map(|x| *x as i32).collect::<Vec<i32>>())).into());
			}
		});
	}

	fn unlock_callback(&self, achievements: &Rc<VecModel<Achievement>>) {
		let weak_app = self.app.as_weak();
		let weak_achievements = Rc::downgrade(&achievements);
		let weak_saves = Rc::downgrade(&self.data);
		
		self.app.global::<UnlockAchievements>().on_unlock( move || {
			let achievements = weak_achievements.upgrade().unwrap();
			let app = weak_app.upgrade().unwrap();
			let saves = weak_saves.upgrade().unwrap();
			saves.saves[app.global::<Search>().get_Savefile() as usize - 1].unwrap()
				.unlock_achievements(achievements.iter().enumerate().fold( [false; ACHIEVEMENTS_TOTAL], |mut acc, (i,x)| {
					acc[i] = x.unlocked;
					acc
				}));
		});
	}

	pub fn run(&self) {
		self.app.run().unwrap();
	}
}
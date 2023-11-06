use crate::savedata::{
	TotalData,
	ACHIEVEMENTS_TOTAL,
	ITEMS_TOTAL
};
use find_all::FindAll;
use std::{rc::Rc, collections::HashSet};

use crate::*;
use slint::*;

const ACHIEVEMENTS_NAMES: &str = include_str!("Achievements.txt");
const DESCRIPTIONS: &str = include_str!("Descriptions.txt");
const ITEMS_NAMES: &str = include_str!("Items.txt");
use include_dir::*;

const IMAGES: Dir = include_dir!("./images");
fn imbed_achivement_images() -> Vec<(slint::Image, String, String)>{
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
	let mut images: Vec<(slint::Image, String, String)> = vec![];
	for ((file, name), descr ) in files.iter().zip(ACHIEVEMENTS_NAMES.lines()).zip(DESCRIPTIONS.lines()) {
		let image = image::load_from_memory(file.contents()).unwrap().into_rgba8();
		let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(image.as_raw(), image.width(), image.height());
		let i = Image::from_rgba8(buffer);
		images.push((i, name.to_string(), descr.to_string()));
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
		let image = image::load_from_memory(file.contents()).unwrap().into_rgba8();
		let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(image.as_raw(), image.width(), image.height());
		let i = Image::from_rgba8(buffer);
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
		Self { data: Rc::new(TotalData::new()), app: App::new().unwrap()}
	}

	pub fn init(&self) {
		let icons = {
			let images = imbed_achivement_images();
			let mut arr = vec![];
			for (im, s, desc) in images.iter(){
				arr.push(IsaacIcon {image: im.clone(), name: s.into(), description: desc.into()});
			}
			arr
		};
	
		let item_icons = {
			let images = imbed_items_images();
			images.iter().fold( Vec::<IsaacIcon>::new(), |mut acc, (im, s)| {
				acc.push(IsaacIcon {image: im.clone(), name: s.into(), description: "".into()} );
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
			self.app.global::<Search>().set_saves(v.into());
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

		self.app.global::<Search>().set_items_icons(item_icons.into());
		self.app.global::<Search>().set_items(items.clone().into());
		self.app.global::<Search>().set_icons(icons.into());
		self.app.global::<Search>().set_achievements(achievements.clone().into());
	
		self.app.global::<Search>().set_indexes(Rc::new(slint::VecModel::from((0..ACHIEVEMENTS_TOTAL as i32).collect::<Vec<i32>>())).into());
		self.app.global::<Search>().set_type_achievement_mask(Rc::new(slint::VecModel::from((0..ACHIEVEMENTS_TOTAL as i32).collect::<Vec<i32>>())).into());
		self.app.global::<Search>().set_search_achievement_mask(Rc::new(slint::VecModel::from((0..ACHIEVEMENTS_TOTAL as i32).collect::<Vec<i32>>())).into());
		self.app.global::<Search>().set_items_indexes(Rc::new(slint::VecModel::from((0..(ITEMS_TOTAL) as i32).collect::<Vec<i32>>())).into());
		self.app.global::<Search>().set_type_items_mask(Rc::new(slint::VecModel::from((0..(ITEMS_TOTAL) as i32).collect::<Vec<i32>>())).into());
		self.app.global::<Search>().set_search_items_mask(Rc::new(slint::VecModel::from((0..(ITEMS_TOTAL) as i32).collect::<Vec<i32>>())).into());

		self.change_save_callback(&achievements, &items);
		self.range_change_callback();
		self.search_callback();
		self.unlock_callback(&achievements);
		self.unlock_all_callback(&achievements);
		self.type_select_callback();
	}

	fn change_save_callback(&self, achievements: &Rc<VecModel<Achievement>>, items: &Rc<VecModel<Achievement>>) {
		let weak_app = self.app.as_weak();
		let weak_achievements = Rc::downgrade(achievements);
		let weak_items = Rc::downgrade(items);
		let weak_saves = Rc::downgrade(&self.data);
		self.app.global::<Search>().on_select_save(move |save| { 
			let app = weak_app.upgrade().unwrap();
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
			
			app.global::<Search>().invoke_type_select(app.global::<Search>().get_type());
		});

	}

	fn range_change_callback(&self) {
		let weak_app = self.app.as_weak();
		self.app.global::<Search>().on_range_change(move |x: i32, y| {
			let app = weak_app.upgrade().unwrap();

			let items_mask = ((x-1).max(0).min(ITEMS_TOTAL as i32)..y.min(ITEMS_TOTAL as i32 )).collect::<Vec<i32>>();
			let achievement_mask = ((x-1).max(0).min(ACHIEVEMENTS_TOTAL as i32)..y.min(ACHIEVEMENTS_TOTAL as i32)).collect::<Vec<i32>>();
			let type_items_mask: HashSet<i32> = app.global::<Search>().get_type_items_mask().iter().collect();
			let type_achievement_mask: HashSet<i32> = app.global::<Search>().get_type_achievement_mask().iter().collect();

			let mut items_index: Vec<i32> = type_items_mask.intersection(&items_mask.clone().into_iter().collect()).fold(Vec::new(), |mut acc, x| {
				acc.push(*x);
				acc
			});
			let mut achievement_index: Vec<i32> = type_achievement_mask.intersection(&achievement_mask.clone().into_iter().collect()).fold(Vec::new(), |mut acc, x| {
				acc.push(*x);
				acc
			});
			
			app.global::<Search>().set_search_items_mask(Rc::new(VecModel::from(achievement_mask)).into());
			app.global::<Search>().set_search_achievement_mask(Rc::new(VecModel::from(items_mask)).into());

			achievement_index.sort();
			items_index.sort();

			app.global::<Search>().set_indexes(Rc::new(VecModel::from(achievement_index)).into());
			app.global::<Search>().set_items_indexes(Rc::new(VecModel::from(items_index)).into());

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
				let index = ns.iter().map(|x| *x as i32).collect::<Vec<i32>>();
				let mask: HashSet<i32> = app.global::<Search>().get_type_achievement_mask().iter().collect();
				
				app.global::<Search>().set_search_achievement_mask(Rc::new(VecModel::from(index.clone())).into());
				let mut index: Vec<i32> = mask.intersection(&index.into_iter().collect()).fold(Vec::new(), |mut acc, x| {
					acc.push(*x);
					acc
				});
				index.sort();
				app.global::<Search>().set_indexes(Rc::new(VecModel::from(index)).into());
			}
			if let Some(ns) = inames {
				let index = ns.iter().map(|x| *x as i32).collect::<Vec<i32>>();
				let mask: HashSet<i32> = app.global::<Search>().get_type_items_mask().iter().collect();
				
				app.global::<Search>().set_search_items_mask(Rc::new(VecModel::from(index.clone())).into());
				let mut index: Vec<i32> = mask.intersection(&index.into_iter().collect()).fold(Vec::new(), |mut acc, x| {
					acc.push(*x);
					acc
				});
				index.sort();
				app.global::<Search>().set_items_indexes(Rc::new(VecModel::from(index)).into());

//				app.global::<Search>()
//				.set_items_indexes(Rc::new(VecModel::from(ns.iter().map(|x| *x as i32).collect::<Vec<i32>>())).into());
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
				.unlock_achievements(achievements.iter().fold( [false; ACHIEVEMENTS_TOTAL], |mut acc, x| {
					acc[x.id as usize] = x.unlocked;
					acc
				}));
		});
	}

	fn unlock_all_callback(&self, achievements: &Rc<VecModel<Achievement>>) {
		let weak_app = self.app.as_weak();
		let weak_achievements = Rc::downgrade(&achievements);
		let weak_saves = Rc::downgrade(&self.data);

		self.app.global::<UnlockAchievements>().on_unlock_all( move || {
			let achievements = weak_achievements.upgrade().unwrap();
			achievements.set_vec(achievements.iter().map(|x| {
				Achievement {id: x.id, unlocked: true}
			}).collect::<Vec::<Achievement>>());
			let app = weak_app.upgrade().unwrap();
			let saves = weak_saves.upgrade().unwrap();
			saves.saves[app.global::<Search>().get_Savefile() as usize - 1].unwrap()
				.unlock_achievements(achievements.iter().fold( [false; ACHIEVEMENTS_TOTAL], |mut acc, x| {
					acc[x.id as usize] = x.unlocked;
					acc
				}));
		});

	}

	fn type_select_callback(&self) {
		let weak_app = self.app.as_weak();

		self.app.global::<Search>().on_type_select(move |ty| {
			let app = weak_app.upgrade().unwrap();
			let indexes: (Vec<i32>, Vec<i32>) = {
				Self::type_filter(ty.as_str(), &app)
			};

			app.global::<Search>().set_type_achievement_mask(Rc::new(VecModel::from(indexes.0.clone())).into());
			app.global::<Search>().set_type_items_mask(Rc::new(VecModel::from(indexes.1.clone())).into());

			let search_mask: (HashSet<i32>, HashSet<i32>) = (
				app.global::<Search>().get_search_achievement_mask().iter().collect(),
				app.global::<Search>().get_search_items_mask().iter().collect(),
			);

			let mut indexes = (
				search_mask.0.intersection(&indexes.0.into_iter().collect()).fold(Vec::new(), |mut ac, x| {ac.push(*x); ac}),
				search_mask.1.intersection(&indexes.1.into_iter().collect()).fold(Vec::new(), |mut ac, x| {ac.push(*x); ac}),
			);
			indexes.0.sort();
			indexes.1.sort();
			app.global::<Search>().set_indexes(Rc::new(VecModel::from(indexes.0)).into());
			app.global::<Search>().set_items_indexes(Rc::new(VecModel::from(indexes.1)).into());
		})

	}

	pub fn type_filter(ty: &str, app: &App) -> (Vec<i32>, Vec<i32>) {
		match ty {
			"All" => ((0..ACHIEVEMENTS_TOTAL as i32).collect(), (0..ITEMS_TOTAL as i32).collect()),
			"Locked" => {
				let ach = app.global::<Search>().get_achievements();
				let items = app.global::<Search>().get_items();
				let mut ac = vec![];
				let mut it = vec![];
				for a in ach.iter() {
					if !a.unlocked {
						ac.push(a.id);
					}
				}
				for a in items.iter() {
					if !a.unlocked {
						it.push(a.id);
					}
				}
				(ac, it)
			},
			"Unlocked" => {
				let ach = app.global::<Search>().get_achievements();
				let items = app.global::<Search>().get_items();
				let mut ac = vec![];
				let mut it = vec![];
				for a in ach.iter() {
					if a.unlocked {
						ac.push(a.id);
					}
				}
				for a in items.iter() {
					if a.unlocked {
						it.push(a.id);
					}
				}
				(ac, it)
			}
			_ => unreachable!()
		}
	}

	pub fn run(&self) {
		self.app.run().unwrap();
	}
}
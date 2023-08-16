#![windows_subsystem = "windows"]
slint::slint! {
	import { GridBox , ScrollView, GroupBox, ListView} from "std-widgets.slint";
	import "./src/upheavtt.ttf";
	struct AchievementIcon {
		image: image,
		name: string,
	}

	export component Icon inherits Image {
		width: 60px;
		height: 60px;
		rect := Rectangle {
			ta:= TouchArea { }
			border-width: ta.has-hover ? 3px : 0;
			border-color: red;
			animate border-width { duration: 150ms; easing: ease-out; }
		}
	} 
	export component App inherits Window {
		title: "Isaac Achievement Unlocker";
		preferred-width: 400px;
		preferred-height: 200px;
		in property <[AchievementIcon]> icons: [
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")}
		];
		property <int> list-width: 16;
		property <int> list-height: Math.ceil(icons.length / list-width);
		ListView  {
			for i in list-height : HorizontalLayout{
				padding: 4px;
				spacing: 4px;
				property <int> list_actual_width: Math.min(icons.length - i * list-width, list-width);
				for t in list_actual_width : VerticalLayout {
					property <int> index: t + i * list-width;
					width: 100px;
					Icon {
						
						source: icons[index].image;
					}
					Text {
						text: index + 1;
						font-weight: 500;
						font-size: 16px;
						font-family: "Upheaval TT (BRK)";
					}Text {
						text: icons[index].name;
						font-weight: 500;
						font-size: 16px;
						font-family: "Upheaval TT (BRK)";
						wrap: word-wrap;
					}
				}
			}
		}
	}
}

fn imbed_images() -> Vec<(slint::Image, String)>{
	use include_dir::*;
	use slint::*;
	const IMAGES: Dir = include_dir!("./images");
	let mut images: Vec<(slint::Image, String)> = vec![];
	for file in IMAGES.files() {
		let image = image::load_from_memory(file.contents()).unwrap().into_rgb8();
		let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(image.as_raw(), image.width(), image.height());
		let i = Image::from_rgb8(buffer);
		let s = file.path().to_str().unwrap();
		images.push((i, s.strip_prefix("Achievement_").unwrap().strip_suffix("_icon.png").unwrap().to_string()));
	}
	images
}

fn main() {
	use slint::Model;
	let images = imbed_images();
	let app = App::new().unwrap();
	let icons = std::rc::Rc::new(slint::VecModel::from(images.iter().fold(Vec::<AchievementIcon>::new(), | mut acc, (x,s)| {
		acc.push(AchievementIcon {image: x.clone(), name: s.into()});
		acc
	})));
	app.set_icons(icons.into());

	app.run().unwrap();
}


slint::slint! {
	import { GridBox , ScrollView, GroupBox, ListView} from "std-widgets.slint";

	struct AchievementIcon {
		image: image,
	}

	export component Icon inherits Image {
		width: 100px;
		height: 100px;
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
		in property <[AchievementIcon]> icons;
		property <int> list-width: 10;
		property <int> list-height: Math.ceil(icons.length / list-width);
		ListView  {
			for i in list-height : HorizontalLayout{
				padding: 4px;
				spacing: 4px;
				property <int> list_actual_width: Math.min(icons.length - i * list-width, list-width);
				for t in list_actual_width : Icon {
					source: icons[t + i * 10].image;
				}
			
			}
		}
	}
}

fn imbed_images() -> Vec<slint::Image>{
	use include_dir::*;
	use slint::*;
	const IMAGES: Dir = include_dir!("./images");
	let mut images: Vec<slint::Image> = vec![];
	for file in IMAGES.files() {
		println!("{:?}", file);
		let image = image::load_from_memory(file.contents()).unwrap().into_rgb8();
		let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(image.as_raw(), image.width(), image.height());
		let i = Image::from_rgb8(buffer);
		images.push(i);
	}
	images
}

fn main() {
	use slint::Model;
	let images = imbed_images();
	let app = App::new().unwrap();
	let icons = std::rc::Rc::new(slint::VecModel::from(images.iter().fold(Vec::<AchievementIcon>::new(), | mut acc, x| {
		acc.push(AchievementIcon {image: x.clone()});
		acc
	})));
	app.set_icons(icons.into());

	app.run().unwrap();
}

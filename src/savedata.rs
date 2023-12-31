use std::{
	fs::{
		File,
		OpenOptions, self,
	},
	os::windows::prelude::FileExt,
};

use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};
lazy_static::lazy_static! {
	pub static ref ISAAC_FOLDER: Result<String, String> = {
		let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
		let steam = hklm.open_subkey(r"SOFTWARE\WOW6432Node\Valve\Steam").map_err(
			|_| -> String{
				"Failed to find steam".to_string()
			}
		)?;
		let path: String = steam.get_value("InstallPath")
			.map_err(|x| {
				x.to_string()
			})?;
		for p in fs::read_dir(path + r"\userdata").unwrap() {
			for ps in fs::read_dir(p.unwrap().path()).unwrap() {
				if ps.as_ref().unwrap().file_name() == "250900"{
					return Ok(ps.unwrap().path().to_str().unwrap().to_string() + r"\remote");
				}
	
			}
		};
		Err("Failed to find isaac folder".to_string())
	};
}

pub const ACHIEVEMENTS_TOTAL: usize = 637;
pub const ITEMS_TOTAL: usize = 717;

const ACHIEVEMENT_OFFSET: u64 = 33;
const ITEMS_OFFSET: u64 = 0xABB;

pub struct SaveData {
	pub achievements: [bool; ACHIEVEMENTS_TOTAL],
	pub items: [bool; ITEMS_TOTAL],
	path: String,
}

pub struct TotalData {
	pub saves: [Option<SaveData>; 3],
}

fn check_sum(buf: Vec<u8>) -> u32 {
	
	const CRC_TABLE:[u32; 256] = [
		0x00000000, 0x09073096, 0x120E612C, 0x1B0951BA, 0xFF6DC419, 0xF66AF48F, 0xED63A535, 0xE46495A3, 
		0xFEDB8832, 0xF7DCB8A4, 0xECD5E91E, 0xE5D2D988, 0x01B64C2B, 0x08B17CBD, 0x13B82D07, 0x1ABF1D91, 
		0xFDB71064, 0xF4B020F2, 0xEFB97148, 0xE6BE41DE, 0x02DAD47D, 0x0BDDE4EB, 0x10D4B551, 0x19D385C7, 
		0x036C9856, 0x0A6BA8C0, 0x1162F97A, 0x1865C9EC, 0xFC015C4F, 0xF5066CD9, 0xEE0F3D63, 0xE7080DF5, 
		0xFB6E20C8, 0xF269105E, 0xE96041E4, 0xE0677172, 0x0403E4D1, 0x0D04D447, 0x160D85FD, 0x1F0AB56B, 
		0x05B5A8FA, 0x0CB2986C, 0x17BBC9D6, 0x1EBCF940, 0xFAD86CE3, 0xF3DF5C75, 0xE8D60DCF, 0xE1D13D59, 
		0x06D930AC, 0x0FDE003A, 0x14D75180, 0x1DD06116, 0xF9B4F4B5, 0xF0B3C423, 0xEBBA9599, 0xE2BDA50F, 
		0xF802B89E, 0xF1058808, 0xEA0CD9B2, 0xE30BE924, 0x076F7C87, 0x0E684C11, 0x15611DAB, 0x1C662D3D, 
		0xF6DC4190, 0xFFDB7106, 0xE4D220BC, 0xEDD5102A, 0x09B18589, 0x00B6B51F, 0x1BBFE4A5, 0x12B8D433, 
		0x0807C9A2, 0x0100F934, 0x1A09A88E, 0x130E9818, 0xF76A0DBB, 0xFE6D3D2D, 0xE5646C97, 0xEC635C01,
		0x0B6B51F4, 0x026C6162, 0x196530D8, 0x1062004E, 0xF40695ED, 0xFD01A57B, 0xE608F4C1, 0xEF0FC457, 
		0xF5B0D9C6, 0xFCB7E950, 0xE7BEB8EA, 0xEEB9887C, 0x0ADD1DDF, 0x03DA2D49, 0x18D37CF3, 0x11D44C65, 
		0x0DB26158, 0x04B551CE, 0x1FBC0074, 0x16BB30E2, 0xF2DFA541, 0xFBD895D7, 0xE0D1C46D, 0xE9D6F4FB, 
		0xF369E96A, 0xFA6ED9FC, 0xE1678846, 0xE860B8D0, 0x0C042D73, 0x05031DE5, 0x1E0A4C5F, 0x170D7CC9, 
		0xF005713C, 0xF90241AA, 0xE20B1010, 0xEB0C2086, 0x0F68B525, 0x066F85B3, 0x1D66D409, 0x1461E49F, 
		0x0EDEF90E, 0x07D9C998, 0x1CD09822, 0x15D7A8B4, 0xF1B33D17, 0xF8B40D81, 0xE3BD5C3B, 0xEABA6CAD, 
		0xEDB88320, 0xE4BFB3B6, 0xFFB6E20C, 0xF6B1D29A, 0x12D54739, 0x1BD277AF, 0x00DB2615, 0x09DC1683, 
		0x13630B12, 0x1A643B84, 0x016D6A3E, 0x086A5AA8, 0xEC0ECF0B, 0xE509FF9D, 0xFE00AE27, 0xF7079EB1, 
		0x100F9344, 0x1908A3D2, 0x0201F268, 0x0B06C2FE, 0xEF62575D, 0xE66567CB, 0xFD6C3671, 0xF46B06E7, 
		0xEED41B76, 0xE7D32BE0, 0xFCDA7A5A, 0xF5DD4ACC, 0x11B9DF6F, 0x18BEEFF9, 0x03B7BE43, 0x0AB08ED5, 
		0x16D6A3E8, 0x1FD1937E, 0x04D8C2C4, 0x0DDFF252, 0xE9BB67F1, 0xE0BC5767, 0xFBB506DD, 0xF2B2364B, 
		0xE80D2BDA, 0xE10A1B4C, 0xFA034AF6, 0xF3047A60, 0x1760EFC3, 0x1E67DF55, 0x056E8EEF, 0x0C69BE79, 
		0xEB61B38C, 0xE266831A, 0xF96FD2A0, 0xF068E236, 0x140C7795, 0x1D0B4703, 0x060216B9, 0x0F05262F, 
		0x15BA3BBE, 0x1CBD0B28, 0x07B45A92, 0x0EB36A04, 0xEAD7FFA7, 0xE3D0CF31, 0xF8D99E8B, 0xF1DEAE1D, 
		0x1B64C2B0, 0x1263F226, 0x096AA39C, 0x006D930A, 0xE40906A9, 0xED0E363F, 0xF6076785, 0xFF005713, 
		0xE5BF4A82, 0xECB87A14, 0xF7B12BAE, 0xFEB61B38, 0x1AD28E9B, 0x13D5BE0D, 0x08DCEFB7, 0x01DBDF21, 
		0xE6D3D2D4, 0xEFD4E242, 0xF4DDB3F8, 0xFDDA836E, 0x19BE16CD, 0x10B9265B, 0x0BB077E1, 0x02B74777, 
		0x18085AE6, 0x110F6A70, 0x0A063BCA, 0x03010B5C, 0xE7659EFF, 0xEE62AE69, 0xF56BFFD3, 0xFC6CCF45, 
		0xE00AE278, 0xE90DD2EE, 0xF2048354, 0xFB03B3C2, 0x1F672661, 0x166016F7, 0x0D69474D, 0x046E77DB, 
		0x1ED16A4A, 0x17D65ADC, 0x0CDF0B66, 0x05D83BF0, 0xE1BCAE53, 0xE8BB9EC5, 0xF3B2CF7F, 0xFAB5FFE9, 
		0x1DBDF21C, 0x14BAC28A, 0x0FB39330, 0x06B4A3A6, 0xE2D03605, 0xEBD70693, 0xF0DE5729, 0xF9D967BF, 
		0xE3667A2E, 0xEA614AB8, 0xF1681B02, 0xF86F2B94, 0x1C0BBE37, 0x150C8EA1, 0x0E05DF1B, 0x0702EF8D
	];
	let mut check: u32 = !0xFEDCBA76;
	for v in buf {
		check = CRC_TABLE[(((check as u8 & 0xFF)) ^ v) as usize] ^ (check >> 8);
	}
	return !check;
	
}

impl TotalData {
	pub fn local_saves() -> Self {
		let mut data = [None, None, None];
		for i in 1..=3 {
			data[i-1] = SaveData::load_local_save(i as u32);
		}
		Self { saves: data }
	}
	pub fn cloud_saves() -> Self {
		let mut data = [None, None, None];
		for i in 1..=3 {
			data[i-1] = SaveData::load_cloud_save(i as u32);
		}
		Self { saves: data }
	}
}

impl SaveData {
	pub fn load_cloud_save(savefile: u32, ) -> Option<Self> {
		let path = ISAAC_FOLDER.as_ref().unwrap().to_string() + format!("\\rep_persistentgamedata{savefile}.dat").as_str();
		Self::load_save_from_path(path)
	}
	fn load_save_from_path(path: String) -> Option<Self> {
		let file = File::open(&path).ok()?;
		let mut buf = [0; 637];
		file.seek_read(&mut buf, ACHIEVEMENT_OFFSET).ok()?;
		let ach = {
			buf.iter().enumerate().fold([false; ACHIEVEMENTS_TOTAL], |mut acc, (i,b)| {
				acc[i] = *b != 0;
				acc
			})
		};
		let mut buf = [0; 732];
		file.seek_read(&mut buf, ITEMS_OFFSET).ok()?;
		let items = {
			let skips = [43,59,61,235,587,613,620,630,648,656,662,666,714,715,718];
			let mut s = 0;
			let mut is = 0;
			buf.iter().enumerate().fold([false; ITEMS_TOTAL], |mut acc, (i, b)| {
				if s < skips.len() && (skips[s] - 1) == i {
					s += 1;
					acc
				}else{
					acc[is] = *b != 0;
					is += 1;
					acc
				}
			})
		};
		Some(Self { achievements: ach, items: items, path: path })
	}
	pub fn load_local_save(savefile: u32) -> Option<Self> {
		let path: String = {
			let mut game_dir = dirs_next::document_dir().unwrap();
			game_dir.push("My Games\\Binding of Isaac Repentance");
			fs::read_dir(game_dir).unwrap()
				.flatten()
				.filter(|x| x.path().as_os_str().to_str().unwrap().contains(format!("rep_persistentgamedata{savefile}.dat").as_str()))
				.max_by_key(|x| x.metadata().unwrap().modified().unwrap())?
				.path().to_str().unwrap().to_string()
		};
		Self::load_save_from_path(path)
	}

	pub fn unlock_achievements(&self, ach: [bool; 637]) {
		let file = OpenOptions::new()
			.read(true)
			.write(true)
			.open(&self.path).expect("Couldn't open savefile");

		file.seek_write(&ach.iter().enumerate().fold( [0 as u8; 637], |mut acc, (i, b)| {
			acc[i] = *b as u8;
			acc
		}), ACHIEVEMENT_OFFSET).expect("Failed to write to savefile");
		let buf: Vec<u8> = fs::read(&self.path).expect("Failed to read file");
		let checksum = check_sum(buf[0x10..buf.len() - 4].to_vec());
		let checksum: [u8; 4] = unsafe {
			std::mem::transmute(checksum.to_le())
		};
		file.seek_write(&checksum, buf.len() as u64 - 4).expect("Failed to write checksum");
	}
}
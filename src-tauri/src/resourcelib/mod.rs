use std::ffi::{CStr, CString};
use std::sync::Mutex;

use anyhow::{bail, Context, Result};
use fn_error_context::context;
use lazy_static::lazy_static;
use quickentity_rs::rt_structs::{RTBlueprint, RTFactory};
use tryvial::try_fn;

mod bindings_2;
mod bindings_2016;
mod bindings_3;

use self::bindings_2::{HM2_GetConverterForResource, HM2_GetGeneratorForResource, JsonString as JsonString2};
use self::bindings_2016::{
	HM2016_GetConverterForResource, HM2016_GetGeneratorForResource, JsonString as JsonString2016
};
use self::bindings_3::{HM3_GetConverterForResource, HM3_GetGeneratorForResource, JsonString as JsonString3};

lazy_static! {
	static ref CONVERTER_MUTEX: Mutex<()> = Mutex::new(());
	static ref GENERATOR_MUTEX: Mutex<()> = Mutex::new(());
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TEMP")]
pub fn h3_convert_binary_to_factory(data: &[u8]) -> Result<RTFactory> {
	let _lock = CONVERTER_MUTEX.lock();

	unsafe {
		let converter = HM3_GetConverterForResource(CString::new("TEMP")?.as_ptr());

		if converter.is_null() {
			bail!("Couldn't get ResourceLib converter")
		}

		let json_string = (*converter).FromMemoryToJsonString.unwrap()(data.as_ptr().cast(), data.len());

		if json_string.is_null() {
			bail!("Couldn't convert data to JsonString")
		}

		let res = serde_json::from_str(
			CStr::from_bytes_with_nul(std::slice::from_raw_parts(
				(*json_string).JsonData.cast(),
				(*json_string).StrSize + 1 // include the null byte in the slice
			))
			.context("Couldn't construct CStr from JsonString data")?
			.to_str()
			.context("Couldn't convert CStr to str")?
		)
		.context("Couldn't deserialise returned JsonString as RTFactory")?;

		(*converter).FreeJsonString.unwrap()(json_string);

		res
	}
}

#[try_fn]
#[context("Couldn't convert ResourceLib TEMP to binary data")]
pub fn h3_convert_factory_to_binary(data: &RTFactory) -> Result<Vec<u8>> {
	let _lock = GENERATOR_MUTEX.lock();

	unsafe {
		let generator = HM3_GetGeneratorForResource(CString::new("TEMP")?.as_ptr());

		if generator.is_null() {
			bail!("Couldn't get ResourceLib generator")
		}

		let json_string = CString::new(serde_json::to_string(data)?)?;
		let json_string = JsonString3 {
			JsonData: json_string.as_ptr(),
			StrSize: json_string.as_bytes().len()
		};

		let resource_mem =
			(*generator).FromJsonStringToResourceMem.unwrap()(json_string.JsonData, json_string.StrSize, false);

		if resource_mem.is_null() {
			bail!("Couldn't convert data to ResourceMem")
		}

		let res = std::slice::from_raw_parts((*resource_mem).ResourceData.cast(), (*resource_mem).DataSize).to_owned();

		(*generator).FreeResourceMem.unwrap()(resource_mem);

		res
	}
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TBLU")]
pub fn h3_convert_binary_to_blueprint(data: &[u8]) -> Result<RTBlueprint> {
	let _lock = CONVERTER_MUTEX.lock();

	unsafe {
		let converter = HM3_GetConverterForResource(CString::new("TBLU")?.as_ptr());

		if converter.is_null() {
			bail!("Couldn't get ResourceLib converter")
		}

		let json_string = (*converter).FromMemoryToJsonString.unwrap()(data.as_ptr().cast(), data.len());

		if json_string.is_null() {
			bail!("Couldn't convert data to JsonString")
		}

		let res = serde_json::from_str(
			CStr::from_bytes_with_nul(std::slice::from_raw_parts(
				(*json_string).JsonData.cast(),
				(*json_string).StrSize + 1 // include the null byte in the slice
			))
			.context("Couldn't construct CStr from JsonString data")?
			.to_str()
			.context("Couldn't convert CStr to str")?
		)
		.context("Couldn't deserialise returned JsonString as RTBlueprint")?;

		(*converter).FreeJsonString.unwrap()(json_string);

		res
	}
}

#[try_fn]
#[context("Couldn't convert ResourceLib TBLU to binary data")]
pub fn h3_convert_blueprint_to_binary(data: &RTBlueprint) -> Result<Vec<u8>> {
	let _lock = GENERATOR_MUTEX.lock();

	unsafe {
		let generator = HM3_GetGeneratorForResource(CString::new("TBLU")?.as_ptr());

		if generator.is_null() {
			bail!("Couldn't get ResourceLib generator")
		}

		let json_string = CString::new(serde_json::to_string(data)?)?;
		let json_string = JsonString3 {
			JsonData: json_string.as_ptr(),
			StrSize: json_string.as_bytes().len()
		};

		let resource_mem =
			(*generator).FromJsonStringToResourceMem.unwrap()(json_string.JsonData, json_string.StrSize, false);

		if resource_mem.is_null() {
			bail!("Couldn't convert data to ResourceMem")
		}

		let res = std::slice::from_raw_parts((*resource_mem).ResourceData.cast(), (*resource_mem).DataSize).to_owned();

		(*generator).FreeResourceMem.unwrap()(resource_mem);

		res
	}
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TEMP")]
pub fn h2_convert_binary_to_factory(data: &[u8]) -> Result<RTFactory> {
	let _lock = CONVERTER_MUTEX.lock();

	unsafe {
		let converter = HM2_GetConverterForResource(CString::new("TEMP")?.as_ptr());

		if converter.is_null() {
			bail!("Couldn't get ResourceLib converter")
		}

		let json_string = (*converter).FromMemoryToJsonString.unwrap()(data.as_ptr().cast(), data.len());

		if json_string.is_null() {
			bail!("Couldn't convert data to JsonString")
		}

		let res = serde_json::from_str(
			CStr::from_bytes_with_nul(std::slice::from_raw_parts(
				(*json_string).JsonData.cast(),
				(*json_string).StrSize + 1 // include the null byte in the slice
			))
			.context("Couldn't construct CStr from JsonString data")?
			.to_str()
			.context("Couldn't convert CStr to str")?
		)
		.context("Couldn't deserialise returned JsonString as RTFactory")?;

		(*converter).FreeJsonString.unwrap()(json_string);

		res
	}
}

#[try_fn]
#[context("Couldn't convert ResourceLib TEMP to binary data")]
pub fn h2_convert_factory_to_binary(data: &RTFactory) -> Result<Vec<u8>> {
	let _lock = GENERATOR_MUTEX.lock();

	unsafe {
		let generator = HM2_GetGeneratorForResource(CString::new("TEMP")?.as_ptr());

		if generator.is_null() {
			bail!("Couldn't get ResourceLib generator")
		}

		let json_string = CString::new(serde_json::to_string(data)?)?;
		let json_string = JsonString2 {
			JsonData: json_string.as_ptr(),
			StrSize: json_string.as_bytes().len()
		};

		let resource_mem =
			(*generator).FromJsonStringToResourceMem.unwrap()(json_string.JsonData, json_string.StrSize, false);

		if resource_mem.is_null() {
			bail!("Couldn't convert data to ResourceMem")
		}

		let res = std::slice::from_raw_parts((*resource_mem).ResourceData.cast(), (*resource_mem).DataSize).to_owned();

		(*generator).FreeResourceMem.unwrap()(resource_mem);

		res
	}
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TBLU")]
pub fn h2_convert_binary_to_blueprint(data: &[u8]) -> Result<RTBlueprint> {
	let _lock = CONVERTER_MUTEX.lock();

	unsafe {
		let converter = HM2_GetConverterForResource(CString::new("TBLU")?.as_ptr());

		if converter.is_null() {
			bail!("Couldn't get ResourceLib converter")
		}

		let json_string = (*converter).FromMemoryToJsonString.unwrap()(data.as_ptr().cast(), data.len());

		if json_string.is_null() {
			bail!("Couldn't convert data to JsonString")
		}

		let res = serde_json::from_str(
			CStr::from_bytes_with_nul(std::slice::from_raw_parts(
				(*json_string).JsonData.cast(),
				(*json_string).StrSize + 1 // include the null byte in the slice
			))
			.context("Couldn't construct CStr from JsonString data")?
			.to_str()
			.context("Couldn't convert CStr to str")?
		)
		.context("Couldn't deserialise returned JsonString as RTBlueprint")?;

		(*converter).FreeJsonString.unwrap()(json_string);

		res
	}
}

#[try_fn]
#[context("Couldn't convert ResourceLib TBLU to binary data")]
pub fn h2_convert_blueprint_to_binary(data: &RTBlueprint) -> Result<Vec<u8>> {
	let _lock = GENERATOR_MUTEX.lock();

	unsafe {
		let generator = HM2_GetGeneratorForResource(CString::new("TBLU")?.as_ptr());

		if generator.is_null() {
			bail!("Couldn't get ResourceLib generator")
		}

		let json_string = CString::new(serde_json::to_string(data)?)?;
		let json_string = JsonString2 {
			JsonData: json_string.as_ptr(),
			StrSize: json_string.as_bytes().len()
		};

		let resource_mem =
			(*generator).FromJsonStringToResourceMem.unwrap()(json_string.JsonData, json_string.StrSize, false);

		if resource_mem.is_null() {
			bail!("Couldn't convert data to ResourceMem")
		}

		let res = std::slice::from_raw_parts((*resource_mem).ResourceData.cast(), (*resource_mem).DataSize).to_owned();

		(*generator).FreeResourceMem.unwrap()(resource_mem);

		res
	}
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TEMP")]
pub fn h2016_convert_binary_to_factory(data: &[u8]) -> Result<RTFactory> {
	let _lock = CONVERTER_MUTEX.lock();

	unsafe {
		let converter = HM2016_GetConverterForResource(CString::new("TEMP")?.as_ptr());

		if converter.is_null() {
			bail!("Couldn't get ResourceLib converter")
		}

		let json_string = (*converter).FromMemoryToJsonString.unwrap()(data.as_ptr().cast(), data.len());

		if json_string.is_null() {
			bail!("Couldn't convert data to JsonString")
		}

		let res = serde_json::from_str(
			CStr::from_bytes_with_nul(std::slice::from_raw_parts(
				(*json_string).JsonData.cast(),
				(*json_string).StrSize + 1 // include the null byte in the slice
			))
			.context("Couldn't construct CStr from JsonString data")?
			.to_str()
			.context("Couldn't convert CStr to str")?
		)
		.context("Couldn't deserialise returned JsonString as RTFactory")?;

		(*converter).FreeJsonString.unwrap()(json_string);

		res
	}
}

#[try_fn]
#[context("Couldn't convert ResourceLib TEMP to binary data")]
pub fn h2016_convert_factory_to_binary(data: &RTFactory) -> Result<Vec<u8>> {
	let _lock = GENERATOR_MUTEX.lock();

	unsafe {
		let generator = HM2016_GetGeneratorForResource(CString::new("TEMP")?.as_ptr());

		if generator.is_null() {
			bail!("Couldn't get ResourceLib generator")
		}

		let json_string = CString::new(serde_json::to_string(data)?)?;
		let json_string = JsonString2016 {
			JsonData: json_string.as_ptr(),
			StrSize: json_string.as_bytes().len()
		};

		let resource_mem =
			(*generator).FromJsonStringToResourceMem.unwrap()(json_string.JsonData, json_string.StrSize, false);

		if resource_mem.is_null() {
			bail!("Couldn't convert data to ResourceMem")
		}

		let res = std::slice::from_raw_parts((*resource_mem).ResourceData.cast(), (*resource_mem).DataSize).to_owned();

		(*generator).FreeResourceMem.unwrap()(resource_mem);

		res
	}
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TBLU")]
pub fn h2016_convert_binary_to_blueprint(data: &[u8]) -> Result<RTBlueprint> {
	let _lock = CONVERTER_MUTEX.lock();

	unsafe {
		let converter = HM2016_GetConverterForResource(CString::new("TBLU")?.as_ptr());

		if converter.is_null() {
			bail!("Couldn't get ResourceLib converter")
		}

		let json_string = (*converter).FromMemoryToJsonString.unwrap()(data.as_ptr().cast(), data.len());

		if json_string.is_null() {
			bail!("Couldn't convert data to JsonString")
		}

		let res = serde_json::from_str(
			CStr::from_bytes_with_nul(std::slice::from_raw_parts(
				(*json_string).JsonData.cast(),
				(*json_string).StrSize + 1 // include the null byte in the slice
			))
			.context("Couldn't construct CStr from JsonString data")?
			.to_str()
			.context("Couldn't convert CStr to str")?
		)
		.context("Couldn't deserialise returned JsonString as RTBlueprint")?;

		(*converter).FreeJsonString.unwrap()(json_string);

		res
	}
}

#[try_fn]
#[context("Couldn't convert ResourceLib TBLU to binary data")]
pub fn h2016_convert_blueprint_to_binary(data: &RTBlueprint) -> Result<Vec<u8>> {
	let _lock = GENERATOR_MUTEX.lock();

	unsafe {
		let generator = HM2016_GetGeneratorForResource(CString::new("TBLU")?.as_ptr());

		if generator.is_null() {
			bail!("Couldn't get ResourceLib generator")
		}

		let json_string = CString::new(serde_json::to_string(data)?)?;
		let json_string = JsonString2016 {
			JsonData: json_string.as_ptr(),
			StrSize: json_string.as_bytes().len()
		};

		let resource_mem =
			(*generator).FromJsonStringToResourceMem.unwrap()(json_string.JsonData, json_string.StrSize, false);

		if resource_mem.is_null() {
			bail!("Couldn't convert data to ResourceMem")
		}

		let res = std::slice::from_raw_parts((*resource_mem).ResourceData.cast(), (*resource_mem).DataSize).to_owned();

		(*generator).FreeResourceMem.unwrap()(resource_mem);

		res
	}
}
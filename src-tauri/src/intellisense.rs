use std::{
	collections::{HashMap, HashSet},
	path::PathBuf,
	sync::Arc
};

use anyhow::{bail, Context, Result};
use fn_error_context::context;
use indexmap::IndexMap;
use itertools::Itertools;
use parking_lot::RwLock;
use quickentity_rs::{
	qn_structs::{Entity, Ref, RefMaybeConstantValue, RefWithConstantValue},
	rt_structs::PropertyID,
	util_structs::{SMatrix43PropertyValue, ZGuidPropertyValue},
	RAD2DEG
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rpkg_rs::runtime::resource::resource_package::ResourcePackage;
use serde_json::{from_value, json, to_value, Value};
use tryvial::try_fn;

use crate::{
	entity::get_local_reference,
	game_detection::GameVersion,
	hash_list::HashList,
	material::{get_material_properties, MaterialProperty, MaterialPropertyData},
	resourcelib::{
		convert_uicb, h2016_convert_cppt, h2016_convert_dswb, h2016_convert_ecpb, h2_convert_cppt, h2_convert_dswb,
		h2_convert_ecpb, h3_convert_cppt, h3_convert_dswb, h3_convert_ecpb, EExtendedPropertyType
	},
	rpkg::{ensure_entity_in_cache, extract_latest_metadata, extract_latest_resource, normalise_to_hash}
};

pub struct Intellisense {
	/// CPPT -> Property -> (Type, Value)
	pub cppt_properties: Arc<RwLock<HashMap<String, HashMap<String, (String, Value)>>>>,

	/// CPPT -> (Input, Output)
	pub cppt_pins: HashMap<String, (Vec<String>, Vec<String>)>,

	/// Property type as number -> String version
	pub uicb_prop_types: HashMap<u8, String>,

	pub matt_properties: Arc<RwLock<HashMap<String, Vec<MaterialProperty>>>>,

	pub all_cppts: HashSet<String>,
	pub all_asets: HashSet<String>,
	pub all_uicts: HashSet<String>,
	pub all_matts: HashSet<String>,
	pub all_wswts: HashSet<String>,
	pub all_ecpts: HashSet<String>,
	pub all_aibxs: HashSet<String>
}

impl Intellisense {
	#[try_fn]
	#[context("Couldn't get properties for CPPT {}", cppt)]
	fn get_cppt_properties(
		&self,
		resource_packages: &IndexMap<PathBuf, ResourcePackage>,
		hash_list: &HashList,
		game_version: GameVersion,
		cppt: &str
	) -> Result<HashMap<String, (String, Value)>> {
		{
			if let Some(cached) = self.cppt_properties.read().get(cppt) {
				return Ok(cached.to_owned());
			}
		}

		let extracted = extract_latest_resource(resource_packages, hash_list, cppt)?;

		let cppt_data = match game_version {
			GameVersion::H1 => h2016_convert_cppt(&extracted.1)?,
			GameVersion::H2 => h2_convert_cppt(&extracted.1)?,
			GameVersion::H3 => h3_convert_cppt(&extracted.1)?
		};

		let mut guard = self.cppt_properties.write();
		guard.insert(
			cppt.into(),
			cppt_data
				.property_values
				.into_iter()
				.map(|property_value| {
					anyhow::Ok((
						match property_value.n_property_id {
							PropertyID::Int(x) => x.to_string(),
							PropertyID::String(x) => x
						},
						(
							match property_value.value.property_type.as_ref() {
								"ZEntityReference" => "SEntityTemplateReference",
								"TArray<ZEntityReference>" => "TArray<SEntityTemplateReference>",
								x => x
							}
							.into(),
							match property_value.value.property_type.as_ref() {
								"ZRuntimeResourceID" => {
									let id_low = property_value
										.value
										.property_value
										.get("m_IDLow")
										.context("Invalid data")?
										.as_u64()
										.context("Invalid data")?;

									if id_low != 4294967295 {
										let reference = extracted
											.0
											.hash_reference_data
											.get(id_low as usize)
											.context("No such referenced resource")?;

										if reference.flag == "1F" {
											json!(reference.hash)
										} else {
											json!({
												"resource": reference.hash,
												"flag": reference.flag
											})
										}
									} else {
										Value::Null
									}
								}

								"ZEntityReference" => Value::Null,

								"TArray<ZEntityReference>" => json!([]),

								"ZGuid" => {
									let guid = from_value::<ZGuidPropertyValue>(property_value.value.property_value)
										.context("Invalid data")?;

									to_value(format!(
										"{:0>8x}-{:0>4x}-{:0>4x}-{:0>2x}{:0>2x}-{:0>2x}{:0>2x}{:0>2x}{:0>2x}{:0>2x}{:\
										 0>2x}",
										guid._a,
										guid._b,
										guid._c,
										guid._d,
										guid._e,
										guid._f,
										guid._g,
										guid._h,
										guid._i,
										guid._j,
										guid._k
									))?
								}

								"SColorRGB" => {
									let map = property_value
										.value
										.property_value
										.as_object()
										.context("SColorRGB was not an object")?;

									to_value(format!(
										"#{:0>2x}{:0>2x}{:0>2x}",
										(map.get("r")
											.context("Colour did not have required key r")?
											.as_f64()
											.context("Invalid data")? * 255.0)
											.round() as u8,
										(map.get("g")
											.context("Colour did not have required key g")?
											.as_f64()
											.context("Invalid data")? * 255.0)
											.round() as u8,
										(map.get("b")
											.context("Colour did not have required key b")?
											.as_f64()
											.context("Invalid data")? * 255.0)
											.round() as u8
									))?
								}

								"SColorRGBA" => {
									let map = property_value
										.value
										.property_value
										.as_object()
										.context("SColorRGBA was not an object")?;

									to_value(format!(
										"#{:0>2x}{:0>2x}{:0>2x}{:0>2x}",
										(map.get("r")
											.context("Colour did not have required key r")?
											.as_f64()
											.context("Invalid data")? * 255.0)
											.round() as u8,
										(map.get("g")
											.context("Colour did not have required key g")?
											.as_f64()
											.context("Invalid data")? * 255.0)
											.round() as u8,
										(map.get("b")
											.context("Colour did not have required key b")?
											.as_f64()
											.context("Invalid data")? * 255.0)
											.round() as u8,
										(map.get("a")
											.context("Colour did not have required key a")?
											.as_f64()
											.context("Invalid data")? * 255.0)
											.round() as u8
									))?
								}

								"SMatrix43" => {
									let mut matrix =
										from_value::<SMatrix43PropertyValue>(property_value.value.property_value)
											.context("Invalid data")?;

									// this is all from three.js

									let n11 = matrix.XAxis.x;
									let n12 = matrix.XAxis.y;
									let n13 = matrix.XAxis.z;
									let n14 = 0.0;
									let n21 = matrix.YAxis.x;
									let n22 = matrix.YAxis.y;
									let n23 = matrix.YAxis.z;
									let n24 = 0.0;
									let n31 = matrix.ZAxis.x;
									let n32 = matrix.ZAxis.y;
									let n33 = matrix.ZAxis.z;
									let n34 = 0.0;
									let n41 = matrix.Trans.x;
									let n42 = matrix.Trans.y;
									let n43 = matrix.Trans.z;
									let n44 = 1.0;

									let det = n41
										* (n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33
											+ n12 * n24 * n33 + n13 * n22 * n34 - n12 * n23 * n34)
										+ n42
											* (n11 * n23 * n34 - n11 * n24 * n33 + n14 * n21 * n33 - n13 * n21 * n34
												+ n13 * n24 * n31 - n14 * n23 * n31) + n43
										* (n11 * n24 * n32 - n11 * n22 * n34 - n14 * n21 * n32
											+ n12 * n21 * n34 + n14 * n22 * n31 - n12 * n24 * n31)
										+ n44
											* (-n13 * n22 * n31 - n11 * n23 * n32 + n11 * n22 * n33 + n13 * n21 * n32
												- n12 * n21 * n33 + n12 * n23 * n31);

									let mut sx = n11 * n11 + n21 * n21 + n31 * n31;
									let sy = n12 * n12 + n22 * n22 + n32 * n32;
									let sz = n13 * n13 + n23 * n23 + n33 * n33;

									if det < 0.0 {
										sx = -sx
									};

									let pos = json!({ "x": n41, "y": n42, "z": n43 });
									let scale = json!({ "x": sx, "y": sy, "z": sz });

									let inv_sx = 1.0 / sx;
									let inv_sy = 1.0 / sy;
									let inv_sz = 1.0 / sz;

									matrix.XAxis.x *= inv_sx;
									matrix.YAxis.x *= inv_sx;
									matrix.ZAxis.x *= inv_sx;
									matrix.XAxis.y *= inv_sy;
									matrix.YAxis.y *= inv_sy;
									matrix.ZAxis.y *= inv_sy;
									matrix.XAxis.z *= inv_sz;
									matrix.YAxis.z *= inv_sz;
									matrix.ZAxis.z *= inv_sz;

									let rotation_x = (if matrix.XAxis.z.abs() < 0.9999999 {
										(-matrix.YAxis.z).atan2(matrix.ZAxis.z)
									} else {
										(matrix.ZAxis.y).atan2(matrix.YAxis.y)
									}) * RAD2DEG;

									let rotation_y = matrix.XAxis.z.clamp(-1.0, 1.0).asin() * RAD2DEG;

									let rotation_z = (if matrix.XAxis.z.abs() < 0.9999999 {
										(-matrix.XAxis.y).atan2(matrix.XAxis.x)
									} else {
										0.0
									}) * RAD2DEG;

									if scale.get("x").expect("We made it").as_f64().expect("We made it") != 1.0
										|| scale.get("y").expect("We made it").as_f64().expect("We made it") != 1.0
										|| scale.get("z").expect("We made it").as_f64().expect("We made it") != 1.0
									{
										json!({
											"rotation": {
												"x": rotation_x,
												"y": rotation_y,
												"z": rotation_z
											},
											"position": pos,
											"scale": scale
										})
									} else {
										json!({
											"rotation": {
												"x": rotation_x,
												"y": rotation_y,
												"z": rotation_z
											},
											"position": pos
										})
									}
								}

								_ => property_value.value.property_value
							}
						)
					))
				})
				.collect::<Result<_>>()?
		);

		guard.get(cppt).expect("We just added it").to_owned()
	}

	#[try_fn]
	#[context("Couldn't get properties for MATT {}", matt)]
	fn get_matt_properties(
		&self,
		resource_packages: &IndexMap<PathBuf, ResourcePackage>,
		hash_list: &HashList,
		matt: &str
	) -> Result<Vec<MaterialProperty>> {
		{
			if let Some(cached) = self.matt_properties.read().get(matt) {
				return Ok(cached.to_owned());
			}
		}

		let (matt_meta, matt_data) = extract_latest_resource(resource_packages, hash_list, matt)?;

		let (_, matb_data) = extract_latest_resource(
			resource_packages,
			hash_list,
			&matt_meta
				.hash_reference_data
				.iter()
				.find(|x| {
					hash_list
						.entries
						.get(&normalise_to_hash(x.hash.to_owned()))
						.map(|entry| entry.resource_type == "MATB")
						.unwrap_or(false)
				})
				.context("MATT has no MATB dependency")?
				.hash
		)?;

		let mut guard = self.matt_properties.write();
		guard.insert(
			matt.into(),
			get_material_properties(&matt_data, &matt_meta, &matb_data)?
		);

		guard.get(matt).expect("We just added it").to_owned()
	}

	/// Get the names, types, default values and post-init status of all properties of a given sub-entity.
	///
	/// Will deadlock if a read or write lock is already held on `cached_entities` by the same thread.
	#[try_fn]
	#[context("Couldn't get properties for sub-entity {} in {}", sub_entity, entity.factory_hash)]
	pub fn get_properties(
		&self,
		resource_packages: &IndexMap<PathBuf, ResourcePackage>,
		cached_entities: &RwLock<HashMap<String, Entity>>,
		hash_list: &HashList,
		game_version: GameVersion,
		entity: &Entity,
		sub_entity: &str,
		ignore_own: bool
	) -> Result<Vec<(String, String, Value, bool)>> {
		let targeted = entity.entities.get(sub_entity).context("No such sub-entity")?;

		let mut found = vec![];

		found.extend(
			targeted
				.property_aliases
				.as_ref()
				.unwrap_or(&Default::default())
				.into_par_iter()
				.map(|(aliased_name, aliases)| {
					Ok({
						let mut found = vec![];
						for alias in aliases {
							if let Ref::Short(Some(ent)) = &alias.original_entity {
								if let Some(data) = self.get_specific_property(
									resource_packages,
									cached_entities,
									hash_list,
									game_version,
									entity,
									ent,
									&alias.original_property
								)? {
									found.push((aliased_name.to_owned(), data.0, data.1, data.2));
								}
								break;
							}
						}

						found
					})
				})
				.collect::<Result<Vec<_>>>()?
				.into_iter()
				.flatten()
		);

		if !ignore_own {
			for (property, property_data) in targeted.properties.as_ref().unwrap_or(&Default::default()) {
				found.push((
					property.to_owned(),
					property_data.property_type.to_owned(),
					property_data.value.to_owned(),
					property_data.post_init.unwrap_or(false)
				));
			}
		}

		found.extend(
			{
				if self.all_asets.contains(&normalise_to_hash(targeted.factory.to_owned())) {
					extract_latest_metadata(
						resource_packages,
						hash_list,
						&normalise_to_hash(targeted.factory.to_owned())
					)?
					.hash_reference_data
					.into_iter()
					.rev()
					.skip(1)
					.rev()
					.map(|x| normalise_to_hash(x.hash))
					.collect_vec()
				} else {
					vec![normalise_to_hash(targeted.factory.to_owned())]
				}
			}
			.into_par_iter()
			.map(|factory| {
				Ok({
					let mut found = vec![];

					if self.all_cppts.contains(&factory) {
						for (prop_name, (prop_type, default_val)) in
							self.get_cppt_properties(resource_packages, hash_list, game_version, &factory)?
						{
							found.push((prop_name, prop_type, default_val, false));
						}
					} else if self.all_uicts.contains(&factory) {
						// All UI controls have the properties of ZUIControlEntity
						for (prop_name, (prop_type, default_val)) in
							self.get_cppt_properties(resource_packages, hash_list, game_version, "002C4526CC9753E6")?
						{
							found.push((prop_name, prop_type, default_val, false));
						}

						for entry in convert_uicb(
							&extract_latest_resource(
								resource_packages,
								hash_list,
								&extract_latest_metadata(resource_packages, hash_list, &factory)?
									.hash_reference_data
									.into_iter()
									.find(|x| {
										hash_list
											.entries
											.get(&normalise_to_hash(x.hash.to_owned()))
											.map(|entry| entry.resource_type == "UICB")
											.unwrap_or(false)
									})
									.context("No blueprint dependency on UICT")?
									.hash
							)?
							.1
						)?
						.m_aPins
						{
							// Property
							if entry.m_nUnk00 == 0 {
								let prop_type = self
									.uicb_prop_types
									.get(&entry.m_nUnk01)
									.context("Unknown UICB property type")?;

								// We can't get the actual default values, if there are any, so we just use sensible defaults
								found.push((
									entry.m_sName,
									prop_type.into(),
									match prop_type.as_ref() {
										"int32" => to_value(0)?,
										"float32" => to_value(0)?,
										"ZString" => to_value("")?,
										"bool" => to_value(false)?,
										_ => bail!("UICB property types has unknown type")
									},
									false
								));
							}
						}
					} else if self.all_matts.contains(&factory) {
						// All materials have the properties of ZRenderMaterialEntity
						for (prop_name, (prop_type, default_val)) in
							self.get_cppt_properties(resource_packages, hash_list, game_version, "00B4B11DA327CAD0")?
						{
							found.push((prop_name, prop_type, default_val, false));
						}

						for property in self.get_matt_properties(resource_packages, hash_list, &factory)? {
							match property.data {
								MaterialPropertyData::Texture(texture) => {
									found.push((
										property.name.to_owned(),
										"ZRuntimeResourceID".into(),
										texture
											.map(|texture| {
												json!({
													"resource": texture,
													"flag": "5F"
												})
											})
											.unwrap_or(Value::Null),
										false
									));

									found.push((format!("{}_enab", property.name), "bool".into(), json!(false), false));

									found.push((
										format!("{}_dest", property.name),
										"SEntityTemplateReference".into(),
										Value::Null,
										false
									));
								}

								MaterialPropertyData::ColorRGB(r, g, b) => {
									found.push((
										property.name.to_owned(),
										"SColorRGB".into(),
										to_value(format!(
											"#{:0>2x}{:0>2x}{:0>2x}",
											(r * 255.0).round() as u8,
											(g * 255.0).round() as u8,
											(b * 255.0).round() as u8
										))?,
										false
									));

									found.push((
										format!("{}_op", property.name),
										"IRenderMaterialEntity.EModifierOperation".into(),
										to_value("eLeave")?,
										false
									));
								}

								MaterialPropertyData::ColorRGBA(r, g, b, a) => {
									found.push((
										property.name.to_owned(),
										"SColorRGBA".into(),
										to_value(format!(
											"#{:0>2x}{:0>2x}{:0>2x}{:0>2x}",
											(r * 255.0).round() as u8,
											(g * 255.0).round() as u8,
											(b * 255.0).round() as u8,
											(a * 255.0).round() as u8
										))?,
										false
									));

									found.push((
										format!("{}_op", property.name),
										"IRenderMaterialEntity.EModifierOperation".into(),
										to_value("eLeave")?,
										false
									));
								}

								MaterialPropertyData::Float(val) => {
									found.push((property.name.to_owned(), "float32".into(), to_value(val)?, false));

									found.push((
										format!("{}_op", property.name),
										"IRenderMaterialEntity.EModifierOperation".into(),
										to_value("eLeave")?,
										false
									));
								}

								MaterialPropertyData::Vector2(x, y) => {
									found.push((
										property.name.to_owned(),
										"SVector2".into(),
										json!({
											"x": x,
											"y": y
										}),
										false
									));

									found.push((
										format!("{}_op", property.name),
										"IRenderMaterialEntity.EModifierOperation".into(),
										to_value("eLeave")?,
										false
									));
								}

								MaterialPropertyData::Vector3(x, y, z) => {
									found.push((
										property.name.to_owned(),
										"SVector3".into(),
										json!({
											"x": x,
											"y": y,
											"z": z
										}),
										false
									));

									found.push((
										format!("{}_op", property.name),
										"IRenderMaterialEntity.EModifierOperation".into(),
										to_value("eLeave")?,
										false
									));
								}

								MaterialPropertyData::Vector4(x, y, z, w) => {
									found.push((
										property.name.to_owned(),
										"SVector4".into(),
										json!({
											"x": x,
											"y": y,
											"z": z,
											"w": w
										}),
										false
									));

									found.push((
										format!("{}_op", property.name),
										"IRenderMaterialEntity.EModifierOperation".into(),
										to_value("eLeave")?,
										false
									));
								}
							}
						}
					} else if self.all_wswts.contains(&factory) {
						// All switch groups have the properties of ZAudioSwitchEntity
						for (prop_name, (prop_type, default_val)) in
							self.get_cppt_properties(resource_packages, hash_list, game_version, "00797DC916520C4D")?
						{
							found.push((prop_name, prop_type, default_val, false));
						}
					} else if self.all_ecpts.contains(&factory) {
						// All extended CPP entities have the properties of ZMaterialOverwriteAspect
						for (prop_name, (prop_type, default_val)) in
							self.get_cppt_properties(resource_packages, hash_list, game_version, "00D3003AAA7B3817")?
						{
							found.push((prop_name, prop_type, default_val, false));
						}

						let ecpb_data = extract_latest_resource(
							resource_packages,
							hash_list,
							&extract_latest_metadata(resource_packages, hash_list, &factory)?
								.hash_reference_data
								.into_iter()
								.find(|x| {
									hash_list
										.entries
										.get(&normalise_to_hash(x.hash.to_owned()))
										.map(|entry| entry.resource_type == "ECPB")
										.unwrap_or(false)
								})
								.context("No blueprint dependency on ECPT")?
								.hash
						)?
						.1;

						let ecpb_data = match game_version {
							GameVersion::H1 => h2016_convert_ecpb(&ecpb_data)?,
							GameVersion::H2 => h2_convert_ecpb(&ecpb_data)?,
							GameVersion::H3 => h3_convert_ecpb(&ecpb_data)?
						};

						for entry in ecpb_data.properties {
							found.push((
								entry.property_name,
								match entry.property_type {
									EExtendedPropertyType::TYPE_RESOURCEPTR => "ZRuntimeResourceID",
									EExtendedPropertyType::TYPE_INT32 => "int32",
									EExtendedPropertyType::TYPE_UINT32 => "uint32",
									EExtendedPropertyType::TYPE_FLOAT => "float32",
									EExtendedPropertyType::TYPE_STRING => "ZString",
									EExtendedPropertyType::TYPE_BOOL => "bool",
									EExtendedPropertyType::TYPE_ENTITYREF => "SEntityTemplateReference",
									EExtendedPropertyType::TYPE_VARIANT => "ZVariant"
								}
								.into(),
								match entry.property_type {
									EExtendedPropertyType::TYPE_RESOURCEPTR => Value::Null,
									EExtendedPropertyType::TYPE_INT32 => to_value(0)?,
									EExtendedPropertyType::TYPE_UINT32 => to_value(0)?,
									EExtendedPropertyType::TYPE_FLOAT => to_value(0)?,
									EExtendedPropertyType::TYPE_STRING => Value::String("".into()),
									EExtendedPropertyType::TYPE_BOOL => Value::Bool(false),
									EExtendedPropertyType::TYPE_ENTITYREF => Value::Null,
									EExtendedPropertyType::TYPE_VARIANT => Value::Null
								},
								false
							));
						}
					} else if self.all_aibxs.contains(&factory) {
						// All behaviour trees have the properties of ZBehaviorTreeEntity
						for (prop_name, (prop_type, default_val)) in
							self.get_cppt_properties(resource_packages, hash_list, game_version, "0028607138892D70")?
						{
							found.push((prop_name, prop_type, default_val, false));
						}
					} else {
						match ensure_entity_in_cache(
							resource_packages,
							cached_entities,
							game_version,
							hash_list,
							&normalise_to_hash(factory.to_owned())
						) {
							Ok(_) => {
								let extracted = cached_entities
									.read()
									.get(&normalise_to_hash(factory.to_owned()))
									.expect("Ensured")
									.to_owned();

								found.extend(self.get_properties(
									resource_packages,
									cached_entities,
									hash_list,
									game_version,
									&extracted,
									&extracted.root_entity,
									false
								)?);
							}

							Err(e) if format!("{:?}", e).contains("Couldn't find the resource in any RPKG") => {}

							x => {
								x?;
							}
						}
					}

					found
				})
			})
			.collect::<Result<Vec<_>>>()?
			.into_iter()
			.flatten()
		);

		found
	}

	/// Get the type, default value and post-init status of a single property of a given sub-entity, by its name.
	///
	/// Will deadlock if a read or write lock is already held on `cached_entities` by the same thread.
	#[try_fn]
	#[context("Couldn't get property {} of sub-entity {} in {}", property_to_find, sub_entity, entity.factory_hash)]
	pub fn get_specific_property(
		&self,
		resource_packages: &IndexMap<PathBuf, ResourcePackage>,
		cached_entities: &RwLock<HashMap<String, Entity>>,
		hash_list: &HashList,
		game_version: GameVersion,
		entity: &Entity,
		sub_entity: &str,
		property_to_find: &str
	) -> Result<Option<(String, Value, bool)>> {
		let targeted = entity.entities.get(sub_entity).context("No such sub-entity")?;

		if let Some(aliases) = targeted
			.property_aliases
			.as_ref()
			.unwrap_or(&Default::default())
			.get(property_to_find)
		{
			for alias in aliases {
				if let Ref::Short(Some(ent)) = &alias.original_entity {
					if let Some(data) = self.get_specific_property(
						resource_packages,
						cached_entities,
						hash_list,
						game_version,
						entity,
						ent,
						&alias.original_property
					)? {
						return Ok(Some((data.0, data.1, data.2)));
					}
				}
			}
		}

		if let Some(property_data) = targeted
			.properties
			.as_ref()
			.unwrap_or(&Default::default())
			.get(property_to_find)
		{
			return Ok(Some((
				property_data.property_type.to_owned(),
				property_data.value.to_owned(),
				property_data.post_init.unwrap_or(false)
			)));
		}

		for factory in if self.all_asets.contains(&normalise_to_hash(targeted.factory.to_owned())) {
			extract_latest_metadata(
				resource_packages,
				hash_list,
				&normalise_to_hash(targeted.factory.to_owned())
			)?
			.hash_reference_data
			.into_iter()
			.rev()
			.skip(1)
			.rev()
			.map(|x| normalise_to_hash(x.hash))
			.collect_vec()
		} else {
			vec![normalise_to_hash(targeted.factory.to_owned())]
		} {
			if self.all_cppts.contains(&factory) {
				for (prop_name, (prop_type, default_val)) in
					self.get_cppt_properties(resource_packages, hash_list, game_version, &factory)?
				{
					if prop_name == property_to_find {
						return Ok(Some((prop_type, default_val, false)));
					}
				}
			} else if self.all_uicts.contains(&factory) {
				// All UI controls have the properties of ZUIControlEntity
				for (prop_name, (prop_type, default_val)) in
					self.get_cppt_properties(resource_packages, hash_list, game_version, "002C4526CC9753E6")?
				{
					if prop_name == property_to_find {
						return Ok(Some((prop_type, default_val, false)));
					}
				}

				for entry in convert_uicb(
					&extract_latest_resource(
						resource_packages,
						hash_list,
						&extract_latest_metadata(resource_packages, hash_list, &factory)?
							.hash_reference_data
							.into_iter()
							.find(|x| {
								hash_list
									.entries
									.get(&normalise_to_hash(x.hash.to_owned()))
									.map(|entry| entry.resource_type == "UICB")
									.unwrap_or(false)
							})
							.context("No blueprint dependency on UICT")?
							.hash
					)?
					.1
				)?
				.m_aPins
				{
					// Property
					if entry.m_nUnk00 == 0 {
						let prop_type = self
							.uicb_prop_types
							.get(&entry.m_nUnk01)
							.context("Unknown UICB property type")?;

						// We can't get the actual default values, if there are any, so we just use sensible defaults
						if entry.m_sName == property_to_find {
							return Ok(Some((
								prop_type.into(),
								match prop_type.as_ref() {
									"int32" => to_value(0)?,
									"float32" => to_value(0)?,
									"ZString" => to_value("")?,
									"bool" => to_value(false)?,
									_ => bail!("UICB property types has unknown type")
								},
								false
							)));
						}
					}
				}
			} else if self.all_matts.contains(&factory) {
				// All materials have the properties of ZRenderMaterialEntity
				for (prop_name, (prop_type, default_val)) in
					self.get_cppt_properties(resource_packages, hash_list, game_version, "00B4B11DA327CAD0")?
				{
					if prop_name == property_to_find {
						return Ok(Some((prop_type, default_val, false)));
					}
				}

				for property in self.get_matt_properties(resource_packages, hash_list, &factory)? {
					match property.data {
						MaterialPropertyData::Texture(texture) => {
							if property.name == property_to_find {
								return Ok(Some((
									"ZRuntimeResourceID".into(),
									texture
										.map(|texture| {
											json!({
												"resource": texture,
												"flag": "5F"
											})
										})
										.unwrap_or(Value::Null),
									false
								)));
							}

							if format!("{}_enab", property.name) == property_to_find {
								return Ok(Some(("bool".into(), json!(false), false)));
							}

							if format!("{}_dest", property.name) == property_to_find {
								return Ok(Some(("SEntityTemplateReference".into(), Value::Null, false)));
							}
						}

						MaterialPropertyData::ColorRGB(r, g, b) => {
							if property.name == property_to_find {
								return Ok(Some((
									"SColorRGB".into(),
									to_value(format!(
										"#{:0>2x}{:0>2x}{:0>2x}",
										(r * 255.0).round() as u8,
										(g * 255.0).round() as u8,
										(b * 255.0).round() as u8
									))?,
									false
								)));
							}

							if format!("{}_op", property.name) == property_to_find {
								return Ok(Some((
									"IRenderMaterialEntity.EModifierOperation".into(),
									to_value("eLeave")?,
									false
								)));
							}
						}

						MaterialPropertyData::ColorRGBA(r, g, b, a) => {
							if property.name == property_to_find {
								return Ok(Some((
									"SColorRGBA".into(),
									to_value(format!(
										"#{:0>2x}{:0>2x}{:0>2x}{:0>2x}",
										(r * 255.0).round() as u8,
										(g * 255.0).round() as u8,
										(b * 255.0).round() as u8,
										(a * 255.0).round() as u8
									))?,
									false
								)));
							}

							if format!("{}_op", property.name) == property_to_find {
								return Ok(Some((
									"IRenderMaterialEntity.EModifierOperation".into(),
									to_value("eLeave")?,
									false
								)));
							}
						}

						MaterialPropertyData::Float(val) => {
							if property.name == property_to_find {
								return Ok(Some(("float32".into(), to_value(val)?, false)));
							}

							if format!("{}_op", property.name) == property_to_find {
								return Ok(Some((
									"IRenderMaterialEntity.EModifierOperation".into(),
									to_value("eLeave")?,
									false
								)));
							}
						}

						MaterialPropertyData::Vector2(x, y) => {
							if property.name == property_to_find {
								return Ok(Some((
									"SVector2".into(),
									json!({
										"x": x,
										"y": y
									}),
									false
								)));
							}

							if format!("{}_op", property.name) == property_to_find {
								return Ok(Some((
									"IRenderMaterialEntity.EModifierOperation".into(),
									to_value("eLeave")?,
									false
								)));
							}
						}

						MaterialPropertyData::Vector3(x, y, z) => {
							if property.name == property_to_find {
								return Ok(Some((
									"SVector3".into(),
									json!({
										"x": x,
										"y": y,
										"z": z
									}),
									false
								)));
							}

							if format!("{}_op", property.name) == property_to_find {
								return Ok(Some((
									"IRenderMaterialEntity.EModifierOperation".into(),
									to_value("eLeave")?,
									false
								)));
							}
						}

						MaterialPropertyData::Vector4(x, y, z, w) => {
							if property.name == property_to_find {
								return Ok(Some((
									"SVector4".into(),
									json!({
										"x": x,
										"y": y,
										"z": z,
										"w": w
									}),
									false
								)));
							}

							if format!("{}_op", property.name) == property_to_find {
								return Ok(Some((
									"IRenderMaterialEntity.EModifierOperation".into(),
									to_value("eLeave")?,
									false
								)));
							}
						}
					}
				}
			} else if self.all_wswts.contains(&factory) {
				// All switch groups have the properties of ZAudioSwitchEntity
				for (prop_name, (prop_type, default_val)) in
					self.get_cppt_properties(resource_packages, hash_list, game_version, "00797DC916520C4D")?
				{
					if prop_name == property_to_find {
						return Ok(Some((prop_type, default_val, false)));
					}
				}
			} else if self.all_ecpts.contains(&factory) {
				// All extended CPP entities have the properties of ZMaterialOverwriteAspect
				for (prop_name, (prop_type, default_val)) in
					self.get_cppt_properties(resource_packages, hash_list, game_version, "00D3003AAA7B3817")?
				{
					if prop_name == property_to_find {
						return Ok(Some((prop_type, default_val, false)));
					}
				}

				let ecpb_data = extract_latest_resource(
					resource_packages,
					hash_list,
					&extract_latest_metadata(resource_packages, hash_list, &factory)?
						.hash_reference_data
						.into_iter()
						.find(|x| {
							hash_list
								.entries
								.get(&normalise_to_hash(x.hash.to_owned()))
								.map(|entry| entry.resource_type == "ECPB")
								.unwrap_or(false)
						})
						.context("No blueprint dependency on ECPT")?
						.hash
				)?
				.1;

				let ecpb_data = match game_version {
					GameVersion::H1 => h2016_convert_ecpb(&ecpb_data)?,
					GameVersion::H2 => h2_convert_ecpb(&ecpb_data)?,
					GameVersion::H3 => h3_convert_ecpb(&ecpb_data)?
				};

				for entry in ecpb_data.properties {
					if entry.property_name == property_to_find {
						return Ok(Some((
							match entry.property_type {
								EExtendedPropertyType::TYPE_RESOURCEPTR => "ZRuntimeResourceID",
								EExtendedPropertyType::TYPE_INT32 => "int32",
								EExtendedPropertyType::TYPE_UINT32 => "uint32",
								EExtendedPropertyType::TYPE_FLOAT => "float32",
								EExtendedPropertyType::TYPE_STRING => "ZString",
								EExtendedPropertyType::TYPE_BOOL => "bool",
								EExtendedPropertyType::TYPE_ENTITYREF => "SEntityTemplateReference",
								EExtendedPropertyType::TYPE_VARIANT => "ZVariant"
							}
							.into(),
							match entry.property_type {
								EExtendedPropertyType::TYPE_RESOURCEPTR => Value::Null,
								EExtendedPropertyType::TYPE_INT32 => to_value(0)?,
								EExtendedPropertyType::TYPE_UINT32 => to_value(0)?,
								EExtendedPropertyType::TYPE_FLOAT => to_value(0)?,
								EExtendedPropertyType::TYPE_STRING => Value::String("".into()),
								EExtendedPropertyType::TYPE_BOOL => Value::Bool(false),
								EExtendedPropertyType::TYPE_ENTITYREF => Value::Null,
								EExtendedPropertyType::TYPE_VARIANT => Value::Null
							},
							false
						)));
					}
				}
			} else if self.all_aibxs.contains(&factory) {
				// All behaviour trees have the properties of ZBehaviorTreeEntity
				for (prop_name, (prop_type, default_val)) in
					self.get_cppt_properties(resource_packages, hash_list, game_version, "0028607138892D70")?
				{
					if prop_name == property_to_find {
						return Ok(Some((prop_type, default_val, false)));
					}
				}
			} else {
				match ensure_entity_in_cache(
					resource_packages,
					cached_entities,
					game_version,
					hash_list,
					&normalise_to_hash(factory.to_owned())
				) {
					Ok(_) => {
						let extracted = cached_entities
							.read()
							.get(&normalise_to_hash(factory.to_owned()))
							.expect("Ensured")
							.to_owned();

						if let Some(data) = self.get_specific_property(
							resource_packages,
							cached_entities,
							hash_list,
							game_version,
							&extracted,
							&extracted.root_entity,
							property_to_find
						)? {
							return Ok(Some(data));
						}
					}

					Err(e) if format!("{:?}", e).contains("Couldn't find the resource in any RPKG") => {}

					x => {
						x?;
					}
				}
			}
		}

		None
	}

	/// Get the names of all input and output pins of a given sub-entity.
	#[try_fn]
	#[context("Couldn't get pins for sub-entity {} in {}", sub_entity, entity.factory_hash)]
	pub fn get_pins(
		&self,
		resource_packages: &IndexMap<PathBuf, ResourcePackage>,
		cached_entities: &RwLock<HashMap<String, Entity>>,
		hash_list: &HashList,
		game_version: GameVersion,
		entity: &Entity,
		sub_entity: &str,
		ignore_own: bool
	) -> Result<(Vec<String>, Vec<String>)> {
		let targeted = entity.entities.get(sub_entity).context("No such sub-entity")?;

		let mut input = vec![];
		let mut output = vec![];

		if !ignore_own {
			input.extend(
				targeted
					.input_copying
					.as_ref()
					.unwrap_or(&Default::default())
					.keys()
					.cloned()
			);

			output.extend(targeted.events.as_ref().unwrap_or(&Default::default()).keys().cloned());

			output.extend(
				targeted
					.output_copying
					.as_ref()
					.unwrap_or(&Default::default())
					.keys()
					.cloned()
			);
		}

		for sub_data in entity.entities.values() {
			for data in sub_data.events.as_ref().unwrap_or(&Default::default()).values() {
				for (trigger, refs) in data {
					for reference in refs {
						if get_local_reference(match reference {
							RefMaybeConstantValue::Ref(r) => r,
							RefMaybeConstantValue::RefWithConstantValue(RefWithConstantValue {
								entity_ref, ..
							}) => entity_ref
						})
						.map(|x| x == sub_entity)
						.unwrap_or(false)
						{
							input.push(trigger.to_owned());
						}
					}
				}
			}

			for data in sub_data.input_copying.as_ref().unwrap_or(&Default::default()).values() {
				for (trigger, refs) in data {
					for reference in refs {
						if get_local_reference(match reference {
							RefMaybeConstantValue::Ref(r) => r,
							RefMaybeConstantValue::RefWithConstantValue(RefWithConstantValue {
								entity_ref, ..
							}) => entity_ref
						})
						.map(|x| x == sub_entity)
						.unwrap_or(false)
						{
							input.push(trigger.to_owned());
						}
					}
				}
			}

			for data in sub_data.output_copying.as_ref().unwrap_or(&Default::default()).values() {
				for (propagate, refs) in data {
					for reference in refs {
						if get_local_reference(match reference {
							RefMaybeConstantValue::Ref(r) => r,
							RefMaybeConstantValue::RefWithConstantValue(RefWithConstantValue {
								entity_ref, ..
							}) => entity_ref
						})
						.map(|x| x == sub_entity)
						.unwrap_or(false)
						{
							output.push(propagate.to_owned());
						}
					}
				}
			}
		}

		let (fac_input, fac_output): (Vec<_>, Vec<_>) = {
			if self.all_asets.contains(&normalise_to_hash(targeted.factory.to_owned())) {
				extract_latest_metadata(
					resource_packages,
					hash_list,
					&normalise_to_hash(targeted.factory.to_owned())
				)?
				.hash_reference_data
				.into_iter()
				.rev()
				.skip(1)
				.rev()
				.map(|x| normalise_to_hash(x.hash))
				.collect_vec()
			} else {
				vec![normalise_to_hash(targeted.factory.to_owned())]
			}
		}
		.into_par_iter()
		.map(|factory| {
			Ok({
				let mut input = vec![];
				let mut output = vec![];

				if self.all_cppts.contains(&factory) {
					let cppt_data = self.cppt_pins.get(&factory).context("No such CPPT in pins")?;
					input.extend(cppt_data.0.to_owned());
					output.extend(cppt_data.1.to_owned());
				} else if self.all_uicts.contains(&factory) {
					// All UI controls have the pins of ZUIControlEntity
					let cppt_data = self.cppt_pins.get("002C4526CC9753E6").context("No such CPPT in pins")?;
					input.extend(cppt_data.0.to_owned());
					output.extend(cppt_data.1.to_owned());

					for entry in convert_uicb(
						&extract_latest_resource(
							resource_packages,
							hash_list,
							&extract_latest_metadata(resource_packages, hash_list, &factory)?
								.hash_reference_data
								.into_iter()
								.find(|x| {
									hash_list
										.entries
										.get(&normalise_to_hash(x.hash.to_owned()))
										.map(|entry| entry.resource_type == "UICB")
										.unwrap_or(false)
								})
								.context("No blueprint dependency on UICT")?
								.hash
						)?
						.1
					)?
					.m_aPins
					{
						// Pin
						if entry.m_nUnk00 == 1 {
							// All UICB pins are inputs, it seems
							input.push(entry.m_sName);
						}
					}
				} else if self.all_matts.contains(&factory) {
					// All materials have the pins of ZRenderMaterialEntity
					let cppt_data = self.cppt_pins.get("00B4B11DA327CAD0").context("No such CPPT in pins")?;
					input.extend(cppt_data.0.to_owned());
					output.extend(cppt_data.1.to_owned());

					for property in self.get_matt_properties(resource_packages, hash_list, &factory)? {
						if !matches!(property.data, MaterialPropertyData::Texture(_)) {
							input.push(property.name);
						}
					}
				} else if self.all_wswts.contains(&factory) {
					// All switch groups have the pins of ZAudioSwitchEntity
					let cppt_data = self.cppt_pins.get("00797DC916520C4D").context("No such CPPT in pins")?;
					input.extend(cppt_data.0.to_owned());
					output.extend(cppt_data.1.to_owned());

					let wswt_meta = extract_latest_metadata(resource_packages, hash_list, &factory)?;

					let dswb_hash = &wswt_meta
						.hash_reference_data
						.iter()
						.find(|x| {
							hash_list
								.entries
								.get(&normalise_to_hash(x.hash.to_owned()))
								.map(|entry| entry.resource_type == "DSWB" || entry.resource_type == "WSWB")
								.unwrap_or(false)
						})
						.context("No blueprint dependency on WSWT")?
						.hash;

					let dswb_data = match game_version {
						GameVersion::H1 => {
							h2016_convert_dswb(&extract_latest_resource(resource_packages, hash_list, dswb_hash)?.1)?
						}
						GameVersion::H2 => {
							h2_convert_dswb(&extract_latest_resource(resource_packages, hash_list, dswb_hash)?.1)?
						}
						GameVersion::H3 => {
							h3_convert_dswb(&extract_latest_resource(resource_packages, hash_list, dswb_hash)?.1)?
						}
					};

					input.extend(dswb_data.m_aSwitches);
				} else if self.all_ecpts.contains(&factory) {
					// All extended CPP entities have the pins of ZMaterialOverwriteAspect
					let cppt_data = self.cppt_pins.get("00D3003AAA7B3817").context("No such CPPT in pins")?;
					input.extend(cppt_data.0.to_owned());
					output.extend(cppt_data.1.to_owned());
				} else if self.all_aibxs.contains(&factory) {
					// All behaviour trees have the properties of ZBehaviorTreeEntity
					let cppt_data = self.cppt_pins.get("0028607138892D70").context("No such CPPT in pins")?;
					input.extend(cppt_data.0.to_owned());
					output.extend(cppt_data.1.to_owned());
				} else {
					match ensure_entity_in_cache(
						resource_packages,
						cached_entities,
						game_version,
						hash_list,
						&normalise_to_hash(factory.to_owned())
					) {
						Ok(_) => {
							let extracted = cached_entities
								.read()
								.get(&normalise_to_hash(factory.to_owned()))
								.expect("Ensured")
								.to_owned();

							let found = self.get_pins(
								resource_packages,
								cached_entities,
								hash_list,
								game_version,
								&extracted,
								&extracted.root_entity,
								false
							)?;

							input.extend(found.0);
							output.extend(found.1);
						}

						Err(e) if format!("{:?}", e).contains("Couldn't find the resource in any RPKG") => {}

						x => {
							x?;
						}
					}
				}

				(input, output)
			})
		})
		.collect::<Result<Vec<_>>>()?
		.into_iter()
		.unzip();

		input.extend(fac_input.into_iter().flatten());
		output.extend(fac_output.into_iter().flatten());

		(
			input.into_iter().unique().collect(),
			output.into_iter().unique().collect()
		)
	}
}

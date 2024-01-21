// This file has been generated by Specta. DO NOT EDIT.

export type AppSettings = { extractModdedFiles: boolean; gameInstall: string | null }

export type ArrayPatchOperation = { RemoveItemByValue: JsonValue } | { AddItemAfter: [JsonValue, JsonValue] } | { AddItemBefore: [JsonValue, JsonValue] } | { AddItem: JsonValue }

/**
 * A comment entity.
 * 
 * Will be displayed in QuickEntity Editor as a tree item with a sticky note icon.
 */
export type CommentEntity = { 
/**
 * The sub-entity this comment is parented to.
 */
parent: Ref; 
/**
 * The name of this comment.
 */
name: string; 
/**
 * The text this comment holds.
 */
text: string }

export type CopiedEntityData = { 
/**
 * Which entity has been copied (and should be parented to the selection when pasting).
 */
rootEntity: string; data: { [key in string]: SubEntity } }

/**
 * A dependency of an entity.
 */
export type Dependency = DependencyWithFlag | 
/**
 * A dependency which is flagged as "1F".
 */
string

/**
 * A dependency with a flag other than the default (1F).
 */
export type DependencyWithFlag = { resource: string; flag: string }

export type EditorEvent = { type: "text"; data: TextEditorEvent } | { type: "entity"; data: EntityEditorEvent }

export type EditorRequest = { type: "text"; data: TextEditorRequest } | { type: "entity"; data: EntityEditorRequest }

export type EditorType = { type: "Nil" } | { type: "Text"; data: { file_type: TextFileType } } | { type: "QNEntity" } | { type: "QNPatch" }

export type EditorValidity = { type: "Valid" } | { type: "Invalid"; data: string }

export type Entity = { 
/**
 * The hash of the TEMP file of this entity.
 */
tempHash: string; 
/**
 * The hash of the TBLU file of this entity.
 */
tbluHash: string; 
/**
 * The root sub-entity of this entity.
 */
rootEntity: string; 
/**
 * The sub-entities of this entity.
 */
entities: { [key in string]: SubEntity }; 
/**
 * Properties on other entities (local or external) to override when this entity is loaded.
 * 
 * Overriding a local entity would be a rather pointless maneuver given that you could just actually change it in the entity instead of using an override.
 */
propertyOverrides: PropertyOverride[]; 
/**
 * Entities (external or local) to delete (including their organisational children) when
 * this entity is loaded.
 * 
 * Deleting a local entity would be a rather pointless maneuver given that you could just actually remove it from this entity instead of using an override.
 */
overrideDeletes: Ref[]; 
/**
 * Pin (event) connections (between entities, external or local) to add when this entity is
 * loaded.
 */
pinConnectionOverrides: PinConnectionOverride[]; 
/**
 * Pin (event) connections (between entities, external or local) to delete when this entity
 * is loaded.
 */
pinConnectionOverrideDeletes: PinConnectionOverrideDelete[]; 
/**
 * The external scenes that this entity references.
 */
externalScenes: string[]; 
/**
 * The type of this entity.
 */
subType: SubType; 
/**
 * The QuickEntity format version of this entity. The current version is 3.1.
 */
quickEntityVersion: number; 
/**
 * Extra resource dependencies that should be added to the entity's factory when converted to the game's format.
 */
extraFactoryDependencies: Dependency[]; 
/**
 * Extra resource dependencies that should be added to the entity's blueprint when converted to the game's format.
 */
extraBlueprintDependencies: Dependency[]; 
/**
 * Comments to be attached to sub-entities.
 * 
 * Will be displayed in QuickEntity Editor as tree items with a sticky note icon.
 */
comments: CommentEntity[] }

export type EntityEditorEvent = { type: "general"; data: EntityGeneralEvent } | { type: "tree"; data: EntityTreeEvent } | { type: "monaco"; data: EntityMonacoEvent } | { type: "metaPane"; data: EntityMetaPaneEvent } | { type: "metadata"; data: EntityMetadataEvent } | { type: "overrides"; data: EntityOverridesEvent }

export type EntityEditorRequest = { type: "tree"; data: EntityTreeRequest } | { type: "monaco"; data: EntityMonacoRequest } | { type: "metaPane"; data: EntityMetaPaneRequest } | { type: "metadata"; data: EntityMetadataRequest } | { type: "overrides"; data: EntityOverridesRequest }

export type EntityGeneralEvent = { type: "setShowReverseParentRefs"; data: { editor_id: string; show_reverse_parent_refs: boolean } }

export type EntityMetaPaneEvent = { type: "jumpToReference"; data: { editor_id: string; reference: string } } | { type: "setNotes"; data: { editor_id: string; entity_id: string; notes: string } }

export type EntityMetaPaneRequest = { type: "setReverseRefs"; data: { editor_id: string; entity_names: { [key in string]: string }; reverse_refs: ReverseReference[] } } | { type: "setNotes"; data: { editor_id: string; entity_id: string; notes: string } }

export type EntityMetadataEvent = { type: "initialise"; data: { editor_id: string } } | { type: "setFactoryHash"; data: { editor_id: string; factory_hash: string } } | { type: "setBlueprintHash"; data: { editor_id: string; blueprint_hash: string } } | { type: "setRootEntity"; data: { editor_id: string; root_entity: string } } | { type: "setSubType"; data: { editor_id: string; sub_type: SubType } } | { type: "setExternalScenes"; data: { editor_id: string; external_scenes: string[] } }

export type EntityMetadataRequest = { type: "initialise"; data: { editor_id: string; factory_hash: string; blueprint_hash: string; root_entity: string; sub_type: SubType; external_scenes: string[] } }

export type EntityMonacoEvent = { type: "updateContent"; data: { editor_id: string; entity_id: string; content: string } } | { type: "followReference"; data: { editor_id: string; reference: string } } | { type: "openFactory"; data: { editor_id: string; factory: string } }

export type EntityMonacoRequest = { type: "replaceContent"; data: { editor_id: string; entity_id: string; content: string } } | { type: "updateIntellisense"; data: { editor_id: string; entity_id: string; properties: ([string, string, JsonValue, boolean])[]; pins: [string[], string[]] } } | { type: "updateDecorationsAndMonacoInfo"; data: { editor_id: string; entity_id: string; decorations: ([string, string])[]; local_ref_entity_ids: string[] } } | { type: "updateValidity"; data: { editor_id: string; validity: EditorValidity } }

export type EntityOverridesEvent = { type: "initialise"; data: { editor_id: string } } | { type: "updatePropertyOverrides"; data: { editor_id: string; content: string } } | { type: "updateOverrideDeletes"; data: { editor_id: string; content: string } } | { type: "updatePinConnectionOverrides"; data: { editor_id: string; content: string } } | { type: "updatePinConnectionOverrideDeletes"; data: { editor_id: string; content: string } }

export type EntityOverridesRequest = { type: "initialise"; data: { editor_id: string; property_overrides: string; override_deletes: string; pin_connection_overrides: string; pin_connection_override_deletes: string } } | { type: "updateDecorations"; data: { editor_id: string; decorations: ([string, string])[] } }

export type EntityTreeEvent = { type: "initialise"; data: { editor_id: string } } | { type: "select"; data: { editor_id: string; id: string } } | { type: "create"; data: { editor_id: string; id: string; content: SubEntity } } | { type: "delete"; data: { editor_id: string; id: string } } | { type: "rename"; data: { editor_id: string; id: string; new_name: string } } | { type: "reparent"; data: { editor_id: string; id: string; new_parent: Ref } } | { type: "copy"; data: { editor_id: string; id: string } } | { type: "paste"; data: { editor_id: string; parent_id: string } } | { type: "search"; data: { editor_id: string; query: string } } | { type: "showHelpMenu"; data: { editor_id: string; entity_id: string } } | { type: "useTemplate"; data: { editor_id: string; parent_id: string; template: CopiedEntityData } }

export type EntityTreeRequest = 
/**
 * Will trigger a Select event from the tree - ensure this doesn't end up in a loop
 */
{ type: "select"; data: { editor_id: string; id: string | null } } | { type: "newTree"; data: { editor_id: string; 
/**
 * ID, parent, name, factory, has reverse parent refs
 */
entities: ([string, Ref, string, string, boolean])[] } } | 
/**
 * Instructs the frontend to take the list of new entities, add any new ones and update any ones that already exist (by ID) with the new information.
 * This is used for pasting, and for ensuring that icons/parent status/name are updated when a sub-entity is updated.
 */
{ type: "newItems"; data: { editor_id: string; 
/**
 * ID, parent, name, factory, has reverse parent refs
 */
new_entities: ([string, Ref, string, string, boolean])[] } } | { type: "searchResults"; data: { editor_id: string; 
/**
 * The IDs of the entities matching the query
 */
results: string[] } } | { type: "showHelpMenu"; data: { editor_id: string; factory: string; input_pins: string[]; output_pins: string[]; default_properties_html: string } } | { type: "setTemplates"; data: { editor_id: string; templates: PastableTemplate[] } }

export type EphemeralQNSettings = { showReverseParentRefs: boolean }

export type Event = { type: "tool"; data: ToolEvent } | { type: "editor"; data: EditorEvent } | { type: "global"; data: GlobalEvent }

/**
 * An exposed entity.
 * 
 * Exposed entities are accessible when referencing this entity through a property on long-form references.
 */
export type ExposedEntity = { 
/**
 * Whether there are multiple target entities.
 */
isArray: boolean; 
/**
 * The target entity (or entities) that will be accessed.
 */
refersTo: Ref[] }

export type FileBrowserEvent = { type: "select"; data: string | null } | { type: "create"; data: { path: string; is_folder: boolean } } | { type: "delete"; data: string } | { type: "rename"; data: { old_path: string; new_path: string } } | { type: "normaliseQNFile"; data: { path: string } } | { type: "convertEntityToPatch"; data: { path: string } } | { type: "convertPatchToEntity"; data: { path: string } }

export type FileBrowserRequest = { type: "create"; data: { path: string; is_folder: boolean } } | { type: "delete"; data: string } | { type: "rename"; data: { old_path: string; new_path: string } } | { type: "select"; data: string | null } | { type: "newTree"; data: { base_path: string; 
/**
 * Relative path, is folder
 */
files: ([string, boolean])[] } }

/**
 * A long-form reference to an entity, allowing for the specification of external scenes and/or an exposed entity.
 */
export type FullRef = { 
/**
 * The entity to reference's ID.
 */
ref: string; 
/**
 * The external scene the referenced entity resides in.
 */
externalScene: string | null; 
/**
 * The sub-entity to reference that is exposed by the referenced entity.
 */
exposedEntity?: string | null }

export type GameBrowserEntry = { hash: string; path: string | null; hint: string | null }

export type GameBrowserEvent = { type: "select"; data: string } | { type: "search"; data: string }

export type GameBrowserRequest = { type: "setEnabled"; data: boolean } | { type: "newTree"; data: { game_description: string; entries: GameBrowserEntry[] } }

export type GameInstall = { version: GameVersion; platform: string; path: string }

export type GameVersion = "h1" | "h2" | "h3"

export type GlobalEvent = { type: "loadWorkspace"; data: string } | { type: "selectTab"; data: string | null } | { type: "removeTab"; data: string } | { type: "saveTab"; data: string }

export type GlobalRequest = { type: "errorReport"; data: { error: string } } | { type: "setWindowTitle"; data: string } | { type: "createTab"; data: { id: string; name: string; editor_type: EditorType } } | { type: "selectTab"; data: string } | { type: "setTabUnsaved"; data: { id: string; unsaved: boolean } } | { type: "removeTab"; data: string }

export type JsonValue = null | boolean | number | string | JsonValue[] | { [key in string]: JsonValue }

export type OverriddenProperty = { 
/**
 * The type of the property.
 */
type: string; 
/**
 * The value of the property.
 */
value: JsonValue }

export type PastableTemplate = { name: string; icon: string; pasteData: CopiedEntityData }

export type Patch = { 
/**
 * The hash of the TEMP file of this entity.
 */
tempHash: string; 
/**
 * The hash of the TBLU file of this entity.
 */
tbluHash: string; 
/**
 * The patch operations to apply.
 */
patch: PatchOperation[]; 
/**
 * The patch version. The current version is 6.
 */
patchVersion: number }

export type PatchOperation = { SetRootEntity: string } | { SetSubType: SubType } | { AddEntity: [string, SubEntity] } | { RemoveEntityByID: string } | { SubEntityOperation: [string, SubEntityOperation] } | 
/**
 * Should no longer be emitted by patch generators.
 */
{ AddPropertyOverride: PropertyOverride } | 
/**
 * Should no longer be emitted by patch generators.
 */
{ RemovePropertyOverride: PropertyOverride } | { AddPropertyOverrideConnection: PropertyOverrideConnection } | { RemovePropertyOverrideConnection: PropertyOverrideConnection } | { AddOverrideDelete: Ref } | { RemoveOverrideDelete: Ref } | { AddPinConnectionOverride: PinConnectionOverride } | { RemovePinConnectionOverride: PinConnectionOverride } | { AddPinConnectionOverrideDelete: PinConnectionOverrideDelete } | { RemovePinConnectionOverrideDelete: PinConnectionOverrideDelete } | { AddExternalScene: string } | { RemoveExternalScene: string } | { AddExtraFactoryDependency: Dependency } | { RemoveExtraFactoryDependency: Dependency } | { AddExtraBlueprintDependency: Dependency } | { RemoveExtraBlueprintDependency: Dependency } | { AddComment: CommentEntity } | { RemoveComment: CommentEntity }

export type PinConnectionOverride = { 
/**
 * The entity that will trigger the input on the other entity.
 * 
 * If this references a local entity, you can simply use an event on the entity itself.
 */
fromEntity: Ref; 
/**
 * The name of the event on the fromEntity that will trigger the input on the toEntity.
 */
fromPin: string; 
/**
 * The entity whose input will be triggered.
 */
toEntity: Ref; 
/**
 * The name of the input on the toEntity that will be triggered by the event on the
 * fromEntity.
 */
toPin: string; 
/**
 * The constant value of the input to the toEntity.
 */
value?: SimpleProperty | null }

export type PinConnectionOverrideDelete = { 
/**
 * The entity that triggers the input on the other entity.
 */
fromEntity: Ref; 
/**
 * The name of the event on the fromEntity that will no longer trigger the input on the
 * toEntity.
 */
fromPin: string; 
/**
 * The entity whose input is triggered.
 */
toEntity: Ref; 
/**
 * The name of the input on the toEntity that will no longer be triggered by the event on
 * the fromEntity.
 */
toPin: string; 
/**
 * The constant value of the input to the toEntity.
 */
value?: SimpleProperty | null }

export type ProjectSettings = Record<string, never>

/**
 * A property with a type and a value. Can be marked as post-init.
 */
export type Property = { 
/**
 * The type of the property.
 */
type: string; 
/**
 * The value of the property.
 */
value: JsonValue; 
/**
 * Whether the property should be (presumably) loaded/set after the entity has been initialised.
 */
postInit?: boolean | null }

/**
 * A property alias.
 * 
 * Property aliases are used to access properties of other entities through a single entity.
 */
export type PropertyAlias = { 
/**
 * The other entity's property that should be accessed from this entity.
 */
originalProperty: string; 
/**
 * The other entity whose property will be accessed.
 */
originalEntity: Ref }

/**
 * A set of overrides for entity properties.
 */
export type PropertyOverride = { 
/**
 * An array of references to the entities to override the properties of.
 */
entities: Ref[]; 
/**
 * A set of properties to override on the entities.
 */
properties: { [key in string]: OverriddenProperty } }

/**
 * A single entity-property override.
 */
export type PropertyOverrideConnection = { 
/**
 * A reference to an entity to override a property on.
 */
entity: Ref; 
/**
 * The property to override.
 */
propertyName: string; 
/**
 * The overridden property.
 */
propertyOverride: OverriddenProperty }

/**
 * A reference to an entity.
 */
export type Ref = FullRef | 
/**
 * A short-form reference represents either a local reference with no exposed entity or a null reference.
 */
string | null

export type RefMaybeConstantValue = RefWithConstantValue | Ref

/**
 * A reference accompanied by a constant value.
 */
export type RefWithConstantValue = { 
/**
 * The entity to reference's ID.
 */
ref: Ref; 
/**
 * The constant value accompanying this reference.
 */
value: SimpleProperty }

export type Request = { type: "tool"; data: ToolRequest } | { type: "editor"; data: EditorRequest } | { type: "global"; data: GlobalRequest }

export type ReverseReference = { from: string; data: ReverseReferenceData }

export type ReverseReferenceData = { type: "parent" } | { type: "property"; data: { property_name: string } } | { type: "platformSpecificProperty"; data: { property_name: string; platform: string } } | { type: "event"; data: { event: string; trigger: string } } | { type: "inputCopy"; data: { trigger: string; propagate: string } } | { type: "outputCopy"; data: { event: string; propagate: string } } | { type: "propertyAlias"; data: { aliased_name: string; original_property: string } } | { type: "exposedEntity"; data: { exposed_name: string } } | { type: "exposedInterface"; data: { interface: string } } | { type: "subset"; data: { subset: string } }

export type SettingsEvent = { type: "initialise" } | { type: "changeGameInstall"; data: string | null } | { type: "changeExtractModdedFiles"; data: boolean }

export type SettingsRequest = { type: "initialise"; data: { game_installs: GameInstall[]; settings: AppSettings } } | { type: "changeProjectSettings"; data: ProjectSettings }

/**
 * A simple property.
 * 
 * Simple properties cannot be marked as post-init. They are used by pin connection overrides, events and input/output copying.
 */
export type SimpleProperty = { 
/**
 * The type of the simple property.
 */
type: string; 
/**
 * The simple property's value.
 */
value: JsonValue }

export type SubEntity = { 
/**
 * The "logical" or "organisational" parent of the entity, used for tree organisation in graphical editors.
 * 
 * Has no effect on the entity in game.
 */
parent: Ref; 
/**
 * The name of the entity.
 */
name: string; 
/**
 * The factory of the entity.
 */
factory: string; 
/**
 * The factory's flag.
 * 
 * You can leave this out if it's 1F.
 */
factoryFlag?: string | null; 
/**
 * The blueprint of the entity.
 */
blueprint: string; 
/**
 * Whether the entity is only loaded in IO's editor.
 * 
 * Setting this to true will remove the entity from the game as well as all of its organisational (but not coordinate) children.
 */
editorOnly?: boolean | null; 
/**
 * Properties of the entity.
 */
properties?: { [key in string]: Property } | null; 
/**
 * Properties to apply conditionally to the entity based on platform.
 */
platformSpecificProperties?: { [key in string]: { [key in string]: Property } } | null; 
/**
 * Inputs on entities to trigger when events occur.
 */
events?: { [key in string]: { [key in string]: RefMaybeConstantValue[] } } | null; 
/**
 * Inputs on entities to trigger when this entity is given inputs.
 */
inputCopying?: { [key in string]: { [key in string]: RefMaybeConstantValue[] } } | null; 
/**
 * Events to propagate on other entities.
 */
outputCopying?: { [key in string]: { [key in string]: RefMaybeConstantValue[] } } | null; 
/**
 * Properties on other entities that can be accessed from this entity.
 */
propertyAliases?: { [key in string]: PropertyAlias[] } | null; 
/**
 * Entities that can be accessed from this entity.
 */
exposedEntities?: { [key in string]: ExposedEntity } | null; 
/**
 * Interfaces implemented by other entities that can be accessed from this entity.
 */
exposedInterfaces?: { [key in string]: string } | null; 
/**
 * The subsets that this entity belongs to.
 */
subsets?: { [key in string]: string[] } | null }

export type SubEntityOperation = { SetParent: Ref } | { SetName: string } | { SetFactory: string } | { SetFactoryFlag: string | null } | { SetBlueprint: string } | { SetEditorOnly: boolean | null } | { AddProperty: [string, Property] } | { SetPropertyType: [string, string] } | { SetPropertyValue: { property_name: string; value: JsonValue } } | { PatchArrayPropertyValue: [string, ArrayPatchOperation[]] } | { SetPropertyPostInit: [string, boolean] } | { RemovePropertyByName: string } | { AddPlatformSpecificProperty: [string, string, Property] } | { SetPlatformSpecificPropertyType: [string, string, string] } | { SetPlatformSpecificPropertyValue: { platform: string; property_name: string; value: JsonValue } } | { PatchPlatformSpecificArrayPropertyValue: [string, string, ArrayPatchOperation[]] } | { SetPlatformSpecificPropertyPostInit: [string, string, boolean] } | { RemovePlatformSpecificPropertyByName: [string, string] } | { RemovePlatformSpecificPropertiesForPlatform: string } | { AddEventConnection: [string, string, RefMaybeConstantValue] } | { RemoveEventConnection: [string, string, RefMaybeConstantValue] } | { RemoveAllEventConnectionsForTrigger: [string, string] } | { RemoveAllEventConnectionsForEvent: string } | { AddInputCopyConnection: [string, string, RefMaybeConstantValue] } | { RemoveInputCopyConnection: [string, string, RefMaybeConstantValue] } | { RemoveAllInputCopyConnectionsForTrigger: [string, string] } | { RemoveAllInputCopyConnectionsForInput: string } | { AddOutputCopyConnection: [string, string, RefMaybeConstantValue] } | { RemoveOutputCopyConnection: [string, string, RefMaybeConstantValue] } | { RemoveAllOutputCopyConnectionsForPropagate: [string, string] } | { RemoveAllOutputCopyConnectionsForOutput: string } | { AddPropertyAliasConnection: [string, PropertyAlias] } | { RemovePropertyAlias: string } | { RemoveConnectionForPropertyAlias: [string, PropertyAlias] } | { SetExposedEntity: [string, ExposedEntity] } | { RemoveExposedEntity: string } | { SetExposedInterface: [string, string] } | { RemoveExposedInterface: string } | { AddSubset: [string, string] } | { RemoveSubset: [string, string] } | { RemoveAllSubsetsFor: string }

export type SubType = "brick" | "scene" | "template"

export type TextEditorEvent = { type: "initialise"; data: { id: string } } | { type: "updateContent"; data: { id: string; content: string } }

export type TextEditorRequest = { type: "replaceContent"; data: { id: string; content: string } } | { type: "setFileType"; data: { id: string; file_type: TextFileType } }

export type TextFileType = { type: "Json" } | { type: "ManifestJson" } | { type: "PlainText" } | { type: "Markdown" }

export type ToolEvent = { type: "fileBrowser"; data: FileBrowserEvent } | { type: "gameBrowser"; data: GameBrowserEvent } | { type: "settings"; data: SettingsEvent }

export type ToolRequest = { type: "fileBrowser"; data: FileBrowserRequest } | { type: "gameBrowser"; data: GameBrowserRequest } | { type: "settings"; data: SettingsRequest }


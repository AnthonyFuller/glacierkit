         // This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

         export const commands = {
async event(event: Event) : Promise<null> {
return await TAURI_INVOKE("plugin:tauri-specta|event", { event });
},
async showInFolder(path: string) : Promise<null> {
return await TAURI_INVOKE("plugin:tauri-specta|show_in_folder", { path });
}
}



/** user-defined types **/

export type ContentSearchEvent = { type: "search"; data: [string, string[], boolean] }
export type ContentSearchResultsEvent = { type: "initialise"; data: { id: string } } | { type: "openResourceOverview"; data: { id: string; hash: string } }
export type CopiedEntityData = { 
/**
 * Which entity has been copied (and should be parented to the selection when pasting).
 */
rootEntity: string; data: { [key in string]: SubEntity } }
export type EditorConnectionEvent = { type: "entitySelected"; data: [string, string] } | { type: "entityTransformUpdated"; data: [string, string, QNTransform] } | { type: "entityPropertyChanged"; data: [string, string, string, string, JsonValue] }
export type EditorEvent = { type: "text"; data: TextEditorEvent } | { type: "entity"; data: EntityEditorEvent } | { type: "resourceOverview"; data: ResourceOverviewEvent } | { type: "repositoryPatch"; data: RepositoryPatchEditorEvent } | { type: "unlockablesPatch"; data: UnlockablesPatchEditorEvent } | { type: "contentSearchResults"; data: ContentSearchResultsEvent }
export type EntityEditorEvent = { type: "general"; data: EntityGeneralEvent } | { type: "tree"; data: EntityTreeEvent } | { type: "monaco"; data: EntityMonacoEvent } | { type: "metaPane"; data: EntityMetaPaneEvent } | { type: "metadata"; data: EntityMetadataEvent } | { type: "overrides"; data: EntityOverridesEvent }
export type EntityGeneralEvent = { type: "setShowReverseParentRefs"; data: { editor_id: string; show_reverse_parent_refs: boolean } } | { type: "setShowChangesFromOriginal"; data: { editor_id: string; show_changes_from_original: boolean } }
export type EntityMetaPaneEvent = { type: "jumpToReference"; data: { editor_id: string; reference: string } } | { type: "setNotes"; data: { editor_id: string; entity_id: string; notes: string } }
export type EntityMetadataEvent = { type: "initialise"; data: { editor_id: string } } | { type: "setFactoryHash"; data: { editor_id: string; factory_hash: string } } | { type: "setBlueprintHash"; data: { editor_id: string; blueprint_hash: string } } | { type: "setRootEntity"; data: { editor_id: string; root_entity: string } } | { type: "setSubType"; data: { editor_id: string; sub_type: SubType } } | { type: "setExternalScenes"; data: { editor_id: string; external_scenes: string[] } }
export type EntityMonacoEvent = { type: "updateContent"; data: { editor_id: string; entity_id: string; content: string } } | { type: "followReference"; data: { editor_id: string; reference: string } } | { type: "openFactory"; data: { editor_id: string; factory: string } } | { type: "signalPin"; data: { editor_id: string; entity_id: string; pin: string; output: boolean } }
export type EntityOverridesEvent = { type: "initialise"; data: { editor_id: string } } | { type: "updatePropertyOverrides"; data: { editor_id: string; content: string } } | { type: "updateOverrideDeletes"; data: { editor_id: string; content: string } } | { type: "updatePinConnectionOverrides"; data: { editor_id: string; content: string } } | { type: "updatePinConnectionOverrideDeletes"; data: { editor_id: string; content: string } }
export type EntityTreeEvent = { type: "initialise"; data: { editor_id: string } } | { type: "select"; data: { editor_id: string; id: string } } | { type: "create"; data: { editor_id: string; id: string; content: SubEntity } } | { type: "delete"; data: { editor_id: string; id: string } } | { type: "rename"; data: { editor_id: string; id: string; new_name: string } } | { type: "reparent"; data: { editor_id: string; id: string; new_parent: Ref } } | { type: "copy"; data: { editor_id: string; id: string } } | { type: "paste"; data: { editor_id: string; parent_id: string } } | { type: "search"; data: { editor_id: string; query: string } } | { type: "showHelpMenu"; data: { editor_id: string; entity_id: string } } | { type: "useTemplate"; data: { editor_id: string; parent_id: string; template: CopiedEntityData } } | { type: "addGameBrowserItem"; data: { editor_id: string; parent_id: string; file: string } } | { type: "selectEntityInEditor"; data: { editor_id: string; entity_id: string } } | { type: "moveEntityToPlayer"; data: { editor_id: string; entity_id: string } } | { type: "rotateEntityAsPlayer"; data: { editor_id: string; entity_id: string } } | { type: "moveEntityToCamera"; data: { editor_id: string; entity_id: string } } | { type: "rotateEntityAsCamera"; data: { editor_id: string; entity_id: string } } | { type: "restoreToOriginal"; data: { editor_id: string; entity_id: string } }
export type Event = { type: "tool"; data: ToolEvent } | { type: "editor"; data: EditorEvent } | { type: "global"; data: GlobalEvent } | { type: "editorConnection"; data: EditorConnectionEvent }
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
export type FileBrowserEvent = { type: "select"; data: string | null } | { type: "create"; data: { path: string; is_folder: boolean } } | { type: "delete"; data: string } | { type: "rename"; data: { old_path: string; new_path: string } } | { type: "normaliseQNFile"; data: { path: string } } | { type: "convertEntityToPatch"; data: { path: string } } | { type: "convertPatchToEntity"; data: { path: string } } | { type: "convertRepoPatchToMergePatch"; data: { path: string } } | { type: "convertRepoPatchToJsonPatch"; data: { path: string } } | { type: "convertUnlockablesPatchToMergePatch"; data: { path: string } } | { type: "convertUnlockablesPatchToJsonPatch"; data: { path: string } }
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
export type GameBrowserEvent = { type: "select"; data: string } | { type: "search"; data: [string, SearchFilter] } | { type: "openInEditor"; data: string }
export type GlobalEvent = { type: "setSeenAnnouncements"; data: string[] } | { type: "loadWorkspace"; data: string } | { type: "selectTab"; data: string | null } | { type: "removeTab"; data: string } | { type: "saveTab"; data: string }
export type JsonValue = null | boolean | number | string | JsonValue[] | { [key in string]: JsonValue }
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
export type QNTransform = { rotation: Vec3; position: Vec3; scale?: Vec3 | null }
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
export type RepositoryPatchEditorEvent = { type: "initialise"; data: { id: string } } | { type: "createRepositoryItem"; data: { id: string } } | { type: "resetModifications"; data: { id: string; item: string } } | { type: "modifyItem"; data: { id: string; item: string; data: string } } | { type: "selectItem"; data: { id: string; item: string } }
export type ResourceOverviewEvent = { type: "initialise"; data: { id: string } } | { type: "followDependency"; data: { id: string; new_hash: string } } | { type: "followDependencyInNewTab"; data: { id: string; hash: string } } | { type: "openInEditor"; data: { id: string } } | { type: "extractAsQN"; data: { id: string } } | { type: "extractAsFile"; data: { id: string } } | { type: "extractTEMPAsRT"; data: { id: string } } | { type: "extractTBLUAsFile"; data: { id: string } } | { type: "extractTBLUAsRT"; data: { id: string } } | { type: "extractAsRTGeneric"; data: { id: string } } | { type: "extractAsImage"; data: { id: string } } | { type: "extractAsWav"; data: { id: string } } | { type: "extractMultiWav"; data: { id: string } } | { type: "extractSpecificMultiWav"; data: { id: string; index: number } } | { type: "extractORESAsJson"; data: { id: string } }
export type SearchFilter = "All" | "Templates" | "Classes" | "Models" | "Textures" | "Sound"
export type SettingsEvent = { type: "initialise" } | { type: "changeGameInstall"; data: string | null } | { type: "changeExtractModdedFiles"; data: boolean } | { type: "changeColourblind"; data: boolean } | { type: "changeCustomPaths"; data: string[] }
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
export type SubType = "brick" | "scene" | "template"
export type TextEditorEvent = { type: "initialise"; data: { id: string } } | { type: "updateContent"; data: { id: string; content: string } }
export type ToolEvent = { type: "fileBrowser"; data: FileBrowserEvent } | { type: "gameBrowser"; data: GameBrowserEvent } | { type: "settings"; data: SettingsEvent } | { type: "contentSearch"; data: ContentSearchEvent }
export type UnlockablesPatchEditorEvent = { type: "initialise"; data: { id: string } } | { type: "createUnlockable"; data: { id: string } } | { type: "resetModifications"; data: { id: string; unlockable: string } } | { type: "modifyUnlockable"; data: { id: string; unlockable: string; data: string } } | { type: "selectUnlockable"; data: { id: string; unlockable: string } }
export type Vec3 = { x: number; y: number; z: number }

/** tauri-specta globals **/

         import { invoke as TAURI_INVOKE } from "@tauri-apps/api";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindowHandle as __WebviewWindowHandle__ } from "@tauri-apps/api/window";

type __EventObj__<T> = {
  listen: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: T extends null
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

type __Result__<T, E> =
  | { status: "ok"; data: T }
  | { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
  mappings: Record<keyof T, string>
) {
  return new Proxy(
    {} as unknown as {
      [K in keyof T]: __EventObj__<T[K]> & {
        (handle: __WebviewWindowHandle__): __EventObj__<T[K]>;
      };
    },
    {
      get: (_, event) => {
        const name = mappings[event as keyof T];

        return new Proxy((() => {}) as any, {
          apply: (_, __, [window]: [__WebviewWindowHandle__]) => ({
            listen: (arg: any) => window.listen(name, arg),
            once: (arg: any) => window.once(name, arg),
            emit: (arg: any) => window.emit(name, arg),
          }),
          get: (_, command: keyof __EventObj__<any>) => {
            switch (command) {
              case "listen":
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case "once":
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case "emit":
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    }
  );
}

     
#![allow(dead_code, unused_imports, non_snake_case, nonstandard_style)]
use rabex_env::rabex::objects::{PPtr, TypedPPtr};
use rabex_env::unity::types::*;

#[derive(Debug, serde::Deserialize)]
pub struct PlayMakerFSM {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub fsm: Fsm,
    pub fsmTemplate: PPtr, /* FsmTemplate */
    pub eventHandlerComponentsAdded: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct Fsm {
    pub dataVersion: i32,
    pub usedInTemplate: PPtr, /* FsmTemplate */
    pub name: String,
    pub startState: String,
    pub states: Vec<FsmState>,
    pub events: Vec<FsmEvent>,
    pub globalTransitions: Vec<FsmTransition>,
    pub variables: FsmVariables,
    pub description: String,
    pub docUrl: String,
    pub showStateLabel: u8,
    pub maxLoopCount: i32,
    pub watermark: String,
    pub password: String,
    pub locked: u8,
    pub manualUpdate: u8,
    pub outVariableIndices: Vec<i32>,
    pub keepDelayedEventsOnStateExit: u8,
    pub preprocessed: u8,
    pub ExposedEvents: Vec<FsmEvent>,
    pub OutputEvents: Vec<FsmEvent>,
    pub RestartOnEnable: u8,
    pub ResetVariablesOnEnable: u8,
    pub EnableDebugFlow: u8,
    pub EnableBreakpoints: u8,
    pub editorFlags: i32,
    pub activeStateName: String,
    pub mouseEvents: u8,
    pub handleLevelLoaded: u8,
    pub handleTriggerEnter2D: u8,
    pub handleTriggerExit2D: u8,
    pub handleTriggerStay2D: u8,
    pub handleCollisionEnter2D: u8,
    pub handleCollisionExit2D: u8,
    pub handleCollisionStay2D: u8,
    pub handleTriggerEnter: u8,
    pub handleTriggerExit: u8,
    pub handleTriggerStay: u8,
    pub handleCollisionEnter: u8,
    pub handleCollisionExit: u8,
    pub handleCollisionStay: u8,
    pub handleParticleCollision: u8,
    pub handleControllerColliderHit: u8,
    pub handleJointBreak: u8,
    pub handleJointBreak2D: u8,
    pub handleOnGUI: u8,
    pub handleFixedUpdate: u8,
    pub handleLateUpdate: u8,
    pub handleApplicationEvents: u8,
    pub handleUiEvents: i32,
    pub handleLegacyNetworking: u8,
    pub handleAnimatorMove: u8,
    pub handleAnimatorIK: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmState {
    pub name: String,
    pub description: String,
    pub position: Rect,
    pub isBreakpoint: u8,
    pub isSequence: u8,
    pub hideUnused: u8,
    pub transitions: Vec<FsmTransition>,
    pub actionData: ActionData,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmEvent {
    pub name: String,
    pub isSystemEvent: u8,
    pub isGlobal: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmTransition {
    pub fsmEvent: FsmEvent,
    pub toState: String,
    pub linkStyle: u8,
    pub linkConstraint: u8,
    pub linkTarget: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmVariables {
    pub floatVariables: Vec<FsmFloat>,
    pub intVariables: Vec<FsmInt>,
    pub boolVariables: Vec<FsmBool>,
    pub stringVariables: Vec<FsmString>,
    pub vector2Variables: Vec<FsmVector2>,
    pub vector3Variables: Vec<FsmVector3>,
    pub rectVariables: Vec<FsmRect>,
    pub quaternionVariables: Vec<FsmQuaternion>,
    pub gameObjectVariables: Vec<FsmGameObject>,
    pub objectVariables: Vec<FsmObject>,
    pub materialVariables: Vec<FsmMaterial>,
    pub textureVariables: Vec<FsmTexture>,
    pub arrayVariables: Vec<FsmArray>,
    pub enumVariables: Vec<FsmEnum>,
    pub categories: Vec<String>,
    pub variableCategoryIDs: Vec<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct ActionData {
    pub actionNames: Vec<String>,
    pub customNames: Vec<String>,
    pub actionEnabled: Vec<u8>,
    pub actionIsOpen: Vec<u8>,
    pub actionStartIndex: Vec<i32>,
    pub actionHashCodes: Vec<i32>,
    pub unityObjectParams: Vec<PPtr /* Object */>,
    pub fsmGameObjectParams: Vec<FsmGameObject>,
    pub fsmOwnerDefaultParams: Vec<FsmOwnerDefault>,
    pub animationCurveParams: Vec<FsmAnimationCurve>,
    pub functionCallParams: Vec<FunctionCall>,
    pub fsmTemplateControlParams: Vec<FsmTemplateControl>,
    pub fsmEventTargetParams: Vec<FsmEventTarget>,
    pub fsmPropertyParams: Vec<FsmProperty>,
    pub layoutOptionParams: Vec<LayoutOption>,
    pub fsmStringParams: Vec<FsmString>,
    pub fsmObjectParams: Vec<FsmObject>,
    pub fsmVarParams: Vec<FsmVar>,
    pub fsmArrayParams: Vec<FsmArray>,
    pub fsmEnumParams: Vec<FsmEnum>,
    pub fsmFloatParams: Vec<FsmFloat>,
    pub fsmIntParams: Vec<FsmInt>,
    pub fsmBoolParams: Vec<FsmBool>,
    pub fsmVector2Params: Vec<FsmVector2>,
    pub fsmVector3Params: Vec<FsmVector3>,
    pub fsmRectParams: Vec<FsmRect>,
    pub fsmQuaternionParams: Vec<FsmQuaternion>,
    pub stringParams: Vec<String>,
    pub byteData: Vec<u8>,
    pub arrayParamSizes: Vec<i32>,
    pub arrayParamTypes: Vec<String>,
    pub customTypeSizes: Vec<i32>,
    pub customTypeNames: Vec<String>,
    pub paramDataType: Vec<i32>,
    pub paramName: Vec<String>,
    pub paramDataPos: Vec<i32>,
    pub paramByteDataSize: Vec<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmFloat {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub value: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmInt {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub value: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmBool {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub value: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmString {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub value: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmVector2 {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub value: Vector2,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmVector3 {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub value: Vector3,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmRect {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub value: Rect,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmQuaternion {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub value: Quaternion,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmGameObject {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub value: PPtr, /* GameObject */
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmObject {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub typeName: String,
    pub value: PPtr, /* Object */
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmMaterial {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub typeName: String,
    pub value: PPtr, /* Object */
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmTexture {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub typeName: String,
    pub value: PPtr, /* Object */
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmArray {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub r#type: i32,
    pub objectTypeName: String,
    pub floatValues: Vec<f32>,
    pub intValues: Vec<i32>,
    pub boolValues: Vec<u8>,
    pub stringValues: Vec<String>,
    pub vector4Values: Vec<Vector4>,
    pub objectReferences: Vec<PPtr /* Object */>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmEnum {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
    pub enumName: String,
    pub intValue: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmOwnerDefault {
    pub ownerOption: i32,
    pub gameObject: FsmGameObject,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmAnimationCurve {
    pub curve: AnimationCurve,
}

#[derive(Debug, serde::Deserialize)]
pub struct FunctionCall {
    pub FunctionName: String,
    pub parameterType: String,
    pub BoolParameter: FsmBool,
    pub FloatParameter: FsmFloat,
    pub IntParameter: FsmInt,
    pub GameObjectParameter: FsmGameObject,
    pub ObjectParameter: FsmObject,
    pub StringParameter: FsmString,
    pub Vector2Parameter: FsmVector2,
    pub Vector3Parameter: FsmVector3,
    pub RectParamater: FsmRect,
    pub QuaternionParameter: FsmQuaternion,
    pub MaterialParameter: FsmMaterial,
    pub TextureParameter: FsmTexture,
    pub EnumParameter: FsmEnum,
    pub ArrayParameter: FsmArray,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmTemplateControl {
    pub targetType: i32,
    pub target: PPtr, /* Object */
    pub inputVariables: Vec<FsmVarOverride>,
    pub outputVariables: Vec<FsmVarOverride>,
    pub outputEvents: Vec<FsmEventMapping>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmEventTarget {
    pub target: i32,
    pub excludeSelf: FsmBool,
    pub gameObject: FsmOwnerDefault,
    pub fsmName: FsmString,
    pub sendToChildren: FsmBool,
    pub fsmComponent: PPtr, /* PlayMakerFSM */
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmProperty {
    pub TargetObject: FsmObject,
    pub TargetTypeName: String,
    pub PropertyName: String,
    pub BoolParameter: FsmBool,
    pub FloatParameter: FsmFloat,
    pub IntParameter: FsmInt,
    pub GameObjectParameter: FsmGameObject,
    pub StringParameter: FsmString,
    pub Vector2Parameter: FsmVector2,
    pub Vector3Parameter: FsmVector3,
    pub RectParamater: FsmRect,
    pub QuaternionParameter: FsmQuaternion,
    pub ObjectParameter: FsmObject,
    pub MaterialParameter: FsmMaterial,
    pub TextureParameter: FsmTexture,
    pub EnumParameter: FsmEnum,
    pub ArrayParameter: FsmArray,
    pub setProperty: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct LayoutOption {
    pub option: i32,
    pub floatParam: FsmFloat,
    pub boolParam: FsmBool,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmVar {
    pub variableName: String,
    pub objectType: String,
    pub useVariable: u8,
    pub r#type: i32,
    pub floatValue: f32,
    pub intValue: i32,
    pub boolValue: u8,
    pub stringValue: String,
    pub vector4Value: Vector4,
    pub objectReference: PPtr, /* Object */
    pub arrayValue: FsmArray,
}

#[derive(Debug, serde::Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct AnimationCurve {
    pub m_Curve: Vec<Keyframe>,
    pub m_PreInfinity: i32,
    pub m_PostInfinity: i32,
    pub m_RotationOrder: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmVarOverride {
    pub variable: NamedVariable,
    pub fsmVar: FsmVar,
    pub isEdited: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct FsmEventMapping {
    pub fromEvent: FsmEvent,
    pub toEvent: FsmEvent,
}

#[derive(Debug, serde::Deserialize)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    pub inSlope: f32,
    pub outSlope: f32,
    pub weightedMode: i32,
    pub inWeight: f32,
    pub outWeight: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct NamedVariable {
    pub useVariable: u8,
    pub name: String,
    pub tooltip: String,
    pub showInInspector: u8,
    pub networkSync: u8,
}

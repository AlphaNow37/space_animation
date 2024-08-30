
// Base bindings

@group(0) @binding(0)
var<uniform> time: f32;

@group(0) @binding(1)
var<uniform> camera: mat4x4<f32>;

@group(0) @binding(2)
var<uniform> camera_transform: mat4x4<f32>;

/// Store bindings

@group(1) @binding(0)
var<storage> matrices: array<mat4x4<f32>>;

@group(1) @binding(1)
var<storage> f32s: array<f32>;

@group(1) @binding(2)
var<storage> vecs2: array<vec2<f32>>;

@group(1) @binding(3)
var<storage> vecs3: array<vec3<f32>>;

@group(1) @binding(4)
var<storage> vecs4: array<vec4<f32>>;

@group(1) @binding(5)
var<storage> colors: array<vec3<f32>>;

@group(1) @binding(6)
var<storage> colors2: array<vec3<f32>>;

@group(1) @binding(7)
var<storage> polys4x4: array<array<vec3<f32>, 16>>;

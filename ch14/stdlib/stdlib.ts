export function diameter(radius: f32): f32 {
    let diam = <f32>(2.0 * radius);
    return diam;
}

export function circumference(radius: f32): f32 {
    let circ = <f32>(2.0 * Math.PI * radius);
    return circ;
}

export function area(radius: f32): f32 {
    let area = <f32>(Math.PI * Math.pow(radius, 2));
    return area;
}


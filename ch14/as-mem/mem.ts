memory.grow(1);

store<u8>(0, 100);

export function whereToStore(): i32 {
    let basePtr = load<u8>(0);
    return basePtr;
}

export function readFromLocation(loc: i32): i32 {
    let value = load<u8>(loc);
    return value;
}

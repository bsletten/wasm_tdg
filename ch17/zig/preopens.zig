const std = @import("std");
const PreopenList = std.fs.wasi.PreopenList;

pub fn main() !void {
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const gpa = general_purpose_allocator.allocator();

    var preopens = PreopenList.init(gpa);
    defer preopens.deinit();

    try preopens.populate();

    for (preopens.asSlice()) |preopen, i| {
        std.debug.print("{}: {}\n", .{ i, preopen });
    }
}

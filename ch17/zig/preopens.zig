// Zig's standard library used to ship a `std.fs.wasi.PreopenList` helper,
// which this example was originally written against. That type was removed
// from the standard library, so we now walk the preopened file descriptors
// ourselves using the raw `wasi_snapshot_preview1` calls in `std.os.wasi`.
const std = @import("std");
const wasi = std.os.wasi;

pub fn main() !void {
    var name_buf: [std.fs.max_path_bytes]u8 = undefined;

    // File descriptors 0, 1 and 2 are stdin/stdout/stderr, so the directories
    // the host granted us ("preopens") start at 3 and run until the runtime
    // tells us there are no more.
    var fd: wasi.fd_t = 3;

    while (true) {
        var prestat: wasi.prestat_t = undefined;
        switch (wasi.fd_prestat_get(fd, &prestat)) {
            .SUCCESS => {},
            .BADF => break, // no more preopens
            else => |err| return std.posix.unexpectedErrno(err),
        }

        const name_len = prestat.u.dir.pr_name_len;
        if (name_len > name_buf.len) return error.NameTooLong;

        switch (wasi.fd_prestat_dir_name(fd, &name_buf, name_len)) {
            .SUCCESS => {},
            else => |err| return std.posix.unexpectedErrno(err),
        }

        std.debug.print("{d}: Preopen{{ .fd = {d}, .type = {s}, .name = '{s}' }}\n", .{
            fd - 3,
            fd,
            @tagName(prestat.pr_type),
            name_buf[0..name_len],
        });

        fd += 1;
    }
}

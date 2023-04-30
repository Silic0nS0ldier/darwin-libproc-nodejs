# @silicon-soldier/darwin-libproc

High-level interface over darwin's `libproc` library.

## Usage

### `pidPath`

```js
import libproc from "@silicon-soldier/darwin-libproc";

// Current process (immune to `process.execPath` manipulation)
console.info(libproc.pidPath(process.pid));
// /opt/homebrew/Cellar/node@16/16.19.1/bin/node

// Parent process
console.info(libproc.pidPath(process.ppid));
// /bin/zsh

// PID 1 (launchd)
console.info(libproc.pidPath(1));
// /sbin/launchd
```

## Status

### C-to-Rust Binding

The crate `darwin-libproc-sys@0.2.0` houses the C-to-Rust bindings.

| `libproc.h` |  |
| - | - |
| `proc_listpidspath` | ✓ |
| `proc_listpids` | ✓ |
| `proc_listallpids` | ✗ |
| `proc_listpgrppids` | ✗ |
| `proc_listchildpids` | ✗ |
| `proc_pidinfo` | ✓ |
| `proc_pidfdinfo` | ✓ |
| `proc_pidfileportinfo` | ✗ |
| `proc_name` | ✓ |
| `proc_regionfilename` | ✓ |
| `proc_kmsgbuf` | ✓ |
| `proc_pidpath` | ✓ |
| `proc_libversion` | ✓ |
| `proc_setpcontrol` | ✗ |
| `proc_track_dirty` | ✗ |
| `proc_set_dirty` | ✗ |
| `proc_get_dirty` | ✗ |
| `proc_terminate` | ✗ |

### NodeJS Binding

The crate `darwin-libproc@0.2.0` acts as a safe and idiomatic wrapper over `darwin-libproc-sys@0.2.0`. This package exposes this API to NodeJS.

| `darwin-libproc@0.2.0` |  |
| - | - |
| `all_pids` | ✗ |
| `name` | ✗ |
| `pgrp_only_pids` | ✗ |
| `pid_cwd` | ✗ |
| `pid_path` | `pidPath` |
| `pid_rusage` | ✗ |
| `ppid_only_pids` | ✗ |
| `ruid_only_pids` | ✗ |
| `task_all_info` | ✗ |
| `task_info` | ✗ |
| `tty_only_pids` | ✗ |
| `uid_only_pids` | ✗ |
| `version` | ✗ |
| `vnode_path_info` | ✗ |

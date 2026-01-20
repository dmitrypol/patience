use lazy_static::lazy_static;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::thread::sleep;
use std::time::Duration;
use valkey_module::alloc::ValkeyAlloc;
use valkey_module::configuration::ConfigurationFlags;
use valkey_module::{
    CommandFilterCtx, RedisModuleCommandFilterCtx, VALKEYMODULE_CMDFILTER_NOSELF, valkey_module,
};

lazy_static! {
    static ref ENABLED: AtomicBool = AtomicBool::default();
    static ref SLEEP_IN_MILLIS: AtomicI64 = AtomicI64::new(1);
    static ref CMD_TO_FILTER: Mutex<String> = Mutex::new("".into());
}

fn sleep_filter_fn(ctx: *mut RedisModuleCommandFilterCtx) {
    if !ENABLED.load(Ordering::Relaxed) {
        return;
    }
    // check if command to filter matches
    let cf_ctx = CommandFilterCtx::new(ctx);
    let _cmd_arg = cf_ctx.arg_get_try_as_str(0).unwrap();
    let sleep_time = SLEEP_IN_MILLIS.load(Ordering::Relaxed) as u64;
    sleep(Duration::from_millis(sleep_time));
}

valkey_module! {
    name: "patience",
    version: 1,
    allocator: (ValkeyAlloc, ValkeyAlloc),
    data_types: [],
    commands: [
    ],
    filters: [
        [sleep_filter_fn, VALKEYMODULE_CMDFILTER_NOSELF],
    ],
    configurations: [
        i64: [
            // default, min, max
            ["sleep", &*SLEEP_IN_MILLIS, 1, 0, 1000, ConfigurationFlags::DEFAULT, None],
        ],
        string: [
            ["cmd", &*CMD_TO_FILTER, "", ConfigurationFlags::DEFAULT, None],
        ],
        bool: [
            ["enabled", &*ENABLED, false, ConfigurationFlags::DEFAULT, None],
        ],
        enum: [],
        module_args_as_configuration: true,
    ]
}

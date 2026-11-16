pub struct TypeOverride<'a> {
    pub name: &'a str,
    /// Generated methods to omit. For records this also covers constructors and functions.
    pub methods: &'a [&'a str],
    /// Members declared in .d.ts files. Subtypes treat these as conflicting methods.
    pub declares: &'a [&'a str],
}

pub struct Override<'a> {
    pub namespace: &'a str,
    pub version: &'a str,
    pub content: Option<&'a str>,
    pub toplevel: &'a [&'a str],
    pub types: &'a [TypeOverride<'a>], // class, interface and record
}

pub const OVERRIDES: &[Override] = &[
    Override {
        namespace: "GObject",
        version: "2.0",
        content: Some(include_str!("overrides/GObject-2.0.d.ts")),
        toplevel: &[
            "signal_handler_find",
            "signal_handlers_block_matched",
            "signal_handlers_unblock_matched",
            "signal_handlers_disconnect_matched",
            "ParamSpec",
            "Value",
            "BindingTransformFunc",
        ],
        types: &[
            TypeOverride {
                name: "BindingGroup",
                methods: &["bind_full"],
                declares: &["bind_full"],
            },
            TypeOverride {
                name: "Object",
                methods: &[
                    "bind_property_full",
                    "get_data",
                    "get_qdata",
                    "set_data",
                    "set_qdata",
                    "steal_data",
                    "steal_qdata",
                    "force_floating",
                    "notify",
                    "ref",
                    "ref_sink",
                    "unref",
                ],
                declares: &[
                    "bind_property_full",
                    "block_signal_handler",
                    "connect",
                    "connect_after",
                    "connect_object",
                    "disconnect",
                    "emit",
                    "notify",
                    "set",
                    "stop_emission_by_name",
                    "unblock_signal_handler",
                ],
            },
        ],
    },
    Override {
        namespace: "Gio",
        version: "2.0",
        content: Some(include_str!("overrides/Gio-2.0.d.ts")),
        toplevel: &[],
        types: &[
            TypeOverride {
                name: "ActionMap",
                methods: &["add_action_entries"],
                declares: &["add_action_entries"],
            },
            TypeOverride {
                name: "DBusProxy",
                methods: &[],
                declares: &["connectSignal", "disconnectSignal"],
            },
            TypeOverride {
                name: "File",
                methods: &["replace_contents_async", "set_attribute"],
                declares: &["replace_contents_async", "set_attribute"],
            },
            TypeOverride {
                name: "FileInfo",
                methods: &["set_attribute"],
                declares: &["set_attribute"],
            },
            TypeOverride {
                name: "InputStream",
                methods: &[],
                declares: &["createSyncIterator", "createAsyncIterator"],
            },
            TypeOverride {
                name: "Settings",
                methods: &["bind_with_mapping"],
                declares: &["bind_with_mapping"],
            },
        ],
    },
    Override {
        namespace: "GLib",
        version: "2.0",
        content: Some(include_str!("overrides/GLib-2.0.d.ts")),
        types: &[TypeOverride {
            name: "Thread",
            methods: &["new", "try_new", "exit", "ref", "unref"],
            declares: &[],
        }],
        toplevel: &[
            "Variant",
            "VariantType",
            "VariantBuilder",
            "log_set_writer_func",
            "LogWriterFunc",
            "stpcpy",
            "ascii_formatd",
        ],
    },
    Override {
        namespace: "Gtk",
        version: "3.0",
        content: Some(include_str!("overrides/Gtk-3.0.d.ts")),
        types: &[],
        toplevel: &[],
    },
    Override {
        namespace: "Gtk",
        version: "4.0",
        content: Some(include_str!("overrides/Gtk-4.0.d.ts")),
        types: &[TypeOverride {
            name: "Builder",
            methods: &["get_objects"],
            declares: &["exposeObjects", "get_objects"],
        }],
        toplevel: &[],
    },
];

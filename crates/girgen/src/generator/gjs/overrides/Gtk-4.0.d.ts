type BuilderJSProps = {
    /** XML interface description, passed to {@link Gtk.Builder.add_from_string} */
    data?: string | Uint8Array
    /** Local filename of an XML interface description, passed to {@link Gtk.Builder.add_from_file} */
    filename?: string
    /** Resource path of an XML interface description, passed to {@link Gtk.Builder.add_from_resource} */
    resource?: string
    /** Object whose properties are the callbacks referenced in the XML */
    callbacks?: object
    /** Objects to expose to the XML, see {@link Gtk.Builder.exposeObjects} */
    objects?: Record<string, GObject.Object>
}

namespace Gtk {
    interface $Exports {
        children: symbol
        cssName: symbol
        internalChildren: symbol
        template: symbol
    }

    interface Widget {
        /** Iterates over the children of the widget. */
        [Symbol.iterator](): IterableIterator<Widget>
    }

    interface WidgetClass {
        /**
         * Adds a shortcut for `keyval` and `modifiers` that calls `callback`.
         * Arguments are packed into a {@link GLib.Variant} of type `format`.
         * @since GJS 1.89.1
         */
        add_binding(
            keyval: number,
            modifiers: Gdk.ModifierType,
            callback: ShortcutFunc,
            format?: string | null,
            ...args: unknown[]
        ): void

        /**
         * Adds a shortcut for `keyval` and `modifiers` that activates the action named `action`.
         * Arguments are packed into a {@link GLib.Variant} of type `format`.
         * @since GJS 1.89.1
         */
        add_binding_action(
            keyval: number,
            modifiers: Gdk.ModifierType,
            action: string,
            format?: string | null,
            ...args: unknown[]
        ): void

        /**
         * Adds a shortcut for `keyval` and `modifiers` that emits `signal`.
         * Arguments are packed into a {@link GLib.Variant} of type `format`.
         * @since GJS 1.89.1
         */
        add_binding_signal(
            keyval: number,
            modifiers: Gdk.ModifierType,
            signal: string,
            format?: string | null,
            ...args: unknown[]
        ): void
    }

    interface BuilderClass {
        /**
         * At most one of `data`, `filename` and `resource` can be passed.
         * `scope` can't be combined with `callbacks` or `objects`.
         * @since GJS 1.87.1
         */
        new (
            props?: Partial<GObject.ConstructorProps<Builder>> & BuilderJSProps,
        ): Builder
    }

    interface Builder {
        /**
         * Calls {@link Gtk.Builder.expose_object} for each entry of `objects`.
         * @since GJS 1.87.1
         */
        exposeObjects(objects: Record<string, GObject.Object>): void

        /**
         * Gets all objects that have been constructed by the builder.
         * Objects are also accessible by their ID as named properties.
         *
         * ```ts
         * const { button } = builder.get_objects<{ button: Gtk.Button }>()
         * ```
         */
        get_objects<
            T extends Record<string, GObject.Object> = Record<
                string,
                GObject.Object
            >,
        >(): GObject.Object[] & T
    }
}

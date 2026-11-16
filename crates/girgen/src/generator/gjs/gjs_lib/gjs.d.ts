/// <reference lib="esnext" />

import type GObject from "gi://GObject?version=2.0"
import type GLib from "gi://GLib?version=2.0"
import type System from "system"
import type Gettext from "gettext"
import type Console from "console"
import type format from "./format"
import type cairo from "./cairo"
import type Legacy from "./legacy"

declare global {
    var console: Console.Console

    /** The global object, for compatibility with browser code. */
    var window: typeof globalThis

    interface LegacyGiImports {}

    interface LegacyImports {
        gi: LegacyGiImports
        system: typeof System
        gettext: typeof Gettext
        cairo: typeof cairo
        format: typeof format
        searchPath: string[]
        package: typeof Legacy.Package
        signals: typeof Legacy.Signals
        /** @deprecated use {@link GLib.MainLoop} */
        mainloop: typeof Legacy.Mainloop
        /** @deprecated */
        lang: typeof Legacy.Lang
        /** @deprecated */
        byteArray: typeof Legacy.ByteArray
    }

    /** @deprecated use ESModules */
    var imports: LegacyImports

    interface ImportMeta {
        /**
         * The absolute file: or resource: URL of the module.
         * @see https://gitlab.gnome.org/GNOME/gjs/-/blob/master/doc/ESModules.md#importmetaurl
         */
        url: string
    }

    /**
     * The command line arguments passed to the program, excluding the program name.
     * Same as {@link System.programArgs}.
     */
    var ARGV: string[]

    interface ErrorConstructor {
        /** @since 1.85.2 */
        isError(instance: unknown): instance is Error | GLib.Error

        /**
         * Creates a `stack` property on `targetObject`. Frames above and including
         * `constructorOpt` are omitted.
         * @since 1.85.2
         */
        captureStackTrace(targetObject: object, constructorOpt?: Function): void
    }

    interface Error {
        matches(
            domain: GLib.Quark | { $gtype: GObject.GType<GLib.Error> },
            code: number,
        ): boolean
    }

    interface BooleanConstructor {
        $gtype: GObject.GType<boolean>
    }

    interface NumberConstructor {
        $gtype: GObject.GType<number>
    }

    interface StringConstructor {
        $gtype: GObject.GType<string>
    }

    interface ObjectConstructor {
        $gtype: GObject.GType<object>
    }
}

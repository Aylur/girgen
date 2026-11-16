/// <reference path="./GLib-2.0.d.ts" />

import type GLib from "gi://GLib?version=2.0"

declare namespace Legacy {
    namespace Package {
        type InitParameters = {
            name: string
            prefix: string
            version: string
            libdir?: string
        }

        const datadir: string
        const libdir: string
        const localedir: string
        const moduledir: string
        const name: string
        const pkgdatadir: string
        const pkglibdir: string
        const prefix: string
        const version: string

        function checkSymbol(
            lib: string,
            ver?: string,
            symbol?: string,
        ): boolean
        /**
         * Initializes the package and sets the global `pkg` to this module.
         * {@link start} calls this too.
         *
         * The global is not declared by default.
         * To use it, declare it in a `.d.ts` file of your project:
         *
         * ```ts
         * // env.d.ts
         * var pkg: LegacyImports["package"]
         * ```
         */
        function init(params: InitParameters): void
        /**
         * Sets `String.prototype.format = imports.format.format`.
         *
         * The method is not declared by default.
         * To use it, declare it in a `.d.ts` file of your project:
         *
         * ```ts
         * // env.d.ts
         * interface String {
         *     format(...args: (string | number | boolean | null | undefined)[]): string
         * }
         * ```
         */
        function initFormat(): void
        /**
         * Binds the text domain of the package and sets the global `_`, `C_` and `N_` functions.
         */
        function initGettext(): void
        function initSubmodule(module: string): void
        function require(libs: { [lib: string]: string }): void
        function requireSymbol(lib: string, ver?: string, symbol?: string): void
        function run<T>(module: { main: (args: string[]) => T }): T
        function start(params: InitParameters): number | undefined
    }

    namespace Signals {
        interface SignalMethods {
            connect(
                name: string,
                handler: (self: object, ...args: any[]) => any,
            ): number
            connectAfter(
                name: string,
                handler: (self: object, ...args: any[]) => any,
            ): number
            disconnect(id: number): void
            disconnectAll(): void
            emit(name: string, ...args: any[]): void
            signalHandlerIsConnected(id: number): boolean
        }

        function addSignalMethods<T>(
            proto: T,
        ): asserts proto is T & SignalMethods
    }

    /** @deprecated */
    namespace Mainloop {
        /** @deprecated Use GLib.idle_add instead. */
        function idle_add(handler: Function, priority?: number): number
        /** @deprecated Use GLib.MainLoop.quit instead. */
        function quit(id: string): void
        /** @deprecated Use GLib.MainLoop.run instead. */
        function run(id: string): void
        /** @deprecated Use GLib.Source.remove instead. */
        function source_remove(id: number): void
        /** @deprecated Use GLib.timeout_add instead. */
        function timeout_add(
            timeout_ms: number,
            handler: Function,
            priority?: number,
        ): number
        /** @deprecated Use GLib.timeout_add_seconds instead. */
        function timeout_add_seconds(
            timeout_s: number,
            handler: Function,
            priority?: number,
        ): number
    }

    /** @deprecated */
    namespace Lang {
        /** @deprecated Use Function.bind instead. */
        function bind<
            T extends object,
            A extends any[],
            B extends any[],
            F extends (...args: [...B, ...A]) => any,
        >(obj: T, func: F, ...bindArguments: B): (...args: A) => ReturnType<F>
        /** @deprecated Use Object.assign instead. */
        function copyProperties(source: object, dest: object): void
        /** @deprecated Use Object.fromEntries instead. */
        function copyPublicProperties(source: object, dest: object): void
        /** @deprecated Use Object.keys().length instead. */
        function countProperties(obj: object): number
    }

    /** @deprecated Use `Uint8Array`, `TextEncoder` and `TextDecoder` instead. */
    namespace ByteArray {
        /** @deprecated Use `new TextEncoder().encode()` instead. */
        function fromString(string: string, encoding?: string): Uint8Array
        /** @deprecated Use {@link GLib.Bytes.toArray} instead. */
        function fromGBytes(bytes: GLib.Bytes): Uint8Array
        /** @deprecated Use `new TextDecoder().decode()` instead. */
        function toString(array: Uint8Array, encoding?: string): string
        /** @deprecated Use `new GLib.Bytes()` instead. */
        function toGBytes(array: Uint8Array): GLib.Bytes
        /** @deprecated Use `Uint8Array.from()` instead. */
        function fromArray(array: Iterable<number>): ByteArray

        /** @deprecated Use `Uint8Array` instead. */
        class ByteArray {
            constructor(arg?: number | Uint8Array)
            [index: number]: number
            length: number
            toString(encoding?: string): string
            toGBytes(): GLib.Bytes
        }
    }
}

export default Legacy

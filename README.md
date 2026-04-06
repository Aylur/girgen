# GirGen

GIR Parser and type generator.

## Install

There are pre built binaries for `linux-x64` and `linux-arm64` published on NPM.

```sh
npm install girgen -D
./node_modules/.bin/girgen --help
```

Otherwise you can install from crates.io using `cargo`.

```sh
cargo install girgen --root .
./bin/girgen --help
```

## TypeScript

Generate a standalone package that contains every namespace found in the given
directories.

```sh
girgen typescript --help
```

> [!TIP]
>
> You can use the Gnome flatpak SDK to acquire GIR files on systems that don't
> have them in one place, e.g NixOS or when you are targeting Flatpak.
>
> ```sh
> flatpak run --command=cp --filesystem=home org.gnome.Sdk -r /usr/share/gir-1.0 gir-1.0
> girgen -d gir-1.0 typescript
> ```

By default it will generate the package to `.types/gi` which you can then source
in `tsconfig.json`.

```json
{
    "compilerOptions": {
        "lib": ["es2024"], // don't forget to specify a `lib` to avoid sourcing TypeScript's `dom` lib
        "skipLibCheck": true, // it's recommended to turn this on
        "typeRoots": [".types"]
    }
}
```

> [!TIP]
>
> Don't forget to gitignore generated files.
>
> ```sh
> echo ".types/gi/" > .gitignore
> ```

Note that when using `--alias` flag to generate non version imports such as
`gi://Gtk` make sure to ignore the version you don't need so that it does not
end up as a union of the two versions.

```sh
girgen typescript -i Gtk-3.0 --alias
```

### TypeScript Annotations

GObject has a few additional concepts about class methods and properties that
cannot be expressed with TypeScript alone. For these girgen generates type only
fields on classes and interfaces.

We have annotations for:

- signals
- readable properties
- writable properties
- construct-only properties

When implementing a GObject subclass you might want to annotate a subset of
these or all of these depending on usecase.

Every class that inherits from GObject is going to include a namespace
containing type declarations where each member is written in `kebab-case`:

```ts
namespace MyClass {
    export interface SignalSignatures extends GObject.Object.SignalSignatures {
        // simple signal
        "my-signal"(arg: number): void
        // detailed signals are annotated with the `::{}` suffix
        "my-detailed-signal::{}"(arg: number): void
    }

    // ReadableProperties is also used for notify signal annotations
    export interface ReadableProperties
        extends GObject.Object.ReadableProperties {
        // property which has a public getter
        "my-prop": number
    }

    export interface WritableProperties
        extends GObject.Object.WritableProperties {
        // property which has a public setter
        "my-prop": number
    }

    export interface ConstructOnlyProperties
        extends GObject.Object.ConstructOnlyProperties {
        // property which can only be set at construction
        "my-ctor-prop": number
    }
}
```

And the Class will refer to these using special `$` prefixed fields:

> [!IMPORTANT]
>
> These fields don't exist at runtime, they are used by other APIs to introspect
> GObjects.

```ts
class MyClass extends GObject.Object {
    declare readonly $signals: MyClass.SignalSignatures
    declare readonly $readableProperties: MyClass.ReadableProperties
    declare readonly $writableProperties: MyClass.WritableProperties
    declare readonly $constructOnlyProperties: MyClass.ConstructOnlyProperties

    static {
        GObject.registerClass(
            {
                Signals: {
                    "my-signal": {
                        param_types: [GObject.TYPE_DOUBLE],
                    },
                    "my-detailed-signal": {
                        param_types: [GObject.TYPE_DOUBLE],
                        flags: GObject.SignalFlags.DETAILED,
                    },
                },
                Properties: {
                    "my-prop": GObject.ParamSpec.double(
                        "my-prop",
                        null,
                        null,
                        GObject.ParamFlags.READWRITE,
                        -GObject.Double.MAX_VALUE,
                        GObject.Double.MAX_VALUE,
                    ),
                    "my-ctor-prop": GObject.ParamSpec.double(
                        "my-ctor-prop",
                        null,
                        null,
                        GObject.ParamFlags.CONSTRUCT_ONLY,
                        -GObject.Double.MAX_VALUE,
                        GObject.Double.MAX_VALUE,
                    ),
                },
            },
            MyClass,
        )
    }

    // GObject.ConstructorProps can be used to infer props from the annotations
    constructor(props: Partial<GObject.ConstructorProps<MyClass>>) {
        super(props)

        // note that properties will be annotated as camelCase
        console.log(props.myProp, props.myCtorProp)
    }
}
```

Methods such as `connect()`, `emit()`, `notify()` will infer from these
annotations.

```ts
const instance = new MyClass()

instance.connect("my-signal", (source, arg) => {
    console.log(arg)
})

instance.connect("my-detailed-signal::detail", (source, arg) => {
    console.log(arg)
})

instance.connect("notify::my-prop", (_, pspec) => {
    console.log(pspec.name)
})
```

Due to how TypeScript `this` type works, you need to annotate `this` or use a
typecast to correctly infer types within the class.

```ts
class MyClass {
    myFn(this: MyClass) {
        this.emit("my-signal", 0)
    }

    myFn() {
        const self = this as MyClass
        self.emit("my-signal", 0)
    }
}
```

### Module Augmentation

If you are using
[`Gio._promisify`](https://gjs.guide/guides/gjs/asynchronous-programming.html#promisify-helper)
you can augment namespaces.

```ts
import Gio from "gi://Gio?version=2.0"
import GLib from "gi://GLib?version=2.0"

Gio._promisify(
    Gio.InputStream.prototype,
    "read_bytes_async",
    "read_bytes_finish",
)

declare module "gi://Gio?version=2.0" {
    namespace GI {
        namespace Gio {
            interface InputStream {
                read_bytes_async(
                    count: number,
                    io_priority: number,
                    cancellable: Gio.Cancellable | null,
                ): GLib.Bytes
            }
        }
    }
}

declare const stream: Gio.InputStream
const bytes = await stream.read_bytes_async(4096, GLib.PRIORITY_DEFAULT, null)
```

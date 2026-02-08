#!/usr/bin/env nu

def "main build" [--os: string, --cpu: string, --target: string] {
    cargo build --release --target $target

    let name = $"($os)-($cpu)"
    let version = open Cargo.toml | get package | get version

    let package = {
        name: $"@girgen/($name)",
        version: $version,
        os: [$os]
        cpu: [$cpu]
        exports: "./girgen"
    }

    let dist = $"dist/($name)"
    mkdir $dist
    mv $"target/($target)/release/girgen" $dist
    $package | save -f $"($dist)/package.json"
}

def main [] {
    nu $env.CURRENT_FILE --help
}


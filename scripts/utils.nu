export def get_allbin [] {
    cargo metadata --format-version 1 --no-deps
        | from json
        | get packages
        | each {|pkg|
            $pkg.targets
            | where {|target| "bin" in $target.kind}
            | get name
        }
        | flatten
}

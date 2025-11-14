export def get_all_bins [] {
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

export def validate_bin [bin: string] {
    if ($bin not-in (get_all_bins)) {
        error make {
            msg: $"Unsupported Binary: ($bin). Available binaries: (get_all_bins)"
        }
    }
}

export def get_bin_matadata [bin: string] {
    # Get the package metadata for the binary
    let metadata = (cargo metadata --format-version 1 --no-deps | from json)
    let packages = ($metadata.packages | where {|pkg| 
        ($pkg.targets | any {|target| 
            $target.name == $bin and ("bin" in $target.kind)
        })
    })
    
    if ($packages | length) != 1 {
        return null
    }
    
    return ($packages | get 0).metadata
}

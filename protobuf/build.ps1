Function Invoke-RsProtoc {
    param (
        [string]$filepath
    )
    $dir = Split-Path $filepath -Parent
    $reldir = $dir.TrimStart(".\")
    $targetDir = "../src/$reldir"
    if (-not (Test-Path $targetDir)) {
        New-Item -ItemType Directory -Force -Path $targetDir | Out-Null
    }
    protoc $filepath --rust_out="../src/$reldir" --proto_path=./ --proto_path=$dir --proto_path=/usr/include --proto_path=/usr/local/include
}

Set-Location protos

# Clear the content of lib.rs
"" | Set-Content ../src/lib.rs

Get-ChildItem -Recurse -Filter *.proto | ForEach-Object {
    Invoke-RsProtoc -filepath $_.FullName

    $mod = [IO.Path]::GetFileNameWithoutExtension($_.Name).Replace(".", "_")
    $pathWithoutExt = $_.FullName.Substring(0, $_.FullName.LastIndexOf('.'))
    $modPath = $pathWithoutExt.TrimStart(".\").Replace("\", "::").Replace(".", "_")
    "pub mod $modPath;" | Add-Content ../src/lib.rs
}

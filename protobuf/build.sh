#!/usr/bin/env bash

mkdir -p ../src/

rsprotoc() {
    local filepath="$1"
    local dir=$(dirname "$filepath")
    local reldir="${dir#./}"
    mkdir -p "../src/$reldir"
    protoc "$filepath" --rust_out=../src/$reldir --proto_path=./ --proto_path=$dir --proto_path=/usr/include --proto_path=/usr/local/include
}

cd protos

echo -n "" > ../src/lib.rs

find . -name '*.proto' | while IFS= read -r filename; do
    rsprotoc "$filename"

    mod=$(basename "$filename" .proto)
    mod="${mod//./_}"
    path_without_ext="${filename%.*}"
    mod_path="${path_without_ext#./}"
    mod_path="${mod_path//\//::}"
    mod_path="${mod_path//./_}"
    echo "pub mod ${mod_path};" >> ../src/lib.rs
done
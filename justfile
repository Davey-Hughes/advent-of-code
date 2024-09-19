set unstable := true

build *FLAGS:
    #!/bin/bash

    sols=$(find . -type d -iregex "./20[0-9]\{2\}/[0-9]\{2\}" -exec realpath {} \; | sort -u)

    for d in $sols; do
      pushd $d > /dev/null;

      if [ -f "Cargo.toml" ]; then
        cargo build {{FLAGS}};
      fi

      popd $d > /dev/null;
    done

clean:
    #!/bin/bash

    sols=$(find . -type d -iregex "./20[0-9]\{2\}/[0-9]\{2\}" -exec realpath {} \; | sort -u)

    for d in $sols; do
      pushd $d > /dev/null;

      if [ -f "Cargo.toml" ]; then
        cargo clean;
      fi

      popd $d > /dev/null;
    done

run INPUT-DIR *FLAGS:
    #!/bin/bash

    sols=$(find . -type d -iregex "./20[0-9]\{2\}/[0-9]\{2\}" -exec realpath {} \; | sort -u)

    for d in $sols; do
      year=$(basename $(dirname $d));
      day=$(basename $d);
      input_file="{{INPUT-DIR}}$year/$day/input.txt";
      output_file="{{INPUT-DIR}}$year/$day/output.txt";

      pushd $d> /dev/null;

      output=""
      lines=$(cargo run --quiet {{FLAGS}} $input_file)
      if [ -f "Cargo.toml" ]; then
        while read line; do
          output+="$(echo $line | cut -d':' -f2 | xargs)\n";
        done <<< "$lines"

      fi

      # echo $d;
      # echo -e $output;
      # echo "";

      if out_diff=$(echo -e ${output%??} | diff $output_file -); then
        echo $year-$day ✅;
      else
        echo "$year-$day ❌";
        echo "$out_diff";
      fi

      popd $d > /dev/null;
    done

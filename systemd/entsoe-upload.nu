#!/usr/bin/env nu

# Upload ENTSO-E electricity price report to S3
# Edit the constants below for your setup

let home = $env.HOME
let db_path = $"($home)/prices.db"
let binary_path = $"($home)/entsoe/target/release"

const S3_BUCKET = "your-bucket-name"
const PRICE_AREA = "FI"
const TIMEZONE = "Europe/Helsinki"

# Create unique temp directory
let temp_dir = (mktemp -d)

# Generate ASCII report
print "Generating report..."
let output = (
    ^$"($binary_path)/entsoe-ascii" $db_path $PRICE_AREA
        --timezone $TIMEZONE --future
    | complete
)

if $output.exit_code != 0 {
    print $"Error: ($output.stderr)"
    rm -rf $temp_dir
    exit 1
}

$output.stdout | save -f $"($temp_dir)/prices.txt"

# Upload to S3
print "Uploading to S3..."
^aws s3 cp $"($temp_dir)/prices.txt" $"s3://($S3_BUCKET)/prices.txt" --content-type "text/plain; charset=utf-8" --cache-control "max-age=300"

# Clean up
rm -rf $temp_dir

print $"Done at (date now | format date '%Y-%m-%d %H:%M:%S')"

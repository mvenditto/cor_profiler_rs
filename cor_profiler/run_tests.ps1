param (
    [string]$test_name = $null,
    [switch]$run_all_tests = $false
)

$env:RUST_LOG="info"

if($run_all_tests -or -not $test_name) {
    $cmd_args = @('test', '--', '--nocapture', '--test-threads=1')
    & 'cargo' $cmd_args 2> $null
} else {
    $cmd_args = @('test', '--', '--nocapture', $test_name)
    & 'cargo' $cmd_args 2> $null
}
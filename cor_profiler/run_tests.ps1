param (
    [string]$test_name = $null,
    [switch]$run_all_tests = $false
)


if($run_all_tests -or -not $test_name) {
    $cmd_args = @('test', '--', '--nocapture')
    & 'cargo' $cmd_args
} else {
    $cmd_args = @('test', '--', '--nocapture', $test_name)
    & 'cargo' $cmd_args
}
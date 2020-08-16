param (
    [Parameter(Mandatory=$true)][string]$example_root # e.g .\http_client_hooks
)

# EnvironemenrVariables.json should define the following variables
#{  
#   "variables": {
#       "PATH":"C:\\path\\to\\clr\\build\\output\\..\\Windows_NT.x64.Debug",
#       "CORE_LIBRARIES":"C:\\Program Files (x86)\\dotnet\\shared\\Microsoft.NETCore.App\\3.1.4",
#       "CORECLR_PROFILER_PATH":"C:\\path\\to\\this\\repo\\..\\cor_prof\\cor_profiler\\target\\debug\\cor_prof.dll"
#   }
#}

$env_vars = Join-Path -Path $PSScriptRoot -ChildPath "environment_vars.json"
$vars = (Get-Content $env_vars | ConvertFrom-Json).variables
$env:PATH = $vars.PATH
$env:CORE_LIBRARIES = $vars.CORE_LIBRARIES
$env:CORECLR_PROFILER_PATH = $vars.CORECLR_PROFILER_PATH
Write-Host ($vars | Format-List | Out-String)

$env:CORECLR_PROFILER="{cf0d821e-299b-5307-a3d8-b283c03916dd}"
$env:CORECLR_ENABLE_PROFILING=1
$env:COMPlus_LogEnable=1
$env:COMPlus_LogLevel=3
$env:COMPlus_LogToConsole=1
$env:RUST_LOG="debug"

$project_dll = Join-Path -Path $example_root -ChildPath "project\bin\Debug\netcoreapp3.1\win10-x64\publish\project.dll"
corerun.exe $project_dll
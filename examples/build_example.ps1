param (
    [Parameter(Mandatory=$true)][string]$example_root
)
$project_proj = Join-Path -Path $example_root -ChildPath "project\project.csproj"
$helpers_proj = Join-Path -Path $example_root -ChildPath "helpers\helpers.csproj"
dotnet publish $project_proj --framework netcoreapp3.1 --runtime win10-x64
dotnet publish $helpers_proj --framework netcoreapp3.1 --runtime win10-x64
$project_root = Join-Path -Path $example_root -ChildPath "project\bin\Debug\netcoreapp3.1\win10-x64\publish\"
$helpers_dlls = Join-Path -Path $example_root -ChildPath "helpers\bin\Debug\netcoreapp3.1\win10-x64\publish\*"
# just in this debug scenarion copy the helpers.dll into the root of the target project
# in a real world scenario other injection techniques are needed (e.g mscorlib injection, startup hooks)
Copy-Item $helpers_dlls -Destination $project_root
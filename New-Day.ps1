param(
    [int]$Day
)
if (-not $Day) {
    $Day = [datetime]::Today.Day;
}
[string] $DayStr = $day.ToString('00');

if (Test-Path "./src/day$DayStr") {
    Write-Host "Day $DayStr already exists";
} else {
    New-Item -ItemType Directory -Path "./src/day$DayStr" | Out-Null;
    Copy-Item ./src/dayXX/* ./src/day$DayStr -Recurse;

    $new_code = Get-Content ./src/day$DayStr/mod.rs -Raw;
    $new_code = $new_code -replace 'Day XX', "Day $DayStr";
    Set-Content ./src/day$DayStr/mod.rs -Value $new_code -NoNewline;

    $root_code = Get-Content ./src/lib.rs -Raw;
    $root_code = $root_code -replace "//(?=$DayStr => day$DayStr\b)";
    Set-Content ./src/lib.rs -Value $root_code -NoNewline;

    Write-Host "Day $DayStr created";
}
# 1. frontendフォルダへ移動
# $PSScriptRoot はこのスクリプトがある場所（C:\dev\nekotimer\script）を指します
$TargetDir = Join-Path $PSScriptRoot "..\frontend"

if (Test-Path $TargetDir) {
    Push-Location $TargetDir
    Write-Host "Moved to: $TargetDir" -ForegroundColor Magenta
} else {
    Write-Error "Error: frontend フォルダが見つかりませんまる！ ($TargetDir)"
    exit
}

# 2. Trunk Serve 実行
Write-Host "Starting Trunk Serve..." -ForegroundColor Green
# trunk serve を実行
trunk serve

# 3. 終了後に元の場所に戻る
Pop-Location
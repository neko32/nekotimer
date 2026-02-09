# 1. ルートディレクトリ（..）へ移動して実行、終わったら戻る
Write-Host "--- Starting Backend [tako] ---" -ForegroundColor Cyan

# Push-Location で一時的に上の階層へ移動
Push-Location ".."

try {
    # 2. cargo run の実行
    # -p tako でパッケージを指定
    cargo run -p nekotimer-backend
}
finally {
    # 3. エラーが起きても、Ctrl+Cで止めても、元の場所に戻る
    Pop-Location
    Write-Host "--- Returned to original directory ---" -ForegroundColor Gray
}
# 清理项目
# cargo clean

# 设置绝对路径（关键修正点）
$currentDir = Get-Location
$FFI = "rust_grpc"
$OHOS_NDK ="F:\OpenHarmony\Sdk\12"  # 确保绝对路径

# 验证NDK路径存在性
if (-not (Test-Path $OHOS_NDK)) {
    throw "OHOS_NDK路径不存在：$OHOS_NDK"
}

# 设置环境变量（使用绝对路径）
$env:OHOS_NDK_HOME = $OHOS_NDK

# 构建release版本
ohrs build --dist=./dist/symbol/ --release

# 处理符号文件（保持原逻辑）
$distPath = "./dist"
Push-Location $distPath
try {
    if (Test-Path "./strip") {
        Remove-Item -Path "./strip" -Recurse -Force
    }
    New-Item -Path "./strip" -ItemType Directory | Out-Null
    Copy-Item -Path "./symbol/*" -Destination "./strip/" -Recurse
}
finally {
    Pop-Location
}

# 剥离调试信息（关键路径修正）
$stripPaths = @(
    "arm64-v8a/lib${FFI}.so",
    "armeabi-v7a/lib${FFI}.so",
    "x86_64/lib${FFI}.so"
)

foreach ($arch in $stripPaths) {
    $soPath = Join-Path $distPath "strip" $arch
    $fullPath = Join-Path $currentDir $soPath
    
    # 双重路径验证
    if (-not (Test-Path $fullPath)) {
        Write-Warning "文件不存在：$fullPath"
        continue
    }
    
    # 使用Join-Path保证路径正确性
    $stripTool = Join-Path $OHOS_NDK "native/llvm/bin/llvm-strip.exe" -Resolve
    & $stripTool "--strip-all" $fullPath
}
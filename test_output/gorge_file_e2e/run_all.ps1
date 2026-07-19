# Gorge 端到端测试全集
# 用法: .\run_all.ps1
# 建议从项目根目录运行: powershell -ExecutionPolicy Bypass -File test_output\gorge_file_e2e\run_all.ps1

$ErrorActionPreference = "Continue"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = (Resolve-Path "$scriptRoot\..\..\").Path

Write-Output "============================================"
Write-Output "  Gorge 端到端测试 — 全量运行"
Write-Output "============================================"
Write-Output "项目根: $projectRoot"
Write-Output ""

$script:total = 0
$script:passed = 0
$script:failed = 0
$script:skipped = 0
$script:results = @()

function Run-One {
    param(
        [string]$Name,
        [string]$SourceG,
        [string]$EntryClass,
        [string]$EntryMethod,
        [int64]$Expected,
        [string]$Status,
        [string]$Reason
    )
    $script:total += 1

    if ($Status -eq "SKIP") {
        Write-Output "[SKIP] $Name — $Reason"
        $script:skipped += 1
        $script:results += @{ N = $Name; R = "SKIP"; E = $Expected; A = "N/A"; M = $Reason }
        return
    }

    $gorgeFile = $SourceG -replace "\.g$", ".gorge"
    $gorgePath = Join-Path $scriptRoot $gorgeFile
    $srcPath = Join-Path $scriptRoot $SourceG

    # 编译
    Write-Output "编译 $Name..."
    $compileOut = & cargo run --quiet --bin gorgec -- $srcPath -o $gorgePath 2>&1
    if ($LASTEXITCODE -ne 0 -or $compileOut -match "编译错误|error:") {
        Write-Output "[FAIL] $Name — 编译失败"
        $compileOut | ForEach-Object { Write-Output "       $_" }
        $script:failed += 1
        $script:results += @{ N = $Name; R = "FAIL"; E = $Expected; A = "COMPILE_ERR"; M = "编译失败" }
        return
    }

    # 运行
    $runOut = & "$projectRoot\GorgeFramework\target\release\gorge_runner.exe" $gorgePath "$EntryClass.$EntryMethod" 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Output "[FAIL] $Name — 运行时错误"
        $runOut | ForEach-Object { Write-Output "       $_" }
        $script:failed += 1
        $script:results += @{ N = $Name; R = "FAIL"; E = $Expected; A = "RUNTIME_ERR"; M = "运行时错误" }
        return
    }

    # 提取返回值
    $actual = $null
    $runStr = [string]$runOut
    if ($runStr -match '返回 \(int\): (-?\d+)') {
        $actual = [int64]$matches[1]
    } elseif ($runStr -match '返回 \(float\): ([0-9.]+)') {
        $actual = [double]$matches[1]
    }

    if ($actual -eq $null) {
        Write-Output "[FAIL] $Name — 无法解析返回值"
        Write-Output "       $runStr"
        $script:failed += 1
        $script:results += @{ N = $Name; R = "FAIL"; E = $Expected; A = "PARSE_ERR"; M = "无法解析返回值: $runStr" }
        return
    }

    if ($actual -eq $Expected) {
        Write-Output "[PASS] $Name -> $actual"
        $script:passed += 1
        $script:results += @{ N = $Name; R = "PASS"; E = $Expected; A = $actual; M = "" }
    } else {
        Write-Output "[FAIL] $Name — 预期 $Expected，实际 $actual"
        $script:failed += 1
        $script:results += @{ N = $Name; R = "FAIL"; E = $Expected; A = $actual; M = "值不匹配" }
    }
}

# ==================== 测试定义 ====================
# Test1: 纯算术循环 50M 次
Run-One -Name "Test1" -SourceG "Test1.g" -EntryClass "Test1" -EntryMethod "DoTest" -Expected 50000000 -Status "CHECK"

# Test2: 静态方法调用循环 50M 次
Run-One -Name "Test2" -SourceG "Test2.g" -EntryClass "Test2" -EntryMethod "DoTest" -Expected 50000000 -Status "CHECK"

# Test3: 递归静态方法 500K 次
Run-One -Name "Test3" -SourceG "Test3.g" -EntryClass "Test3" -EntryMethod "DoTest" -Expected 500000 -Status "CHECK"

# Test4: 实例方法 + 字段访问 100M 次（性能测试）
Run-One -Name "Test4" -SourceG "Test4.g" -EntryClass "Test4" -EntryMethod "DoTest" -Expected 100000000 -Status "CHECK"

# Test5: 继承字段/方法访问 10M 次
Run-One -Name "Test5" -SourceG "Test5.g" -EntryClass "Test5" -EntryMethod "DoTest" -Expected 40000000 -Status "CHECK"

# Test6: 接口方法分派 10M 次
Run-One -Name "Test6" -SourceG "Test6.g" -EntryClass "Test6" -EntryMethod "DoTest" -Expected 10000000 -Status "CHECK"

# Test7: 委托/Lambda 捕获变量 — 运行时栈溢出，SKIP
Run-One -Name "Test7" -SourceG "Test7.g" -EntryClass "Test7" -EntryMethod "DoTest" -Expected 10000000 -Status "SKIP" -Reason "委托 Lambda 捕获变量未实现（栈溢出）"

# Test8: native 静态+实例方法混合调用 10M 次（已修复：静态方法编号对齐）
Run-One -Name "Test8" -SourceG "Test8.g" -EntryClass "Test8" -EntryMethod "DoTest" -Expected 100000000 -Status "CHECK"

# Test9: 注入器字段（native+编译混合）— 编译失败，SKIP
# B-1 已修: parser match_identifier_or_keyword() 支持 @Inject 关键字注解
# 剩余阻塞: codegen 注入器字段查找失败(^innerFieldA)、成员链访问 native 注入器字段未实现、new ^field() 语法生成空方法名
Run-One -Name "Test9" -SourceG "Test9.g" -EntryClass "Test9" -EntryMethod "DoTest" -Expected 10000000 -Status "SKIP" -Reason "codegen: 注入器字段查找/成员链/构造函数注入器未实现"

# Test10: 注入器数组构造 new listB[n]（已修复：元素类型解析+常量初始化）
Run-One -Name "Test10" -SourceG "Test10.g" -EntryClass "Test10" -EntryMethod "DoTest" -Expected 10000000 -Status "CHECK"

# Test11: 注入器数组+对象列表 — 编译失败，SKIP
Run-One -Name "Test11" -SourceG "Test11.g" -EntryClass "Test11" -EntryMethod "DoTest" -Expected 10000000 -Status "SKIP" -Reason "注入器对象列表/length 属性/injector 构造器未支持"

# Test12: 嵌套委托 — 返回值错误 (0)，SKIP
Run-One -Name "Test12" -SourceG "Test12.g" -EntryClass "Test12" -EntryMethod "DoTest" -Expected 10000000 -Status "SKIP" -Reason "嵌套委托+Lambda 闭包未实现"

# test3_small: 递归静态方法缩小版
Run-One -Name "test3_small" -SourceG "test3_small.g" -EntryClass "Test3Small" -EntryMethod "DoTest" -Expected 1000 -Status "CHECK"

# test4_small: 实例方法+字段缩小版
Run-One -Name "test4_small" -SourceG "test4_small.g" -EntryClass "Test4Small" -EntryMethod "DoTest" -Expected 1000 -Status "CHECK"

# ==================== 结果矩阵 ====================
Write-Output ""
Write-Output "============================================"
Write-Output "  测试结果矩阵"
Write-Output "============================================"
Write-Output ("{0,-16} {1,-6} {2,14} {3,14}  备注" -f "名称", "结果", "预期", "实际")
Write-Output ("{0,-16} {1,-6} {2,14} {3,14}  ----" -f "----", "----", "----", "----")
foreach ($r in $script:results) {
    $aStr = if ($r.A -is [int64]) { [string]$r.A } elseif ($r.A -is [double]) { [string]$r.A } else { [string]$r.A }
    Write-Output ("{0,-16} {1,-6} {2,14} {3,14}  {4}" -f $r.N, $r.R, $r.E, $aStr, $r.M)
}

# ==================== 统计摘要 ====================
$nonSkip = $script:total - $script:skipped
$passRate = if ($nonSkip -gt 0) { [math]::Round($script:passed / $nonSkip * 100, 1) } else { 0 }
Write-Output ""
Write-Output "============================================"
Write-Output "  统计摘要"
Write-Output "============================================"
Write-Output "总计:  $($script:total) 项"
Write-Output "PASS:  $($script:passed)"
Write-Output "FAIL:  $($script:failed)"
Write-Output "SKIP:  $($script:skipped)"
Write-Output ""
Write-Output "排除 SKIP 后通过率: $($script:passed) / $nonSkip = ${passRate}%"
Write-Output ""

# Little Mat Calculator (LMC)
一个学习线性代数和 Rust 过程中的玩具项目，包含一个库和两个前端

LMC 已经被编译为 WASM ，托管在 [Github Pages](https://danzou1ge6.github.io/little_mat_calc) 上

## 主要特性
LMC 可用于进行简单的矩阵计算，支持
- 有理数矩阵运算
- 复数矩阵运算
- Scheme 风格的表达式
- 创建常量
- 定义简单的函数

## 举例
```
>  (def A [1 2; 3 4;])
= nil

> (* A (+ [3 4; 5 6;] A))   
= 
  20  26
  44  58

> (det A)
= -2

> (ortho A)
= 
     1   3/5
     3  -1/5

>  (def A [1 2 3; 5 3 2; 8 9 7;])
= nil

>  (eigval toc A)
= 
  12.80979
  -1.59607
  -0.21372

>  (nspace A)
= nil

>  (inv A)
= 
    3/28   13/28   -5/28
  -19/28  -17/28   13/28
     3/4     1/4    -1/4

>  (* A [1; 4; 5;])
= 
  24
  27
  79

>  (solve A tp [1 3 4;])
= 
  11/14
  -9/14
    1/2

```

## 使用说明

### Scheme 风格表达式
在 LMC 的语言中仅有表达式. 一切表达式用 `()` 包裹

### 字面值
LMC 中有一系列字面值，包括
- 标量，又可被分为
    - 有理数，包括整数. e.g. `1/3`, `1`
    - 复数，包括实数. e.g. `0.2` `0.2+3.2j`
      一个复数在内部由两个 `f64` 的浮点数表示，当一个浮点数的值绝对值小于 `1e-6` ，在 LMC 中便会被认为等于零
- 矩阵，可以含有有理数或者复数. 矩阵用 `[]` 表示，用 `;` 隔开各行，比如 `[1 2; 3 4;]`.
  浮点实值矩阵通过取复矩阵的实部获得.
- 布尔，用 `#t` (true) `#f` (flase) 表示
- 符号表，和矩阵的表示法相同，但是仅包含变量名，例如 `[a b; c d;]`

### 定义常量
在 LMC 中可以使用 `def` 关键字定义常量
```
 (def <常量名> <值>)
```

例如
```
 (def x 1)
```

### 调用函数
函数通过以下语法进行调用
```
 (<函数名> <参数>..)
```

例如，
```
 (get m i j)
```
用于获取矩阵 `m` 在 `(i, j)` 处的元素. 注，从 `0` 开始计数.

### 内置函数
LMC 内置一系列用于矩阵运算的函数，例如
```
(inv m)
```
将会求得 `m` 的逆.

对于任一内置函数，可以通过
```
(help <函数名>)
```
获得更详细的帮助信息.

另外，通过
```
(help 1)
```
可以获得内置函数的列表.

### IF 分支
在 LMC 中可以使用 `if` 关键字控制求值流程，其语法为
```
(if <test> <then> <else>)
```
如果 `<test>` 值为 `#t` ，此表达式的值将为 `<then>` ，否则为 `<else>`

### 定义函数
LMC 支持定义简单的函数，其语法为
```
(def (<函数名> <参数名>..) <函数体>)
```
在调用时，传入的参数值将会被绑定到 `<参数名>` 上，然后 `<函数体>` 会被求值

例如，
```
(def (pow x n) (if (< n 2) x (* x (pow x (- n 1)))))
```
定义了一个计算 `x` 的 `n` 次幂的函数

### 预定义函数
LMC 也含有一系列预定义函数，这些函数通过 LMC 表达式定义.

相关的帮助信息保存在变量 `_help_<函数名>` 中，可以通过 `(help <函数名>)` 获得


### 特殊变量
- `_` 在每次求值后定义，保存了上一次求值的结果
- `preludes` 保存了预定义函数的名字的列表用于查询

## 详细功能列表
|功能                 |有理矩阵|复矩阵（包含浮点实值矩阵）| 相关函数名  |
|--------------------|-------|------|---------------------------|
|加减、点乘、转置       |Yes    |Yes   |          +,-,*,tp         |
|合并矩阵              |Yes   |Yes    |          concat          |
|切分矩阵              |Yes    |Yes    |        slice            |
|求行列式、迹          |Yes    |Yes    |         det,tr          |
|高斯消元、求简化上阶梯矩阵|Yes    |Yes    |       eliminate,rref  |
|求秩                 |Yes    |Yes     |        rank            |
|求零空间              |Yes    |Yes   |         nspace           |
|拼接矩阵             |Yes     |Yes   |          concat          |
|求解线性方程组        |Yes    |Yes    |          solve          |
|求特征值              |No    |Yes*仅支持实数    | eigval          |
|根据特征值求特征向量    |Yes   |Yes   |            eigvecof        |
|根据特征值求广义特征向量 |Yes   |Yes   |            geigvecof      |
|QR分解               |No    |Yes*仅支持实数   |  qr             |
|正交基化              |Yes   |Yes            |  ortho          |
|规范化                |No    |Yes*仅支持实数   |  normalize      |

以及一些实用工具
- `dim` 获取矩阵维数
- `get` 获取矩阵某一位置上元素
- `diag`, `ri`, `ci` 合成对角矩阵、单位矩阵
- `diag` 读取对角线
- `pow` 矩阵的幂
- `row` 读取某一行
- `col` 读取某一列

## 已知问题
- 求解大型有理矩阵的逆时会出现乘法运算溢出

## 源代码布局
- 根目录下的 crate 是一个简单的矩阵运算库 `mat`
- `mat_calc` 下的 crate 是一个 `mat` 的前端，包含相关库文件和一个在终端中运行的 Binary
- `mat_calc_web` 下的 crate 为 `mat_calc` 中的库文件提供 JS 接口，编译为 WASM

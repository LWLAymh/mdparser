---
title: 范畴论初步
categories: 大学课程
tags: [数学]
mathjax: true
---
<!-- more -->
## 基础集合论
### 映射
设$A , B$为两个集合,$\varphi$称为从$A$到$B$的一个**映射**,如果对于任意$a \in A$,$\exists b \in B , b = \varphi ( a )$.此时$b$称为$a$在$\varphi$下的像,$a$称为$b$在$\varphi$下的原像或反像.一般地,这个定义可以拓展到$S \subseteq A$,$\varphi ( S ) = \{ \varphi ( a ) \mid a \in S \}$,定义$\varphi^{ - 1 } ( T ) = \{ a | \varphi ( a ) \in T , a \in S \}$.

如果$\forall a_1 , a_2 \in A , a_1 \ne a_2 , \varphi ( a_1 ) \ne \varphi ( a_2 )$,称$\varphi$是**单射**.

如果$\forall b \in B , \exists a \in A , \varphi ( a ) = b$.称$\varphi$为**满射**.

如果$\varphi$既是单射又是满射,称其为**双射**或**一一映射**.

如果$A = B$,我们又称$\varphi$为一个**变换**.

设$f : A \rightarrow B , g : B \rightarrow A$,那么:

1. 如果$g \circ f = id_A$,称$g$是$f$的一个**左逆**,不难发现$f$存在左逆当且仅当$f$是单射.

2. 如果$f \circ g = id_B$,称$g$是$f$的一个**右逆**,不难发现$f$存在右逆当且仅当$f$是满射.

3. 如果$g$既是$f$的左逆又是$f$的右逆,则称$g$为$f$的**逆**,不难发现$f$存在逆当且仅当$f$是双射,并且逆唯一.

### 二元运算与二元关系
集合$A , B$的**笛卡尔积**或**直积**是指$A$的元素与$B$的元素构成的有序对的集合,即$A \times B = \{ ( a , b ) \mid a \in A , b \in B \}$.可以推广到多元对.

集合$A$上的一个**二元运算**是由$A \times A$到$A$的一个映射.对于定义在$U$上的一个二元运算,不妨用$+$来表示,集合$A , B \subseteq U$的**闵可夫斯基和**定义为$A + B = \{ a + b \mid a \in A , b \in B \}$.OI中常见的如取背包等算法,其实就是在做闵和后去除或合并若干点.

集合$A$上的一个**二元关系**$R$定义为$A \times A$的一个子集(可以理解为满足二元关系的解集),如果$( a_1 , a_2 ) \in R$,就称$a_1 , a_2$有关系$R$,记作$a_1 Ra_2$,注意这里有序.

如果$R$满足以下三个性质:

1. 反身性:$\forall a \in A , aRa$.

2. 对称性:$a_1 Ra_2 \Leftrightarrow a_2 Ra_1$.

3. 传递性:$a_1 Ra_2 , a_2 Ra_3 \Leftrightarrow a_1 Ra_3$.

则称$R$是一个**等价关系**,不难发现满足等价关系的元素构成了若干个集合,称作**等价类**.等价关系通常记作$\sim$.

如果将等价关系的(2)改作:

2. 反对称性:$a_1 Ra_2 , a_2 Ra_1 \Leftrightarrow a_1 = a_2$.

则称$R$是一个**偏序关系**,具有某种偏序关系的集合称为**偏序集**,如果任意两个元素都存在偏序关系,称作**全序集**.如果一个全序集的任意一个子集存在最小元,则称其为**良序集**.

偏序关系通常记作$\leq$.

#### 保序映射
对于映射$f : A \rightarrow B$,如果$\forall a , a ' \in A , a \leq a ' \Rightarrow f ( a ) \leq f ( a ' )$,则称其为保序的.

#### Dilworth定理
对于偏序集$( A , R )$来说,定义:

1. 链:$A ' \subseteq A , \forall x , y \in A ' , [ xRy \lor yRx ] = 1$.

2. 反链:$A ' \subseteq A , \forall x , y \in A ' , [ xRy \lor yRx ] = 0$.

3. 链覆盖:挑出最少数量的链,使得其可重复覆盖了所有点.

Dilworth定理是说:一个偏序集的最长反链等于最小链覆盖.

考虑数学归纳,$| A | \leq 3$显然成立.

当$| A | > 3$时,如果存在一个点和其它任何点都没有偏序关系,那显然把它删掉就可以数学归纳.

反之,设其最长反链数为$d$,下面证明其最小链覆盖也是$d$.

考虑取一条尽可能长的链$A '$,设其中最大的为$M$,最小的为$m$(如果有多个就任取一个).

考虑$T = A \setminus A '$,如果$T$中的最长反链数小于$d$,则数学归纳成立.

反之,则一定可以取出一个反链$S$,使得$S \cap A ' = \emptyset$.考虑设$A^{ + } = \{ x | x \in A \land \exists s \in S , sRx \}$,$A^{ - } = \{ x | x \in A \land \exists s \in S , xRs \}$.不难发现$m$一定不在$| A^{ + } |$中,不然取出的那条链$A '$可以更长.同理$M$一定不在$| A^{ - } |$中,也即$| A^+ | , | A^- | < | A |$.

我们又发现$A^+ \cup A^- = A$,原因是如果一个点不在这两个集合中,一定可以添加到最长反链中.

我们还发现由于偏序关系的反身性,$S \subseteq A^{ + } \cap A^{ - }$.

由数学归纳,$A^+$和$A^-$中的最小链覆盖均为$d$,并且一个以$S$中元素开头,一个以$S$中元素结尾,我们把它俩拼起来就是一个大小为$d$的新的大小为$d$的链覆盖.而且显然不会有更优的结果了.

#### 商集
考虑定义一种等价关系$\sim$,我们可以在此基础上定义商集$A / \sim = \{ [ a ] | a \in A \}$,其中$[ a ]$是$a$所在的等价类.

商群有一个很漂亮的应用是,通过$\mathbb{ N }$来定义$\mathbb{ Z }$.(默认加法和乘法都存在幺元,结合律,消去律,交换律,分配律)

这个是怎么做呢?考虑定义一种等价关系$\sim$,$( n , m ) \sim ( n ' , m ' )$当且仅当$n + m ' = m + n '$.虽然我们手上没有加法逆元和乘法逆元,但我们有加法和乘法的消去律,这样可以验证该等价关系满足反身性,传递性,对称性.

然后我们将$\mathbb{ N } / \sim$定义为整数,更确切地,对于任意一组$( n , m )$其代表的就是$n - m$这个整数.

容易定义加法:$( n , m ) + ( n ' , m ' ) = ( n + n ' , m + m ' )$,乘法$( n , m ) \times ( n ' , m ' ) = ( mm ' + nn ' , mn ' + m ' n )$.也可以定义大小关系$( n , m ) \leq ( n ' , m ' ) \Leftrightarrow n + m ' \leq m + n '$.

不过由于我们定义的是等价类,所以还要证明这样的定义是良定义的,这个是容易验证的.

还有相反数,定义$- ( n , m ) = ( m , n )$即可,容易验证二者之和处于$( 0 , 0 )$这个等价类,也容易验证相反数唯一.

同理可以用$\mathbb{ Z }$构造$\mathbb{ Q }$,构造二元组$\mathbb{ Z } \times \mathbb{ N }_+$,定义$( r , s ) \sim ( r ' , s ' ) \Leftrightarrow rs ' = r ' s$,容易验证其满足等价关系的性质.实际上其对应的就是$\frac{ r }{ s }$这个有理数,对着构造四则运算即可.

### ZFC公理体系
#### 外延公理
$A = B \Leftrightarrow ( A \subset B ) \land ( B \subset A )$.

#### 配对公理
对于任意元素$a , b$,存在集合$\{ a , b \}$.特别地,当$a = b$的时候存在集合$\{ a \}$.

值得一提的是可以将数对$( a , b )$定义为$\{ \{ a \} , \{ a , b \} \}$,下面的幂集公理说明了数对属于$2^{ 2^{ A \cup B } } , a \in A , b \in B$.

#### 分离公理(模式)
对于一个集合$A$,和一个性质$P$,若$\alpha$满足性质$P$则称$P ( \alpha )$.那么存在集合:

$$
\{ x | x \in A \land P ( x ) \}
$$

注意全体集合并不构成集合(ZFC公理体系不提供一种方式生成全体集合的集合),因此分离公理避免了罗素悖论.

之所以称为模式,是因为其对于每个性质$P$都构造了相应的公理.

#### 并集公理
允许将集合取并.

#### 幂集公理
一个集合的所有子集构成一个新的集合(可以记作$2^A$).

#### 无穷公理
称满足以下性质的集合为归纳集:

1. $\emptyset \in A$.

2. 若$\alpha \in A$,则$\{ \alpha \} \cup \alpha \in A$.

则归纳集存在.

#### 替换公理(模式)
对于集合$A$和一种定义在集合$A$上的映射$F$,存在集合$A '$使得$x \in A ' \Leftrightarrow x = F ( a ) , a \in A$.

用替换公理可以证明映射也是一种集合,并且从$A \rightarrow B$的所有映射可以构成集合,可以记作$B^A$.

#### 正则公理
对任何非空集合$A$,存在$a \in A$使得$\forall a ' \in A$,$a ' \notin a$,也就是$a \cap A = \emptyset$.

#### 选择公理
说对于任何一族非空集$A$,总能从其中的每个集合$a$选出一个元素.

选择公理独立于其它的集合公理,一些数学家好像还不愿意承认选择公理.

选择公理等价于另外两个命题:Zorn引理和良序定理.

##### Zorn引理
如果$X$上的一个偏序关系$\leq$,满足其每条链$A$都存在上界($\exists x \in X , \forall a \in A , x \geq a$),那么$X$存在极大元$( \exists x \in X , \forall a \in X , a \geq x \Rightarrow a = x )$.

Zorn引理可以证明选择公理,简单来说就是定义偏序关系$( A ' ' , g ' ' ) \leq ( A ' , g ' )$当且仅当$A ' ' \subseteq A '$并且$\forall x \in A ' ' , g ' ' ( x ) = g ' ( x )$.这样拿出来的极大元就是我们需要的$A \rightarrow g ( A )$.

##### 良序定理
所有的集合都可以被良序排列,或者说都存在一种偏序,使得其任意子集都是良序集.

良序定理的证明要用的Zorn引理,即考虑所有的二元对$( S , R )$组成的集合,其中满足$S$对于偏序关系$R$来说是良序的,我们定义两个二元对$a , b$满足$a \leq b$当且仅当它们的$R$相等并且$S_a \subseteq S_b$.现在取出一个全序的子集并找到其中最大的集合(也就是其它所有集合的并)$( S , R )$,注意到如果$S$是全集则证毕,如果不是,则取一个全集中不在$S$的元素$x$接到$S$的后面得到了一个更大的集合,这就产生了矛盾,具体细节过程可以参考下面关于集合基数的全序性的证明.

良序定理可以证明选择公理,因为只需每个集合选最小的元素再用替换公理模式换掉就行了.

### 集合的基数
如果存在单射$\varphi : A \rightarrow B$,则称$| A | \leq | B |$.如果存在双射,则称$| A | = | B |$.

如果使用选择公理的话,可以说明如果存在满射$\varphi : A \rightarrow B$,则称$| A | \geq | B |$.

值得一提的是这种偏序关系是全序的,这个怎么证明呢?

考虑现在有$A , B$两个集合,接下来我们证明一定存在一个从$A$到$B$的映射,且其要么为单射要么为满射.

我们记$X$为所有$A$的子集到$B$的单射组成的集合,并且我们定义这样一种偏序关系:若$f : A_f \rightarrow B , g : A_g \rightarrow B , A_f \subseteq A_g , \forall a \in A_f , f ( a ) = g ( a )$.

这样的话,我们不妨取出$X$的一个全序的子集$I$,显然其存在上界.

根据Zorn引理,$X$存在极大元$g : A_g \rightarrow B$,此时我们开始讨论:

如果$A_g = A$,那么找到了一个单射$g : A \rightarrow B$.

反之,如果$A_g \subsetneq A$,考虑如果$g ( A_g ) = B$,也就是找到了一个满射;反之,考虑取$a ' \in A \setminus A_g , b ' \in B \setminus g ( A_g )$,在$g$的基础上加上映射$a ' \mapsto b '$,这是一个比$g$还要大的元,不符合Zorn引理.

#### Cantor-Bernstein定理
如果$| A | \leq | B | \land | B | \leq | A |$,则$| A | = | B |$.

不妨设$f : A \rightarrow B , g : B \rightarrow A$我们考虑一个感性的做法:考虑将这个东西画成二分图,然后要找它的完美匹配.我们不妨先把不同的连通块拆开,你会发现大部分的图都可以用$f , f^{ - 1 }$来构造双射,只有一种除外:那就是以一个$B$中节点开始不断延伸的无限的,我们在这里使用$g , g^{ - 1 }$来构造即可.

如果要把上面的东西写成形式化的东西,我们可以这么写:取$C_0 = B \setminus f ( A )$,$C_n = f ( g ( C_{ n - 1 } ) )$,那么对于$C = \cup_{ n \geq 0 } C_n$,使用$g , g^{ - 1 }$构造双射,剩下的使用$f , f^{ - 1 }$构造双射.

#### 可数无穷与不可数无穷
我们称集合$A$是**可数**的,当且仅当$| A | = | \mathbb{ N } |$.

我们可以发现可数个可数集合的并一定可数,因为你可以把这些位置全排列起来然后绕着数.

这样就可以发现,首先整系数多项式方程有可数个,因此整系数多项式方程的根也有可数个,我们将这些数称作代数数,其它实数称为超越数.

这又有一个结论是说,代数数对四则运算构成域.

首先一个代数数的加法逆元和乘法逆元(只要不是$0$)肯定一定是代数数,你只需要对着原方程改一改就可以构造出新方程.

幺元肯定也是存在的,因为$1$和$0$显然都是代数数.

我们接下来看两个代数数$x , y$.考虑如何构造$x + y$作为根的方程.

下面是qyc老师给的做法,直接考虑有方程:

$$
\begin{aligned}
X & = \sum_{ k = 0 }^n a_k x^k = 0 \\
Y & = \sum_{ k = 0 }^m b_k y^k = 0
\end{aligned}
$$

对于加法,考虑:$( x + y )^0 , ( x + y )^1 , \cdots$,这些东西,先把它们对$X$取膜再对$Y$取膜,得到的一定是$nm$维的,只需要取$nm + 1$个就会出现线性相关.乘法同理考虑$( xy )^0 , ( xy )^1 , \cdots$即可.

值得一提的是,$| \mathbb{ R } | = 2^{ | \mathbb{ N } | }$.这个是怎么构造的呢?只需要证明$[ 0 , 1 ]$上的实数和自然数集合能一一对应就可以了对吧,考虑将这些实数写成二进制下的形式,然后如果$k$在取出的自然数子集中,那么小数点后第$k + 1$位为$1$否则为$0$,这样就构造了一组映射.

不过其实这样做有一点小问题,那就是$0 . 1 = 0 . 0111 \cdots$,但这两个数字对应的集合并不相等.然而会出问题的点的个数是可数无穷个,因此其实差一下也没什么问题.如果或者可以反复来一下证明$| \mathbb{ R } | \leq 2^{ | \mathbb{ N } | }$并且$| \mathbb{ R } | \geq 2^{ | \mathbb{ N } | }$,用上面的构造就行.

## 简单范畴论
### 范畴
一个**范畴**$\mathcal{ C }$应当包含以下:

1. 一个类$\rm{ Ob } ( \mathcal{ C } )$,其元素称作$\mathcal{ C }$的**对象**.

2. 对于$X , Y \in \rm{ Ob } ( \mathcal{ C } )$指定一个集合$\text{ Hom }_{ \mathcal{ C } } ( X , Y )$,称作$\mathcal{ C }$中从$X$到$Y$的**态射**.

对于态射来说,其还应当具有以下特点:

1. 对于$X \in \text{ Ob } ( \mathcal{ C } )$存在其到自身的**恒等态射**$\text{ id }_X \in \text{ Hom }_{ \mathcal{ C } } ( X , X )$.

2. 态射间可以进行合成,换言之存在**合成映射**$\circ : \text{ Hom }_{ \mathcal{ C } } ( Y , Z ) \times \text{ Hom }_{ \mathcal{ C } } ( X , Y ) \to \text{ Hom }_{ \mathcal{ C } } ( X , Z ) , ( g , f ) \mapsto g \circ f$.

另外,合成映射还应当满足:

1. **结合律**:对于$\mathcal{ C }$中的态射$h , g , f$,如果合成有意义,那么$h ( gf ) = ( hg ) f$.

2. **单位元**:对于$f \in \text{ Hom }_{ \mathcal{ C } } ( X , Y )$,$f \circ \text{ id }_X = f = \text{ id }_Y \circ f$.

由上得知恒等态射的唯一性,因为如果存在两种恒等态射,必定有$\text{ id }_X = \text{ id }_X \text{ id ' }_X = \text{ id ' }_X$.

在此基础上可以定义所谓**交换图表**,意味着态射的合成是所谓殊途同归的.例如,如果下面这个图表是交换的:

$$
\xymatrix{ R \ar[r]^u \ar[d]_x & S \ar[d]^v \\
T \ar[r]_y & U }
$$

这意味着$yx = vu$.

必须要指出的是,我们这里范畴所包含的对象并不是单指某一特定元素,而也有可能是某一数学结构(例如集合).

特别地,如果:

1. 如果对于$f \in \text{ Hom } ( X , Y )$,$\exists g \in \text{ Hom } ( Y , X )$使得$gf = \text{ id }_X$,那么称$g$是$f$的一个**左逆**,称$f$是**左可逆**的或者是**单态射**.同理可以定义**右逆**以及所谓**满态射**.

2. 如果$f$左右均可逆,易见左右逆相等,记作$f^{ - 1 }$,易见$f^{ - 1 }$可逆而且$( f^{ - 1 } )^{ - 1 } = f$.此时称$f$是**同构**的.

同构还有以下性质:

1. $( \text{ id }_X )^{ - 1 } = \text{ id }_X$.

2. 如果$f$和$g$均为同构并且合成有意义,那么$gf$是同构并且$( gf )^{ - 1 } = f^{ - 1 } g^{ - 1 }$.

于是在此基础上追加定义:

1. 自同态幺半群:$\text{ End }_{ \mathcal{ C } } ( X ) = \text{ Hom }_{ \mathcal{ C } } ( X , X )$.

2. 自同构群:$\text{ Aut }_{ \mathcal{ C } } ( X ) = ( \text{ End }_{ \mathcal{ C } } ( X ) )^\times$.

如此起名的原因是由抽象代数知识见到$\text{ End }$对二元运算$\circ$构成幺半群,$\text{ Aut }$则构成群.

我们还可以定义所谓**子范畴**.称$\mathcal{ C } '$是$\mathcal{ C }$的子范畴当且仅当:

1. $\text{ Ob } ( \mathcal{ C } ' ) \subseteq \text{ Ob } ( \mathcal{ C } )$.

2. $\forall X , Y \in \text{ Ob } ( \mathcal{ C } ' )$都有$\text{ Hom }_{ \mathcal{ C } ' } ( X , Y ) \subseteq \text{ Hom }_{ \mathcal{ C } } ( X , Y )$.

3. $\forall X \in \text{ Ob } ( \mathcal{ C } ' )$,$\text{ id }_X = \text{ id }_X '$.

4. 态射在$\mathcal{ C } '$中的合成运算应从$\mathcal{ C }$中继承而来.

特别地,如果$\forall X , Y \in \text{ Ob } ( \mathcal{ C } ' ) , \text{ Hom } ( X , Y )_{ \mathcal{ C } ' } = \text{ Hom }_{ \mathcal{ C } } ( X , Y )$,则称$\mathcal{ C } '$是$\mathcal{ C }$的一个全子范畴.

### 函子
我们定义从$\mathcal{ C } '$到$\mathcal{ C }$的**函子**$F$需要有以下性质:

1. $\forall X \in \text{ Ob } ( \mathcal{ C } ' )$,指定一个$\mathcal{ C }$中的对象$FX$.或记作$F : \text{ Ob } ( \mathcal{ C } ' ) \to \text{ Ob } ( \mathcal{ C } )$.

2. $\forall X , Y \in \text{ Ob } ( \mathcal{ C } ' )$,对于态射$f : X \to Y$指定一个$\mathcal{ C }$中的对象$Ff$.或记作$F : \text{ Hom }_\mathcal{ C } ' ( X , Y ) \to \text{ Hom }_\mathcal{ C } ( FX , FY )$.

上述对态射的函子还应当满足以下性质:

1. $\forall g , f \in \text{ Hom }_{ \mathcal{ C } ' }$,它们的合成有意义,则$F ( gf ) = F ( g ) F ( f )$.

2. $\forall X \in \text{ Ob } ( \mathcal{ C } ' )$,$F ( \text{ id }_X ) = \text{ id }_{ FX }$.

我们一般也将上述函子记作$F : \mathcal{ C } ' \to \mathcal{ C }$.这当然意味着函子是可以合成的并仍然满足结合律.

既然$F$可以在某种程度上看作映射,我们当然还可以引出以下定义:

1. 如果$\forall T \in \text{ Ob } ( \mathcal{ C } ) , \exists X \in \text{ Ob } ( \mathcal{ C } ' )$使得$T \cong FX$,则称$F$是**本质满**的.

2. 如果$\forall X , Y \in \text{ Ob } ( \mathcal{ C } ' )$,$F : \text{ Hom }_{ \mathcal{ C } ' } ( X , Y ) \to \text{ Hom }_{ \mathcal{ C } } ( FX , FY )$均为单射,则称$F$是**忠实**的.如果均为满射,则称其为**全**的.如果均为双射,则称其为**全忠实**的.

回忆到Haskell语言中对函子的定义:

```haskell
class Functor f where
fmap :: (a->b)->f a-> f b
(<$) :: b -> f a -> f b
(<$) = fmap.const
(<$>) :: Functor f => (a->b)->f a -> f b
(<$>) = fmap
```
回忆到Haskell语言中对函子的要求:

```haskell
fmap id == id
fmap (f.g)==(fmap f).(fmap g)
```
容易见到其类似性.

函子的类型很多,比如最平凡的一类函子是所谓**忘却函子**.但我们在这里着重提所谓$\text{ Hom }$函子,其是Haskell语言中Curring化的基础:

对于范畴$\mathcal{ C }$以及其对象$X$,我们定义函子$\text{ Hom }_{ \mathcal{ C } } ( X , \cdot ) : \mathcal{ C } \to Set$,它映对象$Y$为集合$\text{ Hom }_{ \mathcal{ C } } ( X , Y )$,映态射$f : Y \to Z$为映射$\text{ Hom }_{ \mathcal{ C } } ( X , \cdot ) f : \text{ Hom }_{ \mathcal{ C } } ( X , Y ) \to \text{ Hom }_{ \mathcal{ C } } ( X , Z ) , h \mapsto f \circ h$.

同理可以定义函子$\text{ Hom }_{ \mathcal{ C } } ( \cdot , X )$.

对于函子$\text{ Hom }_{ \mathcal{ C } } ( X , \cdot )$,我们可以一窥其在Haskell中的具体定义:

```haskell
instance Functor ((->) a) where
-- fmap :: (b->c)->(a->b)->(a->c)
fmap fbc fab = fbc.fab

instance Applicative ((->) a) where
-- pure :: c->(a->c)
-- <*> :: (a->(b->c))->(a->b)->(a->c)
pure tb = \ta -> tb
(<*>) fabc fab = \ta -> ((fmap (fabc ta) fab) ta)

instance Monad ((->) r) where
-- (>>=) :: (r->a)->(a->(r->b))->(r->b)
(>>=) fra farb = \tr -> ( farb (fra tr) tr )
```
### 范畴实例
#### Kleisli范畴
简单理解Kleisli范畴的话就是,我们被允许给一个元素进行如下的操作:

1. 加上修饰.

2. 合并修饰.

Kleisili范畴在Haskell里的具体表现就是Monad.

```haskell
class Functor f where
fmap        :: (a -> b) -> f a -> f b
(<$)        :: a -> f b -> f a
(<$)        =  fmap . const

class Functor f => Applicative f where
pure :: a -> f a
(<*>) :: f (a -> b) -> f a -> f b

class Applicative m => Monad m where
return :: a -> m a
return = pure
join :: m(m a)-> m a
(>>=) :: m a -> (a -> m b) -> m b
ma >>= f = join (fmap f ma)
(>>) :: m a -> m b -> m b
m >> k = m >>= \_ -> k
```

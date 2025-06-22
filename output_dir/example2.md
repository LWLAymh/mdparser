---
title: 高等代数
categories: 大学课程
tags: [数学]
mathjax: true
---
<!-- toc -->
<!-- more -->
## 抽象代数
### 群
#### 定义
如果一个非空集合$G$上定义了一个二元运算$\circ : G \times G \to G$(不引起混淆的前提下,通常省略不写,例如$a \circ b$记作$ab$),满足:

0. 封闭性:$a \circ b \in G$.

1. 结合律:$( a \circ b ) \circ c = a \circ ( b \circ c )$.

那么称作$G$是一个**半群**.

如果它还满足:

2. 存在幺元:$\exists e \in G$,使得$\forall a , e \circ a = a \circ e = a$.

那么称作$G$是一个**幺半群(monoid)**.

如果它还满足:

3. 存在逆元:对任意的$a \in G$,存在$b \in G$,使得$a \circ b = e$.

那么称作$G$是一个**群**.

如果它还满足:

4. 交换律:$\forall a , b , a \circ b = b \circ a$.

那么称作$G$是一个**交换群**或**阿贝尔群**.特别地,一般而言在交换群里,我们的运算使用加法符号(有时也直接叫它加法群),这只是一个惯例.

下面主要研究群.

#### 基本概念和事实
定义群的**阶**为群中元素个数,记作$| G |$,如果$| G | < \infty$称为**有限群**,否则为**无限群**.

1. 群的幺元唯一.

设$e , e '$都是幺元,有$e = ee ' = e '$.

2. 群中任一元素的逆元唯一.

设$b , b '$都是$a$的逆元,有$b = bab ' = b '$.

这同样说明了虽然群上的二元运算有可能不满足交换律,但是仍然不存在左逆和右逆这种东西(或者说无意义),它俩都等价于逆.

3. 群中有消去律,即$ax = ay \Leftrightarrow x = y$(左消去律),$xa = ya \Leftrightarrow x = y$(右消去律).

显然.

于是可以定义乘方,并定义$a^{ 0 } = e , a^{ - 1 } = inv ( a )$.一切的指数的常规操作在这里都可以照常进行.

##### Example1
证明:对于有限集合$G$和满足结合律的二元运算$\circ$来说,如果其有左右消去律,那么该二元运算在该集合上构成群.

考虑是有限集合,所以单射$f_a : b \rightarrow ab$同时也是满射,这意味着$\forall a , b \in G , xa = b$和$ay = b$一定有解,令$b = a$可以拿到幺元,令$b = e$可以拿到逆元,自然可以推出其是群.

##### Example2
设$G$是群,且$\exists a , b , aba^{ - 1 } = b^r$,求证:$a^i ba^{ - i } = b^{ r^i }$.

显然$b^{ rk } = ( b^k )^r = ab^k a^{ - 1 }$,使用数学归纳,注意到$b^{ r^i } = ab^{ r^{ i - 1 } } a^{ - 1 } = a^i ba^{ - i }$.

#### 子群
设$H$为群$G$的非空子集,如果$H$在$G$的运算下构成群,则称$H$为$G$的**子群**,记作$H \leq G$.

**子群的判定定理**:

设$G$是群,$H \subseteq G , H \ne \emptyset$,下列命题等价:

1. $H \leq G$.

2. $\forall a , b \in H , ab \in H , a^{ - 1 } \in H$.

3. $\forall a , b \in H$,有$ab^{ - 1 } \in H$.

显然(1)$\Rightarrow$(2),(2)$\Rightarrow$(3),下面证明(3)$\Rightarrow$(1):

满足(3),有$e = aa^{ - 1 } \in H$,于是存在幺元.有$ea^{ - 1 } \in H$,于是存在逆元.因为$a ( b^{ - 1 } )^{ - 1 } = ab \in H$,于是对运算封闭.同样的运算显然满足结合律.于是证毕.

如果$G$是群,$H , K \subseteq G$,类似闵可夫斯基和,我们定义$HK = \{ hk \mid h \in H , k \in K \}$.如果$H = \{ a \}$,可以写作$aK$.类似记$H^{ - 1 } = \{ h^{ - 1 } \mid h \in H \}$,注意$HH^{ - 1 } \ne \{ e \}$.

再定义$H^n = \{ h_1 h_2 \cdots h_n \mid h_1 , h_2 , \cdots , h_n \in H \}$.

于是子群的判定定理可以改写成:

下列命题等价:

1. $H \leq G$.

2. $H^2 \subseteq H \land H^{ - 1 } \subseteq H$.

3. $HH^{ - 1 } \subseteq H$.

显然,任意群$G$,$G$和$\{ e \}$都是它的子群,我们称为**平凡子群**,如果$H \ne G$,称为**真子群**,记作$H < G$.

若干个群的交仍然是子群,但是若干个群的并不一定.我们可以证明一下若干个群的交仍然是子群:考虑$h \in A , h \in B , w \in A , w \in B$,那么$hw^{ - 1 } \in A \cap B$.

设集合$M \subseteq G$,则称$G$的所有包含$M$的子群的交为由$M$**生成的子群**,记作$\langle M \rangle$.这显然是包含$M$的最小的子群.

如果$\langle M \rangle = G$,则称$M$是$G$的一个**生成系**.可由有限多个元素生成的群叫做**有限生成群**,若$\exists M , | M | = 1 , \langle M \rangle = G$,则称$G$是**循环群**.

对于$a \in G$,我们称$o ( a ) = | \langle a \rangle |$为$a$的**阶**,不难发现$o ( a )$是满足$a^n = e$的最小的正整数$n$.如果这样的$n$不存在,记$o ( a ) = \infty$.群中所有元素的阶的$lcm$称作群的**方次数**(如果不存在,则记为$\infty$),记作$\exp ( G )$.

另一个不平凡的例子是群$G$的**中心**定义为$Z_G = \{ z \in G \mid \forall g \in G , zg = gz \}$,这显然也是一个子群.

##### Example1
证明不存在恰有两个二阶元素的群.

不妨设这两个二阶元素分别为$a \ne b , a , b \ne e$,则注意到$aba$也是一个二阶元素.

如果$aba = a$,则$ba = e$,说明$a = b$.

如果$aba = b$,则$ba = ab$,说明$ab$是第三个二阶元素.

类似还可以证明如果$\exp ( G ) = 2$,则$G$一定是交换群(一定满足$\forall a , b \in G , ab = ba$).原因是假设$ab = c$,$ba = d$,则$cd = e$.

##### Example2
对于一个群$G$,如果$\forall a , b \in G$,$\exists i \in \mathbb{ N_+ } , ( ab )^i = a^i b^i , ( ab )^{ i + 1 } = a^{ i + 1 } b^{ i + 1 } , ( ab )^{ i + 2 } = a^{ i + 2 } b^{ i + 2 }$,求证其是交换群.

考虑:

$$
\begin{aligned}
( ab )^{ i + 1 } & = a^{ i + 1 } b^{ i + 1 } = a ( ab )^i b \\
( ab )^i & = a ( ab )^i a^{ - 1 }
\end{aligned}
$$

同理有$( ab )^{ i + 1 } = a ( ab )^{ i + 1 } a^{ - 1 }$.两式合并一下有$a ( ab )^i a^{ - 1 } ab = a ( ab )^{ i + 1 } a^{ - 1 } \Rightarrow b = aba^{ - 1 }$.

##### Example3
证明若$\exp ( G ) \in \text{ even }$,则存在阶为$2$的元素.

一定存在一个阶为$2 k$的元素$a$,显然$o ( a^k ) = 2$.

##### Example4
当$n > 2$的时候,证明在有限群$G$中阶为$n$的元素个数是偶数.

显然构造双射$a \mapsto a^{ n - 1 }$即可.

##### Example5
对于群$G$,证明$o ( ab ) = o ( ba )$.

考虑反证,不妨设$o ( ab ) = n < o ( ba ) = m$.

则$e = ( ba )^m = ( ba )^{ m - n } b ( ab )^{ n - 1 } a = ( ba )^{ m - n }$,说明$o ( ba ) \leq m - n < m$,不符题意.

##### Example6
证明:如果$H , K \leq G$,那么$HK \leq G \Leftrightarrow HK = KH$.

如果$HK = KH$,那么$HK \leq G \Leftrightarrow HK ( HK )^{ - 1 } = HK$.考虑:

$$
HK ( HK )^{ - 1 } = HKK^{ - 1 } H^{ - 1 } = HKH^{ - 1 } = KHH^{ - 1 } = HK
$$

如果$HK \leq G$,那么$HK = ( HK )^{ - 1 } = K^{ - 1 } H^{ - 1 } = KH$.

##### Example7
设$G$是交换群,证明$G$中的全体有限阶元素构成$G$的一个子群.

等价于证明如果$a , b$的阶有限,那么$ab^{ - 1 }$的阶有限.这对交换群来说是显然的.

##### Example8
如果$G$只有有限多个子群,证明$G$是有限群.

考虑若其有无限阶元素$a$,则$\langle a \rangle$已经有无限个子群(取幂次为$k$的倍数).反之则每个元素都可以生成一个循环群,也有无限个子群.

#### 同态和同构
考虑在两个群$G$和$G_1$之间构造映射$\varphi$,如果$\varphi$保持群运算,即对于所有的$a , b \in G$,都有$\varphi ( ab ) = \varphi ( a ) \varphi ( b )$,则称其为由$G$到$G_1$的一个**群同态**.如果同时$\varphi$又是单(满)射,则称其为**单(满)同态**,又单又满的同态称为**同构**.如果存在一个映射$\varphi$,使得$G , G_1$同构,则称这两个群**同构**,记作$G \cong G_1$.

容易见到,同构等价于两方都有$f : G \to G_1$和$g : G_1 \to G$,并且二者的合成$fg = gf = \text{ id }$.而且同构的合成仍是同构.

我们以$End ( G )$表示$G$的全体**自同态**组成的集合,$Aut ( G )$表示全体**自同构**组成的集合.不难发现,$End ( G )$组成一个幺半群,而$Aut ( G )$组成一个群.

我们可以搞一个伴随映射:$G \to Aut ( G ) , g \mapsto Ad_g$,其中$Ad_g ( x ) = gxg^{ - 1 }$.容易见到$Ad_g = id$的充要条件是$g \in Z ( G )$也就是群的中心.

群同态$\varphi : G \rightarrow G_1$把$G$的幺元映为$G_1$的幺元.原因是:

$$
\varphi ( e )^2 = \varphi ( e^2 ) = \varphi ( e ) e_1
$$

然而应当说明,这里保持单位元对幺半群未必成立,因此如果要将群同态的定义挪到幺半群上,需要额外添加$\varphi ( e ) = e_1$的条件.

我们还有:

$$
e_1 = \varphi ( e ) = \varphi ( aa^{ - 1 } ) = \varphi ( a ) \varphi ( a^{ - 1 } )
$$

所以$\varphi ( a^{ - 1 } ) = ( \varphi ( a ) )^{ - 1 }$.

我们将$\varphi ( G )$称为$G$的**像**,记为$\text{ im } \ \varphi$.又将$e_1$的原像的集合称为$\varphi$的**核**,记为$\ker \ \varphi$.

注意到$\varphi$不是单射等价于:

$$
\begin{aligned}
& \Leftrightarrow \exists a , b \in G , a \ne b , \varphi ( a ) = \varphi ( b ) \\
& \Leftrightarrow \varphi ( ab^{ - 1 } ) = \varphi ( a ) \varphi ( b )^{ - 1 } = e_1 \\
& \Leftrightarrow \ker \ \varphi \ne \{ e \}
\end{aligned}
$$

#### 群的实例
##### 全变换群,对称群与交错群
设$M$是一个非空集合,$M$到自身的双射的全体对于映射的复合构成一个群,称作$M$的**全变换群**,记作$S ( M )$.

设$M$是含有$n$个元素的集合,$M$的全变换群$S ( M )$称为**n级对称群**,记作$S_n$.不失一般性,我们可以设$M = \{ 1 , 2 , \cdots , n \}$,$S_n$的元素称为**n元置换**,任一置换可以用列表的方法表示,即如果$\sigma$是一个映射,我们将其写作:

$$
\sigma = \left ( \begin{array}
{ c } 1 & 2 & \cdots & n \\
\sigma_1 & \sigma_2 & \cdots & \sigma_n
\end{array} \right )
$$

显然$| S_n | = n !$.

设$\sigma \in S_n$,$\{ i_1 , i_2 , \cdots , i_t \} \subseteq \{ 1 , 2 , \cdots , n \}$,有$\sigma ( i_t ) = i_1 , \forall 1 \leq k < t , \sigma ( i_k ) = i_{ k + 1 }$,我们称这样的置换是对于$\{ i_1 , i_2 , \cdots , i_t \}$的**轮换**,记作$\left ( \begin{array}{ c } i_1 & i_2 & \cdots & i_t\end{array} \right )$,$t$为轮换的**长度**,长度为$2$的轮换称作**对换**.显然,一个置换可以被分解成若干两两无交轮换(注意到如果两个轮换无交,则它们可交换)的乘积.又显然,一个轮换可以被分解成若干对换的乘积,原因是总有:

$$
( a_1 , \cdots , a_l ) = ( a_1 , a_l ) ( a_1 , \cdots , a_{ l - 1 } )
$$

另外轮换的性质给出任何一个置换$\sigma$的$o ( \sigma )$就是其拆出的所有轮换的$\text{ lcm }$.

根据逆序对相关,我们知道一个置换的逆序对数量的奇偶性等于其分解成的对换的数量.我们将逆序对数量是奇数的置换称为**奇置换**,是偶数的称之为**偶置换**,$S_n$中的所有偶置换也构成一个群,称之为**n级交错群**,记为$A_n$.$A_n$实际上是$S_n$的一个子群.

##### 一般线性群
对于定义在域$F$上的线性空间$V$,它上面的所有自同构构成一个**一般线性群**$GL ( V ) = \{ T \in \text{ End } ( V ) \mid \det T \ne 0 \}$.特别地,当$\dim V < \infty$的时候,其有一个子群**特殊线性群**$SL ( V ) = \{ T \in \text{ End } ( V ) \mid \det T = 1 \}$.

在配套的内积空间上还可以把正交变换组成的群一块拿到,称为**正交群**或者**酉群**$O ( V )$,还可以定义$SO ( V ) = O ( V ) \cap SL ( V )$.

当然上面的版本都可以用矩阵来表示,这里略去不表.然而对于矩阵来代替上面的情形,我们疑似需要引入群同构的概念,这会在下面提出.

##### 克莱因四元群
取$V = \{ \text{ id } , \sigma , \tau , \sigma \tau \}$,其中:

$$
\begin{aligned}
\sigma & = \left ( \begin{matrix}
1 & 2 & 3 & 4 \\
2 & 1 & 4 & 3
\end{matrix} \right ) \\
\tau & = \left ( \begin{matrix}
1 & 2 & 3 & 4 \\
3 & 4 & 1 & 2
\end{matrix} \right ) \\
\sigma \tau & = \tau \sigma = \left ( \begin{matrix}
1 & 2 & 3 & 4 \\
4 & 3 & 2 & 1
\end{matrix} \right ) \\

\end{aligned}
$$

见到这确实是一个群,而且任何一个元素的逆元都是它本身,而且两个不同的非$1$元的乘积会得到另一个非$1$元.

克莱因群的实例非常多,例如$\{ 1 , 3 , 5 , 7 \}$在$\mod 8$意义下对乘法构成克莱因群,$\{ 0 , 1 , 2 , 3 \}$对异或构成克莱因群.以及:

$$
\left \{ \begin{bmatrix}
1 & 0 \\
0 & 1
\end{bmatrix} , \begin{bmatrix}
1 & 0 \\
0 & - 1
\end{bmatrix} , \begin{bmatrix}
- 1 & 0 \\
0 & 1
\end{bmatrix} , \begin{bmatrix}
- 1 & 0 \\
0 & - 1
\end{bmatrix} \right \}
$$

对矩阵乘法也构成群.

##### 循环群
如果对于一个群$G$,它可以由其中的某个元素$\sigma$生成,记作$G = \langle \sigma \rangle$,当$\forall g \in G , \exists k \in \mathbb{ Z } , g = \sigma^k$.

显然$\mathbb{ Z } / n \mathbb{ Z }$对加法就是一个循环群,其中$\sigma = 1 + n \mathbb{ Z }$.

特别地,容易见到对于任意循环群,如果$| G | = \infty$,则$G \cong \mathbb{ Z }$;反之则$G \cong \mathbb{ Z } / | G | \mathbb{ Z }$.

#### 相反群
群中的左右区别如此之大,是否说对于任何一个左的版本都存在一个与之对应的右的版本呢?

留神到我们其实可以定义**相反群**$G^{ op } = ( G , \odot )$,其中的运算满足$g \odot g ' = g ' g$,也就是在$G$中使得乘法倒置.容易检验其保留了幺元和逆元等性质,而且$( G^{ op } )^{ op } = G$.另外只需观察$G \to G^{ op } , g \mapsto g^{ - 1 }$就可构造双射.有时干脆将相反群记作$G^{ - 1 }$.

###### Example1
证明:群$G$是交换群当且仅当$x \mapsto x^{ - 1 }$是群同构.

显然是单射且是双射,只需证明是群同态即可.此时就需要证明$y^{ - 1 } x^{ - 1 } = ( xy )^{ - 1 } = ( yx )^{ - 1 }$,那就搞定了.

#### 陪集
设$G$是一个群,$H \leq G$,定义等价关系$a \sim b \Leftrightarrow \exists h \in H , a = bh$,容易检查其满足等价类的三条性质.不难发现对于元素$a$,其所在的等价类就是$aH$.类似可以定义$Ha$.我们定义形如$aH$的子集为$H$的一个**右陪集**,形如$Ha$的称作**左陪集**.由于左陪集是一个等价类,因此会将原集合划分为若干个等价类.这种划分用商集刻画,记作$G / H$.需要强调的是这个符号需要区分左右,有的人会盗用反斜杠来表述此关系,但这种符号容易与差集相混淆.因此,我们在下面会直接不加区分(或者说根据上下文判断)来一律统一使用$G / H$.如若的确需要区分,则采用$( G / H )_L , ( G / H )_R$的写法.

一个自然的想法是$G / H$是否构成一个群.先看取逆,容易发现$( aH )^{ - 1 } = Ha^{ - 1 }$,这给出左陪集和右陪集之间一个自然的同构:

$$
Hg \mapsto g^{ - 1 } H = ( Hg )^{ - 1 }
$$

所以不同的左右陪集个数相等.

我们将$H$的不同左陪集的个数(不一定有限)称为$H$在$G$中的**指数**,记为$| G : H |$,用消去律注意到$| H | = | aH | = | Ha |$,因此其实右陪集个数也是这个指数.既然是等价类,就完成了对原群的一个划分,而且容易见到每个划出的等价类大小都是$| H |$.

对于有限群,这引出重要的**拉格朗日定理**:

$$
| G | = | G : H | \times | H |
$$

对于每一个陪集$C \in G / H$,我们可以选定其一个代表元$x_c$使得$x_C H = C$.考虑映射$H \times ( G / H ) \to G , ( h , C ) \mapsto x_C h$.下面证明这的确是个同构.

由于这里在映射前就钦定了代表元(需要使用选择公理),所以无需验证良定性.

接下来检查其单射,由于$x_C h \in C$,所以如果$x_{ C ' } h ' = x_C h$,意味着$C ' \cap C \ne \emptyset$,这是不符合的,因为陪集肯定两两不交.

而满射性只需要找到自己所在的那个等价类即可.

该定理有以下推论(以下默认有限群):

1. 任何一个子群的大小都一定整除原群大小.

2. 若$H , K \subseteq G$,如果$| H | , | K |$的大小互素,则$H \cap K = \{ 1 \}$

3. $\forall \sigma \in G , o ( \sigma ) \mid ( | G | )$.

4. $\forall \sigma \in G , \sigma^{ | G | } = e$.

5. 如果$p = | G | \in \mathrm{ prime }$,则$G$是一个循环群.

6. $K \leq H \leq G$,则$[ G : K ] = [ G : H ] \times [ H : K ]$.

(1)显然.

(2)的原因是$H \cap K$一定是一个子群,则它的大小要整除$\gcd ( | H | , | K | ) = 1$.

(3)是因为$\langle \sigma \rangle \leq G$,而$o ( \sigma ) = | \langle \sigma \rangle |$.

(4)根据(3)显然.这其实就是费马小定理和欧拉定理在群上的体现.

(5)是因为任取一个$\sigma \ne 1$,则$1 \ne o ( \sigma ) | p$,所以$o ( \sigma ) = p$,这是循环群.

(6)的话仍是取代表元,不妨设$C \in G / H$的代表元是$x_C$,$D \in H / K$的代表元是$y_D$.则$G = \bigsqcup x_C H = \bigsqcup x_C y_D K$,容易检验两边是一一对应的.它的另一个证明策略是取道第一同构定理.

#### 群作用
我们早应看出群往往是由一类双射组成的集合,而并非应该将其看作单一运算的环.因此,考虑群在一个集合上的作用是很有意义的.

我们称群$G$的一个**左作用**是$\varphi : G \times X \to X$,使得:

1. $\varphi ( 1_G , x ) = x$.

2. $\varphi ( g , \varphi ( h , x ) ) = \varphi ( gh , x )$.

这种作用看似隔了一层,只是多引入了符号,而并非将$G$与$X$直接联系起来.然而回忆到柯里化过程,上述作用也等价于$G \to ( X \to X )$,设这个映射是$\Phi : G \to \mathrm{ End } ( X )$,容易见到上面两条性质等价于:

1. $\Phi ( 1_G ) = \text{ id }_X$.

2. $\Phi ( g_1 g_2 ) = \Phi ( g_1 ) \Phi ( g_2 )$.

容易见到$\Phi$是群同态.

当然同理可以定义**右作用**.

对于左作用,我们定义$x$的$G$**轨道**是$Gx = \{ gx | g \in G \} \subseteq X$.我们还可以定义其**稳定化子群**是$\mathrm{ Stab }_G ( x ) = \{ g \in G | gx = x \}$,容易检验这的确是一个子群,原因是封闭性和逆存在都满足,而且还满足$\mathrm{ Stab }_G ( gx ) = g \mathrm{ Stab }_G ( x ) g^{ - 1 }$.因此一个轨道中的点的稳定化子群大小相等.

定义$X$的$G$**不动点**是$X^G = \{ x \in X | \forall g \in G , gx = x \}$,容易见到$X^G$中的每一个点都自己构成一个轨道,还可以对单个$g$定义不动点集合$X^g = \{ x \in X | gx = x \}$.对于$\forall g \in G$.还可以引入以下定义:

1. 如果$\forall x \in X , gx = x$当且仅当$g = 1_G$,或说$\bigcap_{ x \in X } \mathrm{ Stab }_{ G } ( x ) = \{ 1_G \}$,则称这个作用是**忠实的**.此时还会满足$\Phi ( g ) = id_X \Leftrightarrow g = 1_G$,也就是上述柯里化是单射.

2. 若$X$只有一个轨道,换言之$\forall x , y \in X , \exists g \in G , y = gx$.则称此群作用**传递**.

3. 若$\forall x \in X$,$\mathrm{ Stab }_G ( x ) ={ 1_G }$,则称此群作用**自由**.自由性严格强于忠实性.

对于$x , y \in X$,我们定义等价关系$x \sim y$当且仅当$y \in Gx$.容易见到其满足传递性,自反性,对称性.在此等价关系下的商集也就是分解出的所有轨道,称此为轨道分解,或者盗用陪集符号写作$X / G$.

##### Cayley定理
由上面可以见到,任何一个$G$都可以用柯里化表为一个集合上的变换.并且如果群作用是忠实的,那么这种表示事实上总是单射.然而$G$作用于自己本身的时候因为消去律一定能导出自由性.这自然是一个置换.

Cayley定理的一个很重要的作用是理解轨道分解.我们早就用图论手段理解过置换可以拆解成轮换的性质.如今我们可以用群论的手段去理解.对于任何一个置换$\sigma$,考虑其作用在$X = \{ 1 , \cdots , n \}$上.考虑取一个循环群$G = \langle \sigma \rangle$,$G$作用在$X$上给出了$X$的一个轨道分解.容易见到这里的轨道分解就是给出了若干个轮换.由轨道分解的唯一性,立刻见到轮换分解的唯一性.

##### Burnside引理
下面选定$x \in X$考虑映射$G \to Gx , g \mapsto gx$.考虑取$H = \text{ Stab }_G ( x )$,现在考虑有:

$$
\begin{aligned}
gx & = g ' x \\
& \Leftrightarrow g^{ - 1 } g ' x = x \\
& \Leftrightarrow g^{ - 1 } g ' \in H \\
& \Leftrightarrow g ' \in gH \\
& \Leftrightarrow g ' H = gH \\

\end{aligned}
$$

所以我们可以搞一个同构映射$G / H \to Gx$.注意这里未必是群同构,原因是$H$未必是正规子群,但$G / H$这个右陪集总可以定义.对于有限集合的情况,回忆到$X$可以进行轨道分解,设其不同的轨道分别是$Gx_1 , \cdots , Gx_m$(回忆到$m = | X / G |$),并且对应的稳定化子群分别是$H_1 , \cdots , H_m$,上述命题给出:

$$
| X | = \sum | Gx_k | = \sum [ G : H_k ]
$$

另一个结论是$| X / G | \cdot | G | = \sum_{ g \in G } | X^g |$,这个结论是染色计数的基础.

为了证明上面这个结论,我们下面证明把$X$分解为若干轨道后,对于每个轨道$X_i$都有$\sum_{ g \in G } | X_i^g | = | G |$,由于轨道之间互不干扰,下面只需证明$X$是传递性的单个轨道的情况即可,回忆到单个轨道上的稳定化子群大小均相等,任取$x_0 \in X$,由于此时只着眼于单个轨道,因此根据上述结论$| X | = [ G : \mathrm{ Stab }_G ( x_0 ) ]$.此时:

$$
\begin{aligned}
\sum_{ g \in G } | X^g | & = \sum_{ x \in X } | \mathrm{ Stab }_G ( x ) | \\
& = | X | \cdot | \mathrm{ Stab }_G ( x_0 ) | \\
& = [ G : \mathrm{ Stab }_G ( x_0 ) ] \cdot | \mathrm{ Stab }_G ( x_0 ) | \\
& = | G |
\end{aligned}
$$

这就搞定.

##### $p$-群
设$p \in \mathrm{ prime }$,如果$| G | = p^m$(当$m = 0$的时候称其平凡),则称$G$为$p$-**群**.对此我们有以下结论:

1. $| X | \equiv | X^G | \pmod{ p }$.

2. 如果$| G |$非平凡,则$Z_G \ne \{ 1 \}$.进一步地,$p \mid ( | Z_G | )$.

(1)的原因是轨道分解给出:

$$
| X | = | X^G | + \sum_{ H_i \ne G } [ G : H_i ]
$$

然而拉格朗日定理给出$| H_i |$总是$| G |$的因子,也就是说$\frac{ | G | }{ | H_i | } = p^k$,于是证毕.

(2)的证明,考虑映射$a : G \times G \to G : a ( g , x ) = Ad_g ( x ) = gxg^{ - 1 }$,这本身是$G$对自身的一个左作用.而容易见到此作用下的不动点集就是$Z_G$.而$| Z_G | \equiv | G | \equiv 0 \pmod{ p }$,于是证毕.

我们知道轨道分解事实上给出了一种划分集合的策略,而这里的映射其实也给出了一种划分集合的策略:按**共轭类**划分.换言之,我们设$a \sim b$当且仅当$\exists g \in G , a = gbg^{ - 1 }$,容易见到其满足等价关系的三条性质.因此我们得到一个共轭类的大小总是整除群的大小,以及:

$$
| G | = | Z_G | + \sum_{ H_i \ne G } [ G : H_i ]
$$

这个被称为**类方程**.

###### Example1
求证:阶为$p^2$的有限群都是交换的.

考虑其中心$Z_G$,由于该群非平凡,所以$| Z_G | > 1$,所以$| Z_G | = p$或$| Z_G | = p^2$,后者显然已经证毕.

如果能取出一个元素$c$使得$\langle c \rangle = G$则显然证毕,下面假设所有的非平凡循环子群的阶都是$p$.

取$a \in Z_G , a \ne 1$,则$Z_G = \langle a \rangle$.任取$b \in G \setminus Z_G$,考虑$\langle b \rangle \cap Z_G = \{ 1 \}$,原因是如果$\exists i , j , a^i = b^j$,则取$j$在$\bmod p$下的逆元$k$,有$a^{ ik } = b$,这就不符.

于是有$G / Z_G \cong \langle b \rangle$,立刻证毕.

#### 正规子群
设$G$是一个群,$H \leq G$,如果$\forall a \in G , aH = Ha$或回忆到商群的概念只需检验$a^{ - 1 } Ha \subseteq H$,我们称$H$是$G$的**正规子群**,记作$H \trianglelefteq G$.当一个子群是正规的时候,它的左右陪集不必区分.

显然,$G$和$\{ e \}$都是$G$的正规子群,如果$G$除此之外没有别的正规子群,称其为**单群**.

容易发现$Z_G \trianglelefteq G$.此外,如果$N \trianglelefteq G$,而且$H \leq G$,则$N \cap H \trianglelefteq H$,只需套用定义即可发现.

于此还可以引入两个定义,假设$K \leq G$:

1. 定义$K$的**中心化子**:$Z_G ( K ) = \{ g \in G | \forall k \in K , gkg^{ - 1 } = k \}$.

2. 定义$K$的**正规化子**:$N_G ( K ) = \{ g \in G | gKg^{ - 1 } = K \}$.

显然$K \trianglelefteq N_G ( K ) \geq Z_G ( K )$.

##### 正规子群的判定定理
当$H \leq G$时,下列命题等价:

1. $H \trianglelefteq G$.

2. $\forall a , b \in G , \exists c \in G , ( aH ) ( bH ) = cH$.

3. $\forall a \in G , a^{ - 1 } Ha = H$.

4. $\forall a \in G , h \in H , a^{ - 1 } ha \in H$.

先证明(2)$\Rightarrow$(1):

由于$a^2 H \subseteq ( aH )^2$,$a^2 \subseteq ( aH )^2$并且$( aH )^2 = cH$是左陪集,那么由于左陪集是等价类,所以$a^2 H = ( aH )^2$.两边同时乘以$a^{ - 1 }$,有$Ha \subseteq HaH = aH$.再次两边左右乘以$a^{ - 1 }$,得到$a^{ - 1 } H \subseteq Ha^{ - 1 } , \forall a \in G$,也就证毕.

再证明(1)$\Rightarrow$(2):

$$
( aH ) ( bH ) = aHbH = a ( bH ) H = abH
$$

这里还可以进一步说明如果这里干脆在正规子群的陪集上定义二元运算$( aH , bH ) \to abH$后其陪集构成群.首先我们要检验良定性质.注意到如果$a ' = au , b ' = bv$,其中$u , v \in H$,则:

$$
a ' b ' H = aubvH = abb^{ - 1 } ubvH = abH
$$

原因是$b^{ - 1 } ub \in H , v \in H$.

逆元只需要对$( aH )^{ - 1 } = a^{ - 1 } H$即可.至于结合律也可以用上述性质转化,这样就搞定了.

剩下的部分,(1)$\Leftrightarrow$(3),(3)$\Rightarrow$(4)都显然,考虑证明(4)$\Rightarrow$(3).

(4)显然等价于$ha \in aH$,于是有$Ha \subseteq aH$.而又等价于$a^{ - 1 } h \in Ha^{ - 1 }$,于是类似证毕.

设$G$是群,$H \trianglelefteq G$,则$H$的陪集在乘法下构成群,这个群称为$G$关于$H$的**商群**,记作$G / H$.

考虑封闭律:$( aH ) ( bH ) = a ( bH ) H = abH$,又显然满足结合律.又有$\forall aH \in G / H , eH ( aH ) = ( aH ) H = aH$,于是$H$是幺元.同样知道$a^{ - 1 } H = ( aH )^{ - 1 }$,于是存在逆元.

###### Example1
证明:如果$[ G : H ] = 2$的话则一定有$H \trianglelefteq G$.

原因是其可以分解成两个陪集,而其中一个陪集自动是$H$本身,所以另一个陪集也同样不分左右.

###### Example2
求证:如果$G / Z_G$是循环群,则$G$交换.

不妨假设$G / Z_G = \{ Z_G , aZ_G , a^2 Z_G , \cdots \}$.

对于任意$b , c \in G$,都存在一组$i , j , g_1 , g_2$满足$i , j \in \mathbb{ Z }$,$g_1 , g_2 \in Z_G$有$b = a^i g_1 , c = a^j g_2$,显然$bc = cb$.

##### 正规子群与同态
事实上有以下命题:

1. $\text{ im } \ \varphi \leq G_1$.

2. $\text{ ker } \ \varphi \trianglelefteq G$.

先看(1),考虑首先$e_1 \in \text{ im } \ \varphi$,所以其不为空.其次对于任意$a_1 , b_1 \in \text{ im } \ \varphi$,$\exists a , b \in G$,使得$\varphi ( a ) = a_1 , \varphi ( b ) = b_1$.于是:

$$
a_1 b_1^{ - 1 } = \varphi ( a ) \varphi ( b^{ - 1 } ) = \varphi ( ab^{ - 1 } ) \in \text{ im } \ \varphi
$$

再看(2),同上可知$\text{ ker } \ \varphi \ne \emptyset$,对于任意的$a , b \in \text{ ker } \ \varphi$,$\varphi ( ab^{ - 1 } ) = \varphi ( a ) \varphi ( b )^{ - 1 } = e_1$,所以$ab^{ - 1 } \in \text{ ker } \ \varphi$,说明$\text{ ker } \ \varphi \leq G$.而$\forall g \in G$,$\varphi ( gag^{ - 1 } ) = \varphi ( g ) e_1 \varphi ( g )^{ - 1 } = e_1$,所以$gag^{ - 1 } \in \text{ ker } \ \varphi$.

我们也可以说正规子群均可以成为某个映射的$\ker$,方法是构造映射$\pi : G \to G / N , g \mapsto gN$.容易检验这的确是一个群同态.

另外如果$f$是同构,那么$f^{ - 1 }$也是同态,原因是:

$$
\begin{aligned}
f ( f^{ - 1 } ( x ) f^{ - 1 } ( y ) ) & = f ( f^{ - 1 } ( x ) ) f ( f^{ - 1 } ( y ) ) = xy = f ( f^{ - 1 } ( xy ) ) \\
f^{ - 1 } ( x ) f^{ - 1 } ( y ) & = f^{ - 1 } ( xy )
\end{aligned}
$$

以及如果$f , g$都是同态,那么$f \circ g$也是同态.

更进一步地,我们实际上可以在这里画交换图表,对于任何一个群同态$f : G \to G '$,取$\pi : G \to G / \ker f$,我们存在唯一的$\bar{ f }$使得$f = \bar{ f } \circ \pi$,或言使得下述交换图表成立:

$$
\xymatrix{ G \ar[r]^f \ar[d]_\pi & G ' \\
G / \ker f \ar[ru]_{ \bar{ f } } & \\
 }
$$

这被称为商群的泛性质.事实上这也很一眼,原因是要满足$\forall x \in G , \bar{ f } ( xN ) = f ( x )$,这其实也就定义了$\bar{ f }$,而存在性只需要堆堆语言就行.

进一步地,如果$f : G \to G '$,而$N \trianglelefteq G , N ' \trianglelefteq G '$,并且$f ( N ) \subseteq N '$,则存在唯一的映射$\bar{ f } : G / N \to G ' / N '$使得我们可以搞定下面这个图表的交换性:

$$
\xymatrix{ G \ar[r]^f \ar[d] & G ' \ar[d] \\
G / N \ar[r]_{ \bar{ f } } & G ' / N ' }
$$

##### 同态基本定理
设$\varphi : G \rightarrow G_1$是群同态,则$G / \text{ ker } \ \varphi \cong \text{ im } \ \varphi$.

不妨先设$H = \text{ ker } \ \varphi$.我们定义映射$\psi : G / H \rightarrow \text{ im } \ \varphi$,满足$\psi ( aH ) = \varphi ( a )$.

不过首先要判断$\psi$是不是良定义,也就是当$aH = bH$的时候,我们要判断$\varphi ( a )$是否等于$\varphi ( b )$.若$aH = bH$,说明$\exists h \in H , a = bh$,那么$\psi ( aH ) = \varphi ( a ) = \varphi ( bh ) = \varphi ( b ) \varphi ( h ) = \varphi ( b )$.

然后还要证明其是群同态,注意到$\psi ( ( aH ) ( bH ) ) = \psi ( abH ) = \varphi ( ab ) = \varphi ( a ) \varphi ( b ) = \psi ( aH ) \psi ( bH )$.

接下来要证明它是单射,显然只有$\psi ( H ) = e_1$,$\text{ ker } \ \psi = \{ H \}$.

接下来要证明它是满射,$\forall g \in \text{ im } \ \varphi$,$\exists a , \varphi ( a ) = g$,则$\psi ( aH ) = g$.

这样就证明了同态基本定理.

当$\varphi$是满射的时候$G_1 = \text{ im } \ \varphi$,所以此时有$G / \text{ ker } \ \varphi \cong G_1$.

##### 第一同构定理
我们定义$H \trianglelefteq G$时,定义$\pi : G \rightarrow G / H , a \mapsto aH$.显然其是群同态,定义其为由$G$到$G / H$的**典范同态**.

那么我们有第一同构定理,即对于一个$G$的一个正规子群$H$,在典范同态$\pi : G \rightarrow G / H$下,我们有:

1. $G$的包含$H$的子群和$G / H$的子群在$\pi$下是群同构.

2. 在此对应下,正规子群对应于正规子群.

3. 若有$H \trianglelefteq K \trianglelefteq G$,则$G / K \cong ( G / H ) / ( K / H )$.

如若考虑满同态的情况,我们有另一种形式的表达.设$f : G \rightarrow G '$是群的满同态,根据同态基本定理有$G / \ker f \cong G '$,立刻有:

1. $G$的包含$\ker f$的子群和$G '$的子群在$f$下一一对应.

2. 在此对应下,正规子群对应正规子群.

3. 若有$\ker f \leq H \trianglelefteq G$,则$G / H \cong ( G / \ker f ) / ( H / \ker f ) \cong G ' / H '$.

此形式与一开始的形式无太大差别,因为任何一个正规子群总对应某个映射的$\ker$.

然而这个证明要比原本的大量堆砌要更为漂亮.我们断言这种一一对应恰好由$f$给出.更进一步地,对于$\ker f \leq H \leq G$,其对应了一个$H ' \leq G '$,满足:$f ( H ) = H '$,而且$f^{ - 1 } ( H ' ) = H$.

先来证明我们找到是一个双射,那就需要对每个$H$证明其对应的那个$H '$确实是对应自己的,反之亦然:

首先我们要证明$H = f^{ - 1 } ( f ( H ) )$.显然有$H \subseteq f^{ - 1 } ( f ( H ) )$,下面证明$f^{ - 1 } ( f ( H ) ) \subseteq H$.也就是要证明$\forall f ( x ) \in f ( H )$,也就是$f ( x ) = f ( h ) , x \in G , h \in H$,都一定有$x \in H$.

可如若$f ( x ) = f ( h )$就一定有$xh^{ - 1 } \in \ker f \subseteq H$,这就搞定了$x \in H$.

反过来的$H ' = f ( f^{ - 1 } ( H ' ) )$,回忆到$f$是满射,其总有右逆,因此这是显然成立的.这就搞定了一一对应的关系,而且顺便还给出如果$H_1 \subseteq H_2$,则$H_1 ' = f ( H_1 ) \subseteq H_2 ' = f ( H_2 )$.至于$H '$的正规性,只需要回忆到$\forall \bar{ x } \in G '$,$\exists x \in G , \bar{ x } = f ( x )$.这就给出$\bar{ x } f ( H ) ( \bar{ x } )^{ - 1 } = f ( xHx^{ - 1 } ) = f ( H )$.

最后考虑映射$G \to G ' / H ' , g \mapsto f ( g ) H '$,它的$\ker = f^{ - 1 } ( H ' ) = H$,所以同态基本定理给出$G / H \cong G ' / H '$,这就搞定.

总之,第一同构定理给出的是"商"这个操作究竟对一个群有了怎样的影响,其实就是切片了对吧,然后把每一片缩起来.有下述交换图表:

$$
\xymatrix{ G \ar[r]^f \ar[d] & G ' \ar[d] \\
G / H \ar[r] & G ' / H ' }
$$

##### 第二同构定理
设$G$是群,$H \trianglelefteq G , K \leq G$,则:

1. $H \cap K \trianglelefteq K$.$HK = KH \leq G$.

2. $( HK ) / H \cong K / ( H \cap K )$.

考虑(1)的证明:

先看前半部分,只需证明$\forall k \in K , w \in H \cap K$,$kwk^{ - 1 } \in H \cap K$.

注意到$k , w \in K$,所以$kwk^{ - 1 } \in K$;又注意到$H \trianglelefteq G$,所以$kwk^{ - 1 } \in H$,这就证明了该结论.

对于后半部分先证明$HK = KH$,考虑$hk \in Hk = kH \subseteq KH$.

再证$HK$的确是一个子群,我们早在子群判定引理处就证明了此结论.

再看(2)的证明:

考虑构造$\varphi : K \to ( HK ) / H , k \mapsto kH$.这显然是良定义的群同态.

考虑$\varphi ( k ) = H \Leftrightarrow k \in H \Leftrightarrow k \in K \cap H$,于是$\ker \varphi = K \cap H$.

接下来还需要解释其满性,考虑$\forall hkH = H ( hk ) = Hk = kH$,这就证明了其满性,于是上述命题自然成立.

根据第二同构定理还可以立刻得到:$H , K \leq G$,$G$是有限群,$| HK | = \frac{ | H | | K | }{ | H \cap K | }$.

另外容易见到,其实只要$H \leq N_G ( K )$,上述结论依然过得去,因为只需要把群限制在$N_G ( K )$上即可.

##### 交换化
如果群$G$的子群$H$对于所有自同构$\varphi : G \cong G$都满足$\varphi ( H ) = H$,则称$H$是一个**特征子群**.取$\varphi = \mathrm{ Ad }$可以见到特征子群总是正规子群.

还可以见到群的中心总是特征子群,原因是任取$z \in Z_G , \forall g \in G$,都满足:

$$
\begin{aligned}
zg & = gz \\
\varphi ( z ) \varphi ( g ) & = \varphi ( g ) \varphi ( z )
\end{aligned}
$$

由于$g$可以取遍$G$,所以$\varphi ( g )$可以取遍$G$,所以$\varphi ( z ) \in Z_G$.

我们还可以定义群的**导出子群**$G_{ der } = \langle aba^{ - 1 } b^{ - 1 } | a , b \in G \rangle$也是特征子群.原因是$\varphi ( aba^{ - 1 } b^{ - 1 } ) = \varphi ( a ) \varphi ( b ) \varphi ( a )^{ - 1 } \varphi ( b )^{ - 1 }$.

既如此,考虑商群$G_{ ab } = G / G_{ der }$.留意到$\forall g_1 , g_2 \in G_{ ab }$:

$$
\begin{aligned}
g_1 g_2 G_{ der } & = g_2 g_1 G_{ der } \\
& \Leftrightarrow g_1^{ - 1 } g_2^{ - 1 } g_1 g_2 G_{ der } = G_{ der } \\
& \Leftrightarrow g_1^{ - 1 } g_2^{ - 1 } g_1 g_2 \in G_{ der } \\

\end{aligned}
$$

所以见到此商群交换,我们将此操作称为**群的交换化**.

我们可能还想说明这样的交换化是最为合理的,我们可以证明如果存在一个同态$f : G \to A$,其中$A$是一个交换群.则存在唯一的$\bar{ f } : G_{ ab } \to A$,使下列图表交换:

$$
\xymatrix{ G \ar[d] \ar[r]^f & A \\
G_{ ab } \ar[ru]_{ \bar{ f } } & }
$$

首先注意到必须满足$\bar{ f } ( gG_{ der } ) = f ( g )$,这就给出唯一性,其次要给出存在性,就要证明如果$g_1 G_{ der } = g_2 G_{ der }$则$f ( g_1 ) = f ( g_2 )$.只需检验$G_{ der } \subseteq \ker f$就行,然而由于$f ( aba^{ - 1 } b^{ - 1 } ) = e_A$,原因是$A$的交换性,因此这就给定了.

###### Example1
证明$GL ( n , F )_{ der } = SL ( n , F )$,其中$F$是含有至少三个元素的域.

显然$GL ( n , F )_{ der } \subseteq SL ( n , F )$,只需要证明上述导出子群的确生成了$SL ( n , F )$即可.

考虑所有的矩阵都可以通过若干次初等行列变换变到对角线全$1$的矩阵,只需要把这些初等行列变换全都拿到即可,略去不谈.

此时也可以见到$GL ( n , F ) / SL ( n , F ) = F$,其实也就是行列式.

###### Example2(岩泽健吉(Iwasawa)判准)
设群$G$作用在集合$X$上,$| X | \geq 2$,若$\forall ( x , y ) , ( x ' , y ' ) \in X^2$,其中$x \ne y , x ' \ne y '$,都存在$g \in G$使得$gx = x ' , gy = y '$,则称该作用是**双传递的**.下面我们设$G$双传递于$X$,并且记$H_x = \mathrm{ Stab }_G ( x )$.

先证明一个引理:$\forall H_x$都是$G$的极大真子群.

为证明此,只需要证明任意往里面多添一个$g \in G \setminus H_x$,都会使得这个群扩充到$G$本身.一个想法是证明$G = H_x \cup H_x g H_x$.

既然如此,考虑只要证$\forall g_1 \in G \setminus H_x , \exists h_1 , h_2 \in H_x$使得$g_1 = h_1 gh_2$即可,也就是$\exists h_1$使得$g_1^{ - 1 } h_1 g \in H_x \Leftrightarrow g_1^{ - 1 } h_1 g ( x ) = x \Leftrightarrow g ( x ) = h_1^{ - 1 } g_1 ( x )$.

考虑到$g ( x ) \ne x , g_1 ( x ) \ne x$.由双传递性一定存在一个$h_1^{ - 1 }$把$( x , g_1 ( x ) ) \to ( x , g ( x ) )$,这就搞定了.

接下来再证明一个引理:任何正规子群$N \trianglelefteq G$在$X$上的作用要么是平凡的,要么是传递的.

若$N$的作用不是平凡的,也就是存在$x , n$使得$nx \ne x$,那我们下面证明对于任意$y \ne y '$,都可以找到一个元素$m \in N$使得$my = y '$.原因是可以取$g : ( x , nx ) \mapsto ( y , y ' )$,于是$gx = y , gnx = y ' , gng^{ - 1 } y = y '$,由于$N$正规知道$gng^{ - 1 } \in N$,这就搞定了.

接下来证明岩泽判准,假设$G$作用忠实,$G = G_{ der }$,而且存在$x$使得$H_x$有正规交换子群$U$,而$U$在$G$中的所有共轭生成$G$.我们下面证明$G$是单群.

取定$N \trianglelefteq G$和$x \in X$,由于$H_x$是极大真子群,所以要么$NH_x = H_x$,要么$NH_x = G$.

当$NH_x = H_x , N \subseteq H_x$的时候,此时$N$作用并非传递,所以$N$作用平凡.由于作用是忠实的,作用平凡的元素也就只有$\{ id \}$.

当$NH_x = G$的时候,我们先证明$NU \trianglelefteq NH_x = G$.首先第二同构定理的部分已经给出了$NU \leq NH_x$,现在考虑$\forall nu \in NU , n_1 h_1 \in NH_x$,下面证明$n_1 h_1 nuh_1^{ - 1 } n_1^{ - 1 } \in NU$.

而考虑:

$$
\begin{aligned}
n_1 h_1 nuh_1^{ - 1 } n_1^{ - 1 } \\
= n_1 ( h_1 nh_1^{ - 1 } ) ( h_1 uh_1^{ - 1 } ) n_1^{ - 1 } \\
\in NUN = NU
\end{aligned}
$$

此时由于$U \leq NU$,而后者正规,前者的所有共轭生成$G$,这必然意味着$NU = G$,第二同态定理给出$G / N \cong U / ( N \cap U )$而后者交换,这给出了一种$G$向交换群的同态,我们知道此时必然有$G_{ ab }$上也有一个向交换群的同态,然而$G_{ ab }$是平凡群,因此$G / N$必然也是平凡群,意味着$N = G$.

###### Example3
设$F$为域且$| F | \geq 4$,记$Z$是$SL ( n , F )$的中心,下面我们证明当$n \geq 2$的时候,$PSL ( 2 , F ) = SL ( 2 , F ) / Z$是单群.

我们想要用岩泽健吉判准,首先要检验$SL ( n , F ) = SL ( n , F )_{ der }$,这个与之前的做法几乎类似,只需要简单说明.

接下来设$\mathbb{ P }^{ n - 1 } ( F )$为$F^n$的$1$维子空间,让$PSL ( n , F )$作用在上面,留意到中心的作用也的确是平凡的.

当$n = 2$的时候,记录$( x , y ) \in F^2 \setminus \{ 0 \}$生成的空间为$( x : y )$,我们来证明它是双传递的,事实上这较为显然.

接下来我们来看$( 1 : 0 )$上的稳定化子群$H$,其实也就是所有的上三角矩阵,然后还要验证作用是忠实的,总之请检验以上事实.然后取$U = \{ \begin{bmatrix}1 & \_ \\ 0 & 1\end{bmatrix} Z \}$.

当$n \geq 3$和上面是同理的,对于$( 1 : 0 \cdots : 0 )$的稳定化子群取分块上三角矩阵$\begin{bmatrix}a & \_ \\ & A\end{bmatrix} Z$,然后取$U = \{ \begin{bmatrix}1 & \_ \\ 0 & I\end{bmatrix} Z \}$.

#### 群的直和与直积
##### 直和
在两个群$G_1 , G_2$的笛卡尔积$G_1 \times G_2$集合上定义二元运算,满足$( a_1 , b_1 ) ( a_2 , b_2 ) = ( a_1 a_2 , b_1 b_2 )$,这个集合和这个二元运算显然组成群,称这个群为$G_1$和$G_2$的**外直和**,记为$G_1 \oplus G_2$,$G_1 , G_2$称作其的**直和因子**.如果要对无限个群做类似操作,则可以区分出**直积**的版本,仍是因为是否有限个分量非零的问题.

显然$G_1 \oplus G_2$中存在两个正规子群$\bar{ G_1 } = \{ ( a , e_2 ) | a \in G_1 \} , \bar{ G_2 } = \{ ( e_1 , b ) | b \in G_2 \}$,并且不难发现$G_1 \oplus G_2 = \bar{ G_1 } \bar{ G_2 }$.

这给出了一种分解群的策略,事实上,设$G$是群,$H , K \trianglelefteq G , G = HK$,我们有以下命题互推:

1. 映射$\sigma : H \oplus K \rightarrow G , ( h , k ) \mapsto hk$是同构.

2. $G$的任一元素表示为$H , K$的乘积的表示法唯一.

3. $G$的幺元表示为$H , K$的元素的乘积的表示法唯一.

4. $H \cap K = \{ e \}$.

显然$( 1 ) \Rightarrow ( 2 ) \Rightarrow ( 3 )$.对于$( 3 ) \Rightarrow ( 4 )$,如果$\exists g \in H \cap K , g \ne e$,则$e = gg^{ - 1 } = ee$,这就给出了两种表示法,不符.

只需证明$( 4 ) \Rightarrow ( 1 )$.根据我们的经验,先证良定义(显然),再证群同态,再证单射,再证满射(显然).

我们先干个事,我们断言$\forall h \in H , k \in K , hk = kh$.

这个怎么断言呢?我们考虑由于$H \trianglelefteq G$,有$k^{ - 1 } hk \in H$,所以$h^{ - 1 } k^{ - 1 } hk \in H$,同理可证明$h^{ - 1 } k^{ - 1 } hk \in K$,于是$h^{ - 1 } k^{ - 1 } hk = e$,$kh = hk$.由此显然证明是群同态.

接下来只需要证明单射,只需要证明$\ker \sigma = \{ e , e \}$.不妨设$( h , k ) \in \ker \sigma$,则$hk = e$,$h = k^{ - 1 } \in H \cap K = e$.所以$\ker \sigma = \{ e , e \}$.

我们也将$G$称作$H$和$K$的**内直和**,将$H , K$称作其**直和因子**.

上面的概念还可以扩展,设$G$是群,$H_1 , \cdots , H_n \trianglelefteq G , G = H_1 \cdots H_n$,我们有以下命题互推:

1. 映射$\sigma : H_1 \oplus \cdots \oplus H_n \rightarrow G , ( h_1 , \cdots , h_n ) \mapsto h_1 \cdots h_n$是同构.

2. $G$的任一元素表示为$H_1 , \cdots , H_n$的乘积的表示法唯一.

3. $G$的幺元表示为$H_1 , \cdots , H_n$的元素的乘积的表示法唯一.

4. $H_i \cap ( H_1 \cdots H_{ i - 1 } H_{ i + 1 } \cdots H_n ) = \{ e \}$.

##### 半直积
取$H , N$是给定的两个任意的群,以及给定的群同态$\varphi : H \to \mathrm{ Aut } ( N ) , h \mapsto \varphi_h$.

接下来在$N \times H$上定义一种二元运算,定义为$( n , h ) ( n ' , h ' ) = ( n \varphi_h ( n ' ) , hh ' )$.我们下面证明这种运算的确使得$N \times H$成为群,记作$N \rtimes_\varphi H$.见到:

1. 幺元存在:$1_{ N \times H } = ( 1_N , 1_H )$.

2. 逆元存在:$( n , h )^{ - 1 } = ( \varphi_{ h^{ - 1 } } ( n^{ - 1 } ) , h^{ - 1 } )$.

结合律的验证比较多,考虑:

$$
\begin{aligned}
& ( n_1 , h_1 ) ( n_2 , h_2 ) ( n_3 , h_3 ) \\
= & ( n_1 \varphi_{ h_1 } ( n_2 ) \varphi_{ h_1 h_2 } ( n_3 ) , h_1 h_2 h_3 )
\end{aligned}
$$

而:

$$
\begin{aligned}
& ( n_1 , h_1 ) ( ( n_2 , h_2 ) ( n_3 , h_3 ) ) \\
= & ( n_1 , h_1 ) ( n_2 \varphi_{ h_2 } ( n_3 ) , h_2 h_3 ) \\
= & ( n_1 \varphi_{ h_1 } ( n_2 \varphi_{ h_2 } ( n_3 ) ) , h_1 h_2 h_3 )
\end{aligned}
$$

于是我们要证明的就是$\varphi_{ h_1 } ( n_2 ) \varphi_{ h_1 h_2 } ( n_3 ) = \varphi_{ h_1 } ( n_2 \varphi_{ h_2 } ( n_3 )$.由于$\varphi$是同态,里面的乘积可以拆出,只需证明$\varphi_{ h_1 h_2 } = \varphi_{ h_1 } \circ \varphi_{ h_2 }$,然而这恰是同态$\varphi : H \to \mathrm{ Aut } ( N )$本身的性质,这就搞定.

接下来,我们考虑搞两个嵌入映射,考虑$N \to N \times H , n \mapsto ( n , 1_H )$.同理定义$H \to N \times H , h \mapsto ( 1_N , h )$.在这种嵌入下见到$( n , h ) = ( n , 1_H ) ( 1_N , h )$,可以发现$N \trianglelefteq N \times H$.原因是:

$$
\begin{aligned}
& ( n_1 , h_1 ) ( n , 1_H ) ( n_1 , h_1 )^{ - 1 } \\
= & ( n_1 , 1_H ) ( 1_N , h_1 ) ( n , 1_H ) ( 1_N , h_1 )^{ - 1 } ( n_1 , 1_H )^{ - 1 }
\end{aligned}
$$

而考虑:

$$
\begin{aligned}
& ( 1_N , h ) ( n , 1_H ) ( 1_N , h )^{ - 1 } \\
= & ( 1_N , h ) ( n , 1_H ) ( 1_N , h^{ - 1 } ) \\
= & ( \varphi_h ( n ) , h ) ( 1_N , h^{ - 1 } ) \\
= & ( \varphi_h ( n ) , 1_H )
\end{aligned}
$$

这就搞定了.

上述操作看上去一团迷雾,我们想要说明半直积定义的动机:其实就是想让$N , H$嵌入同一个群中,并且使得$N$是这个群的一个正规子群.

于是我们的目标终于明晰:假设$H \leq G , N \trianglelefteq G , H \cap N = \{ 1 \} , G = NH = HN$,考虑由$\mathrm{ Ad }_h ( n ) = hnh^{ - 1 }$给出的映射,下面我们要证明存在同构$\Phi : N \rtimes_\mathrm{ Ad } H \cong G , ( n , h ) \mapsto nh$.

首先验证这的确是个同态,考虑:

$$
\begin{aligned}
& \Phi ( ( n , h ) ( n ' , h ' ) ) \\
= & \Phi ( nhn ' h^{ - 1 } , hh ' ) \\
= & nhn ' h^{ - 1 } hh ' \\
= & nhn ' h ' = \Phi ( n , h ) \Phi ( n ' , h ' )
\end{aligned}
$$

满性显然,单性的话考虑$\Phi ( n , h ) = 1 \Leftrightarrow nh = 1 \Leftrightarrow n = 1 , h = 1$,这就搞定.

上述构造舍弃了$\varphi$,因此比起之前的版本,这个更像是一种内的版本.事实上上述也有第二同构定理给出$G / N \cong H$,所以你可以将商看作半直积的逆运算,然而反之不可.例如$( \mathbb{ Z } / 4 \mathbb{ Z } ) / ( 2 \mathbb{ Z } / 4 \mathbb{ Z } ) \cong ( \mathbb{ Z } / 2 \mathbb{ Z } )$,注意到$( 2 \mathbb{ Z } / 4 \mathbb{ Z } ) \cong ( \mathbb{ Z } / 2 \mathbb{ Z } )$,但$\mathbb{ Z } / 4 \mathbb{ Z }$和$( \mathbb{ Z } / 2 \mathbb{ Z } )^2$并不同构,前者是循环群后者是克莱因四元群.

半直积的作用要么是通过两个群得到一个新的群,要么反之将一个群拆分为两个不同的群.

###### Example1
一个例子是将置换群$S_n$拆成偶置换群$A_n$和任何一个单置换$\tau = ( i , j )$的半直积,有:$S_n = A_n \rtimes \langle \tau \rangle$.

然而当$n > 2$的时候,见到永远不可能出现$S_n = A_n \times \langle \tau \rangle$的情形,证明可以考虑左侧的中心只有$id$而右侧至少有$( 1 , 1 )$和$1 , \tau$两个.

###### Example2(二面体群)
回忆到$O ( 2 )$为$\mathbb{ R }^2$上的正交变换,上面有一个正规子群$SO ( 2 )$为$\det = 1$的部分.

设$n \geq 3$,取平面上的一个中心为$( 0 , 0 )$的正$n$边形,适当缩放后可以使得顶点分别为$\omega_n^k = e^{ \frac{ 2 k \pi i }{ n } }$,设$D_{ 2 n }$为所有使得正$n$边形不变的正交变换,容易见到$D_{ 2 n } \leq O ( 2 )$.

接下来看$D_{ 2 n }$中的两种元素:

1. 旋转:那肯定要将一个角挪到另一个角上,因此等价于复平面上的$\omega_n^k$,记$\sigma = \omega_n$则旋转群也就是$\langle \sigma \rangle \cong \mathbb{ Z } / n \mathbb{ Z }$.

2. 镜射:任取一个$\tau$使得图形沿某条轴翻转.容易见到$\langle \tau \rangle \cong \mathbb{ Z } / 2 \mathbb{ Z }$.

下面我们说明$D_{ 2 n } \cong \langle \sigma \rangle \rtimes \langle \tau \rangle \cong \mathbb{ Z } / n \mathbb{ Z } \rtimes_\varphi \mathbb{ Z } / 2 \mathbb{ Z }$.其中$\varphi_{ 1 + 2 \mathbb{ Z } } ( a + n \mathbb{ Z } ) = - a + n \mathbb{ Z }$.

首先见到$D_{ 2 n } \cap SO ( 2 ) = \langle \sigma \rangle$,原因显然.这样两边取交就可以说明$\langle \sigma \rangle \trianglelefteq D_{ 2 n }$.

此外容易见到$\langle \sigma \rangle \cap \langle \tau \rangle = \{ 1 \}$.其余性质也容易说明,唯一可能稍不显然的是为何$\langle \sigma \rangle \rtimes \langle \tau \rangle$的确生成了整个群.

策略是转回去!假设这个变换将$0$号顶点搞到了某个位置,你可以把它转回去,于是此时图形保持了$0$号顶点不动,此时要么就是恒等,要么就是按照实轴翻转,这样就搞定了.

### 环
定义一个**环**是一个集合$R$和两种二元运算$+ , \times$.它应当满足如下性质:

1. 集合对两种运算封闭.

2. $( R , + )$构成阿贝尔群,加法幺元一般记作$0_R$,$x$的加法逆元一般记作$- x$.

6. 乘法有结合律.

8. 乘法对加法有分配律.

8. (幺环存在乘法幺元,一般记作$1_R$)

9. (交换环的乘法具有交换律)

最平凡的环只包含一个$0$元素,被称为**零环**.

下面证明一些环的基础性质:

1. $\forall a \in R , a 0 = 0 a = 0$.

注意到$a 0 = a ( 0 + 0 ) = a 0 + a 0$,所以$a 0 = 0$.

2. $\forall a , b \in R , ( - a ) b = a ( - b ) = - ( ab )$.

注意到$( - a ) b + ab = b ( a - a ) = 0$,所以$- ( ab ) = ( - a ) b$.

3. $\forall a , b \in R , ( - a ) ( - b ) = ab$.

由(2)是显然的.

注意到环的定义中并没有提及乘法逆元.如果环$R$是幺环,且对于某一个$a \in R$,$a$有逆元(或者既有左逆元又有右逆元,类似群,左右逆元必相等且唯一),那么称其为**可逆元**或**单位元**,有时又称其为unit.

容易验证幺环的可逆元的全体构成乘法群,记作$R^\times$.

对于一个元素$a \in R$来说,如果$\exists b \in R \setminus \{ 0 \}$,$ab = 0$,我们称$a$是$R$中的一个**左零因子**,同理可定义**右零因子**,如果一个$a$既是左零因子又是右零因子,我们称其为一个**零因子**.

我们事实上可以说明这里的加法和乘法与我们平时使用的相当类似,换言之,我们可以说明对于任意环$R$而言,单同态$\varphi : \mathbb{ Z } \rightarrow R$唯一.

首先我们应该满足$\varphi ( 1 ) = 1_R , \varphi ( 0 ) = 0_R$.

其次我们应当有$\varphi ( n ) = \varphi ( 1 + 1 + \cdots + 1 ) = \varphi ( 1 ) + \varphi ( 1 ) + \cdots + \varphi ( 1 )$.我们不妨把后者简写为$n \varphi ( 1 )$,倍数运算是环中相当重要的运算.

#### 子环
要验证$S$是$R$的子环,只需验证$S$对加法下是子群,然后$S$对乘法封闭.

而事实上由于$- x = ( - 1 ) x$,所以实际上只需要验证$S$对加法和乘法都封闭即可.

#### 理想
由于环上有两种运算,如果我们想在陪集上保持环的两种运算,就需要在上面加一些限制.

进一步地,如果$I$是$R$的一个加法子群(由于加法有交换律,所以加法子群一定是加法正规子群),并且$\forall r \in R$,有$rI \subseteq I$(那么自然有$rI = I$),则称$I$是$R$的一个**左理想**,同理定义**右理想**,既是左理想又是右理想则称其为一个**理想**.

事实上,如果对于$R$上的一个理想$I$,我们容易证明$I = R \Leftrightarrow 1 \in I$.

由于这是一个加法子群,所以加法运算肯定是满足的,容易发现如果$I$是$R$的一个理想,那么$\forall r , s \in R , ( r + I ) ( s + I ) = rs + I$.

类似群中的结构,子环的交仍然是子环,理想的交仍然是理想.于是类似可以定义由$M \subseteq R , M \ne \emptyset$生成的理想$( M )$.可由一个元素生成的理想称为**主理想**,可由有限多个元素生成的理想叫做**有限生成理想**.容易发现如果$R$是交换环,那么$( a ) = aR$.不然$( a ) = RaR$.

既然我们在陪集上保持了环的运算,我们自然可以称$R / I$为$R$关于$I$的**商环**,也就是$R / I = \{ r + I | r \in R \}$.必须要说明的是商环本身也是一个环,其加法幺元是$0 + I$,乘法幺元是$1 + I$.

子环和理想疑似没有对应的符号表示,我们下面不严谨地盗用子群和正规子群的符号(正常情况下好像不让这么干),在不引起混淆的前提下将$( R , + , \times )$简写为$R$.

#### 同态和同构
类似群同态和同构,要同时保持两种运算并且需要将乘法幺元映射到对应的乘法幺元.

不过,如果将环同态的定义中的$\varphi ( ab ) = \varphi ( a ) \varphi ( b )$改为$\varphi ( ab ) = \varphi ( b ) \varphi ( a )$,则称这样的映射为环的**反同态**.

和群略有区别的地方在于如何定义$\ker \varphi$,事实上我们一般用加法幺元(因为乘法逆元不一定存在)定义$\ker \varphi = \{ a \in R | \varphi ( a ) = 0 \}$.那么$\varphi$是单射当且仅当$\ker \varphi = \{ 0 \}$.

类比一下群同态,可以知道对于$\varphi : R \rightarrow R_1$,$\text{ im } \varphi \leq R_1 , \ker \varphi \trianglelefteq R$.

对于前者:由于加法上的性质已经很好了,只需验证其对乘法封闭,这个是好做的.

对于后者:只需验证其封闭且是理想,同样可行.

##### 同态基本定理
也就是$R / \ker \varphi \cong \text{ im } \varphi$.

设$f : R_1 \to R_2$是线性映射,$I_1 \trianglelefteq R_1 , I_2 \subseteq R_2$并构造两个商映射$\varphi_1 , \varphi_2$,并且$f ( I_1 ) \subseteq I_2$,那么存在唯一的线性映射$\bar{ f } : R_1 / I_1 \to R_2 / I_2$使得$f \varphi_2 = \bar{ f } \varphi_1$.具体地,$\bar{ f } ( r + I_1 ) = f ( r ) + I_2$.

如果再推广上述结论的话还可以来多个,具体可以看下面的交换图表:

$$
\xymatrix{ R_1 \ar[r]^f \ar[d]_{ \varphi_1 } & R_2 \ar[r]^g \ar[d]_{ \varphi_2 } & R_3 \ar[d]_{ \varphi_3 } \\
R_1 / I_1 \ar[r]^{ \bar{ f } } & R_2 / I_2 \ar[r]^{ \bar{ g } } & R_3 / I_3 \\
 }
$$

##### 第一同构定理
设$R$是环,$I \trianglelefteq R$,考虑典范同态:$\pi : R \rightarrow R / I , r \mapsto r + I$,那么:

1. $R$的包含$I$的子环与$R / I$的子环在$\pi$下一一对应.

2. 在此对应下,理想对应于理想.

3. 若$I \trianglelefteq J \trianglelefteq R$,则$R / J \cong ( R / I ) / ( J / I )$.

##### 第二同构定理
设$R$是环,$I \trianglelefteq R , S \leq R$,则:

1. $I + S \leq R$.$I \cap S \trianglelefteq S$.$I \trianglelefteq I + S$

2. $( I + S ) / I \cong S / ( S \cap I )$.

#### 整环
如果一个交换幺环至少含有两个元素(或说$0 \ne 1$),且其满足$xy = 0 \Rightarrow x = 0 \lor y = 0$(或说其没有非零零因子),那么我们称其是一个整环.

显然,整环上对乘法满足左右消去律.原因是$ax = ay \Leftrightarrow a ( x - y ) = 0$.

##### 特征
我们之前证明过$\mathbb{ Z }$可以唯一射到任何一个环上,如果这个环是整环,那么其实有更好的性质:

对于任意一个整环$R$,存在唯一一个环的**特征**$\text{ char } R \in \mathbb{ N }$使得$n 1_R = 0_R \Leftrightarrow \text{ char } R | n$.我们还可以知道$\text{ char } R$要么是$0$要么是素数.

怎么证明呢?考虑如果$\text{ char } R = ab$,自然有$ab 1_R = 0_R$,于是$( a 1_R ) ( b 1_R ) = 0_R$,由于这是一个整环,我们当然有$a 1_R = 0_R \lor b 1_R = 0_R \Rightarrow \text{ char } R | a \lor \text{ char } R | b$.

如果$R$中有一个整子环$R_0$,当然可以发现$\text{ char } R = \text{ char } R_0$,于是当然有$\text{ char } R = \text{ char } \ \text{ Frac } ( R )$.

另一个性质在于容易发现$\text{ char } R \times x = 0_R$.

##### 整除性
$\forall a , b \in R$,如果$\exists c \in R , a = bc$,就称$b$是$a$的**因子**,记作$b | a$.我们可以说明$b | a \Leftrightarrow ( a ) \subseteq ( b )$,因为对于必要性:$( a ) = aR = bcR \subseteq bR$,而对于充分性则是因为$a \in ( a ) \subseteq ( b )$.

如果两个元素互为因子,则称它们**相伴**,暂且用$\sim$代表这种关系.容易验证这等价于$\exists u \in R^\times , a = bu$.原因是如果$\exists u , v \in R , a = ub , b = va$,那么$a = uva$,由消去律得知$uv = 1$.此时它们生成的主理想必然相等.相伴关系显然是一种等价关系,$R$对于相伴关系的商集上的因子关系是一种偏序关系(但不是全序的).

事实上还可以定义**真因子**:如果$( a ) \subsetneq ( b )$,那么显然$b | a$,我们称此时$b$是$a$的真因子.容易发现$b$是$a$的真因子当且仅当$b | a$并且$b$不与$a$相伴.

称一个元素$a$是**不可约元**,当它不是$0$也不是单位元而且$\forall b , c \in R , a = bc \Rightarrow b \in R^\times \lor c \in R^{ \times }$.或者说:$a = bc \Rightarrow c \sim a \lor b \sim a$.或者说$a$不存在真因子.

称一个元素$a$是**素元**,当它不是$0$也不是单位元而且$\forall b , c \in R , a | bc \Rightarrow a | b \lor a | c$.

我们可以证明:在整环中,素元一定是不可约元,但是不可约元不一定是素元.

如何证明素元一定是不可约元呢?考虑如果$a \in \mathrm{ prime }$,并且$a = bc$,则$a | bc \Rightarrow a | b \lor a | c$,不妨假设$a | b$,那么设$b = ad$,自然有$a = adc$,$( dc ) = 1$,于是$c \in \mathrm{ unit }$.

##### 唯一分解整环(UFD)
称$R$是唯一分解整环,如果$\forall a \in R , a \ne 0$,都可以将$a$分解成有限个不可约元的乘积的相伴,也就是可以将$a = u \prod_{ k = 1 }^n p_k$,其中$u \in \mathrm{ unit }$,$p_k$是不可约元.并且这种分解需要在相伴意义下唯一,也就是如果存在另一种分解$a = v \prod_{ k = 1 }^m q_k$,那么需要满足$n = m$并且可以交换乘积顺序使得$p_k$和$q_k$相伴.

我们下面可以证明:在唯一分解整环中,不可约元一定也是素元.

不妨设$a$是不可约元,那么我们要证明$a | bc \Rightarrow a | b \lor a | c$.考虑设$ad = bc$,对$b , c , d$均作唯一分解,那么$a$会剩下来并且和对面的某个分解出来的不可约元相伴,自然会有$a | b \lor a | c$.

在唯一分解整环中可以定义最大公因子.最大公因子当然可能不唯一,但最大公因子之间一定相伴.进一步地,如果最大公因子是可逆元,我们称两个数互素.

当且仅当一个整环有以下两个性质,它是一个唯一分解整环:

1. 任意不可约元都是素元.

2. (存在分解)所有的$r \in R \setminus \{ 0 \}$都能写成有限不可约元的乘积.

先数学归纳,然后用反证法,假设不成立,那么$r = \prod p_k = \prod q_j$.

接下来任取左边的一个$p_1$,如果右边也有一个$q_1 \sim p_1$就直接消去.反之考虑不可约元也是素元,因此$p_1 | \prod q_j$,因此$\exists j , p_1 | q_j$,不妨设$tp_1 = q_j$,因为$q_j$是不可约元,所以$t \in R^\times$,所以$p_1 \sim q_j$.于是唯一分解性自然得出.

另一种等价条件是:

1. 任意不可约元都是素元.

1. 主理想的Noether性质:对于一条主理想的链$( a_1 ) \subseteq ( a_2 ) \subseteq \cdots$,一定存在一个$k$,使得从$k$之后$( a_k ) = ( a_{ k + 1 } ) = \cdots$.这条性质也等价于一条因子链一定需要是有限的.

用主理想的Noether性质直接就可以得出任何一个$r$都存在有限分解,于是等价于上面那个等价性质.

另外,真实的Noether性质其实是任何一个理想列(不一定是主理想),然而UFD并不一定满足此.最经典的例子是环$\mathbb{ Q } [ x_1 , x_2 , \cdots ]$,显然$( x_1 ) \subseteq ( x_1 , x_2 ) \subseteq \cdots$.

##### 主理想整环(PID)
如果一个整环的所有理想都是主理想,则称其为一个主理想整环.交换除环当然是主理想整环的一个典型例子.

需要证明PID一定是UFD,对此,我们考虑:

首先证明PID上的不可约元是素元,这里需要拿出裴蜀定理.然而我们目前不能定义最大公因子的概念,因此退而求其次只定义互素的概念:如果$\forall c , c | a \land c | b \Rightarrow c \in R^\times$,则称$a$和$b$互素.

我们需要证明在主理想整环上有裴蜀定理,换言之如果两个元素互素,那么$\exists x , y \in R , ax + by = 1$,或者说$( \{ a , b \} ) = R$.原因是$\exists h , ( h ) = ( \{ a , b \} )$,那么此时必有$h | a \land h | b$,于是$h \sim 1$.

其次需要证明PID上的Noether性质,注意到一条主理想的链$I_1 \subseteq I_2 \subseteq \cdots$,直接取$I = \bigcup_j I_j$,这个$I$必然也是理想(乘法封闭性是显然的,加法封闭性的话考虑$\forall x \in I_a , y \in I_b$,$x + y \in I_{ \max ( a , b ) }$),那么它就必然是一个主理想,也就是$\exists h , I = ( h )$.此时注意到$\exists k , h \in I_k$,那么$I_k = I_{ k + 1 } = \cdots = I$,这就证毕了.

综上可看出PID一定是UFD.

设$R$为主理想整环,$t \in ( R \setminus \{ 0 \} ) \setminus R^\times$.那么以下命题等价:

1. $R / ( t )$是域.

2. $R / ( t )$是整环.

3. $t$是素元.

(1)$\Rightarrow$(2)当然是平凡的,考虑(2)$\Rightarrow$(3)如何证明:

由于$R / ( t )$是整环,考虑$t = ab$,那么我们有$( a + ( t ) ) ( b + ( t ) ) = ab + ( t ) = 0 + ( t )$,由于这是一个整环,这必然意味着:

$$
\begin{aligned}
a + ( t ) & = 0 + ( t ) \lor b + ( t ) = 0 + ( t ) \\
& \Rightarrow t | a \lor t | b \\
& \Rightarrow t \sim a \lor t \sim b
\end{aligned}
$$

因此$t$是不可约元,因此$t$是素元.

接下来考虑(3)$\Rightarrow$(1),只需证明非零元$a + ( t )$均可逆即可.考虑主理想整环上的裴蜀定理,存在$x , y$使得$ax = 1 - yt$,此时$x + ( t )$就是$a + ( t )$的逆元.

还可进一步扩展裴蜀定理,容易证明在主理想环中$( \{ r_1 , \cdots , r_n \} ) = \gcd ( r_1 , \cdots , r_n ) R$.

事实上还可以在这里拿出拓展版的中国剩余定理,具体地,如果$a_1 , \cdots , a_n \in R \setminus \{ 0 \}$,并且它们两两互素,取$a = \prod a_k$,那么我们事实上有环同构$\varphi : R / ( a ) \to \prod_{ k = 1 }^n R / ( a_k ) , r + ( a ) \mapsto ( r + ( a_i ) )_{ k = 1 }^n$.

用数学归纳只需证明$n = 2$的情形就可以了.

先证明$\varphi$是单射,观察$\ker \varphi$,如果$\varphi ( r + ( a ) ) = ( 0 + ( a_1 ) , 0 + ( a_2 ) )$自然意味着$a_1 | r , a_2 | r \Rightarrow a | r$,于是$r + ( a ) = 0 + ( a )$,这立刻得到$\varphi$是单射.

再证明$\varphi$是满射,用裴蜀定理容易取$x_1 , x_2 \in R$使得$a_1 x_1 + a_2 x_2 = 1$,那么$ra_1 x_1 + ra_2 x_2 = r$,此时注意到$\varphi ( ra_1 x_1 + ( a ) ) = ( 0 + ( a_1 ) , r + ( a_2 ) )$,$\varphi ( ra_2 x_2 + ( a ) ) = ( r + ( a_1 ) , 0 + ( a_2 ) )$.这样就可以看出其满性.其实这当然也是中国剩余定理在整数上的构造的类似版本.

### 多项式环
#### 一元多项式环
一个系数属于特定非零环$R$的一元多项式定义为系数在$R$上的一元多项式环,记作$R [ x ]$,注意到如果$R$是整环,$R [ x ]$仍然是整环.多元多项式环可以不断在多元多项式环上定义一元多项式环.不妨将一个多项式记作$f ( x )$.另外如果我们记$R^{ \times }$为$R$中可逆元素的集合,那么显然$R [ x ]^{ \times } = R^{ \times }$.

一般而言,我们下面默认讨论$R$是整环的情况.

在多项式环上可以定义最高次数函数$\deg$,容易验证$\deg ( fg ) = \deg f + \deg g , \deg ( f + g ) \leq \max \{ \deg f , \deg g \}$,可定义$\deg 0 = - \infty$.

整环上的多项式有长除法和取模运算,也就是对于任意多项式$f$,以及最高项系数为$1$的多项式$d$,存在唯一一组$q , r$满足$f = dq + r$并且$\deg r$最小.这样还可以定义整除也就是当$r = 0$的时候.

长除法的正确性可以反证,如果存在两组,那么自然有$d ( q_1 - q_2 ) = r_2 - r_1$,如果$q_1 \ne q_2$,那么$\deg ( q_1 - q_2 ) \geq 0$,那么$\deg d ( q_1 - q_2 ) \geq \deg d$,但是$\deg ( r_2 - r_1 ) < \deg d$,这样的话等式就不能成立了.因此必定有$q_1 = q_2 , r_1 = r_2$.

这就可以保证代入数值操作.因为我们考虑存在唯一的多项式$q$和唯一一个$s \in R$满足$f = ( x - \alpha ) q + s$,那我们就可以称$f ( \alpha ) = s$.

另外,定义在域$F$上的多项式$F [ x ]$是UFD,原因是可以两边对某一个根作长除法.

#### 多元多项式环
显然,$R [ x_1 , \cdots , x_n ] \cong ( R [ x_1 , \cdots , x_{ n - 1 } ] ) [ x_n ]$.

#### 不可约多项式
不可约多项式,也就是定义在域上的多项式环上的不可约元.换言之如果$f$是不可约的,那么对于$f$的任何一个分解$f = g_1 g_2$,都一定有$g_1 \in F^\times$或者$g_1 \in F^\times$.

##### 本原多项式
为何我们定义不可约多项式的时候需要定义在域上,而不是简单定义在整环上呢?这是因为,例如,$2 x \in \mathbb{ Z } [ x ]$,它并非是一个不可约元.但只要把$\mathbb{ Z }$扩成$\mathbb{ Q }$,它就成为了一个不可约元.

那么在整数上怎么搞呢?我们考虑设$f = a_0 + \cdots + a_n x^n \in \mathbb{ Z } [ x ] \setminus \{ 0 \}$,并设$c ( f ) = | \gcd ( a_0 , \cdots , a_n ) |$,如若$c ( f ) = 1$,则称其是一个**本原多项式**.

接下来我们有**高斯引理**:如果$g , h \in \mathbb{ Z } [ x ]$都是本原多项式,那么$gh$也是本原多项式.

如何证明?考虑对于任何一个素数$p$,我们找到$g$中的最高次系数不被$p$整除,设为$r$.换言之设$g = \sum_{ k \geq 0 } a_k x^k$,则$\forall k > r , p | a_k$,而$p \nmid a_r$.对$f = \sum_{ k \geq 0 } b_k x^k$同样根据上面找到一个最高次$s$,满足$p \nmid b_s$.

现在观察:

$$
\begin{aligned}
& [ x^{ r + s } ] gh \\
= & \sum_{ i + j = r + s } a_i b_j \equiv a_r b_s \pmod{ p } \\
\ne & 0 \pmod{ p }
\end{aligned}
$$

这就搞定.

另一个证明策略是干脆把$\mathbb{ Z } \to \mathbb{ Z } / p \mathbb{ Z }$,后者是一个域,于是可以见到其非零.其实和上面是同一个想法,也许后者在做完映射后更好描述.

高斯引理的推论是$c ( gh ) = c ( g ) c ( h )$.

接下来我们来搞一些$\mathbb{ Q }$上的结构,我们下面来证明,对于本原多项式来说,只要其在$\mathbb{ Q } [ x ]$上可约,那它就在$\mathbb{ Z } [ x ]$上可约.换言之,假设$f \in \mathbb{ Z } \setminus \{ 0 \}$是本原多项式,如果$f = gh$,其中$g , h \in \mathbb{ Q } [ x ] , \deg g > 0 , \deg h > 0$,那我们断言$\exists \alpha \in \mathbb{ Q }^\times$使得$g_1 = \alpha g , h_1 = \alpha^{ - 1 } h$使得$g_1 , h_1 \in \mathbb{ Z } [ x ]$且它们都是本原多项式.

考虑总存在一对$u , v \in \mathbb{ N }_+$使得$ug , vh \in \mathbb{ Z } [ x ]$,此时有:

$$
uv = c ( uvf ) = c ( ugvh ) = c ( ug ) c ( vh )
$$

所以:

$$
f = \frac{ ugvh }{ uv } = \frac{ ug }{ c ( ug ) } \frac{ vh }{ c ( vh ) }
$$

这就搞定.

还有一条性质是说,假设$f \in \mathbb{ Z } \setminus \{ 0 \}$是本原多项式,当$\deg f > 0$的时候,下面两个命题等价:

1. $f$是$\mathbb{ Q } [ x ]$上的不可约多项式.

2. $f$是$\mathbb{ Z } [ x ]$上的不可约多项式.

(1)$\Rightarrow$(2)的话考虑反证,如若存在$\mathbb{ Z } [ x ]$上的$g , h$使得$f = gh$,那$f$在$\mathbb{ Z } [ x ]$上都可约,当然在$\mathbb{ Q } [ x ]$上可约.

(2)$\Rightarrow$(1)的话仍然考虑反证,如果存在$\mathbb{ Q } [ x ]$上的$g , h$使得$f = gh$,那之前的推论告诉我们$f$可以分解为两个本原多项式的乘积,这就矛盾了.

接下来我们证明$\mathbb{ Z } [ x ]$是一个UFD,在此之前,我们先对$\mathbb{ Z } [ x ]$上的不可约元做分类,我们断言其分为以下两种:

1. $\mathbb{ Z }$的不可约元.

2. $\deg f > 0$并且满足$f$作为$\mathbb{ Q } [ x ]$上的不可约元的本原多项式$f$.

首先$\deg f = 0$的情况自然是(1),下面尝试分类$\deg f > 0$的情况.

如若$c ( f ) \ne 1$,当然有$f = c ( f ) \frac{ f }{ c ( f ) }$,因此可约.所以不可约当且仅当$c ( f ) = 1$,也就是$f$得是一个本原多项式.那就知道其不可约性可以从$\mathbb{ Q } [ x ]$过渡到$\mathbb{ Z } [ x ]$上.

最终我们来说明$\mathbb{ Z } [ x ]$是唯一分解的,首先可以提系数使得只对本原多项式说明唯一分解性.

既然如此,考虑如果$f$可以被分解为两种$a_1 \cdots a_r$和$b_1 \cdots b_s$两种,它们都是本原多项式,乘积后亦然,其中$a_i , b_i$均满足$\deg > 0$,那么直接扩域到$\mathbb{ Q } [ x ]$上,由于定义在域上的多项式有唯一分解行,因此$s = r$而且适当重排后$a_i$和$b_i$差了一个$\mathbb{ Q }^\times$,然而两个本原多项式如果差了一个$\mathbb{ Q }^\times$,当然意味着它俩只差一个正负号,这是显然的,就给出了分解的唯一性.

接下来有**Eisenstein判准**,不妨设$n \geq 1$,而$f = \sum_{ k = 0 }^n a_k x^k \in \mathbb{ Z } [ x ]$,如果存在一个素数$p$满足$p \nmid a_n$,而且$p | a_0 , \cdots , a_{ n - 1 }$,并且$p^2 \nmid a_0$,则$f$是$\mathbb{ Q } [ x ]$中的不可约多项式.

为证明此,考虑既然$p \nmid a_n$,那当然$p \nmid c ( f )$,因此,我们可以将$f$调整为$\frac{ f }{ c ( f ) }$,这并不会对$p$的性质产生影响,而可以将$f$化约成本原多项式的情形.

如何证明呢?考虑反证,对本原多项式来说$\mathbb{ Q } [ x ]$上不可约等价于在$\mathbb{ Z } [ x ]$上不可约,假设此情形下$f = gh$,其中$g = \sum_{ k = 0 }^m b_k x^k , h = \sum_{ k = 0 }^l c_k x^k$,其中$l , m > 0$,而且$g , h \in \mathbb{ Z } [ x ]$.

既如此,注意到$a_n = b_m c_l$,因此一定有$p \nmid b_m , p \nmid c_l$.而又有$p | a_0 = b_0 c_0$,不妨假设$p | b_0$.此时,可以取一个$1 \leq k \leq m$,使得$p | b_0 , \cdots , p | b_{ k - 1 }$但是有$p \nmid b_k$,注意到:

$$
\begin{aligned}
a_k & \equiv 0 \pmod{ p } \\
\sum_{ j = 0 }^k b_j c_{ k - j } & \equiv 0 \pmod{ p } \\
b_k c_0 & \equiv 0 \pmod{ p }
\end{aligned}
$$

所以$p | c_0$,所以$p^2 | a_0 = b_0 c_0$,矛盾.

我们可以在此基础上证明一些有趣的结论:

1. $\forall n \geq 1$,$p$是质数,则$p + x^n$是不可约的.

2. $p$是质数,则$\Phi_p = 1 + x + \cdots + x^{ p - 1 } = \frac{ x^p - 1 }{ x - 1 }$是不可约的.

对于(2),考虑:

$$
\begin{aligned}
\Phi_p & = \frac{ x^p - 1 }{ x - 1 } \\
& = \frac{ ( x - 1 + 1 )^p - 1 }{ x - 1 } \\
& = \frac{ \sum_{ k = 0 }^p \binom{ p }{ k } ( x - 1 )^k - 1 }{ x - 1 } \\
& = \sum_{ k = 1 }^p \binom{ p }{ k } ( x - 1 )^{ k - 1 }
\end{aligned}
$$

取$h ( x ) = \sum_{ k = 1 }^p \binom{ p }{ k } x^{ k - 1 }$,则$\Phi_p ( x ) = h ( x - 1 )$,显然,二者的不可约性是等价的,下面观察$h ( x )$的不可约性.然而$h ( x )$当然满足Eisenstein判准,这就搞定.

这里可以见到,其实上面的过程可以全部挪到某个UFD以及与其配套的分式域上,均可以起效果.而且也可以看到在某一个UFD上定义的多项式环也一定是UFD.

##### 一个判定不可约的算法
考虑判定一个$f$是否可以被拆成两个$f = gh$,并且它们都在$\mathbb{ Z } [ x ]$上,其中$\deg g \leq k$.

如何判断呢?首先我们任取不同的$x_0 , \cdots , x_k \in \mathbb{ Z }$,并取$f ( x_0 ) , \cdots , f ( x_k )$,如果它们中有$0$就完工了对吧.如果没有,我们一定有$g ( x_i ) | f ( x_i )$,因此$g ( x_i )$只有有限种可能性.我们枚举所有的可能性并插值判$g$.这就给出了一个有限次实现的算法.

##### 不可约多项式上的扩域
如果域$F$上的多项式$f \in F [ x ]$可以分解为若干个一次多项式的乘积,则称其可**分裂**.

那么如果定义在一个域上的每个$\deg \geq 1$的多项式都分裂,那么就称这个域是**代数闭**的.

定义在域$F$上的多项式未必有根,但理应在其上面可以做扩域使其有根.具体而言我们对于域$F$,想要找到一个单同态$\varphi : F \to L$使得$L$是一个更大的域.而应当见到$L$实际上在某种意义上是定义在$F$上的向量空间.

在这里需要发现的是,定义在域上的$F [ x ]$一定是主理想整环,原因是可以在理想$I$中取出$\deg$最小的一个非零的$f$,那么其中的任何一个$g \in I$,用带余除法立刻得到$g = pf + r$,其中$\deg r < \deg f , r \in I$,必然得到$r = 0$.这意味着$I = ( f )$.同时容易见到$\deg f = 0$的时候,$( f ) = F [ x ]$.

我们尝试研究$F [ x ] / ( f )$的形态,那么见到$\deg f \geq 1$的时候这个形态应当才是有意义的.构造一个同态$\tau : F \to F [ x ] / ( f ) , a \mapsto a + ( f )$,容易见到$\tau$应当是一个单射,原因是在$\deg f \geq 1$的前提下,有$\ker \tau = \{ 0 \}$.此时应当见到$F [ x ] / ( f )$可以看作一个定义在$F$上的向量空间.而运用代余除法得知$\{ 1 + ( f ) , x + ( f ) , \cdots , x^{ \deg f - 1 } + ( f ) \}$显然给出了$F [ x ] / ( f )$的一组基.

在此基础上,若$f = \sum a_k x^k$,定义$f^\varphi = \sum \varphi ( a_k ) \varphi ( x )^k$.

由于$F [ x ]$是主理想环,我们在之前已经证明过了当$f$不可约的时候,$F [ x ] / ( f )$是一个域,令$E = F [ x ] / ( f )$.回到我们一开始的目的.当$\deg f \geq 1$,并且$f$在$F [ x ]$环上不可约的时候,若令$\alpha = x + ( f )$,此时$\alpha$应作为一个$E$上的元素,注意到此时:

$$
\begin{aligned}
& f^\tau ( x + ( f ) ) \\
= & \sum ( a_k + ( f ) ) ( x + ( f ) )^k \\
= & \sum a_k x^k + ( f ) \\
= & f + ( f ) = 0 + ( f )
\end{aligned}
$$

这意味着在$E [ X ]$上$f^\tau$是一个可约的多项式,因为提供了其一个元素$\alpha \in E$作为其的一个零点.

从上面的过程来看,我们理应可以通过若干次扩张使得一个$f$在某个$E_r [ X ]$上是分裂的,其中$r \leq \deg f$,也即每一次扩域次数都至少降低$1$.

我们应当思考这个域扩大了多少,见到如果$\deg f = n$,我们一开始已经声明过此时选取基的大小,自然得到$[ E_r : F ] = [ E_r : E_{ r - 1 } ] \cdots [ E_1 : F ] \leq n !$.

不过我们理应简单说明这样的构造是泛的,具体来讲,我们已经声明过我们需要构造一个交换环$L$并将$F$以环同态$\varphi : F \to L$的形式嵌入其中,并在环$L$中找到一个$\beta \in L$使得$f^\varphi ( \beta ) = 0$,那么此时应当存在一个唯一的环同态$\psi : E \to L$使得$\psi ( \alpha ) = \beta$,并且下面是一个交换图表:

$$
\xymatrix{ E \ar[r]^\psi & L \\
F \ar[u]^\tau \ar[ru]_\varphi }
$$

考察商同态$\pi : F [ x ] \to E = F [ x ] / ( f )$,将此关系嵌入上述图表应当得到上述图表交换的等价条件是下述图表交换:

$$
\xymatrix{ F [ x ] \ar[r]^\pi & E \ar[r]^\psi & L \\
& F \ar[ul] \ar[u]^\tau \ar[ru]_\varphi }
$$

直接令$\Psi = \psi \pi$,上图可以简化为:

$$
\xymatrix{ F [ x ] \ar[rr]^\Psi & & L \\
& F \ar[ul] \ar[ru]_\varphi }
$$

现在观察此结构,若我们一开始的假设成立,此图表的确交换并且满足求值同态$\Psi ( x ) = \psi ( \pi ( x ) ) = \psi ( \alpha ) = \beta$,那么对于一般的多项式$g = \sum b_k x^k$,立刻得到$\Psi ( g ) = \sum \varphi ( b_k ) \beta^n = g^\varphi ( \beta )$,这立刻得到了$\Psi$的唯一性,而直接使用此同态容易证明其存在性.

这个交换图表有什么用呢?来看其在复数上的表现情况.取$F = \mathbb{ R }$并且$f = x^2 + 1$,此时$- 1$自然出现了平方根$\alpha = x + ( x^2 + 1 )$.此时在上述图表中只需简单规定$i$具体的正负取值立刻得到一个$\mathbb{ R } [ x ] / ( x^2 + 1 ) \cong \mathbb{ C }$的同构.

#### 对称多项式
将置换的定义,挪到多元多项式环上,具体而言,$\sigma f ( x_1 , \cdots , x_n ) = f ( x_{ \sigma ( 1 ) } , \cdots , x_{ \sigma ( n ) } )$.

那么,如果带$n$个变元的多项式$f$其对任意置换都不变,换言之$\forall \sigma , \sigma f = f$,则称其是一个**对称多项式**.

我们记$F [ x_1 , \cdots , x_n ]^S$为所有在域$F$上定义的$n$个变元的对称多项式组成的集合,其当然是$F [ x_1 , \cdots , x_n ]$上的一个子环,因为如果$f , g$对称,那么$fg$和$\alpha f + \beta g$亦然.从这里也可以见到其还是一个线性空间.

接下来定义$e_k = \sum_{ 1 \leq i_1 < \cdots < i_k \leq n } x_{ i_1 } \cdots x_{ i_k }$为第$k$个**初等对称多项式**,例如$e_n = x_1 \cdots x_n , e_1 = x_1 + \cdots + x_n$.并且还见到**Vieta公式**:$\prod_{ k = 1 }^n ( y + x_k ) = \sum_{ k = 0 }^n e_k y^{ n - k }$或写作$\prod_{ k = 1 }^n ( y - x_k ) = \sum_{ k = 0 }^n ( - 1 )^k e_k y^{ n - k }$

接下来我们要证明**对称多项式基本定理**,$\forall f \in F [ x_1 , \cdots , x_n ]^S , \exists g \in F [ x_1 , \cdots , x_n ] , f = g ( e_1 , \cdots , e_n )$.

接下来我们定义$f_d = \sum_{ i_1 + \cdots + i_n = d } c_{ i_1 , \cdots , i_n } x_1^{ i_1 } \cdots x_n^{ i_n }$为$d$**齐次多项式**,容易见到任何一个多项式都可以写成若干齐次多项式之和.定义$\deg f$为它分解出来的最大的可能非零齐次多项式的次数.

接下来我们证明一个引理:对于$f \in F [ x_1 , \cdots , x_n ]^S$,则$f ( x_1 , \cdots , x_{ n - 1 } , 0 ) = 0$的充分必要条件是$e_n | f$.

充分性显然,因为$x_n | e_n$.

必要性的话,如若$f ( x_1 , \cdots , x_{ n - 1 } , 0 ) = \sum_{ i_n = 0 } c_{ i_1 , \cdots , i_n } x_1^{ i_1 } \cdots x_{ n - 1 }^{ i_{ n - 1 } } = 0$,这也就意味着$c_{ i_1 , \cdots , i_n } \ne 0 \Rightarrow i_n \geq 1$,而由于对称,意味着$c_{ i_1 , \cdots , i_n } \ne 0 \Rightarrow i_k \geq 1$,这就说明了$e_n | f$.

对$n$元对称多项式$h$,记$h^b = h ( x_1 , \cdots , x_{ n - 1 } , 0 )$,观察到$h^b$当然是$n - 1$次多项式,而且$e_1^b , \cdots , e_{ n - 1 }^b$正好是第$1 , \cdots , n - 1$个初等$n - 1$元对称多项式.

继续考虑证明对称多项式基本定理,首先对于$f = \sum f_d$来说,$f$是对称的当然等价于每一个$f_d$都是对称的,所以我们下面不妨假设$f = f_d$.

对于一个$g \in F [ x_1 , \cdots , x_n ]$,定义其权重为$wt ( g ) = \begin{cases}\max \{ \sum_{ k = 1 }^n k i_k \mid c_{ i_1 , \cdots , i_n } \ne 0 \} & g \ne 0 \\ - \infty & g = 0\end{cases}$.

容易见到$\deg g ( e_1 , \cdots , e_n ) \leq wt ( g )$,这恰也是我们如此定义权重的原因.

下面我们证明:如果$f$是$d$次齐次的,则断言中的$g$不仅存在还能取到$wt ( g ) \leq d$.下面我们对$n + d$递归地论证.

当$d = 0 , f \in F$当然是平凡的,下面设$d \geq 1$.由于数学归纳,我们知道$f^b$的元数量减少了$1$.

当$f^b \ne 0$时,那么如果$f$是$d$齐次的,容易见到$f^b$也该是$d$齐次的,那此时数学归纳给出存在$g_1 \in F [ x_1 , \cdots , x_{ n - 1 } ]$使得$f^b = g_1 ( e_1^b , \cdots , e_{ n - 1 }^b )$,而且$\deg g_1 ( e_1 , \cdots , e_{ n - 1 } ) \leq wt ( g_1 ) \leq d$.

此时设$f_1 = f - g_1 ( e_1 , \cdots , e_{ n - 1 } )$,立刻见到$f_1^b = 0$,那么$e_n | f_1$,取$f_2 = \frac{ f_1 }{ e_n }$,这里就可以看到如若$f^b = 0$,这里直接取$f_1 = f , g_1 = 0$即可.所以上述讨论在这一步就消掉了.

于是,这里的$f_2$当然也是对称多项式,并且$\deg f_2 \leq d - n$,那将其分解为齐次部分的和,运用递归假设知道存在$f_2 = g_2 ( e_1 , \cdots , e_n ) , wt ( g_2 ) \leq d - n$.此时:

$$
\begin{aligned}
f & = f_1 + g_1 ( e_1 , \cdots , e_{ n - 1 } ) \\
& = e_n g_2 ( e_1 , \cdots , e_n ) + g_1 ( e_1 , \cdots , e_{ n - 1 } )
\end{aligned}
$$

这就搞定了.

另一个问题是,上述算法只搞定了$f = g ( e_1 , \cdots , e_n )$的存在性,它是否有唯一性呢?如若有$g ( e_1 , \cdots , e_n ) = h ( e_1 , \cdots , e_n )$,则$( g - h ) ( e_1 , \cdots , e_n ) = 0$.我们应当证明以下定理,称为$e_1 , \cdots , e_n$的**代数无关性**:

若$g \in F [ x_1 , \cdots , x_n ]$满足$g ( e_1 , \cdots , e_n ) = 0$,则$g = 0$.

考虑证明其逆否命题,也就是$g \ne 0$的话就一定有$g ( e_1 , \cdots , e_n ) \ne 0$.

首要的观察是先扩域,如果这个定义在更大的域上满足,在更小的域上当然也满足.因此可以先一步将$F$扩到一个无穷域上,例如直接将其扩到有理分式域$F ( t )$上.下面假设$F$是无穷域.则此时对于任意有限次数非零多项式$g = 0$,一定存在一组$( y_1 , \cdots , y_n )$使得$g ( y_1 , \cdots , y_n ) \ne 0$,这是因为可以数学归纳,当$n = 1$的时候,其最多只有$\deg g$个根但是$F$无限,因此肯定存在.当$n > 1$的时候,直接取$g = \sum g_k x_n^k$,那么由归纳假设一定存在一组$( y_1 , \cdots y_{ n - 1 } )$使得至少有一个$g_k \ne 0$,此时再由$n = 1$的结论就可以知道原命题成立.

接下来考虑$p = \sum_{ k = 0 }^n x^{ n - k } ( - 1 )^k y_k$,回忆到可以扩域使得该多项式分裂,假设分裂成了$p = \prod_{ k = 1 }^n ( x - x_i )$,此时由于Vieta公式,$p = \sum_{ k = 0 }^n ( - 1 )^k e_k ( x_1 , \cdots , x_n ) x^{ n - k }$,于是$e_k ( x_1 , \cdots , x_n ) = y_k$,这就导出了$g ( e_1 , \cdots , e_n ) = g ( y_1 , \cdots , y_n ) \ne 0$.

容易见到上述论证不涉及域中的除法,所以上述结论对整环也是成立的.

##### 牛顿公式
设$p_k = \sum_{ j = 1 }^n x_j^k$,这里的$p_0$其实比较特殊,我们一般不去讨论$p_0$.那么牛顿公式说:

1. 当$1 \leq k \leq n$的时候,$\sum_{ j = 0 }^{ k - 1 } ( - 1 )^j e_j p_{ k - j } = ( - 1 )^{ k - 1 } k e_k$.

2. 当$k > n$的时候,$\sum_{ j = 0 }^n ( - 1 )^j e_j p_{ k - j } = 0$.

牛顿公式的一个相当大的作用是可以用递归的方式用$\{ e_k \}$和$\{ p_k \}$相互表示.更进一步地,可以用$\{ e_k \}$的整系数多项式表示$\{ p_k \}$也可以用$\{ p_k \}$的有理系数多项式表示$\{ e_k \}$,当然对应的域要包含$\mathbb{ Z }$和$\mathbb{ Q }$.

如何证明上面那个结论?考虑以$y$为变元的形式幂级数,定义:

$$
\begin{aligned}
P ( y ) & = \sum_{ k \geq 1 } p_k y^{ k - 1 } \\
& = \sum_{ k \geq 1 } \sum_{ i = 1 }^n x_i^k y^{ k - 1 } \\
& = \sum_{ i = 1 }^n \frac{ x_i }{ 1 - x_i y }
\end{aligned}
$$

再定义:

$$
\begin{aligned}
E ( y ) & = \sum_{ k = 0 }^n e_k y^k \\
& = \prod_{ i = 1 }^n ( 1 + x_i y )
\end{aligned}
$$

接下来观察到:

$$
\begin{aligned}
P ( - y ) & = \sum_{ i = 1 }^n \frac{ x_i }{ 1 + x_i y } \\
& = \frac{ \mathrm{ d } }{ \mathrm{ d } y } \ln E ( y ) \\
& = \frac{ E ' ( y ) }{ E ( y ) }
\end{aligned}
$$

接下来观察:

$$
\begin{aligned}
E ( y ) P ( - y ) & = E ' ( y ) \\
& = \sum_{ k = 1 }^n k e_k y^{ k - 1 }
\end{aligned}
$$

可是左边直接展开就有:

$$
\begin{aligned}
E ( y ) P ( - y ) & = ( \sum_{ l = 0 }^n e_l ( - y )^l ) ( \sum_{ j \geq 1 } p_j y^{ j - 1 } ) \\

\end{aligned}
$$

两边提取$[ y^{ k - 1 } ]$,就有:

$$
\sum_{ l + j = k , 0 \leq l \leq n , j \geq 1 } e_l p_{ j } ( - 1 )^{ j - 1 } = k e_k
$$

两边倒腾一下正负号就是牛顿公式.

#### 结式
让$F$是一个域并且$f , g \in F [ x ]$,我们想要检验它们是否互素.一个好的算法当然是辗转相除法,但我们更想去找一个精确的量来判定.例如矩阵的行列式可以用消元法求,然而我们也可以用代数余子式展开.

具体地,考虑$F [ x ]$的元素:

$$
\begin{aligned}
f & = v_0 x^n + \cdots + v_n \\
g & = w_0 x^m + \cdots + w_m
\end{aligned}
$$

注意这里并不要求$v_0 , w_0 \ne 0$,结式并不在乎这个.

我们定义$f , g$的**结式**为:

$$
Res ( f , g ) = \left | \begin{matrix}
v_0 & \cdots & \cdots & v_n & & \\
& \ddots & & & \ddots & \\
& & v_0 & \cdots & \cdots & v_n \\
w_0 & \cdots & \cdots & w_m & & \\
& \ddots & & & \ddots & \\
& & w_0 & \cdots & \cdots & w_m
\end{matrix} \right |
$$

其中上面那个平行四边形是$m$行的,下面那个是$n$行的.这样总共就是$( n + m ) \times ( n + m )$的,该行列式的定义是有意义的.如若将系数$v_0 , \cdots , w_m$视作变量,容易见到上述行列式仍然是关于$v_0 , \cdots , w_m$的整系数$n + m + 2$元多项式,且系数和$F$无关.

考虑下述性质:

1. $[ v_0^m w_m^n ] Res ( f , g ) = 1$.

2. $Res ( g , f ) = ( - 1 )^{ nm } Res ( f , g )$.

3. $\forall t \in F , Res ( tf , g ) = t^m Res ( f , g ) , Res ( f , tg ) = t^n Res ( f , g )$.

考虑(1),应当见到第$m$行的那个$v_0$恰好是第$m$列,所以上述行列式对角线恰好由$m$个$v_0$和$n$个$w_m$构成,根据行列式最初始的那个排列定义,要凑出$v_0^m w_m^n$就只能沿着对角线乘.

考虑(2),只需要交换行列即可对吧,每次把$v$那里的最后一行暴力换到最下面,这样就需要交换$nm$次.

(3)是显然的.

接下来证明一个引理:如果$Res ( f , g ) = 0$,当且仅当$\exists f_1 , g_1 \in F [ x ]$满足$\deg f_1 < n , \deg g_1 < m$并且$f_1$和$g_1$不全为$0$,使得$fg_1 + gf_1 = 0$.

不妨展开$f_1$和$g_1$如下:

$$
\begin{aligned}
f_1 & = a_1 x^{ n - 1 } + \cdots + a_n \\
g_1 & = b_1 x^{ m - 1 } + \cdots + b_m
\end{aligned}
$$

如若设$H = fg_1 + gf_1 = 0$,那么:

$$
\begin{aligned}
[ x^{ n + m - 1 } ] H & = b_1 v_0 + a_1 w_0 = 0 \\
[ x^{ n + m - 2 } ] H & = b_1 v_1 + b_2 v_0 + a_1 w_1 + a_2 w_0 = 0 \\
\cdots \\
[ x^{ 0 } ] H & = b_m v_n + a_n w_m = 0
\end{aligned}
$$

如果假设$v_0 , \cdots , w_m$均是已知量而尝试求解$f_1 , g_1$,自然拿到了一个大小为$n + m$的线性方程组,容易见到其系数矩阵为:

$$
\begin{bmatrix}
v_0 & & & w_0 & & \\
\vdots & \ddots & & \vdots & \ddots & \\
v_n & & v_0 & w_m & & w_0 \\
& \ddots & \vdots & & \ddots & \vdots \\
& & v_n & & & w_m
\end{bmatrix}
$$

(很遗憾这里把第$n + 1$行和第$m + 1$行画到一行了,然而是无奈之举,请读者自行在脑中错开一下位置)

线性方程组理论告诉我们这有一组非$0$解当且仅当行列式为$0$,然而这恰好是$Res ( f , g )$的转置,而行列式转置不变,这就证毕.

下面我们证明,$Res ( f , g ) = 0$当且仅当以下任何一个条件成立:

1. $v_0 = w_0 = 0$.

2. $\deg \gcd ( f , g ) > 0$.

(1)是平凡的,只需观察上述行列式定义即可发现此时第一列全部为$0$,那行列式当然为$0$.

对于(2),先证必要性,考虑如果$\exists h , \deg h > 0$,$h | f , h | g$,那么注意到$\deg \frac{ f }{ h } < n , \deg \frac{ g }{ h } < m$而且由于$v_0$和$w_0$不全为$0$,所以$\frac{ f }{ h } , \frac{ g }{ h }$不全为$0$,而$f \frac{ g }{ h } - g \frac{ f }{ h } = 0$,用引理得证$Res ( f , g ) = 0$.

再证(2)的充分性.如若$f , g$其一为$0$,不失一般性设$f = 0$.如若此时$\deg g > 0$,则取$h = g$自然是公因式;如若$\deg g = 0$,容易见到$w_0 = Res ( f , g ) = 0$,这就是(1)的情况.

如若$f , g$都不是$0$,而且$v_0 , w_0$不全为$0$,不妨设$w_0 \ne 0$,既如此$g \ne 0$,此时如若$Res ( f , g ) = 0$,由引理拿到了$f_1 , g_1$不全为$0$并且使得$fg_1 + gf_1 = 0$,其中$\deg g_1 < m$.

直接把当前的多项式环$F [ x ]$扩到有理分式域$F ( x )$,此时有$\frac{ f }{ g } g_1 = f_1$,两边通分立刻见到,如果$\deg gcd ( f , g ) = 0$,那么左边就是$f \frac{ g_1 }{ g }$,这应该给出$g | g_1$,然而$\deg g = m , \deg g_1 < m$,不符.这就证毕.

最后,我们来讨论一下当$f , g$均是分裂的时候如何体现,不妨设$f = a \prod_{ k }^n ( x - \alpha_k ) , g = b \prod_{ k }^m ( x - \beta_k )$,我们下面证明:

$$
\begin{aligned}
Res ( f , g ) & = a^m \prod_k^n g ( \alpha_k ) = ( - 1 )^{ nm } b^n \prod_j^m f ( \beta_j ) \\
& = a^m b^n \prod_{ k , j } ( \alpha_k - \beta_j )
\end{aligned}
$$

有一连串等式,其中中间两个是对称的$Res ( f , g )$和$Res ( g , f )$的形式,而最后一个等式只需带入就属显然.因此需要证明的只有第一个等式.

由于$Res ( tf , g ) = t^m Res ( f , g )$,所以不妨设$a = 1$.

假设$g ( \alpha_1 ) , \cdots , g ( \alpha_n )$两两相异,则引入一个新的变元$y$并在$F [ y ]$上考虑$Res ( f , g - y )$,回看一开始的矩阵形状,对于$[ y^n ] Res ( f , g - y )$,见到其意味着下半部分的对角线被全部选择,上半部分选择的话如果非零当然也要选择对角线,所以$[ y^n ] Res ( f , g - y ) = ( - 1 )^n$,而显然$[ y^0 ] Res ( f , g - y ) = Res ( f , g )$.既然如此,注意到$\alpha_k$是$f$和$g - g ( \alpha_k )$的公共根,则$Res ( f , g - g ( \alpha_k ) ) = 0$,从而$( g ( \alpha_k ) - y ) | Res ( f , g - y )$,又由于$g ( \alpha_1 ) , \cdots , g ( \alpha_n )$两两相异,$\prod_k ( g ( \alpha_k ) - y ) , Res ( f , g - y )$的最高次系数都是$( - 1 )^n$,这立刻意味着它们相等,原命题自然成立.

然而既然如此,可以见到直接假设$\alpha_1 , \cdots , \alpha_n$并非常数而是若干个变元,具体而言用$z_1 , \cdots , z_n$代替它们,于是上述立刻有$g ( z_1 ) , \cdots , g ( z_n )$两两相异,得到结论后带入$\alpha_1 , \cdots , \alpha_n$即可.

#### 判别式
对于一个多项式$f = \sum_k c_k ( - 1 )^{ k } x^{ n - k }$,假设其可分裂而且$f = \prod_j ( x - \alpha_j )$,我们称其判别式$disc ( f ) = \prod_{ i < j } ( \alpha_i - \alpha_j )^2$如果我们将$\alpha_1 , \cdots , \alpha_n$看作变量,则我们注意到$disc ( f )$是一个对称多项式,原因是$disc ( f ) = ( \prod_{ i < j } ( \alpha_i - \alpha_j ) )^2$,而中间那个东西在重排后会差一个$\mathrm{ { sgn } } ( \sigma )$,这就证毕.

既然如此,$disc ( f )$就可以拆成$e_1 , \cdots , e_n$这若干个对称多项式的多项式,然而注意到由于Vieta公式,$e_1 = c_1 , \cdots , e_n = c_n$.于是$disc ( f )$当然是关于$c_1 , \cdots , c_n$的多项式.这里由于没有讨论具体的值,而只是把这些涉及到的值用变量来表示,所以你可以先一步把范围控制在整环$\mathbb{ Z }$上,见到$disc ( f )$当然是关于$c_1 , \cdots , c_n$的整系数多项式.

例如当$n = 2$的时候,$f ( x ) = x^2 - bx + c = ( x - \alpha_1 ) ( x - \alpha_2 )$,那么$disc ( f ) = ( \alpha_1 - \alpha_2 )^2 = ( \alpha_1 + \alpha_2 )^2 - 4 \alpha_1 \alpha_2 = b^2 - 4 c$,这就是二次方程的判别式.

取$n = 3$的时候,特别地取二次项为$0$的情况,$f = x^3 + px + q = ( x - \alpha_1 ) ( x - \alpha_2 ) ( x - \alpha_3 ) , \alpha_1 + \alpha_2 + \alpha_3 = 0$,注意到:

$$
\begin{aligned}
disc ( f ) & = \left ( ( \alpha_1 - \alpha_2 ) ( \alpha_2 - \alpha_3 ) ( \alpha_1 - \alpha_3 ) \right )^2 \\
& = - 4 p^3 - 27 q^2
\end{aligned}
$$

而如果将判别式的定义推广到非首一的情形,也就是对于$f = a \prod_j ( x - \alpha_j )$,我们称其判别式$disc ( f ) = a^{ 2 n - 2 } \prod_{ i < j } ( \alpha_i - \alpha_j )^2$,对$f$有形式导数$f '$,我们下面证明:$a \cdot disc ( f ) = ( - 1 )^{ \frac{ n ( n - 1 ) }{ 2 } } Res ( f , f ' )$.

证明的话,用Leibniz法则,知道:

$$
\begin{aligned}
f ' & = a \sum_{ k = 1 }^n \prod_{ j \ne k } ( x - \alpha_j ) \\
f ' ( \alpha_k ) & = a \prod_{ j \ne k } ( \alpha_k - \alpha_j )
\end{aligned}
$$

因此:

$$
\begin{aligned}
Res ( f , f ' ) & = a^{ n - 1 } \prod_{ k = 1 }^n f ' ( \alpha_k ) \\
& = a^{ 2 n - 1 } \prod_{ k = 1 }^n \prod_{ j \ne k } ( \alpha_k - \alpha_j ) \\
& = a^{ 2 n - 1 } ( - 1 )^{ \frac{ n ( n - 1 ) }{ 2 } } \prod_{ i < j } ( \alpha_i - \alpha_j )^2
\end{aligned}
$$

这就证毕.

顺便一提,将$f = a_0 x^n + \cdots + a_n$,则$Res ( f , f ' )$是$a_0 , \cdots , a_n$的整系数多项式,而其行列式首列能提出$a = a_0$,因此$a^{ - 1 } Res ( f , f ' )$确实表作$a_0 , \cdots , a_n$的整系数多项式,这说明$disc ( f )$在$f$非首一的时候定义确实合理.

### 域
设$D$是含有至少两个元素的幺环,如果$D$的每个元素都可逆,则称$D$是一个**体**.有乘法交换律的体称为**域**.

#### 分式域
之前尝试使用过商集来用$\mathbb{ Z }$构造$\mathbb{ Q }$,我们同样可以用整环的商集来构造形如$\frac{ f }{ g }$的域,将其记作$\text{ Frac } ( R )$,也就是**分式域**.

可以说明如果$\varphi : R \rightarrow R '$并且$\varphi ( R \setminus \{ 0 \} ) \subseteq R '^{ \times }$,又取一个$i : R \rightarrow \text{ Frac } R$,那么存在唯一一个映射$\Phi : \text{ Frac } R \rightarrow R '$的映射.更具体来说,只需验证$\Phi ( \frac{ f }{ g } ) \equiv \varphi ( f ) \varphi ( g )^{ - 1 }$即可(当然需要检验良定义).

如果我们将一开始的映射改为$\varphi : R \hookrightarrow F$,其中$F$是一个域并且$\forall x \in F , \exists f , g \in R , x = \varphi ( f ) \varphi ( g )^{ - 1 }$,那我们可以证明$\text{ Frac } ( R ) \cong F$,过程太无聊,略过.

这说明分式域具有某种唯一性和泛性质.

一般把$\text{ Frac } ( R [ x ] )$写作$R ( x )$.

#### 子域
由于子域的交还是子域,我们当然可以取所有子域的交得到一个子域$F_{ \mathrm{ prime } }$,容易验证当$\text{ char } F_{ \mathrm{ prime } } = 0$的时候,$F_{ \mathrm{ prime } } \cong \mathbb{ Q }$;当$p = \text{ char } F_{ \mathrm{ prime } } \in \mathrm{ prime }$的时候,$F_{ \mathrm{ prime } } \cong \mathbb{ Z } / \mathbb{ Z }_p$.

## 线性代数
### 矩阵引入
我们称有$m$行$n$列的**矩阵**为$A_{ m \times n }$,其中第$i$行第$j$列的元素称作$a_{ i , j }$,也有的称其为**矩阵元**.

也可以将其中中的列写作$F^m$中列向量的形式,即写作$A_{ m \times n } = \begin{bmatrix}\vec{ a_1 } & \vec{ a_2 } & . . . & \vec{ a_n }\end{bmatrix}$.

若$n = m$,则称$A$为**方阵**.

#### 矩阵运算
大小相同的矩阵本身应当是一个线性空间.事实上,我们可以定义:

1. **矩阵加法**:若矩阵$A$和$B$都是$m \times n$矩阵,则它们的和也是$m \times n$矩阵,且元素为$A$和$B$中对应元素的加和.显然有交换律.

2. **标量乘法**:若$c$是标量而$A$是一个$m \times n$矩阵,则$cA$是一个$m \times n$矩阵,且元素为A中对应元素的$c$倍.显然有交换律以及对矩阵加法的分配律.

事实上,我们还可以定义:

3. **矩阵乘法**:若$A$是$m \times n$矩阵,$B$是$n \times p$矩阵,那么它们的乘积$AB_{ i , j } = \sum_{ k = 1 }^n A_{ i , k } B_{ k , j }$.

矩阵乘法也可以表示为:若$A$是$m \times n$矩阵,$B$是$n \times p$矩阵,且$B = \begin{bmatrix}\vec{ b_1 } & \vec{ b_2 } & . . . & \vec{ b_p }\end{bmatrix}$那么它们的乘积$AB = \begin{bmatrix}A \vec{ b_1 } & A \vec{ b_2 } & . . . & A \vec{ b_p }\end{bmatrix}$.

若$AB = BA$我们称$A$和$B$彼此**可交换**.

不难发现$\forall A_{ m \times n }$,对于矩阵$I_{ n \times n } = \begin{bmatrix}1 & 0 & . . . & 0 \\ 0 & 1 & . . . & 0 \\ . . . & . . . & . . . & . . . \\ 0 & 0 & . . . & 1\end{bmatrix}$有$AI = A$,我们称$I_{ n \times n }$为**单位矩阵**.单位矩阵的第$i$列通常记作$\vec{ e }_i$.

定义**乘幂**$A^k$为$k$个$A$连续相乘的乘积.

设$A$为$m \times n$矩阵,$B$和$C$的维数使下列各式的乘积有定义,c是标量,有:

1. 乘法结合律:$( AB ) C = A ( BC )$.

2. 乘法左分配律:$A ( B + C ) = AB + AC$.

3. 乘法右分配律:$( B + C ) A = BA + CA$.

4. $c ( AB ) = ( cA ) B = A ( cB )$.

5. 乘法恒等式:$I_m A = A = AI_n$.

注意到向量可以看为一个$n \times 1$的矩阵,事实上,我们有:

若$A$是$m \times n$矩阵,它的各列为$\vec{ a_1 } , \vec{ a_2 } , . . . \vec{ a_n }$,而$\vec{ u } , \vec{ v } \in \mathbb{ R }^n$,那么:

1. $A ( \vec{ u } + \vec{ v } ) = A \vec{ u } + A \vec{ v }$.

2. $A ( c \vec{ u } ) = c ( A \vec{ u } )$.

定义**矩阵转置**:对于$m \times n$矩阵$A$,它的**转置**是一个$n \times m$矩阵,该矩阵的列是由$A$的对应行构成的,记作$A^T$,即$A^T_{ i , j } = A_{ j , i }$.有的时候也写作$^t A$.

设$A$和$B$的维数使下列各式的乘积有定义,c是标量,有:

1. $( A^T )^T = A$.

2. $( A + B )^T = A^T + B^T$.

3. $( rA )^T = rA^T$.

4. $( AB )^T = B^T A^T$.

#### 分块矩阵
我们考虑将一个矩阵看作若干个子矩阵的拼接,类似于我们求逆矩阵时做的那样:将若干个矩阵拼接起来.

换句话说,我们尝试将矩阵看作元素放入矩阵中,并尝试根据基本的矩阵运算去做这种矩阵的运算.

分块矩阵的**矩阵加法**:若矩阵$A$和$B$维数相同且都以同样的方式分块,则矩阵的和$A + B$也以同样的方式分块且每一块都是$A$和$B$对应块的和.

分块矩阵的**标量乘法**:若$c$是一个常数,$A$是一个分块矩阵,则$cA$是$A$的子矩阵乘$c$再以同样的方式组合起来的结果.

分块矩阵的**矩阵乘法**:若矩阵$A$和$B$的维数使得$AB$有定义,并且$A$列的分法与$B$行的分法一致,那我们显然可以直接将矩阵作为元素来直接进行矩阵乘法.

由上面的讨论自然发现:

若$A$是$m \times n$矩阵,$B$是$n \times p$矩阵,我们定义$row_i ( A )$为$A$的第$i$列,$col_i ( A )$为$A$的第$i$行,则:

$AB = \begin{bmatrix}col_1 ( A ) & col_2 ( A ) & . . . & col_n ( A )\end{bmatrix} \begin{bmatrix}row_1 ( B ) \\ row_2 ( B ) \\ . . \\ row_n ( B )\end{bmatrix}$

证明是显然的.

### 线性方程组
包含变量$x_1 , x_2 . . . , x_n$的**线性方程**是形如$a_1 x_2 + a_2 x_2 + . . . + a_n x_n = b$的方程,其中b与系数是实数或复数,通常是已知数.

**线性方程组**是由一个或几个包含相同变量$x_1 , x_2 , . . . , x_n$的线性方程组成的.

**线性方程组的解**是一组数$\{ s_1 , s_2 , . . . , s_n \}$,用这组数分别代替$x_1 , x_2 , . . . x_n$时所有方程的两边相等.

线性方程组所有可能的解的集合称为**线性方程组的解集**.

若两个线性方程组有相同的解集,则称这两个方程组是**等价的**.

我们称一个线性方程组是**相容的**,当且仅当它有一个解或无穷多个解,否则,称其为**不相容的**.

一个线性方程组要么无解,要么有一个解,要么有无穷多个解.

#### 系数矩阵和增广矩阵
考虑以下m个线性方程所组成的线性方程组:

$$
\begin{cases}
a_{ 1 , 1 } x_1 + a_{ 1 , 2 } x_2 + . . . + a_{ 1 , n } x_n = b_1 \\
a_{ 2 , 1 } x_1 + a_{ 2 , 2 } x_2 + . . . + a_{ 2 , n } x_n = b_2 \\
. . . \\
a_{ m , 1 } x_1 + a_{ m , 2 } x_2 + . . . + a_{ m , n } x_n = b_m
\end{cases}
$$

我们称它的**系数矩阵**为:

$$
\begin{bmatrix}
a_{ 1 , 1 } & a_{ 1 , 2 } & . . . & a_{ 1 , n } \\
a_{ 2 , 1 } & a_{ 2 , 2 } & . . . & a_{ 2 , n } \\
. . . & . . . & . . . & . . . \\
a_{ m , 1 } & a_{ m , 2 } & . . . & a_{ m , n }
\end{bmatrix}
$$

两个系数矩阵是**等价的**当且仅当它们所对应的线性方程组是等价的.

而称它的**增广矩阵**为:

$$
\begin{bmatrix}
a_{ 1 , 1 } & a_{ 1 , 2 } & . . . & a_{ 1 , n } & b_1 \\
a_{ 2 , 1 } & a_{ 2 , 2 } & . . . & a_{ 2 , n } & b_2 \\
. . . & . . . & . . . & . . . & . . . \\
a_{ m , 1 } & a_{ m , 2 } & . . . & a_{ m , n } & b_m
\end{bmatrix}
$$

两个增广矩阵是**等价的**当且仅当它们所对应的线性方程组是等价的.

#### 矩阵的初等行变换
考虑如何解一个线性方程:根据我们已有的知识,等式可以任意两端加减相等的两项,也可以同时乘以一个不为0的数,同时等式的顺序并不重要.

我们用这一点来考虑如何解线性方程组.这里主要是考虑如何对线性方程组的增广矩阵进行变换.

由上面的知识,我们意识到对线性方程组的增广矩阵进行以下变换,是不会改变其解集的:

1. (倍加变换)把某一行换成它本身与另一行的倍数的和,记作$R_i + kR_j \rightarrow R_i$.

2. (对换变换)把两行对换,记作$R_i \leftrightarrow R_j$.

3. (倍乘变换)把某一行的所有元素乘以同一个非零数$k$,记作$kR_i \rightarrow R_i , k \ne 0$.

以上三种变换称为**矩阵的初等行变换**,同理可以定义初等列变换.

由此可以发现,两个增广矩阵是等价的当且仅当其中一个增广矩阵可以通过若干次初等行变换变为另一个增广矩阵.

我们可以使用这些初等行变换来变换一个矩阵,使得它有一些特殊性质.

如果两个矩阵可以通过若干次初等行变换而转化,我们称二者**行等价**,不难证明行等价满足等价关系的三个性质(反身性,对称性,传递性).

由于简化行阶梯矩阵的唯一性,我们可以用其来判定行等价类.

#### 阶梯形矩阵
若矩阵的一行的元素全为0,称这一行为矩阵的一个**零行**,否则称为**非零行**.

非零行的最左边的非零元素称为该行的**先导元素**.

若一个矩阵有以下性质,则称它为**阶梯形矩阵(REF)**:

1. 每一个非零行都在零行之上

2. 某一行的先导元素所在的列位于前一行先导元素的右边

3. 某一先导元素所在列下方元素都是0

若一个矩阵还满足以下性质,则称它为**简化阶梯形(RREF)**:

4. 每一非零行的先导元素是1

5. 每一先导元素1是该元素所在列唯一的非零元素

显然,任意一个矩阵都可以通过若干次初等行变换变为一个阶梯形矩阵或简化阶梯形矩阵.

一个矩阵所对应的阶梯形矩阵中先导元素的位置,被称为这个矩阵的一个**主元位置**,包含一个主元位置的列称为**主元列**.

注意:这里的位置是矩阵中的位置而非元素的位置,这也就是说主元位置上的元素可以不是最终的主元位置上的元素移动而来的.

##### 简化阶梯形矩阵的唯一性
每个矩阵通过初等行变换只有可能对应唯一一个简化阶梯形矩阵.

证明:假设两个行等价的简化阶梯形矩阵不相等.那么我们找到它们的第一个不相等的列,设为$j$.

如果$A , B$中的$j$均为主元列,显然它们这一列必然相等,不符.

反之,我们不妨去掉第$j$列后面的所有列以及第$j$列前面的所有非主元列,保留第$j$列考虑剩下的两个矩阵一定也是行等价的,但它们对应的线性方程组的解显然不同,这导出了矛盾.

#### 解线性方程组
考虑将线性方程组的增广矩阵通过初等行变换变换为一个简化阶梯形矩阵,我们称对应于主元列的变量为**基本变量**,其它变量称为**自由变量**.

如果线性方程组是相容的,显然我们可以使用自由变量表示基本变量,从而得到一组解.我们得到的基本变量的关于自由变量的表达式称作解集的**参数表示**.

这也意味着,如果没有自由变量,该方程组只有一解;如果有自由变量,由于自由变量取值任意,该方程组有无数组解.

而考虑何时线性方程组不相容,由于自由变脸取值任意,显然唯一的不相容可能性只是出现了$0 x_1 + 0 x_2 + . . . + 0 x_n = b , b \ne 0$的情况.

以上的分析过程引出下面的定理.

##### 存在与唯一性定理
线性方程组相容的充要条件是增广矩阵的最右列不是主元列.

若线性方程组相容,则它的解集可能有两种情形:

1. 没有自由变量时,只有一个解.

2. 有自由变量时,有无数个解.

#### 矩阵方程
设$A$是$m \times n$的矩阵,它的各列是$\vec{ a_1 } , \vec{ a_2 } , . . . , \vec{ a_n }$,若$\vec{ x }$是$F^n$中向量,则$A \vec{ x }$即A的各列以$\vec{ x }$中对应元素为权的线性组合,也就是:

$A \vec{ x } = \begin{bmatrix}\vec{ a_1 } & \vec{ a_2 } & . . . & \vec{ a_n }\end{bmatrix} \begin{bmatrix}x_1 \\ x_2 \\ . . . \\ x_n\end{bmatrix} = x_1 \vec{ a_1 } + x_2 \vec{ a_2 } + . . . + x_n \vec{ a_n }$

如果令$\vec{ b } \in F^m$,那么上式可以写成$A \vec{ x } = \vec{ b }$,我们把形如这样的方程称作矩阵方程.

不难发现,若$A$是$m \times n$矩阵,它的各列为$\vec{ a_1 } , \vec{ a_2 } , . . . \vec{ a_n }$,而$\vec{ b } \in F^m , \vec{ x } \in F^n$,那么:

矩阵方程$A \vec{ x } = \vec{ b }$,向量方程$x_1 \vec{ a_1 } + x_2 \vec{ a_2 } + . . . + x_n \vec{ a_n } = \vec{ b }$,增广矩阵为$\begin{bmatrix}\vec{ a_1 } & \vec{ a_2 } & . . . & \vec{ a_n } & \vec{ b }\end{bmatrix}$的线性方程组有相同的解集.

事实上,若$A$是$m \times n$矩阵,它的各列为$\vec{ a_1 } , \vec{ a_2 } , . . . \vec{ a_n }$,那么以下命题逻辑等价:

1. $\forall \vec{ b } \in F^m$,方程$A \vec{ x } = \vec{ b }$有解.

2. $\forall \vec{ b } \in F^m$,$\vec{ b }$是$A$的列的一个线性组合.

3. $A$的各列生成$\mathbb{ R }^m$.

4. $A$在每一行都有一个主元位置.

#### 齐次线性方程组
若一个线性方程组可以被写成$A \vec{ x } = \vec{ 0 }$的形式,其中$A$是$m \times n$的矩阵,$\vec{ x }$是$\mathbb{ R }^n$中的向量,称其为**齐次线性方程组**.

显然齐次线性方程组至少有一个解,即$\vec{ x } = \vec{ 0 }$,这个解称作线性齐次方程组的**平凡解**.

而满足$\vec{ x } \ne \vec{ 0 }$的解称作线性齐次方程组的**非平凡解**.

根据存在与唯一性定理,显然其存在非平凡解的充要条件是该方程至少有一个自由变量.

考虑自由变量的取值任意,而且基本变量一定是由若干个自由变量乘以定值之和得到的.

我们把这些定值看作向量,把自由向量看作权重,我们发现:齐次线性方程组的解集总可以表示为$\mathrm{ Span } \{ \vec{ v_1 } , \vec{ v_2 } , . . . , \vec{ v_{ p } } \}$,当然,如果该方程组仅有平凡解,那表示为$\mathrm{ Span } \{ \vec{ 0 } \}$.

把解表示为$\mathrm{ Span } \{ \vec{ v_1 } , \vec{ v_2 } , . . . , \vec{ v_{ p } } \}$中元素的形式,称之为**解的参数向量形式**.

对于方程$A \vec{ x } = \vec{ b }$,它的解一定可以写成$\vec{ x } = \vec{ p } + \vec{ x ' }$的形式,其中$\vec{ x ' }$是方程$A \vec{ x } = \vec{ 0 }$的一组解,$\vec{ p }$是方程$A \vec{ x } = \vec{ b }$的一组特解.

对于该定理的几何意义,我们考虑$\vec{ x } \in \mathbb{ R }^2$中,即$\vec{ x } = \vec{ p } + t \vec{ u }$,如果我们把它看为向量加法的形式,它的解集应该是在一条直线上的任意向量,这条直线即$\vec{ u }$所在直线沿向量$\vec{ p }$方向平移后的直线.

### 线性空间
我们可以在一个域$F$上定义一个**线性空间**(向量空间)$V$,除了域本身带有的性质(加法的结合律交换律,加法单位元,加法逆元,乘法对加法有分配律)它应当满足以下公理:

1. 定义向量加法$V \times V \rightarrow V$.向量加法应当有交换律和结合律.

2. 定义标量乘法$F \times V \rightarrow V$.标量乘法应当有交换律,结合律以及对向量加法的分配律.

3. 存在加法幺元以及加法逆元.

由于$V$对加法构成群,相应的性质可以从群的性质搬过来.

值得一提的是,其实标量乘法如果定义成$V \times F \rightarrow V$后会有相当多的好处,但是由于历史原因一般还是定义左标量乘法.

当然容易验证我们平常用的线性空间$V = F^n$满足以上性质.

当然容易验证定义在域上的多项式环满足以上性质.

#### 相关运算
##### 直积
定义$\prod_{ i \in I } V_i = ( \vec{ v }_i )_{ i \in I } , \vec{ v }_i \in V_i$为直积,容易发现线性空间的直积也是线性空间.

##### 直和
我们如果取直积的一个子空间,使得至多只有有限个$i \in I$使得$\vec{ v }_i \ne 0$,则称这是一个直和.

##### 叉乘
对$\mathbb{ R }^3$,定义新的向量运算:向量积(叉乘)$\vec{ a } \times \vec{ b }$, 其运算结果仍为向量,设为$\vec{ c }$,它的模定义为$\vec{ a } \times \vec{ b } = | \vec{ a } | | \vec{ b } | \sin \theta$,其中$\theta$为向量$\vec{ a } , \vec{ b }$的夹角,$| \vec{ c } |$为以$a , b$为两边的平行四边形的面积,$\vec{ c }$的方向定义为与$\vec{ a } , \vec{ b }$所形成的平面垂直,且$\vec{ a } , \vec{ b } , \vec{ c }$构成右手螺旋定则(即现将四指指向$\vec{ a }$方向再将四指弯曲指向$\vec{ b }$的方向,则此时大拇指的方向为$\vec{ c }$的方向),

向量叉乘满足以下性质:

1. $\vec{ a } \times \vec{ b } = - \vec{ b } \times \vec{ a }$.

2. $\vec{ a } \times \vec{ a } = \vec{ 0 }$

3. $\vec{ a } \times ( \vec{ b } + \vec{ c } ) = \vec{ a } \times \vec{ b } + \vec{ a } \times \vec{ c }$.

在三维坐标系中,设:

$$
\begin{aligned}
\vec{ a } & = a_1 \vec{ i } + b_1 \vec{ j } + c_1 \vec{ k } \\
\vec{ b } & = a_2 \vec{ i } + b_2 \vec{ j } + c_2 \vec{ k }
\end{aligned}
$$

由叉乘定义,得到:$\vec{ c } = ( a_y b_z - a_z b_y ) \vec{ i } + ( a_z b_x - a_x b_z ) \vec{ j } + ( a_x b_y - a_y b_x ) \vec{ k }$.也就是:

$$
\vec{ c } = \vec{ a } \times \vec{ b } = \left | \begin{matrix}
\vec{ i } & \vec{ j } & \vec{ k } \\
a_x & a_y & a_z \\
b_x & b_y & b_z
\end{matrix} \right |
$$

对于任意一个面的法向量，我们总可以选取该面上的两个不共线向量来直接叉乘出来.

#### 线性无关与线性相关
称$V$中一组向量$\{ \vec{ v_1 } , \vec{ v_2 } , . . . , \vec{ v_p } \}$是**线性无关**的,当且仅当向量方程$x_1 \vec{ v_1 } + x_2 \vec{ v_2 } + . . . + x_p \vec{ v_p } = \vec{ 0 }$仅有平凡解.

反之,称为**线性相关**,即存在一组不全为0的数$c_1 , c_2 , . . . , c_p$使$c_1 \vec{ v_1 } + c_2 \vec{ v_2 } + . . . + c_p \vec{ v_p } = \vec{ 0 }$,这个式子也被称为$\{ \vec{ v_1 } , \vec{ v_2 } , . . . , \vec{ v_p } \}$之间的**线性相关关系**.

容易发现,两个或更多向量的集合$S = \{ \vec{ v_1 } , \vec{ v_2 } , . . . , \vec{ v_p } \}$线性相关,当且仅当其中至少有一个向量是其它向量的线性组合.

#### 子空间
我们定义$V$的一个**子空间**是它的一个非空子集$V_0$,具有以下三个性质:

1. $\vec{ 0 } \in V_0$.

2. $\forall \vec{ u } , \vec{ v } \in V_0 , \vec{ u } + \vec{ v } \in V_0$.

3. $\forall \vec{ u } \in V_0 , c \in F , c \vec{ u } \in V_0$.

线性空间的两个平凡子空间当然是零空间和它本身.

容易验证两个子空间的交仍然是子空间.

##### 向量的线性组合
给定$V$中向量$\vec{ v }_1 , \vec{ v }_2 , . . . , \vec{ v }_p$和标量$c_1 , c_2 , . . . , c_p \in F$.

我们称向量$\overrightarrow{ y } = c_1 \overrightarrow{ v_1 } + c_2 \overrightarrow{ v_2 } + . . . + c_p \overrightarrow{ v_p }$为向量$\overrightarrow{ v_1 } , \overrightarrow{ v_2 } , . . . , \overrightarrow{ v_p }$以$c_1 , c_2 , . . . , c_p$为权的**线性组合**.

对于向量$\overrightarrow{ v_1 } , \overrightarrow{ v_2 } , . . . , \overrightarrow{ v_p }$,我们称它们的所有线性组合所成的集合为$\mathrm{ Span } \{ \overrightarrow{ v_1 } , \overrightarrow{ v_2 } , . . . , \overrightarrow{ v_p } \}$,也称为由$\overrightarrow{ v_1 } , \overrightarrow{ v_2 } , . . . , \overrightarrow{ v_p }$所**生成**的$V$的子集,有时也记作$\langle \vec{ v }_1 , \vec{ v }_2 , \cdots , \vec{ v }_p \rangle$.

不难发现,$\mathrm{ Span }$集合是$V$的一个子空间,而且容易证明其是包含$\{ \vec{ v }_1 , \vec{ v }_2 , \cdots , \vec{ v }_p \}$的最小的子空间,所以我们又称其为由$\overrightarrow{ v_1 } , \overrightarrow{ v_2 } , . . . , \overrightarrow{ v_p }$所**生成**的子空间.

可以考虑一些特殊线性空间上$\mathrm{ Span }$的几何意义:

对于$\mathbb{ R }^2$,当$\vec{ v_1 }$和$\vec{ v_2 }$不在一条直线上的时候,$\mathrm{ Span } \{ \vec{ v_1 } , \vec{ v_2 } \}$显然对应了整个平面.

对于$\mathbb{ R }^3$,当$\vec{ v_1 }$和$\vec{ v_2 }$不在一条直线上的时候,$\mathrm{ Span } \{ \vec{ v_1 } , \vec{ v_2 } \}$显然对应了三维空间中的一个过$\vec{ v_1 } , \vec{ v_2 }$这两条直线的平面.

要判断$\overrightarrow{ b }$是否在$\mathrm{ Span } \{ \overrightarrow{ v_1 } , \overrightarrow{ v_2 } , . . . , \overrightarrow{ v_p } \}$中，即判断向量方程$x_1 \overrightarrow{ v_1 } + x_2 \overrightarrow{ v_2 } + . . . + x_p \overrightarrow{ v_p } = \overrightarrow{ b }$是否有解.

##### 线性映射
映射$T : V \rightarrow W$是线性的,当且仅当它满足以下两个条件:

1. 对$T$的定义域中一切的向量$\vec{ u } , \vec{ v }$,满足$T ( \vec{ u } + \vec{ v } ) = T ( \vec{ u } ) + T ( \vec{ v } )$.

2. 对$T$的定义域中一切的向量$\vec{ u }$和数$c$,满足$T ( c \vec{ u } ) = cT ( \vec{ u } )$.

线性映射有以下性质:

1. $T ( \vec{ 0 } ) = \vec{ 0 }$.

2. (叠加原理):$T ( c_1 \vec{ v }_1 + c_2 \vec{ v }_2 + . . . + c_p \vec{ v }_p ) = c_1 T ( \vec{ v }_1 ) + c_2 T ( \vec{ v }_2 ) + . . . + c_p T ( \vec{ v }_p )$.

称线性映射$T : V \rightarrow W$是**可逆的**或者**同构的**,当且仅当存在$T^{ - 1 } : W \rightarrow V$.

我们可以验证$T^{ - 1 }$也是线性映射,原因在于此时$T$是双射:

$$
\begin{aligned}
T^{ - 1 } ( \vec{ w } + \vec{ w } ' ) & = T^{ - 1 } ( \vec{ w } ) + T^{ - 1 } ( \vec{ w } ' ) \\
& \Leftrightarrow \\
T ( T^{ - 1 } ( \vec{ w } + \vec{ w } ' ) ) & = T ( T^{ - 1 } ( \vec{ w } ) + T^{ - 1 } ( \vec{ w } ' ) )
\end{aligned}
$$

而且:

$$
\begin{aligned}
T^{ - 1 } ( t \vec{ w } ) & = tT^{ - 1 } ( \vec{ w } ) \\
& \Leftrightarrow \\
T ( T^{ - 1 } ( t \vec{ w } ) ) & = T ( tT^{ - 1 } ( \vec{ w } ) )
\end{aligned}
$$

我们记$\text{ Hom } ( V , W )$为所有的$V \rightarrow W$,并且其中的$V , W$都是定义在$F$上的线性空间的线性映射组成的集合.容易验证$\text{ Hom } ( V , W )$构成$F$上的线性空间.特别地,定义$\text{ End } ( V ) = \text{ Hom } ( V , V )$,容易发现$\text{ End } ( V )$是对复合和加法操作一个环.

特别地,定义矩阵映射$T : F^n \rightarrow F^m$为$T ( \vec{ x } ) = A \vec{ x }$,其中$A$是一个$m \times n$的矩阵.矩阵映射显然是线性映射.

##### 基和维数
$V$中子空间$H$的一组**基**是$H$中一个线性无关集,它生成$H$.

事实上,以下命题等价:

1. $S$是$V$的一组基.

2. $S$是极小的能生成$V$的集合(不存在其的一个真子集能生成$V$).

3. $S$是极大的$V$中的线性无关集(不存在其的一个真张集线性无关).

证明上列命题等价是容易的.

对于$F^n$来说,其的基有更好的性质,我们称$\{ \vec{ e }_1 , . . . , \vec{ e }_n \} , \vec{ e }_i = \{ 0 , 0 , \cdots , 0 , 1 , 0 , \cdots \}$为$F^n$的**标准基**.

对于$n \times m$的矩阵组成的线性空间,其当然也有一组标准基$\{ \vec{ e }_{ i , j } \}$.

有如下结论:

1. (承认选择公理)对于任何一个线性无关的子集$S$,存在一个基$\mathcal{ B }$使得$S \subseteq \mathcal{ B }$.特别地,取$S = \emptyset$可以说明任何一个线性空间总是有基.

2. (有限生成的情况)如果一个线性空间的子集$S$满足$| S | > | \mathcal{ B } |$,其中$\mathcal{ B }$是一组有限大小的基,那么$S$线性相关.

3. 如果一个线性空间有两组基$\mathcal{ B_1 } , \mathcal{ B_2 }$,那么$| \mathcal{ B_1 } | = | \mathcal{ B_2 } |$.

现在考虑(1)的证明,拿出所有的线性无关的$T \supseteq S$并组成一个集合,用包含关系作为偏序关系,如果能证明其满足Zorn引理的条件,那当然可以取出一个极大的线性无关集合,这当然是一组基.为此我们取出一条链并且将上面的所有集合并起来得到一个集合$T '$.此时就可以看出我们之前(几乎一笔略过的)要求从基扩张的时候只需要有限个元素的作用,它可以立刻断言$T '$是一个线性无关集合.

(2)的证明是容易的,不妨设$n = | \mathcal{ B } | , m = | S |$,由于$m > n$,这当然是一个$m$个变量的大小为$n$的方程组,当然存在不全为$0$的解.

(3)的定义较为复杂,略过.

在上述定理的基础上,我们可以定义非零向量空间$V$的**维数**,记作$\dim V = | \mathcal{ B } |$,零子空间$\{ \vec{ 0 } \}$的维数定义为$0$.

一般而言,我们大部分情况下都只讨论有限生成的情况.

有限生成的情况,我们还有以下若干条定理:

4. (基定理)设$H$是$V$的$p$维子空间,$H$中任何恰好由$p$个元素组成的线性无关集构成$H$的一个基.

这当然是自然的,根据上面的推导,这个线性无关集合就是极大的线性无关集合.

5. 如果线性空间的子空间$V_0 \subseteq V$,那么$\dim V_0 \leq \dim V$.特别地,$V_0 = V \Leftrightarrow \dim V_0 = \dim V$.

只需要继续在$V_0$里面取基就可以了.

6. $V$定义在域$F$上,如果$\dim V = n$,那么$V \cong F^n$.

考虑$F^n$的一组标准基$\vec{ e_1 } , \cdots \vec{ e }_n$,然后我们取出$V$的一组有序基$B = \{ \vec{ v }_1 , \cdots , \vec{ v }_n \}$,然后把$T : F^n \rightarrow V , \sum k_i \vec{ e }_i \mapsto \sum k_i \vec{ v }_i$.接下要当然要验证良定义和双射,通过基的定义是显然的.

这可以引出所谓坐标表示,即:设$\mathcal{ B } = \{ \vec{ b }_1 , . . . , \vec{ b }_n \}$是$V$的一个基,则$\forall \vec{ x } \in V$,$\exists$唯一的一组$\{ c_1 , . . . , c_n \}$满足$\vec{ x } = c_1 \vec{ b }_1 + . . . + c_n \vec{ b }_n$.我们称$\begin{bmatrix}c_1 \\ \vdots \\ c_n\end{bmatrix}$是$\vec{ x }$的$\mathcal{ B } -$坐标向量,记作$[ x ]_{ \mathcal{ B } }$,映射$x \mapsto [ x ]_{ \mathcal{ B } }$称为由$\mathcal{ B }$确定的**坐标映射**.

事实上,如果我们记$P_{ \mathcal{ B } } = \begin{bmatrix}\vec{ b }_1 & \vec{ b }_2 & \cdots & \vec{ b }_n\end{bmatrix}$,则我们有$\vec{ x } = P_{ \mathcal{ B } } [ x ]_{ \mathcal{ B } }$.我们称$P_{ \mathcal{ B } }$为$\mathcal{ B }$的坐标变换矩阵.

7. 线性映射由它在基上的作用完全确定.

原因是考虑$V$的一组基$B = \{ \vec{ v }_1 , \cdots , \vec{ v }_n \}$,那么$T ( \sum k_i \vec{ v }_i ) = \sum k_i T ( \vec{ v }_i )$.

8. $\text{ Hom } ( V , W ) \cong W^n$,其中$\dim V = n$.

由(7),我们知道如果选定了$C = \{ \vec{ w }_1 , \cdots \vec{ w }_n \}$,则只需要考虑在基上的具体变换即可.

9. (线性映射同构于矩阵映射)如果$V , W$都是定义在$F$上的线性空间,$\dim V = n , \dim W = m$,那么$\text{ Hom } ( V , W ) \cong W^n \cong F^{ m \times n }$.更一般地,如果$V$的基的指标集是$J$,$W$的基的指标集是$I$.那么$\text{ Hom } ( V , W ) \cong F^{ I \times J }$.另外地,$\dim \text{ Hom } ( V , W ) = mn$.特殊地,$\text{ End } ( V ) \cong F^{ n \times n }$.

由(6)(8)显然.

现在我们就可以拿出矩阵了,我们在$F$上定义$m \times n$的矩阵$M$,于是$\mathcal{ M } = \text{ Hom } ( V , W ) \cong M_{ m \times n }$.

简单来说,如果$B = \{ \vec{ v }_1 , \cdots \vec{ v }_n \}$是$V$的一组基,$C = \{ \vec{ w }_1 , \cdots \vec{ w }_m \}$是$W$的一组基,我们事实上是把$\vec{ v }_j \mapsto \sum a_{ i , j } \vec{ w }_i$.

10. 线性映射的复合同构于矩阵乘法.

只需操演定义即可发现.

11. 如果$F$是$E$的子域,那么$E$是在$F$上的向量空间.同样,如果$V$是$E$的向量空间,那么$V$是$F$的向量空间.

定义的简单操演.

12. 如果$V$是$E$的向量空间,基为$B$,$E$是$F$的向量空间,基为$C$.那么$V$是$F$的向量空间,而且基是$CB$.作为其的一个推论,如果我们定义$[ V : E ] = \dim_E V$,即在$E$上的$V$的维数.我们应当能见到$[ V : F ] = [ V : E ] [ E : F ]$.

定义的简单操演.可以将这里理解为$[ V : F ] = \log_F V$这个样子.

#### 可逆矩阵
称一个$n \times n$的矩阵$A$是**可逆的**,当且仅当$\exists C_{ n \times n }$满足$CA = AC = I_{ n }$,我们记$C = A^{ - 1 }$.其实也就是$F_{ n \times n }$这个环上的可逆元.这当然意味着其对应的线性映射是可逆的,也就是其代表的线性映射是同构.

不可逆矩阵又被称为**奇异矩阵**.

事实上$m \times n$的矩阵$A$在$m \ne n$的时候当然是不可逆的,因为其代表的线性映射不可能是同构(不然两边的$\dim$应当相等,应当有$m = n$).

可逆矩阵有以下简单性质:

1. 若$A$是可逆方阵,则$A^{ - 1 }$也可逆且$( A^{ - 1 } )^{ - 1 } = A$.

2. 若$A$和$B$都是可逆方阵,则$AB$也可逆且$( AB )^{ - 1 } = B^{ - 1 } A^{ - 1 }$.

##### 初等矩阵
将单位矩阵经过一次初等行变换得到的矩阵称作**初等矩阵**.

对任意$m \times n$矩阵进行初等行变换一定等价于将其左乘一个初等矩阵,原因在于只需要对单位矩阵验证以下性质即可.

1. 倍加变换:$R_i + kR_j \rightarrow R_i , i \ne j$:

$$
\begin{aligned}
E_{ x , y } & = \begin{cases}
k & x = i , y = j \\
1 & x = y \\
0 & \text{ Otherwise }
\end{cases} \\

\end{aligned}
$$

2. 对换变换:$R_i \leftrightarrow R_j$:

$$
\begin{aligned}
E_{ x , y } & = \begin{cases}
1 & x = i , y = j \\
1 & x = j , y = i \\
1 & x = y , x \ne i , x \ne j \\
0 & \text{ Otherwise }
\end{cases} \\

\end{aligned}
$$

3. 倍乘变换:$kR_i \rightarrow R_i , k \ne 0$:

$$
\begin{aligned}
E_{ x , y } & = \begin{cases}
k & x = y , x = i \\
1 & x = y , x \ne i \\
0 & \text{ Otherwise }
\end{cases} \\

\end{aligned}
$$

考虑如下事实:行变换是可逆的,那么初等矩阵显然是可逆的.

这引出下面的定理:

$n \times n$矩阵是可逆的,当且仅当$A$行等价于$I_n$,也即$A$可以通过一系列初等行变换变换为$I_n$.

如果$A$可以通过一系列初等行变换变换为$I_n$,根据初等矩阵的知识,这等价于:

$E_1 E_2 . . . E_p A = I_n$,由于初等矩阵显然可逆,我们有$A = ( E_1 E_{ 2 } . . . E_p )^{ - 1 }$,有$A^{ - 1 } = ( E_1 E_2 . . . E_p )$.

如果$A$是可逆矩阵,显然$A$的简化阶梯形是$I$,也即二者行等价.

由于初等矩阵转置后仍然是初等矩阵,所以$A$可逆等价于$A^T$可逆.此时发现列变换等价于右乘初等矩阵.此外$( A^T )^{ - 1 } = ( A^{ - 1 } )^T$.

##### 求解逆矩阵
若$A$可逆,我们考虑将$A$和$I$放在同一个$n \times 2 n$的矩阵中,记作$\begin{bmatrix}A & I\end{bmatrix}$,它显然行等价于$\begin{bmatrix}I & A^{ - 1 }\end{bmatrix}$.

另外,考虑将$I$写作$\begin{bmatrix}\vec{ e_1 } & \vec{ e_2 } & . . . & \vec{ e_n }\end{bmatrix}$的形式,则我们发现上述过程等价于求若干个形如$A \vec{ x }_i = \vec{ e }_i$的方程的解,并且$A^{ - 1 } = \begin{bmatrix}\vec{ x_1 } & \vec{ x_2 } & . . . & \vec{ x_n }\end{bmatrix}$.

###### Example1(LU分解)
考虑上面对一个矩阵做的过程,考虑把一个矩阵消成上三角的.在此过程中如果没有发生行交换操作,那所做的操作就是一列下三角的操作.因此可以把一个矩阵分解为$A = LU$.当然这里有一些条件才能进行,回头我们再讨论此.

#### 核与像与秩
选取$T : V \rightarrow W$为线性映射,定义$\ker T = T^{ - 1 } ( 0 )$,$\text{ im } \ T = T ( V )$.

容易证明$\ker T$是$V$的线性子空间,$\text{ im } \ T$是$W$的线性子空间.

对于任意$w \in \text{ im } T$,选取任意一个特解$v_0 \in V , T ( v_0 ) = w$,那么容易看出$T^{ - 1 } ( w ) = v_0 + \ker T$.于是$T$是单射当且仅当$\ker T = \{ 0 \}$.

当$\dim V$有限的时候,我们有$\dim V = \dim \ker T + \dim \text{ im } \ T$.

原因是可以选取$\text{ im } \ T$的一组基$\{ \vec{ w }_1 , \cdots \vec{ w }_r \}$,找到其原像$B = \{ \vec{ v }_1 , \cdots , \vec{ v }_r \}$,再取$\ker T$的一组基$C = \{ \vec{ u }_1 , \cdots \vec{ u }_s \}$.

先来证明$B \cup C$是线性无关的.也就是考察$\sum a_j \vec{ v }_j + \sum b_i \vec{ u }_i = 0$.此时两边取像,当然有$\sum a_j \vec{ w }_j + 0 = 0$,于是所有的$a_j = 0$.此时考虑$\ker T$是$V$的子空间,所以当$0 + \sum b_i \vec{ u }_i = 0$的时候也可以推出所有的$b_i = 0$.

再来证明它确实可以生成$V$.考虑$\forall \vec{ v } \in V$,首先$T ( \vec{ v } ) = \sum a_j \vec{ w }_j$,这当然是可以找到的.那么$T ( \vec{ v } - \sum a_j \vec{ v }_j ) = 0$,于是$\vec{ v } - \sum a_j \vec{ v }_j \in \ker T$,于是$\vec{ v } - \sum a_j \vec{ v }_j = \sum b_i \vec{ u }_i$,这就证毕了.利用下面直和分解的原理也可以写作$\ker T \oplus \text{ im } T \cong V$.

而我们知道$T$的单性等价于$\dim \ker T = 0$,满性等价于$\dim \text{ im } \ T = \dim W = \dim V$,于是$\dim W = \dim V$时单性等价于满性.

这样的话我们定义一个线性映射的秩$\text{ rk } \ T = \dim ( \text{ im } \ T )$.

我们来看一个特殊情况$T : F^n \rightarrow F^m$.那此时我们就可以定义这个映射所代表的矩阵$M$的秩.我们注意到$\text{ im } \ T = \text{ span } \{ M \vec{ e }_1 , \cdots M \vec{ e }_{ n } \} = \text{ span } \{ \vec{ m }_1 , \cdots , \vec{ m }_n \}$.也就是$\text{ rk } \ M$其实就是列向量生成的空间的维度.特别地,如果$\text{ rk } \ M = \min ( n , m )$,那么我们称矩阵是满秩的.也就等于其消成简化阶梯型矩阵后的主元数量.那此时$\dim \ker T = n - \text{ rk } \ M$.

容易从上面看出$\text{ rk } ( ST ) \leq \min ( \text{ rk } S , \text{ rk } T )$.且当$T$是满的时候$\text{ rk } ( ST ) = \text{ rk } ( S )$,当$S$是单的时候$\text{ rk } ( ST ) = \text{ rk } ( T )$.

##### Sylvester秩不等式
考虑$T : U \to V , S : V \to W$,则$\text{ rk } ( ST ) \geq \text{ rk } \ S + \text{ rk } \ T - \dim V = \text{ rk } \ T - ( \dim V - \text{ rk } \ S )$.可以认为后者$\dim V - \text{ rk } \ S$实际上是$S$这个映射所带来的维度损失($\ker S$),而这个损失有可能叠到$T$上也可能不叠到$T$上.

对于其证明,考虑$\text{ rk } ( ST )$实际上应当是$S : \text{ im } \ T \to W$的秩,这给出$\text{ rk } ( ST ) = \text{ rk } \ T - \dim ( \text{ im } \ T \cap \ker S )$.

而$\dim ( \text{ im } \ T \cap \ker S ) \leq \dim \ker S = \dim V - \text{ rk } \ S$.于是得证.

其一个特例是$\text{ rk } ( ST ) = 0$时给出$\dim V \geq \text{ rk } \ S + \text{ rk } \ T$.

在上述式子左右两边进行简单变形:

$$
\begin{aligned}
\text{ rk } ( ST ) & \geq \text{ rk } \ T - \dim \ker S \\
\dim U - \text{ rk } ( T ) & \geq \dim U - \text{ rk } \ ( ST ) - \dim \ker S \\
\ker T + \ker S & \geq \ker ( ST )
\end{aligned}
$$

换言之就是线性映射的$\ker$扩张的过程中可能会出现包含的情况,所以扩不到简单相加的情形.

##### Frobenius秩不等式
有$\text{ rk } ( ABC ) \geq \text{ rk } ( AB ) + \text{ rk } ( BC ) - \text{ rk } B$.

不妨设$B : U \to V$,考虑:

$$
\begin{aligned}
\text{ rk } ( ABC ) & = \text{ rk } ( BC ) - \dim ( \text{ im } ( BC ) \cap \ker A ) \\
& \geq \text{ rk } ( BC ) - \dim ( \text{ im } ( B ) \cap \ker A ) \\

\end{aligned}
$$

而我们事实上有$\text{ rk } ( AB ) = \text{ rk } \ B - \dim ( \text{ im } B \cap \ker A )$,于是上述式子得证.

仍然对上式进行变形:

$$
\ker ( AB ) + \ker ( BC ) \geq \ker ( ABC ) + \ker ( B )
$$

这个式子还不能简单从Sylvester不等式推导,但可以见到这仍然是关于$\ker$扩张上的意义.

#### 对偶空间
定义**对偶空间**$V^* = \text{ Hom } ( V , F )$(也可以记作$\check{ V }$),其中$V$是定义在$F$上的向量空间.对偶空间当然是线性空间.

当线性空间是有限维的时候.我们考虑以下事实:

取$V$的一组基$\cal B = \{ \vec{ v }_1 , \cdots , \vec{ v }_n \}$,我们考虑取$\check{ v }_i \in V^\vee$使得$\check{ v }_i ( \vec{ w } ) = a_i$,当$\vec{ w } = \sum_j a_j \vec{ v }_j$.容易验证它们构成一组**对偶基**,这告诉我们$\dim V = \dim V^\vee$.

特别地,当$V = F^n$的时候,容易验证如果取标准基$\cal B$,那么得到的对偶基实际上等价于标准基的转置(也就是列向量改为行向量).这可能会给我们带来错觉,认为对偶空间无非是行向量组成的空间,但如此理解有所偏差.既然是向量空间,就应当理解为列向量,否则线性变换理解为矩阵乘法就会收到阻碍,只是其作用效果等价于先转置为行向量再作矩阵乘法.

如果$T : V \to W$是线性映射,那我们可以定义其**转置映射**$^t T : W^\vee \to V^\vee , \lambda \mapsto \lambda T$.容易验证转置映射仍然是线性映射.另外,如果$S , T$均是线性映射,那么$^t{ ( ST ) } = \ ^t{ T }^t{ S }$.简单画一下交换图表:

$$
\xymatrix{ V \ar[d] \ar[r]^T & W \ar[d] \\
\check{ V } & \check{ W } \ar[l]_{ T^t } }
$$

我们接下来说明转置映射对应的矩阵是原映射的矩阵的转置.

不妨设$\dim V = n , \dim W = m$,原矩阵为$A_{ m \times n }$.

接下来看它们分别对对应基的操作:

不妨设$T ( \vec{ v }_j ) = \sum a_{ i , j } \vec{ w }_i$,则$^t T ( \check{ w }_i )$代表一个映射:

$$
\begin{aligned}
\vec{ v }_k & \mapsto \check{ w_i } T ( \vec{ v }_k ) \\
& = \check{ w_i } ( \sum_h a_{ h , k } w_h ) \\
& = \sum_h a_{ h , k } \check{ w }_i ( w_h ) \\
& = a_{ i , k } \\
& = \sum_{ j } a_{ i , j } \check{ v }_j ( \vec{ v }_k )
\end{aligned}
$$

因此$^t T ( \check{ w }_i ) = \sum_{ j } a_{ i , j } \check{ v }_j$.

如果我们将$\lambda$看成一个$1 \times m$的矩阵,那$\lambda \to \lambda T$不言自明为矩阵乘法.然而正是我们试图总是将空间表示为列向量,因此我们就自然带了一个转置,对于一个$\vec{ v }$,我们对它实际上求的是乘法$^t ( \lambda T ) \vec{ v } = (^t T ) (^t \lambda ) \vec{ v }$的结果,如此一来上述结论便显明了.

我们会在后面谈论矩阵的相抵的部分证明行秩等于列秩这一事实,然而,既然转置实际上表现了对偶空间上的性质,我们理应可以抛开矩阵本身而只从对偶空间的角度理解.换言之,取对偶基是一种非典范的无奈之举,我们会在讨论双线性形式的时候重新讨论于此.

#### 直和分解与分块矩阵
不妨取映射$\sigma : \bigoplus_{ i \in I } V_i \to V , ( v_i )_{ i \in I } \mapsto \sum v_i$,其中$V_i$都是$V$的子空间.容易验证其当且仅当在$V_i \cap ( \sum_{ i \ne j } V_j ) = \{ 0 \}$时是单射.我们将此称作其**直和分解**.如果不满足上述条件一般称作**外直和**,满足的话则称作**内直和**,并容易发现内直和$\bigoplus V_i \cong \sum V_i$.

外直和的一个经典应用是构造$\varphi : V_1 \oplus V_2 \to V , ( \vec{ v }_1 , \vec{ v }_2 ) \mapsto \vec{ v }_1 - \vec{ v }_2$的映射,立刻得到$\ker \varphi = V_1 \cap V_2 , \text{ im } \ \varphi = V_1 + V_2$,于是$\dim V_1 + \dim V_2 = \dim ( V_1 \cap V_2 ) + \dim ( V_1 + V_2 )$.这其实就是第二同构定理的维数表示.

接下来考虑$V = \bigoplus_{ i = 1 }^n V_i , W = \bigoplus_{ j = 1 }^m W_j$,我们可以断言$\text{ Hom } ( V , W ) \cong \bigoplus_{ 1 \leq i \leq n , 1 \leq j \leq m } \text{ Hom } ( V_i , W_j )$.这个的证明只需定义算子$p_i : \bigoplus_{ j } V_j \to V , ( \vec{ v }_j )_{ j \in J } \mapsto \vec{ v }_i$即可.

对于对偶空间,事实上取$W = F$,就会有$V^\vee \cong \bigoplus V_i^\vee$.

接下来考虑映射的合成$T : U \to V , S : V \to W$.不妨设$U \cong \bigoplus U_k , V \cong \bigoplus_j V_j , W \cong \bigoplus_i W_i$,容易验证$( ST )_{ i , k } = \sum_{ j } S_{ i , j } T_{ j , k }$.

此时我们终于意识到了这个东西应当同构于分块矩阵.更具体地,$\text{ Hom } ( V , W ) \cong A_{ n \times m } , A_{ j , i } \cong T_{ j , i } , A_{ j , i } \in M_{ m_j \times n_i } , m_j = \dim W_j , n_i = \dim V_i$.

容易类比普通矩阵定义对角线分块矩阵,上(下)三角分块矩阵之类的东西.并且还容易发现上三角矩阵相乘后仍是上三角,对角线矩阵相乘后仍是对角线.

如果考虑$T \in \text{ End } ( V )$,事实上对角线矩阵有相当漂亮的刻画,只需观察矩阵结构就可以发现此时$T ( V_i ) \subseteq V_i$.上三角矩阵有类似的刻画,观察矩阵结构也可以发现$T ( V_i ) \subseteq \bigoplus_{ j \leq i } V_j$.

下面我们来证明对于上三角分块矩阵$A_{ r \times r }$,如果对角线上的$A_{ i \times i }$均可逆,那么$A_{ r \times r }$的分块矩阵可逆,而且其逆的对角线上的矩阵是一列$A_{ i \times i }^{ - 1 }$.

考虑数学归纳,这样的话就只需要证明$\begin{bmatrix}A_{ 1 , 1 } & A_{ 1 , 2 } \\ 0 & A_{ 2 , 2 }\end{bmatrix}$,其中$A_{ 1 , 1 }$和$A_{ 2 , 2 }$均可逆即可.容易发现取$\begin{bmatrix}A_{ 1 , 1 }^{ - 1 } & - A_{ 1 , 1 }^{ - 1 } A_{ 1 , 2 } A_{ 2 , 2 }^{ - 1 } \\ 0 & A_{ 2 , 2 }^{ - 1 }\end{bmatrix}$即可.

那么反之成不成立呢?是不是此时按照合理的分块方式一定能得到对角线上的矩阵都可逆呢?也是可行的.假设$\begin{bmatrix}A_{ 1 , 1 } & A_{ 1 , 2 } \\ 0 & A_{ 2 , 2 }\end{bmatrix}$可逆,其逆为$\begin{bmatrix}A_{ 1 , 1 } ' & A_{ 1 , 2 } ' \\ A_{ 2 , 1 } ' & A_{ 2 , 2 } '\end{bmatrix}$.容易发现:

$$
\begin{aligned}
\begin{bmatrix}
A_{ 1 , 1 } ' & A_{ 1 , 2 } ' \\
A_{ 2 , 1 } ' & A_{ 2 , 2 } '
\end{bmatrix} \begin{bmatrix}
A_{ 1 , 1 } & A_{ 1 , 2 } \\
0 & A_{ 2 , 2 }
\end{bmatrix} = \begin{bmatrix}
A_{ 1 , 1 } A_{ 1 , 1 } ' & \_ \\
\_ & \_
\end{bmatrix} \\
\begin{bmatrix}
A_{ 1 , 1 } & A_{ 1 , 2 } \\
0 & A_{ 2 , 2 }
\end{bmatrix} \begin{bmatrix}
A_{ 1 , 1 } ' & A_{ 1 , 2 } ' \\
A_{ 2 , 1 } ' & A_{ 2 , 2 } '
\end{bmatrix} = \begin{bmatrix}
\_ & \_ \\
\_ & A_{ 2 , 2 } A '_{ 2 , 2 }
\end{bmatrix} \\

\end{aligned}
$$

于是立刻见到$A_{ 1 , 1 }$和$A_{ 2 , 2 }$都有逆.

#### 基的变换
##### 矩阵的共轭(相似)
考虑线性映射$T : V \to W$,这个线性映射理应和所取的基无关,但如果我们想写出它的矩阵形式,写出来的矩阵当然取决于$V$和$W$取的基分别是什么(从而决定不同的坐标).不妨设$V , W$都是定义在$F$上的,维度分别为$n , m$.现在取它们的有序基$\mathcal{ B } = \{ \vec{ v }_1 , \cdots , \vec{ v }_n \}$,$\mathcal{ C } = \{ \vec{ w }_1 , \cdots , \vec{ w }_m \}$,这样就可以确定一个同构$\mathcal{ M }_{ \mathcal{ B } \to \mathcal{ C } } : \text{ Hom } ( V , W ) \to M_{ m \times n }$.再构造一个同构$\varphi_{ \mathcal{ V } } : F^n \to V , ( x_1 , \cdots , x_n ) \mapsto \sum x_i \vec{ v }_i$.然后我们可以画出如下交换图表(对其交换性的验证可以直接考虑对每个基验证交换性):

$$
\xymatrix{ V \ar[r]^T & W \\
F^n \ar[u]^{ \varphi_{ \mathcal{ B } } } \ar[r]_{ \mathcal{ M }_{ \mathcal{ B } \to \mathcal{ C } } ( T ) } & F^m \ar[u]_{ \varphi_{ \mathcal{ C } } } }
$$

如果我们尝试考虑更换它们的基,理论上说当然会得到不同的矩阵.换言之,我们取另一组有序基$\mathcal{ B } ' = \{ \vec{ v }_1 ' , \cdots , \vec{ v }_n ' \} , \mathcal{ C } ' = \{ \vec{ w }_1 ' , \cdots , \vec{ w }_m ' \}$,这样可以确定另一个同构$\mathcal{ M }_{ B ' \to C ' }$.我们下面试图探索二者之间的关系.

由于选取的有序基的方式不同,我们再构造一个$P_{ B ' \to B } : ( F^n ) ' \to F^n , ( x '_1 , \cdots , x '_n ) \mapsto ( x_1 , \cdots x_n ) , \sum x '_i \vec{ v } '_i = \sum x_i \vec{ v }_i$,容易验证取$P_{ V ' \to V } ={ ( \varphi_V ) }^{ - 1 } \varphi_{ V ' }$即可并且是自同构,并且$( P_{ V \to V ' } )^{ - 1 } = P_{ V ' \to V }$.写成交换图表的话应当形如:

$$
\xymatrix{ & V & \\
( F^n ) ' \ar[ru]^{ \varphi_{ \mathcal{ B } ' } } \ar[rr]_{ P_{ \mathcal{ B } ' \to \mathcal{ B } } } & & F^n \ar[lu]_{ \varphi_{ \mathcal{ B } } } }
$$

这里所有的箭头当然都是可逆的.

事实上,不妨设$\vec{ v }_i ' = \sum_{ j } p_{ j , i } \vec{ v }_j$,也就是将$\vec{ v } '$展开成列向量,对于$\vec{ x } = \sum_{ i } x '_i \vec{ v } '_i$,考虑$\sum_{ i } x '_i \vec{ v } '_i = \sum_{ i } x '_i \sum_{ j } p_{ j , i } \vec{ v }_j = \sum_{ j } \vec{ v }_j \sum_{ i } x '_i p_{ j , i } = \sum_j \vec{ v }_j x_j$,那么$p_{ j , i }$其实就是$P_{ \mathcal{ B } ' \to \mathcal{ B } }$的对应矩阵的第$j$行$i$列元素.此时我们称$P_{ n \times n }$为从有序基$\cal B ' \to \cal B$的**转换矩阵**.事实上容易见到转换矩阵都是可逆的,并且事实上所有的可逆矩阵都可以看作标准基对某个矩阵的转换矩阵.

这样的话,我们就容易见到:$\mathcal{ M }_{ B ' \to C ' } ( T ) = P_{ C \to C ' } \mathcal{ M }_{ B \to C } ( T ) P_{ B ' \to B } = ( P_{ C ' \to C } )^{ - 1 } \mathcal{ M }_{ B \to C } ( T ) P_{ B ' \to B }$.这里一定要注意$\mathcal{ M }_{ B \to C }$是一个将映射$T$变为矩阵$M$的映射.

具体地,我们可以看下面的交换图表:

$$
\xymatrix{ ( F^n ) ' \ar[rrr]^{ \mathcal{ M }_{ B ' \to C ' } ( T ) } \ar[dd]_{ P_{ B ' \to B } } \ar[rd]^{ \varphi_{ B ' } } & & & ( F^m ) ' \ar[dd]^{ P_{ C ' \to C } } \ar[ld]_{ \varphi_{ C ' } } \\
& V \ar[r]^T & W \\
F^n \ar[rrr]_{ \mathcal{ M }_{ B \to C } ( T ) } \ar[ru]_{ \varphi_B } & & & F^m \ar[lu]^{ \varphi_C } }
$$

接下来我们考虑$T \in \text{ End } ( V )$,自然可以导出$\mathcal{ M }_{ B ' \to B ' } ( T ) = P^{ - 1 } \mathcal{ M }_{ B \to B } ( T ) P$,其中$P = P_{ B ' \to B }$.

于是综上,我们称两个矩阵$A_{ n \times n } , B_{ n \times n }$相似,当且仅当存在可逆矩阵$P_{ n \times n }$使得$A = P^{ - 1 } BP$.由上面可以看出来,相似的两个矩阵实际上是不同基下的同种变换.

另外我们注意到,固定$P_{ n \times n }$后定义映射$P : M_{ n \times n } \to M_{ n \times n } , A \mapsto P^{ - 1 } AP$不仅仅是线性同构,还是一个环同构.容易验证其满足环同构的性质.

此时注意到另一个事情:我们知道我们做共轭实际上是换基操作,那么如果我们需要换基,难道我们必须总是对着矩阵做矩阵乘法么?

事实上,我们应当观察如下矩阵:

$$
\begin{bmatrix}
& \vec{ e }_1 & \cdots & \vec{ e }_n \\
\vec{ e }_1 & a_{ 1 , 1 } & \cdots & a_{ n , 1 } \\
\vdots & \cdots & \ddots & \vdots \\
\vec{ e }_n & a_{ 1 , n } & \cdots & a_{ n , n }
\end{bmatrix}
$$

最简单的改变,我们如果想要让它变成在另一组有序基下的操作,我们应当如何操作呢?如果我们带着这些向量去做,我们实际上可以发现:放在列上的这些向量随着列变换而操作,放在行上的这些向量随着行变换的逆变换而改变.这就是更加方便的做相似矩阵的办法.而这事实上也就是因为,任何一个可逆的矩阵都可以拆成若干初等矩阵,因此只需要将$P$拆成初等矩阵两边分别做就可以.初等矩阵求逆当然是平凡的.

##### 矩阵的相抵
我们称两个矩阵$A$和$B$**相抵**,当且仅当$\exists P , Q$均可逆,$A = PBQ$.容易看出相抵是等价关系.由于可逆矩阵等价于若干初等矩阵的乘积,那么$A , B$相抵当且仅当$B$可以通过若干初等行变换和列变换变成$A$.下面我们证明$A , B$相抵等价于$\text{ rk } \ A = \text{ rk } \ B$.

$A , B$相抵的时候当然有$\text{ rk } \ A = \text{ rk } \ B$,因为此时$\text{ rk } \ A = \dim \text{ im ( PBQ ) }$,而$P , Q$都是同构,所以此时有$\text{ rk } \ A = \dim \text{ im } \ B = \text{ rk } \ B$.

当$\text{ rk } \ A = \text{ rk } \ B = k$时,我们考虑二者都可以经过若干次行变换或者列变换变成形如$m_{ i , j } = \begin{cases}1 & i = j \leq k \\ 0 & \text{ otherwise }\end{cases}$.于是二者等价.

类似上面就可以发现$\text{ rk } \ A = \text{ rk } \ A^T$,原因是二者都可以消成上述那种矩阵,而那种矩阵的主元数量显然是确定且相等的.

用矩阵的相抵可以更快证明Frobenius秩不等式,具体地,考虑分块矩阵$\begin{bmatrix}ABC & 0 \\ 0 & B\end{bmatrix}$.考虑$\begin{bmatrix}I & - A \\ 0 & I\end{bmatrix} \begin{bmatrix}ABC & 0 \\ 0 & B\end{bmatrix} \begin{bmatrix}I & 0 \\ C & I\end{bmatrix} = \begin{bmatrix}0 & - AB \\ BC & B\end{bmatrix}$,于是我们知道$\begin{bmatrix}ABC & 0 \\ 0 & B\end{bmatrix}$和$\begin{bmatrix}0 & - AB \\ BC & B\end{bmatrix}$相抵.

而$\text{ rk } \ \begin{bmatrix}ABC & 0 \\ 0 & B\end{bmatrix} = \text{ rk } \ ABC + \text{ rk } \ B$,下面考虑证明$\text{ rk } \ \begin{bmatrix}0 & - AB \\ BC & B\end{bmatrix} \geq \text{ rk } \ AB + \text{ rk } \ BC$,而直接考虑$BC$和$AB$的行,$BC$中线性无关行和$AB$中线性无关行必定也线性无关,而原本线性相关的那些加上右下角的$B$也有可能变得线性无关,于是上述不等式就得证了.

#### 商空间
如果我们考虑在线性空间上定义等价关系$\vec{ v }_1 \sim \vec{ v }_2 \Leftrightarrow T \vec{ v }_1 = T \vec{ v }_2$,其中$T$是一个线性映射,容易发现此时$\vec{ v }_1 - \vec{ v }_2 \in \ker T$.所以其实和$T$本身关系不大,而只和$\ker T$这个子空间有关.

那我们不妨设$U$是$V$的子空间,立刻发现$\vec{ v }_1 \sim \vec{ v }_2 \Leftrightarrow ( \vec{ v }_1 - \vec{ v }_2 ) \in U$是一个等价关系.

在此基础上定义**陪集**$\vec{ v } + U$,容易发现陪集组成了一个新的线性空间(当然要验证良定义),将其称作**商空间**,并且可验证映射$\varphi : V \to V / U , \vec{ v } \mapsto \vec{ v } + U$是线性映射.类比陪群,得知$\ker \varphi = U$,综合前面的讨论得知此时$\vec{ v }_1 \sim \vec{ v }_2 \Leftrightarrow \vec{ v }_1 + U = \vec{ v }_2 + U$.

顺便可以顺手引入**余核**$\text{ coker } ( T ) = W / \text{ im } \ T$,容易发现$T$是满射当且仅当$\text{ coker } ( T ) = \{ 0 \}$.

##### 同态基本定理
下面来抛出另一个命题.对于线性映射$T : V \to W , \varphi : V \to V / U$,如果$U \subseteq \ker T$,那么存在唯一的线性映射$\bar{ T } : V / U \to W$.使得$T = \bar{ T } \varphi$.

这个证明看上去是相当正确的.由于$\varphi$是满射,那么必定有$\bar{ T } ( \vec{ v } + U ) = T \vec{ v }$,这是显然是唯一的映射,也容易看出其是正确的线性映射.具体来讲,存在唯一的线性映射$\bar{ T }$使得下图交换:

$$
\xymatrix{ V \ar[r]^T \ar[d]_\varphi & W \\
V / U \ar[ru]_{ \bar{ T } } }
$$

在上述命题中如果$T$也是满射,也就是$W = \text{ im } \ T$,取$U = \ker T$,那么此时$\bar{ T } \varphi = T$,这必定意味着$\text{ im } \ \bar{ T } = W$.接下来考虑一下$\ker \bar{ T }$的模样,考虑$\bar{ T } ( \vec{ v } + U ) = 0 \Leftrightarrow T ( \vec{ v } ) = 0 \Leftrightarrow \vec{ v } \in U \Leftrightarrow \vec{ v } + U = U$,从而$\ker \bar{ T } = \{ U \}$,于是$\bar{ T }$是同构.

就可以发现对于一个线性映射$T$,$V / U = V / \ker T \cong \text{ im } \ T$,这其实就是同态基本定理在线性空间上的表现.不过线性空间上还多一个标量乘法的运算,需要在群的基础上对此进行简单验证.另外线性空间上还有维数的概念,当然立即断言$\dim V = \dim U + \dim ( V / U )$.值得一提的是这同样也告知我们$U$的基和$V / U$的反像的基合起来就是一组$V$的基.

上述说了这么多其实都在着重于所谓商空间的结构,现在我们看到陪集的定义可以在相当的意义上刻画商空间.适当推广上面的结论可以有:

设$T : V_1 \to V_2$是线性映射,$U_1 \subseteq V_1 , U_2 \subseteq V_2$并构造两个商映射$\varphi_1 : V_1 \to V_1 / U_1 , \varphi_2 : V_2 \to V_2 / U_2$,并且$T ( U_1 ) \subseteq U_2$,那么存在唯一的线性映射$\bar{ T } : V_1 / U_1 \to V_2 / U_2$使得$T \varphi_2 = \bar{ T } \varphi_1$.具体地,$\bar{ T } ( \vec{ v } + U_1 ) = T \vec{ v } + U_2$.

如果再推广上述结论的话还可以来多个,具体可以看下面的交换图表:

$$
\xymatrix{ V_1 \ar[rr]^T \ar[d]_{ \varphi_1 } & & V_2 \ar[rr]^S \ar[d]_{ \varphi_2 } & & V_3 \ar[d]_{ \varphi_3 } \\
V_1 / U_1 \ar[rr]^{ \bar{ T } }_{ \ker \bar{ T } = T^{ - 1 } ( U_2 ) / U_1 } & & V_2 / U_2 \ar[rr]^{ \bar{ S } }_{ \ker \bar{ S } = S^{ - 1 } ( U_3 ) / U_2 } & & V_3 / U_3 \\
 }
$$

##### 第一同构定理
将群论中的第一同构定理(不过线性空间下没有正规的概念)拿过来,即对于线性空间$V$的一个子空间$U$,在典范同态$\pi : V \rightarrow V / U$下,我们有:

1. $V$的包含$U$的子空间$W$和$V / U$的子空间$\bar{ W }$在$\pi$下一一对应.不妨设此对应为$\varphi : W \mapsto \bar{ W }$.

1. 此对应是严格保序的,也就是$W_1 \subseteq W_2 \Leftrightarrow \bar{ W }_1 \subseteq \bar{ W }_2$.

2. 若有$U \subseteq W \subseteq V$,则$V / W \cong ( V / U ) / ( W / U )$.

考虑(1)(2)的证明:

先证明单射,对于两个包含$U$的子空间$W_1 \ne W_2$来说,不妨设$W_1 \nsubseteq W_2$,$\exists \vec{ a } \in W_1 , \vec{ a } \notin W_2$.此时考虑$\bar{ a }$,注意到如果$\bar{ a } \in \bar{ W_2 }$,那么$\vec{ a } + U \in W_2 + U$.这表明$\exists \vec{ b } \in W_2$,$\vec{ a } + U = \vec{ b } + U$,此时$\vec{ a } - \vec{ b } \in U \subseteq W_2$,那么$\vec{ a } \in W_2$,这就不符合了,于是当然$\bar{ W_1 } \ne \bar{ W_2 }$并且(2)上严格保序.

证明单射还有一种办法是考虑只需证明$\pi^{ - 1 } ( \pi ( W ) ) = W$即可,而$W \subseteq \pi^{ - 1 } ( \pi ( W ) )$是显然的.又考虑$\forall \vec{ v } \in \pi^{ - 1 } ( \pi ( W ) ) , \pi ( \vec{ v } ) \in \pi ( W )$,这意味着$\exists \vec{ w } \in W , \pi ( \vec{ v } ) = \pi ( \vec{ w } )$,于是$\vec{ v } \in \vec{ w } + \ker \pi = \vec{ w } + U \subseteq \vec{ w } + W = W$.这样就证明了$W \supseteq \pi^{ - 1 } ( \pi ( W ) )$.

接下来要证明满射,也就是证明$\forall N \leq V / U , U \leq \pi^{ - 1 } ( N ) \leq G$.

首先要证明$\pi^{ - 1 } ( N )$是一个子空间,这个是容易验证的.

而由于保序性已经被证明了,我们就可以注意到由于$\vec{ 0 } \in N$,于是有$\bar{ 0 } = U \leq \pi^{ - 1 } ( N )$.

对于(3):

由同态基本定理,考虑构造满射$\varphi : V / U \rightarrow V / W$,只需证明$\ker \ \varphi = W / U$即可.

直觉上会认为映射$\varphi : \vec{ v } + U \mapsto \vec{ v } + W$即可,下面验证其满足上述性质:

首先验证良定义,$\vec{ v }_1 + U = \vec{ v }_2 + U \Leftrightarrow \vec{ v }_1 - \vec{ v }_2 \in U \subseteq W \Rightarrow \vec{ v }_1 + W = \vec{ v }_2 + W$.

然后验证线性映射,这个验证过程很平凡.

其满性显然.只需验证其$\ker \varphi = W / U$即可,而考虑$\varphi ( \vec{ v } + U ) = W \Leftrightarrow \vec{ v } + W = W \Leftrightarrow \vec{ v } \in W \Leftrightarrow \vec{ v } + U \in W / U$.这样就证明了上述结论.

##### 第二同构定理
$V , W$是一个更大的线性空间的子空间,那么我们有:

$V / ( V \cap W ) \cong ( V + W ) / W , \vec{ v } + ( V \cap W ) \mapsto \vec{ v } + W$.

考虑$\varphi : V \to ( V + W ) / W , \vec{ v } \mapsto \vec{ v } + W$,那么显然$\varphi ( \vec{ v } ) = W \Leftrightarrow \vec{ v } \in V \cap W$,于是$\ker \varphi = V \cap W$.

接下来只需要证明它是满射就可以了.考虑$\forall \vec{ v } + \vec{ w } + W = \vec{ v } + W \in ( V + W ) / W$,这当然是满射.

第二同构定理的一个平凡推论是$\dim ( V + W ) = ( \dim V ) + ( \dim W ) - \dim ( V \cap W )$.

第二同构定理的另一个推论是说如果$V = U \oplus W$,那么$W \cong V / U$.原因是$U + W = V$并且$U \cap W = \{ \vec{ 0 } \}$.换言之取商是直和的逆运算.

第二同构定理证明的另一种思路是考虑$\varphi : V \times W \to V + W , ( \vec{ v } , \vec{ w } ) \mapsto ( \vec{ v } - \vec{ w } )$,此时注意到$\text{ im } \varphi = ( V + W ) , \ker \varphi = V \cap W$.

##### 旗
回看我们一开始拿到的映射$T : V \to V '$和映射$\bar{ T } : V / U \to V ' / U '$.也就是下面这个交换图表:

$$
\xymatrix{ V \ar[r]^T \ar[d]_{ \varphi } & V ' \ar[d]^{ \varphi ' } \\
V / U \ar[r]_{ \bar{ T } } & V ' / U ' \\
 }
$$

现在我们尝试用矩阵刻画$T$和$\bar{ T }$之间的关系.首先我们需要开始取基,不妨取$\{ \vec{ u }_1 , \cdots , \vec{ u }_k \}$是$U$的一组有序基,$\{ \bar{ v }_1 , \cdots , \bar{ v }_m \}$是$V / U$的一组有序基.$\{ \vec{ u }_1 ' , \cdots , \vec{ u }_{ k ' } ' \}$是$U '$的一组有序基,$\{ \bar{ v }_1 ' , \cdots , \bar{ v }_{ m ' } ' \}$是$V ' / U '$的一组有序基.此时由于$\ker \varphi = U$,我们之前证明$\ker$的维度关系的时候已经说明过,$\{ \vec{ u }_1 , \cdots , \vec{ u }_k , \vec{ v }_1 , \cdots \vec{ v }_m \}$实际上是$V$的一组有序基.尝试取出一个矩阵$A \in M_{ ( k ' + m ' ) \times ( k + m ) }$来作为$T$对应的矩阵.此时将$A$分块,我们将其写作$A = \begin{bmatrix}A_{ U \to U ' } & A_{ V / U \to U ' } \\ A_{ U \to V ' / U ' } & A_{ V / U \to V ' / U ' }\end{bmatrix}$.由于$T ( U ) \subseteq U '$,我们知道$A_{ U \to V ' / U ' } = 0$,而又有$A_{ V / U \to V ' / U ' }$当然就是$\bar{ T }$对应的矩阵$\bar{ A }$.总结一下就是$A = \begin{bmatrix}A_{ U \to U ' } & \_ \\ 0 & \bar{ A }\end{bmatrix}$.

如此观察可以继续推广并得到**旗**的概念.我们称$V$的**旗**是一列子空间$\{ \vec{ 0 } \} = U_0 \subsetneq \cdots \subsetneq U_d = V$.特别地,如果$d = \dim V$,则称其为**完备旗**,容易见到完备旗一定满足$\dim V_i = i$.对于给定的旗,如果线性映射$T \in \text{ End } ( V )$对$\forall 0 \leq i \leq m$满足$T ( V_i ) \subseteq V_i$,则称$T$保持此旗.

如果一个映射$T : V \to V '$满足$\forall h , T ( U_h ) \subseteq U '_h$,那我们可以对每个$U_h / U_{ h - 1 }$和$U '_h / U '_{ h - 1 }$选定有序基,那么得到的矩阵$A$应当是一个分块上三角矩阵,而且$A_{ i , i }$实际上就是$\bar{ T }_h : U_h / U_{ h - 1 } \to U '_h / U '_{ h - 1 }$所对应的矩阵.

##### 不变子空间
给定线性映射$T \in \text{ End } ( V )$,如果子空间$U \subseteq V$满足$T ( U ) \subseteq U$,那我们称$U$是在$T$下的一个**不变子空间**.根据前面我们已经讨论过的内容,得到下面这个交换图表:

$$
\xymatrix{ V \ar[r]^T \ar[d]_{ \varphi } & V \ar[d]^{ \varphi ' } \\
V / U \ar[r]_{ \bar{ T } } & V / U \\
 }
$$

其中$\bar{ T }$是唯一的.

提取$T |_U$为只考虑$T$在线性空间$U$上的作用得到的线性映射,上述交换图表给出以下矩阵:$T \cong \begin{bmatrix}T |_U & \_ \\ 0 & \bar{ T }\end{bmatrix}$.这个形式更好的用处是结合之后的迹/行列式/特征多项式等概念给出$T , T |_U , \bar{ T }$三者的关系:事实上$\det ( T |_U ) \det ( \bar{ T } ) = \det ( T ) , \text{ tr } ( T |_U ) + \text{ tr } ( \bar{ T } ) = \text{ tr } ( T )$.

### 行列式
#### 置换(排列)
不妨假设$X$是一个非空集合,那么我们定义其**置换集合**$S_X = \{ \sigma : X \to X , \sigma \ \text{ is a bijection } \}$.容易看出$S_X$对置换复合构成一个群.

我们也可以用$\left ( \begin{matrix}1 & \cdots & n \\ \sigma ( 1 ) & \cdots & \sigma ( n )\end{matrix} \right )$.

我们还可以定义**轮换**为$\left ( \begin{matrix}1 & 2 & \cdots & n - 1 & n \\ 2 & 3 & \cdots & n & 1\end{matrix} \right )$.

还可以定义**对换**$( i , j ) = \left ( \begin{matrix}1 & \cdots & i & \cdots & j & \cdots & n \\ 1 & \cdots & j & \cdots & i & \cdots & n\end{matrix} \right )$,其中$i \ne j$.

在此基础上定义**单对换**$s_i = ( i , i + 1 )$.

在置换的基础上可以定义逆序对:$\text{ Inv }_\sigma = \{ ( i , j ) \mid 1 \leq i < j \leq n , \sigma ( i ) > \sigma ( j ) \}$,并定义逆序数$l ( \sigma ) = | \text{ Inv }_\sigma |$.容易看出$l ( \sigma ) = 0 \Leftrightarrow \sigma = \text{ id }$,$l ( \sigma ) = \frac{ n ( n - 1 ) }{ 2 } \Leftrightarrow \sigma ( k ) = n - k + 1$.并且注意到$l ( \sigma ) = l ( \sigma^{ - 1 } )$,原因是$( i , j ) \mapsto ( \sigma ( j ) , \sigma ( i ) )$是双射.

容易发现一个$\sigma$可以最少通过$l ( \sigma )$次单对换变为$\text{ id }$,原因是每次交换一对$( i , i + 1 )$当$\sigma ( i ) > \sigma ( i + 1 )$时会让逆序数减少$1$.或者说更一般地$| l ( \sigma ) - l ( s_i \sigma ) | = 1$.由于$( s_i )^{ - 1 } = s_i$,容易发现$| l ( \sigma ) - l ( \sigma s_i ) | = 1$.而事实上$| l ( \sigma ) - l ( ( i , j ) \sigma ) | \in \text{ odd }$.这意味着对换会改变逆序数的奇偶性.所以容易看出来一个置换$\sigma$如果能拆成若干个对换,这些对换的数量的奇偶性一定与$l ( \sigma )$相同.

而由于可以如上分解并合并,我们知道$l ( \sigma \tau )$和$l ( \sigma ) + l ( \tau )$的奇偶性必然相同.

于是我们可以定义$\text{ sgn } ( \sigma ) = ( - 1 )^{ l ( \sigma ) }$,并且容易发现$\text{ sgn } ( \sigma \tau ) = \text{ sgn } ( \sigma ) \text{ sgn } ( \tau )$.还可以看出$\text{ sgn } ( \sigma^{ - 1 } ) = \text{ sgn } ( \sigma )$,那么立刻有$\text{ sgn } ( \sigma^{ - 1 } \tau \sigma ) = \text{ sgn } ( \tau )$.用此可以定义奇偶置换的概念,也就是$l ( \sigma ) \in \text{ odd }$的置换$\sigma$是奇置换.

#### 交错形式
尝试刻画一种交错形式$D : V^m \to F$,其中$V$是定义在$F$上的线性空间,满足以下性质:

1. 其对每一个分量都是线性的.也即固定其他$m - 1$个向量后,剩下的那一个向量的改变也会引起线性的改变.换言之$D ( \cdots , t \vec{ v }_i , \cdots ) = tD ( \cdots , \vec{ v }_i , \cdots ) , D ( \cdots , \vec{ v }_i + v_i ' , \cdots ) = D ( \cdots , \vec{ v }_i , \cdots ) + D ( \cdots , \vec{ v }_i ' , \cdots )$.

2. 如果有两个分量相等,那么得到的是$0$.

在上述基础上容易发现交错形式也是线性空间,不妨将其记作$D_{ V , m }$.

容易发现$D_{ V , 1 } = V^\vee$.

初看会觉得(2)是一个很奇怪的事情,但实际上其定义了一种很好的性质:首先我们应当需要一种东西使得如果有一个分量是$\vec{ 0 }$则给出$0$,但如果直接这么定义的话容易看到难以判定一个东西在没有分量为$0$的时候是否为$0$,因此这个定义的性质更为优良.事实上,我们有以下性质:

1. 对于一个固定的$D$而言,将其某一个分量乘以若干倍加到另一个分量上不会引起取值的变化.

2. 对于一个固定的$D$而言,将某一个分量乘以$t$倍,则取值需要乘以$t$倍.

3. 对于一个固定的$D$而言,如果其收到的向量组是线性相关的,那么取值为$0$.

4. 作为(3)的推论,如果$m > \dim V$,那么$D_{ V , m } = \{ 0 \}$.

5. 对于一个固定的$D$而言,$D ( \cdots , \vec{ v }_i , \cdots , \vec{ v }_j , \cdots ) = - D ( \cdots , \vec{ v }_j , \cdots , \vec{ v }_i , \cdots )$.

6. 作为(5)的推论,$D ( \vec{ v }_{ \sigma^{ - 1 } ( 1 ) } , \cdots , \vec{ v }_{ \sigma^{ - 1 } ( m ) } ) = \text{ sgn } ( \sigma ) D ( \vec{ v }_1 , \cdots , \vec{ v }_m )$.

其它几个都是平凡的,下面考虑(5)的证明,我们有:

$$
\begin{aligned}
0 & = D ( \cdots , \vec{ v }_i + \vec{ v }_j , \cdots , \vec{ v }_j + \vec{ v }_i , \cdots ) \\
& = D ( \cdots , \vec{ v }_i , \cdots , \vec{ v }_j , \cdots ) + D ( \cdots , \vec{ v }_i , \cdots , \vec{ v }_i , \cdots ) \\
+ D ( \cdots , \vec{ v }_j , \cdots , \vec{ v }_i , \cdots ) + D ( \cdots , \vec{ v }_j , \cdots , \vec{ v }_j , \cdots ) \\
& = D ( \cdots , \vec{ v }_i , \cdots , \vec{ v }_j , \cdots ) + D ( \cdots , \vec{ v }_j , \cdots , \vec{ v }_i , \cdots )
\end{aligned}
$$

下面我们定义$D_V = D_{ V , n }$,其中$n = \dim V$,取出其一组有序基$\{ \vec{ e }_1 , \cdots , \vec{ e }_n \}$.对于一组向量$\{ \vec{ v }_{ 1 } , \cdots , \vec{ v }_n \}$,尝试将其分解为$\vec{ v }_i = \sum_{ j } a_{ i , j } \vec{ e }_j$.此时逐个展开$D ( \vec{ v }_1 , \cdots , \vec{ v }_n )$中的向量,理应得知:

$$
\begin{aligned}
D ( \vec{ v }_1 , \cdots , \vec{ v }_n ) & = \sum_{ \sigma \in S_n } \prod_{ k = 1 }^n a_{ k , \sigma ( k ) } D ( \vec{ e }_{ \sigma ( 1 ) } , \cdots , \vec{ e }_{ \sigma ( n ) } ) \\
& = \sum_{ \sigma \in S_n } \text{ sgn } ( \sigma ) \prod_{ k = 1 }^n a_{ k , \sigma ( k ) } D ( \vec{ e }_1 , \cdots , \vec{ e }_n )
\end{aligned}
$$

这说明任何一个$D$实际上只由其在基上的作用$D ( \vec{ e }_1 , \cdots , \vec{ e }_n )$唯一确定.因此$\dim D \leq 1 = \dim F$,原因是线性映射$D_V \to F , D \mapsto D / D ( \vec{ e }_1 , \cdots , \vec{ e }_n )$是单射.事实上只要$\dim V \geq 1$,当然可以如上构造$D_e = \sum_{ \sigma \in S_n } \text{ sgn } ( \sigma ) \prod_{ k = 1 }^n a_{ k , \sigma ( k ) } \in D_V \setminus \{ 0 \}$,于是$\dim D \ne 0$,于是$\dim D = 1$.

验证$D_e$映射符合定义的过程较为无聊,验证有相同元素则为$0$是考虑交换它们后$\text{ sgn }$会变号,而还容易简单验证$D_e ( \vec{ e }_1 , \cdots \vec{ e }_n ) = 1$.

#### 定义
考虑有限维的$V , W$,并考虑映射$T^* : D_{ W , m } \to D_{ V , m } , D \mapsto ( ( T^* D ) : ( \vec{ v }_1 , \cdots , \vec{ v }_m ) \mapsto D ( T \vec{ v }_1 , \cdots T \vec{ v }_m )$.

现在取$W = V , m = n = \dim V$,于是$T \in \text{ End } ( V ) , T^* \in \text{ End } ( D_V )$,由于$\dim D_V = 1$,于是$T^*$必然是伸缩映射$D \mapsto tD$,那么这个$t$其实就是行列式.更确切地说,我们定义:

$$
\begin{aligned}
T^* ( D ) & = ( \det T ) D \\
D ( T \vec{ v }_1 , \cdots , T \vec{ v }_n ) & = \det T \times D ( \vec{ v }_1 , \cdots \vec{ v }_n )
\end{aligned}
$$

那么既然其是伸缩映射,我们就当然注意到$\det T = \frac{ D_e ( T \vec{ e }_1 , \cdots , T \vec{ e }_n ) }{ D_e ( \vec{ e }_1 , \cdots , \vec{ e }_n ) } = D_e ( T \vec{ e }_1 , \cdots , T \vec{ e }_n )$.

只需简单利用定义就可以注意到行列式的以下性质:

1. $\det \text{ id }_V = 1$.

2. $\det ( ST ) = \det S \times \det T$.

3. $T$可逆时,$\det ( T^{ - 1 } ) = ( \det T )^{ - 1 }$.

3. $T$不可逆时,$\det T = 0$.

应当警觉的一个事情是我们上面给出的计算$D$的方式是观察$D$在某一组基上的作用,而我们理应判定当选取的基不同的时候$\det T$的唯一性,我们还应当确认的是我们可以将一个$\dim V = n$的线性空间用某一个基打到$F^n$后也不会有问题,从而引出矩阵的行列式的定义.综上,我们应当验证当以下图表是交换时:

$$
\xymatrix{ V \ar[r]^T \ar[d]_S & V \ar[d]^S \\
W \ar[r]_{ T ' } & W }
$$

也即$T ' = STS^{ - 1 }$时$\det T = \det T '$.那么这由行列式的乘法性质当然就可以予以说明.

综上就可以定义矩阵的行列式,我们称方阵$A_{ n \times n }$的行列式是一个将其映射到标量的函数,记作$\det ( A )$或者$| A |$.由于一般自然地将$\text{ End } ( V ) \cong M_{ n \times n }$,于是从上面的计算方式容易见到$\det A = \sum_{ \sigma \in S_n } \text{ sgn } ( \sigma ) \prod_{ k = 1 }^n a_{ k , \sigma ( k ) } = \sum_{ \sigma \in S_n } \text{ sgn } ( \sigma ) \prod_{ k = 1 }^n a_{ \sigma ( k ) , k }$.

我们应当容易从上述过程中看出以下事实:

1. $\det A = \det A^T$.

2. $\det I_{ n \times n } = 1$.

3. $\det A$可以看作列向量(或行向量)上的某种交错形式.

4. 如果两个矩阵相似,那么它们的$\det$相等.

而由于行列式是某种交错形式,我们立即可以断言当对其作三种初等行变换的时候:

1. (倍加变换):行列式的值不变.

2. (对换变换):行列式的值取反.

3. (倍乘变换):行列式的值取相同倍数.

说到这里要声明的某些事情是,我们应当将行列式看作某种交错形式,而并非将所有的交错形式都看作行列式.原因是交错形式一般带有单位,而行列式只是单纯的比例常数.例如我们用行列式求面积的时候,算的其实并不是面积,而是其相对于单位面积的比例常数.

#### 余子式与代数余子式
对于矩阵$A$,$A$的**余子式**$M_{ i , j }$定义为$A$去掉第$i$行第$j$列的矩阵的行列式.其中形如$M_{ i , i }$的余子式称为**主余子式**.

对于矩阵$A$,其**代数余子式**(又称**余因子**)$C_{ i , j } = ( - 1 )^{ i + j } M_{ i , j } \\$.

那么我们有一种计算行列式的方式是**代数余子式展开**:

$$
\det A = \sum_{ k = 1 }^n a_{ i , k } C_{ i , k } = \sum_{ k = 1 }^n a_{ k , j } C_{ k , j }
$$

原因是$\sigma ( i ) = j$的位置对逆序对的贡献的奇偶性等同于$i + j$的奇偶性.每次我看这个结论都无法第一时间理解这个性质.但你可以考虑以$\sigma ( i ) = j$作为分界线,然后发现此时怎么交换其它的部分,这个点的奇偶性贡献都是不变的.因此直接考虑在最普通的排列上挪一下,奇偶性就等价于$i - j$的奇偶性了.

值得一提的是我们其实还有类似的公式,当$i \ne j$的时候:

$$
0 = \sum_{ k = 1 }^n a_{ i , k } C_{ j , k } = \sum_{ k = 1 }^n a_{ k , i } C_{ k , j }
$$

原因是直接定义一个矩阵$B$使得$B$的第$j$行与$A$的第$i$行相等,其余行照搬$A$,那么展开也是上面这个东西,而由于$B$有两行相等,所以$\det B = 0$.

代数余子式构成的矩阵$C = [ C_{ i , j } ]$称为**余子矩阵**.其转置称为**伴随矩阵**,我们不妨将伴随矩阵直接记作$\check{ A }$,也有时记其为$\text{ adj } ( A )$.

其满足$A \check{ A } = \check{ A } A = ( \det A ) I$.二者类似,只考虑证明前者:

考虑前者的第$i$行第$j$列应该是$\sum_{ k } a_{ i , k } C_{ j , k }$,我们早在前面就证明过当$i = j$的时候这个东西应当为$\det A$,反之为$0$.

由上面立刻得到推论为$A^{ - 1 } = \frac{ \check{ A } }{ \det A }$.

我们可能还希望去了解$\check{ A }$的$\text{ rk }$与$\det$.事实上我们给出以下若干命题:

1. $\det \check{ A } = ( \det A )^{ n - 1 }$.

2. $\text{ rk } ( \check{ A } ) = \begin{cases}n & \text{ rk } \ A = n \\ 1 & \text{ rk } \ A = n - 1 \\ 0 & \text{ rk } \ A < n - 1\end{cases}$.

(1)无非是(2)的推论,下面只看(2).

(2)的话,由于$\det A \ne 0 \Rightarrow \det \check{ A } \ne 0$,所以当$A$满秩的时候也很显然.

当$\text{ rk } \ A < n - 1$的时候,由于取出的所有余子式都线性相关,于是此时$\check{ A }$是$0$矩阵.

当$\text{ rk } \ A = n - 1$的时候,首先证明其$\text{ r } k \geq 1$,也就是只需证明至少存在一个余子式不为$0$.取出$n - 1$个线性无关的列组成一个$n \times ( n - 1 )$大小的矩阵.我们之前已经声明过其行秩等于列秩,这样就可以删去其中一行得到一个$( n - 1 ) \times ( n - 1 )$的满秩矩阵,这个矩阵的行列式不为零.

于是其$\text{ rk } \geq 1$,我们又根据Frobenius秩不等式,知道考虑$0 = \text{ rk } \ ( AI \check{ A } ) \geq \text{ rk } \ A + \text{ rk } \ \check{ A } - \text{ rk } \ I$,于是$\text{ rk } \ \check{ A } \leq 1$.

#### 行列式定向
我们知道我们用行列式求平行四边形面积的时候求的是所谓有向面积,问题在于这个方向具体是什么样子呢.

容易见得不同的基对应的矩阵行列式应当是相等的.但基的变换可能拥有不同的行列式.我们曾经研究过$P_{ \mathcal{ B } \to \mathcal{ B } ' }$这个坐标变换矩阵的若干性质,而这个变换当然是同构的,所以这个矩阵行列式理应非零,那它就会天然带有正负的区别.我们定义一种基上的等价关系$\sim$,或者称为**同定向的**.具体来说$\mathcal{ B } \sim \mathcal{ B } ' \Leftrightarrow \det P_{ \mathcal{ B } ' \to \mathcal{ B } } > 0$,由于行列式的乘法性质,立刻见到这将所有的基分为了两类.对于$F^n$来说,我们一般将其标准基所在的那一类称为**标准定向**.这就是为什么我们求行列式的时候会出现正负的差别.

#### 克拉默(Cramer)法则
若$A$可逆,用$A_i ( \vec{ b } )$表示将$A$的第$i$列替换为$\vec{ b }$后的矩阵,则$A \vec{ x } = \vec{ b }$的唯一解可由下式给出:

$$
x_i = \cfrac{ \det A_i ( \vec{ b } ) }{ \det A }
$$

证明:由于$A$实际上可以看作线性映射不难发现$A \times I_i ( \vec{ x } ) = A_i ( \vec{ b } )$,则根据行列式的乘法原理,有$( \det A ) ( \det I_i ( \vec{ x } ) ) = \det A_i ( \vec{ b } )$.而$\det I_i ( \vec{ x } ) = x_i$,于是该定理显然.

或者直接考虑$\vec{ x } = A^{ - 1 } \vec{ b } = \frac{ 1 }{ \det A } \check{ A } \vec{ b }$,于是$x_i = \frac{ 1 }{ \det A } \sum_{ k = 1 }^n C_{ k , i } b_k$,后者刚好是$A_i ( \vec{ b } )$沿第$i$列作余子式展开后的结果.

值得一提的是,Cramer法则揭示了$\det \in R^\times$的环上的矩阵必有逆矩阵

#### 一些特殊行列式
##### 置换矩阵
对于一个置换$\sigma$,我们记它的置换矩阵$P_\sigma$满足$P_{ \sigma , i , j } = \begin{cases}1 & i = \sigma ( j ) \\ 0 & \text{ otherwise }\end{cases}$,容易发现它对应了线性映射$\vec{ e }_j \mapsto \vec{ e }_{ \sigma ( j ) }$.容易看出$P_\sigma P_\tau = P_{ \sigma \tau }$并且$P_{ \sigma^{ - 1 } } = ( P_\sigma )^{ - 1 } = ( P_\sigma )^T$.容易由行列式理论得知$| P_\sigma | = \text{ sgn } ( \sigma )$.

##### 上三角矩阵
用代数余子式展开立刻见到上三角矩阵的行列式就是其对角线的乘积.这比用高斯消元更好,因为对角线未必可逆.

##### 分块上三角矩阵
也即$\left | \begin{matrix}A_{ 1 , 1 } & \cdots & \cdots \\ 0 & \ddots & \vdots \\ 0 & 0 & A_{ n , n }\end{matrix} \right | = \prod_{ i = 1 }^n \det A_{ i , i }$.

证明考虑数学归纳,先把左上角那一坨变成一个分块矩阵,那我们只需要证明$\left | \begin{matrix}A_{ 1 , 1 } & A_{ 1 , 2 } \\ 0 & A_{ 2 , 2 }\end{matrix} \right | = \det A_{ 1 , 1 } \det A_{ 2 , 2 }$.

这个怎么证明呢?考虑直接套定义,我们知道行列式的定义是:$\det A = \sum_{ \sigma \in S_n } \text{ sgn } ( \sigma ) \prod_{ k = 1 }^n a_{ k , \sigma ( k ) }$,此时意识到如果$k > n_1 , \sigma ( k ) \leq n_1$那么此时$a_{ k , \sigma ( k ) } = 0$,所以这个$\sigma$应当分为两部分,一部分是$[ 1 , n_1 ]$部分内部交换,另一部分是$[ n_1 + 1 , n_1 + n_2 ]$内部交换,这样我们把$\sigma = \sigma_1 \sigma_2$,上述表达式变为:

$$
\begin{aligned}
\det A & = \sum_{ \sigma \in S_n } \text{ sgn } ( \sigma_1 ) \prod_{ k = 1 }^{ n_1 } a_{ k , \sigma_1 ( k ) } \text{ sgn } ( \sigma_2 ) \prod_{ k = n_1 + 1 }^{ n_1 + n_2 } a_{ k , \sigma_2 ( k ) } \\
& = \det A_{ 1 , 1 } \det A_{ 2 , 2 }
\end{aligned}
$$

##### 范德蒙德(Vandermonde)行列式
即$\left | \begin{matrix}1 & \cdots & 1 \\ x_1 & \cdots & x_n \\ \vdots & & \vdots \\ x_1^{ n - 1 } & \cdots & x_n^{ n - 1 }\end{matrix} \right | = \prod_{ 1 \leq i < j \leq n } ( x_j - x_i )$.

这个怎么求呢?我们先逐个消第一列(先用第$n - 1$行消第$n$行,再用第$n - 2$行消第$n - 1$行,以此类推),我们得到的就是以下行列式:

$$
\left | \begin{matrix}
1 & 1 & \cdots & 1 \\
0 & x_2 - x_1 & \cdots & x_n - x_1 \\
0 & \vdots & & \vdots \\
0 & x_2^{ n - 1 } - x_1 x_2^{ n - 2 } & \cdots & x_n^{ n - 1 } - x_1 x_n^{ n - 2 }
\end{matrix} \right |
$$

对这个的第一列作代数余子式展开,知道它应当等于:

$$
\begin{aligned}
& \left | \begin{matrix}
x_2 - x_1 & \cdots & x_n - x_1 \\
\vdots & & \vdots \\
x_2^{ n - 1 } - x_1 x_2^{ n - 2 } & \cdots & x_n^{ n - 1 } - x_1 x_n^{ n - 2 }
\end{matrix} \right | \\
= & \left | \begin{matrix}
1 & \cdots & 1 \\
\vdots & & \vdots \\
x_2^{ n - 2 } & \cdots & x_n^{ n - 2 }
\end{matrix} \right | \prod_{ 1 < j \leq n } ( x_j - x_1 )
\end{aligned}
$$

然后数学归纳就证完了.

##### 一类分块矩阵
考虑$\left | \begin{matrix}I & B \\ A & I\end{matrix} \right | = \det ( I - AB )$.此时观测其对基的作用,注意到其和$\begin{bmatrix}I & A \\ B & I\end{bmatrix}$无非是在不同基下的同种线性映射,于是$\det ( I - AB ) = \det ( I - BA )$.

#### 特征多项式
自映射$T = \text{ End } ( V )$本身作为一个定义了乘法和加法的元素,理应可以嵌入多项式结构中.因此不妨定义多项式$f ( T ) = \sum_{ k = 0 }^m a_k T^k$.

此时设$\dim V = n$有限,那么$\dim T = n^2$,考虑以下$n^2 + 1$个此空间上的向量$T^0 , \ldots , T^{ n^2 }$,它们必定线性相关,也就是存在一组不全为$0$的$\{ c_k \}$使得$f ( T ) = \sum_{ k = 0 }^{ n^2 } c_k T^k = 0$.

如果$T$可逆,那我们考虑可以取一个满足上述条件的$f$使得$c_0 \ne 0$,原因是可以不断两边除去$T$.那最终就会拿到$c_0 \text{ id } + \sum_{ k = 1 }^m c_k T^k = 0$,对此式子再简单处理可知以下结论:对于一个可逆的$T$,存在一个多项式$g ( T )$使得$T^{ - 1 } = g ( T ) = - \frac{ 1 }{ c_0 } \sum_{ k = 1 }^{ m } c_k T^{ k - 1 }$.并且注意到$\deg g ( T ) \leq n^2$.

将此定义挪到矩阵上,定义其特征多项式$\text{ char }_A ( x ) = \det ( xI - A )$.由定义立刻得知$\text{ char }_A ( x ) = \sum_{ \sigma } \text{ sgn } ( \sigma ) \prod_{ i = 1 }^n ( [ i = \sigma ( i ) ] x - a_{ i , \sigma ( i ) } )$.于是容易发现$\text{ char }_A ( 0 ) = ( - 1 )^n \det A$.并且这个多项式应该首项为$1$.

容易观察到相似的矩阵应当有相同的特征多项式,原因是$\det ( xI - A ) = \det ( P^{ - 1 } ( xI - B ) P ) = \det ( xI - B )$.这意味着其和基的选取无关,因此特征多项式可以拓展到一般的线性映射上.

另外转置不影响特征多项式的取值,原因是转置也不影响$\det$的取值.

分块上三角矩阵的特征多项式就是对角线矩阵特征多项式的乘积.

我们还可以有以下性质:当$A \in M_{ m \times n } , B \in M_{ n \times m }$,则$\cfrac{ \text{ char }_{ AB } }{ x^m } = \cfrac{ \text{ char }_{ BA } }{ x^n }$.

原因是考虑:

$$
\begin{aligned}
& x^{ - m } \text{ char }_{ AB } \\
= & x^{ - m } \det ( xI - AB ) \\
= & \det ( I - x^{ - 1 } AB ) \\
= & \det ( I - x^{ - 1 } BA ) \\
= & x^{ - n } \text{ char }_{ BA }
\end{aligned}
$$

原因是我们已经证明过了$\det ( I - AB ) = \det ( I - BA )$.

仅仅如此当然不够好用,然而注意到如果$AB$以$\lambda_0 \ne 0$为特征值的特征向量是$\vec{ v }$,那么$BA$以$\lambda_0$为特征值的特征向量就是$B \vec{ v }$.

应当研究伴随矩阵在特征多项式下的情形,事实上有以下定理:如果$\text{ char }_A = \sum_{ k = 0 }^n c_k x^k$,那么$( - 1 )^{ n - 1 } \check{ A } = \sum_{ k = 1 }^n c_{ k } A^{ k - 1 }$.

考虑证明以上定理,考虑$\text{ adj } ( xI - A )$的每一个元素都是一个次数$\leq n - 1$的多项式,对其进行分解得到$\text{ adj } ( xI - A ) = \sum_{ k = 0 }^{ n - 1 } x^k D_k$,其中$D_k$是一个矩阵,代表不同位置$x^k$前的系数.

而我们知道$\text{ char }_A ( x ) I = ( xI - A ) \times \text{ adj } ( xI - A ) = ( xI - A ) \sum_{ k = 0 }^{ n - 1 } x^k D_k$.

两边提取系数,于是:

$$
\begin{aligned}
c_n I & = D_{ n - 1 } \\
c_{ n - 1 } I & = - AD_{ n - 1 } + D_{ n - 2 } \\
c_{ n - 2 } I & = - AD_{ n - 2 } + D_{ n - 3 } \\
\cdots
\end{aligned}
$$

将上面这些若干式子右边乘以$A^{ k }$并相加就可以做到相消,得到$D_0 = \sum_{ k = 1 }^n c_{ k } A^{ k - 1 }$,而$D_0$按照定义就是$\text{ adj } ( 0 I - A ) = ( - 1 )^{ n - 1 } \check{ A }$.

根据上述定理有以下结论:

1. $( A^\vee )^T = ( A^T )^\vee$.

2. $\text{ adj }{ ( P^{ - 1 } AP ) } = P^{ - 1 } \check{ A } P$.

另外,如果$\text{ char }_A = \sum_{ k = 0 }^n c_k x^k$,那么$\text{ char }_{ A^{ - 1 } } = \sum_{ k = 0 }^n \frac{ c_k }{ c_0 } x^{ n - k }$.

原因是观察到$( - 1 )^n c_0 \times \text{ char }_{ A^{ - 1 } } = \det A \times \det ( xI - A^{ - 1 } ) = ( - x )^n \det ( x^{ - 1 } I - A )$.

#### Cayley-Hamilton定理
即:$\text{ char }_A ( A ) = 0$.

原因是根据上述定理,由于$c_0 = ( - 1 )^n \det A$,我们有:

$$
\begin{aligned}
\sum_{ k = 0 }^n c_k A^k & = ( ( - 1 )^n \det A ) \times I + A \sum_{ k = 1 }^n c_k A^{ k - 1 } \\
& = ( ( - 1 )^n \det A ) \times I + ( - 1 )^{ n - 1 } A \check{ A } \\
& = 0
\end{aligned}
$$

#### 矩阵的迹
定义$A \in M_{ n \times n } ( F )$的**迹**$\text{ tr } \ A = \sum_{ i = 1 }^n a_{ i , i }$.显然$tr ( A + B ) = tr ( A ) + tr ( B )$.容易验证$tr : M_{ n \times n } ( F ) \to F$是线性映射,并且$\forall A \in M_{ n \times m } ( F ) , B \in M_{ m \times n } ( F )$有$tr ( AB ) = tr ( BA )$.

于是共轭的矩阵有相同的迹,原因是$tr ( P^{ - 1 } AP ) = tr ( APP^{ - 1 } ) = tr ( A )$.这应当意味着迹这个东西有一些更好的性质.事实上,迹的等价定义是$tr ( A ) = - [ x^{ n - 1 } ] \text{ char }_A$,原因是$\text{ char }_A ( x ) = \sum_{ \sigma } \text{ sgn } ( \sigma ) \prod_{ i = 1 }^n ( [ i = \sigma ( i ) ] x - a_{ i , \sigma ( i ) } )$,从这里立刻看出只有恒等映射$\sigma = \text{ id }$的时候才会对$[ x^{ n - 1 } ]$产生贡献.

###### Example
证明:如果$tr ( A ) = 0$,而且域的特征$\text{ char } \ F = 0$,那么存在$A ' = PAP^{ - 1 }$使得$\forall i \in [ 1 , n ] , a '_{ i , i } = 0$.

如果$A = 0$就下班了对吧,如果$A \ne 0$,我们下面来数学归纳证明.假设$< n$的时候已经得证.

首先我们说明:如果对于所有的$\vec{ v } \in F^n$都存在$c_v \in F$使得$A \vec{ v } = c_v \vec{ v }$,那么存在$c \in F$使得$A = cI$.

考虑线性无关的两个向量$\vec{ v } , \vec{ w }$,如果$c_v \ne c_w$,考虑$c_v \vec{ v } + c_w \vec{ w } = A ( \vec{ v } + \vec{ w } ) = c_{ v + w } ( \vec{ v } + \vec{ w } )$,于是自然导出$c_v = c_{ v + w } = c_w$,矛盾.于是上述命题成立.

对于一个$tr ( A ) = 0$的矩阵当然不是$cI$的形式,那我们就可以找到一个$\vec{ v }$使得$A \vec{ v } \ne c \vec{ v }$,于是$A \vec{ v }$和$\vec{ v }$线性无关.此时考虑将其扩充为一个基$\{ \vec{ v } , A \vec{ v } , \vec{ v }_3 , \cdots , \vec{ v }_n \}$,考虑当然存在一个$C$使得$C = P_1^{ - 1 } AP_1$这样$C$是对上述基的作用.那我们考虑$C ( \vec{ v } ) = A \vec{ v }$,于是$C$应当形如:

$$
\begin{bmatrix}
0 & \cdots \\
1 & \vdots \\
0 & \vdots \\
\vdots & \vdots
\end{bmatrix}
$$

不妨设$C = \begin{bmatrix}0 & T \\ L & B\end{bmatrix}$,我们知道根据归纳假设,对于$B$来说存在一个$P_2$使得$P_2^{ - 1 } BP_2$的对角线全零,那我们考虑:

$$
\begin{bmatrix}
1 & 0 \\
0 & P_2^{ - 1 }
\end{bmatrix} \begin{bmatrix}
0 & T \\
L & B
\end{bmatrix} \begin{bmatrix}
1 & 0 \\
0 & P_2
\end{bmatrix} = \begin{bmatrix}
0 & \_ \\
\_ & P_2^{ - 1 } BP_2
\end{bmatrix}
$$

这样就做完了.

#### Binet-Cauchy定理
定义大小分别为$n \times m$和$m \times n$(其中$n \leq m$)的矩阵$A$和$B$,令$A [ S ]$表示矩阵$A$只取$S$集合中的列所形成的矩阵,$B [ S ]$表示矩阵$B$只取$S$集合中的行所形成的矩阵,则有:

$$
\det ( AB ) = \sum_{ S \subseteq \{ 1 , 2 , . . . , m \} , | S | = n } ( \det ( A [ S ] ) ) ( \det ( B [ S ] ) )
$$

下面设$\lambda ( P )$为排列$P$的逆序对数.

$$
\begin{aligned}
& \sum_{ S \subseteq \{ 1 , 2 , . . . , m \} , | S | = n } ( \det ( A [ S ] ) ) ( \det ( B [ S ] ) ) \\
= & \sum_{ S \subseteq \{ 1 , 2 , . . . , m \} , | S | = n } ( \sum_{ P } ( - 1 )^{ \lambda ( P ) } \prod_{ i = 1 }^n A_{ i , S_{ P_i } } ) ( \sum_{ Q } ( - 1 )^{ \lambda ( Q ) } \prod_{ i = 1 }^n B_{ S_i , Q_i } ) \\
= & \sum_{ S \subseteq \{ 1 , 2 , . . . , m \} , | S | = n } \sum_{ P } \sum_{ Q } ( - 1 )^{ \lambda ( P ) + \lambda ( Q ) } \prod_{ i = 1 }^n A_{ i , S_{ P_i } } B_{ S_i , Q_i }
\end{aligned}
$$

若让$R$为可重集合,换言之让其取遍$m^n$种情况,又有:

$$
\begin{aligned}
\det ( AB ) & = \sum_{ P } ( - 1 )^{ \lambda ( P ) } \prod_{ i = 1 }^n ( \sum_{ j = 1 }^m A_{ i , j } B_{ j , P_i } ) \\
& = \sum_{ P } ( - 1 )^{ \lambda ( P ) } \sum_{ R , \forall i , R_i \in [ 1 , m ] } ( \prod_{ i = 1 }^n A_{ i , R_i } B_{ R_i , P_i } ) \\
& = \sum_{ R , \forall i , R_i \in [ 1 , m ] } \sum_P ( - 1 )^{ \lambda ( P ) } ( \prod_{ i = 1 }^n A_{ i , R_i } ) ( \prod_{ i = 1 }^n B_{ R_i , P_i } )
\end{aligned}
$$

如果$\exists 1 \leq i , j \leq n , i \ne j$,$R_i = R_j$,则交换$P_i , P_j$后,后面的值仍然不变,可逆序对的奇偶性改变了.如果我们对于每一个重复序列都选取最靠前的一对,那么显然只有$R$不重的情况才有可能产生贡献.既然如此,$R$的生成方式就应该是先顺序取子集再用一个排列打乱顺序.

所以我们有:

$$
\begin{aligned}
\det ( AB ) & = \sum_{ S \subseteq \{ 1 , 2 , . . . , m \} , | S | = n } \sum_{ Q } \sum_{ P } ( - 1 )^{ \lambda ( P ) } ( \prod_{ i = 1 }^n A_{ i , S_{ Q_i } } ) ( \prod_{ i = 1 }^n B_{ S_{ Q_i } , P_i } ) \\
& = \sum_{ S \subseteq \{ 1 , 2 , . . . , m \} , | S | = n } \sum_{ Q } \sum_{ P_{ Q ' } } ( - 1 )^{ \lambda ( P_{ Q ' } ) } ( \prod_{ i = 1 }^n A_{ i , S_{ Q_i } } ) ( \prod_{ i = 1 }^n B_{ S_i , P_{ Q '_i } } ) \\
& = \sum_{ S \subseteq \{ 1 , 2 , . . . , m \} , | S | = n } \sum_{ Q } \sum_{ P_{ Q ' } } ( - 1 )^{ \lambda ( P ) + \lambda ( Q ) } ( \prod_{ i = 1 }^n A_{ i , S_{ Q_i } } ) ( \prod_{ i = 1 }^n B_{ S_i , P_{ Q '_i } } ) \\

\end{aligned}
$$

左右显然相等.

另外,这个定理有更简单的证明方法,即使用LGV引理与组合意义证明.

不妨设现在有个点的集合$A , B , C$,现在想求出$A \rightarrow B \rightarrow C$的不交路径的奇偶数量差,会发现左右的组合意义是相等的.

于此之外,还有一个纯代数的证明策略,不妨定义符号$A \binom{ I }{ J } = A \binom{ i_1 , \cdots , i_r }{ j_1 , \cdots , j_s }$表示提取下标集$I$和$J$分别作为行列得到的矩阵.

先证明一个引理:$C \in M_{ n \times n } , [ x^{ n - k } ] \det ( xI + C ) = \sum_{ I \in \{ 1 , \cdots , n \} , | I | = k } \det C \binom{ I }{ I }$.

回忆到运算符$\delta_{ i , j } = [ i = j ]$,应当注意到:$\det ( xI + C ) = \sum_{ \sigma } \text{ sgn } ( \sigma ) \prod_{ i = 1 }^n ( x [ i = j ] + c_{ i , \sigma ( i ) } )$,此时要想凑出$x^{ n - k }$,就需要有$n - k$个位置被卡死,而剩下的部分无非是提取剩下$k$行$k$列后的结果.就可以见到上述引理无非定义的平凡推论.

然后开始证明该定理,回忆到我们已经证明过$x^{ m - n } \det ( xI + AB ) = \det ( xI + BA )$.此时对左右两边取$[ x^{ m - n } ]$,左边取出了$\det ( xI + AB )$的常数项,亦即$\det ( AB )$.而右边,按上述引理得知得到了$\sum_{ I \in \{ 1 , \cdots , n \} , | I | = k } \det \left ( ( BA ) \binom{ I }{ I } \right )$,而这恰为原本右式的矩阵乘法定义.

由该定理得知,对于$A \in M_{ m \times n }$,$\det{ A^T A } \geq 0$.

#### 特征值与特征向量
我们之前应当已经见到,如果一个矩阵是分块对角矩阵,那这个矩阵应当相当容易处理,原因是其加减乘运算无非只是在对角线上的适当操作而已.这引发我们选取适当的直和分解$V = \bigoplus V_k$将原本的矩阵化为合适的分块对角矩阵.

然而如果我们想要拿到具体的矩阵就不可避免地需要选基,因此在固定的基意义下,我们对于一个矩阵$A$,应当思考能否通过换基操作使得$A = P^{ - 1 } BP$,其中$B$是一个分块对角矩阵,那此时$A$也拥有相当好的性质.

考虑最简单的情况,即每个分块都是单个元素的情况,此时该矩阵应当就是一个对角矩阵.这相当于选定一组基$\{ \vec{ v }_1 , \cdots , \vec{ v }_n \}$后,取$V_k = \text{ span } \{ \vec{ v }_k \}$并将这些$V_k$取直和得到$V$,此时该线性映射应当给出$T \vec{ v }_k = \lambda_k \vec{ v }_k$,即只能做一个伸缩变换.上面的种种操作都引诱我们去思考能否进行以上操作,这就引入下面的定义:

设$T \in \text{ End } ( V )$并且$\lambda \in F$,我们引入如下定义:

1. 称子空间$V_\lambda = \ker ( \lambda \times \text{ id }_V - T )$为$T$的$\lambda$**特征子空间**,若$V_\lambda \ne \{ 0 \}$,则称$\lambda$为$T$的**特征值**.

2. 对于$\vec{ v } \in V_\lambda$,换言之$T \vec{ v } = \lambda \vec{ v }$,而且$\vec{ v } \ne 0$,则称$\vec{ v }$是$T$的一个**特征向量**,以$\lambda$为特征值.

考虑特征值的定义等价于$\det ( \lambda \times I - T ) = 0$,于是等价于$\text{ char }_T ( \lambda ) = 0$.立刻见到一个$\lambda$是特征值等价于其是$\text{ char }_T = 0$的一个根.反之,$\text{ char }_T = 0$的一个根也一定对应了一个非空的特征子空间,也就对应了一组非零特征向量.如果$T$分裂,见到行列式其实是特征值的乘积,而迹实际上是特征值的加和.应当见到,一个矩阵可逆等价于其不存在$0$特征值.

考虑对角化,如果我们能在上面取出若干$\vec{ v }$组成一组基,那直接用这组基就可以把$T$拆成在这组基上的伸缩变换,这样就实现了对角化.因此如果我们能取出一组基使得每一个基对$A$都是特征向量,我们称$A$是**可对角化的**.立刻见到可对角化的必要条件是$\text{ char }_A$分裂.

如果$T$可对角化,我们有$\dim V_\lambda = d$,其中$d$是$\lambda$作为特征多项式的根的重数.如果其可对角化,直接按上述方式取基得到$z$,并且$PTP^{ - 1 }$就是在标准基下的变换矩阵.那么直接考虑$\text{ rk } ( \lambda_n I - T ) = n - d$,其中$d$是$\lambda_n$的重数.用同态基本定理立刻得到$V_{ \lambda_n } = d$.(注意这里都是在讨论可对角化的前提下)

我们希望能把$V_{ \lambda_k }$给做直和,那我们就需要做前置工作.具体而言,我们需要证明选取$\vec{ v }_k \in V_{ \lambda_k }$,那么$\sum \vec{ v }_k = \vec{ 0 } \Leftrightarrow \forall k , \vec{ v }_k = \vec{ 0 }$.使用数学归纳,注意到$T ( \sum \vec{ v }_k ) = \sum \lambda_k \vec{ v }_k = 0$,又知道$\lambda_1 \sum \vec{ v }_k = 0$,两者一消就可以进行数学归纳.

由上面的过程得知下述命题等价:

1. $T$在$F$上可对角化.

2. $\sum_{ \lambda \in F } \dim V_{ \lambda } = \dim V$.

3. $\bigoplus_{ \lambda \in F } V_\lambda = V$.

其推论是如果所有的$\lambda_k$互不相同,那么$\dim V_{ \lambda_k } \geq 1$,这必然给出$T$可对角化.这似乎告诉我们去除重数有某种很好的性质.

#### 极小多项式
考虑对于一个多项式$h \in F [ x ]$,当我们将$T \in \text{ End } ( V )$带入之后当然会得到一个$h ( T ) \in \text{ End } ( V )$,现在固定$h , T$,以此构造一个子空间$V [ h ] = \ker ( h ( T ) ) = \{ \vec{ v } \in V | h ( T ) \vec{ v } = 0 \}$.

此时注意到$T ( V [ h ] ) \subseteq V [ h ]$,原因是$T$与自身交换,于是$\forall \vec{ v } \in V [ h ]$,应当有$h ( T ) ( T \vec{ v } ) = Th ( T ) \vec{ v } = T ( \vec{ 0 } ) = \vec{ 0 }$,这意味着$T \vec{ v } \in V [ h ]$.于是可见到$V [ h ]$实际上是一个$T$不变子空间.

取$h = x - \lambda I$,那么$V [ h ]$其实就是我们上述所说的$V_\lambda$,因此$V_\lambda$也是$T$不变的.从这里立刻能看出些什么,比如当$\dim V_\lambda = 1$的时候,$T$不变当然意味着伸缩变换.

那我们既然需要这个东西用在$V_\lambda$上,首要要做的当然是分析在其上面的直和分解.换言之,我们下面试图证明如下引理(核引理):$f \bot g$的时候,$V [ fg ] = V [ gf ] = V [ f ] \oplus V [ g ]$.

原因是考虑$F [ x ]$的主理想性引出其裴蜀定理,那么就存在多项式$a , b$使得$af + bg = 1$.此时$\vec{ v } = ( af + bg ) ( T ) \vec{ v } = a ( T ) ( f ( T ) \vec{ v } ) + b ( T ) ( g ( T ) \vec{ v } )$.注意到如果$\vec{ v } \in V [ fg ]$,那么$f ( T ) \vec{ v } \in V [ g ] , g ( T ) \vec{ v } \in V [ f ]$,而$V [ g ]$和$V [ f ]$对$a ( T )$和$b ( T )$都是不变子空间,原因是它们对$T$都是不变子空间,这必然意味着$a ( T ) ( f ( T ) \vec{ v } ) \in V [ g ]$,后者亦然同理.这意味着$V [ fg ] \subseteq V [ f ] + V [ g ]$,而$V [ f ] \subseteq V [ fg ] , V [ g ] \subseteq V [ fg ]$,这立刻得到$V [ fg ] = V [ f ] + V [ g ]$.此时检查$\vec{ v } \in V [ g ] \cap V [ f ]$,发现$\vec{ v } = ( af + bg ) ( T ) \vec{ v } = \vec{ 0 }$,这就意味着$V [ fg ] = V [ f ] \oplus V [ g ]$.

考虑$I = \{ f \in F [ x ] | f ( T ) = 0_V \}$,容易见到其应当也有某种$\ker$的性质,事实上可以证明它是一个理想,原因是当然满足加法和乘法的封闭律.而由于$F [ x ]$是一个主理想环,那么上面就应当存在一个$\deg$极小的$h$,使得$I = ( h )$,不妨将$h$规范化为首一的多项式.那么以上操作给出:$f ( T ) = 0 \Leftrightarrow h | f$.

由Cayley-Hamilton定理,当维度有限的时候,这样的多项式必然存在,这就意味着$I \ne \{ 0 \}$,此时我们就把上面的这个$h$叫做$T$的**极小多项式**$\text{ Min }_T$.

值得一提的是如果$A$是在$F$上的矩阵,其极小多项式为$\text{ Min }_{ A , F }$,$F$是$E$的子域,$A$在$E$上的极小多项式为$\text{ Min }_{ A , E }$,则$\text{ Min }_{ A , F } = \text{ Min }_{ A , E }$.原因是首先$\text{ Min }_{ A , E } | \text{ Min }_{ A , F }$,于是$\text{ Min }_{ A , E } = \text{ Min }_{ A , F }$等价于$A^0 , \cdots , A^{ d - 1 }$线性无关,其中$d = \deg \text{ Min }_{ A , F }$.而判定线性无关实际上是用高斯消元判定的,和域的选取无关.

用一次Cayley-Hamilton定理,知道$\text{ Min }_T | \text{ char }_T$.既然如此,$\text{ Min }_T$保留了$\text{ char }_T$的哪些具体性质呢?它们之间的关系又如何呢?我们下面尝试证明:如果$\lambda$是$T$的特征值,当且仅当$\text{ Min }_T ( \lambda ) = 0$.

充分性显然,因为我们已经声明了只要是$\text{ char }_T$的一个根就自动是特征值.

下面证明必要性:注意到如果$\lambda$是特征值,此时$\vec{ 0 } = \text{ Min }_T ( T ) \vec{ v } = \text{ Min }_T ( \lambda ) \vec{ v }$,原因是观察到如果$T \vec{ v } = \lambda \vec{ v }$,那么立刻得到$f ( T ) \vec{ v } = f ( \lambda ) \vec{ v }$.于是立刻得到$\text{ Min }_T ( \lambda ) = 0$.

此时见到端倪,我们再证明以下定理:

对于固定的$T \in \text{ End } ( V )$如果有直和分解$V = \bigoplus V_k$,其中每个$V_k$都是$T$不变子空间,那么就可以把$T$限制到相应的$V_k$空间内,记作$T |_{ V_k } \in \text{ End } ( V_k )$,那在这些空间内部自然有它们自己的极小多项式$\text{ Min }_{ T_k }$,我们断言$\text{ Min }_T = \text{ lcm }_k ( \text{ Min }_{ T_k } )$.原因是由于直和的性质,$f ( T ) = \vec{ 0 }$当且仅当对于每一个$k$,$f ( T_{ k } ) = 0$,应用极小多项式的定义立刻见到上述结论显然.

对于$T \in \text{ End } ( V )$,我们应当能见到此时$V = V [ \text{ Min }_{ T } ]$.那么反过来,$V [ \text{ Min }_T ] = V$的极小多项式当然也是$\text{ Min }_T$本身,这只是定义.

不妨设$\text{ Min }_T = fg$并且$f \bot g$,从上面当然可以知道$V = V [ f ] \oplus V [ g ]$.此时我们应当思考的是$V [ f ]$中的极小多项式是否是$f$,$V [ g ]$中的极小多项式是否是$g$.而$\text{ Min }_T$是$V [ f ]$和$V [ g ]$中极小多项式的乘积,它们中的极小多项式又只是$f , g$的因子,那必然见到极小多项式就是它们.

我们理应从上面的分析中见到极小多项式形态的一种刻画,我们接下来尝试证明,如果$T$在$F$上可对角化,其特征值去重后是$\lambda_1 , \cdots , \lambda_m$,那么最小多项式$\text{ Min }_T = \prod_{ k = 1 }^m ( x - \lambda_k )$.

原因是考虑$T$可对角化导致$V$可以被拆成特征子空间的直和$V = \bigoplus_{ k = 1 }^m V_{ \lambda_k }$.而其限制在每一个特征子空间上都是一个伸缩变换,其对应的极小多项式无外乎$x - \lambda_k$,这样就得到上面的结论.

上述结论反之依然成立,那我们可以拿出一个更好的结论了,即:$T$在$F$上可对角化的充要条件是极小多项式$\text{ Min }_T \in F [ x ]$分裂并且无重根.

必要性在上面已经证明,接下来说明充分性.

当$\text{ Min }_T = \prod_{ k = 1 }^m ( x - \lambda_k )$的时候,我们下面证明$T$可对角化,顺便说明此时$\lambda_1 , \cdots , \lambda_m$就是$T$的特征值去重后的结果.运用上述结论可以知道$V = \bigoplus V [ x - \lambda_k ]$.然而$V [ x - \lambda_k ] = V_{ \lambda_k }$,这立刻就回到了可对角化的等价条件,于是充分性得证.

应当能从上面看出可对角化的某种拆分性,事实上,如果$T \in \text{ End } ( V )$可对角化,$V_0 \subseteq V$是$T$下的不变子空间,则$T |_{ V_0 } \in \text{ End } ( V_0 )$和$\bar{ T } \in \text{ End } ( V / V_0 )$都可以对角化.原因是它们的极小多项式都应当是$\text{ Min }_T$的因子.

然而也可以看出来上述命题的逆并不成立,因为拆出来的两个的极小多项式并不一定互质.

#### 同步对角化
设$\mathcal{ S } \subseteq \text{ End } ( V )$,如果存在$V$的基$\vec{ v }_1 , \cdots , \vec{ v }_n$使得每个$\vec{ v }_i$都是所有$T \in \mathcal{ S }$的共同的特征向量,换言之每一个$T$都可以用同样的一个可逆矩阵$P$使得$P^{ - 1 } AP$是对角的,那我们称$\mathcal{ S }$在$F$上可以**同步对角化**.

我们首先证明一个引理:如果$ST = TS$,那么$T$的任何一个特征子空间$V_\lambda$都是$S$的一个不变子空间.原因是对于$\vec{ v } \in V_\lambda$:

$$
\begin{aligned}
T \vec{ v } & = \lambda \vec{ v } \\
ST \vec{ v } & = \lambda ( S \vec{ v } ) \\
T ( S \vec{ v } ) & = \lambda ( S \vec{ v } )
\end{aligned}
$$

所以$S \vec{ v } \in V_\lambda$.

我们得知同步对角化的充要条件应该是以下两条同时成立:

1. 每个$T \in \mathcal{ S }$在$F$上都可对角化.

2. $\forall T , T ' \in \mathcal{ S }$,$TT ' = T ' T$.

必要性显然,下面来说明充分性.

取出$T_1$的所有特征子空间$V_\lambda$,由于$T '$可对角化,所以其限制在$T ' |_{ V_\lambda }$上肯定也可对角化.如果我们能说明限制在每一个特征子空间$V_\lambda$上时都可以同步对角化,由于特征子空间是直和分解,所以取直和后当然也可以.此时注意到限制后的$T '$必然也有交换律,而$T_1 |_{ V_\lambda }$本身应当是$\lambda I$.这个东西怎么着都是可对角化的,因此就只需要对除了$T_1$剩下部分继续做,这样就实现了数学归纳.

#### 上三角化
回忆到我们之前曾经提过的旗的概念,我们在下面断言:若$T \in \text{ End } ( V )$保持$V$的一组完备旗,则$T$在$F$上**可上三角化**,也就是其可以换基得到一个上三角矩阵.

我们已经得知一般的矩阵未必能对角化,那能不能退而求其次讲一个矩阵通过共轭转化为上三角的形式呢?我们下面证明:$\text{ char }_T$在$F$上分裂是$T$在$F$上可上三角化的充要条件.

先证明必要性:当其可上三角化的时候,可以取$V$的一组有序基,使得$T$对应的矩阵形如$\begin{bmatrix}\lambda_1 & \cdots & \cdots \\ 0 & \ddots & \cdots \\ 0 & 0 & \lambda_n\end{bmatrix}$,此时立刻能得到其$\text{ char }_T = \prod ( x - \lambda_k )$,这立刻给出其可分裂.

再证明充分性:数学归纳,取出$\lambda_1$和$\vec{ v }_1 \in V_{ \lambda_1 } \setminus \{ 0 \}$,再令$V_1 = \text{ span } \{ \vec{ v }_1 \}$,这里拿到的$V_1$当然是一个不变子空间.仍取$\pi : V \to V / V_1$,接下来观察$V / V_1$的维数降了一维,根据数学归纳那其当然可以上三角化.回忆到我们曾经声明过了此时$\text{ char }_T = ( x - \lambda_1 ) \text{ char }_{ \bar{ T } }$,原因是之前曾提过的在此分解下行列式之间的关系.可上三角化应当给出在$V / V_1$上的一个被$\bar{ T }$保持的完备旗$\{ 0 \} = \bar{ V_1 } \subsetneq \cdots \subsetneq \bar{ V_n } = \bar{ V }$.第一同构定理已经声明过对他们取原像后仍然是子空间,于是我们取原像应当可以重新得到一列完备旗:$\{ 0 \} = V_0 \subsetneq V_1 \subsetneq \cdots \subsetneq V_n = V$.

我们断言上述的旗也被$T$保持,原因是$\vec{ v } \in V_i \Rightarrow \pi ( T \vec{ v } ) = \bar{ T } ( \pi \vec{ v } ) \in \bar{ T } ( \bar{ V }_i ) \subseteq \bar{ V }_i = \pi ( V_i )$,这当然意味着$T \vec{ v } \in V_i$,于是数学归纳就成立了.

值得一提的是上述证明可以原样拿来证明Cayley-Hamilton定理.方法是仍采取数学归纳,取$V_1 = \text{ span } \{ \vec{ v }_1 \}$后,用数学归纳应当有$\prod_{ k = 2 }^n ( \bar{ T } - \lambda_i I ) = \text{ char }_{ \bar{ T } } ( \bar{ T } ) = 0$.然而我们早就知道了$\bar{ T } = \pi T$,于是立刻得知$\forall \vec{ v } \in V , \prod_{ k = 2 }^n ( T - \lambda_i I ) ( \vec{ v } ) \in V_1$,两边同做$T - \lambda_1 I$映射立刻得到结论.

#### 广义特征子空间
定义$V_{ [ \lambda ] } = \bigcup_k V [ ( x - \lambda )^k ]$.

先来分析其与通常讨论的特征子空间之间的关系,我们声明:$V_{ [ \lambda ] } \ne 0 \Leftrightarrow V_\lambda \ne 0$.

由于$V_\lambda \subseteq V_{ [ \lambda ] }$,右推左是显然的.

而考虑左推右,$\exists \vec{ v } \in V_{ [ \lambda ] } , ( T - \lambda )^N \vec{ v } = \vec{ 0 } , ( T - \lambda )^{ N - 1 } \vec{ v } \ne \vec{ 0 }$,那么$( T - \lambda )^{ N - 1 } \vec{ v } \in V_{ \lambda }$.

这个无限取并能不能降下去呢?而且上面这个东西应当需要有一种划分原空间的策略,这就意味着其应当和极小多项式有一定关系.事实上,如果特征多项式$\text{ char }_T = \prod ( x - \lambda_i )^{ a_i }$,极小多项式$\text{ Min }_T = \prod ( x - \lambda_i )^{ b_i }$,那么$V_{ [ \lambda_i ] } = V [ ( x - \lambda_i )^{ b_i } ]$.首先肯定有$V [ ( x - \lambda_i )^{ b_i } ] \subseteq V_{ [ \lambda_i ] }$,所以只需证明反向包含即可.

此时注意到$\forall \vec{ v } \in V_{ [ \lambda_i ] }$,则存在$k$使得$( T - \lambda_i )^k \vec{ v } = \vec{ 0 }$.此时取$h = \gcd ( ( x - \lambda_i )^k , \text{ Min }_T )$,用裴蜀定理立刻得到$h = a ( x - \lambda_i )^k + b \text{ Min }_T$,于是立刻见到$h ( T ) \vec{ v } = \vec{ 0 }$.但容易注意到$h | ( x - \lambda_i )^{ b_i }$,这就证完了.

那么从之前我们的分析立刻得知$V = V [ \text{ Min }_T ] = \bigoplus_{ i } V_{ [ \lambda_i ] }$.并且见到$V_{ [ \lambda_i ] }$的极小多项式就是$( x - \lambda_i )^{ b_i }$,而因为极小多项式和特征多项式的根集相同并且原本的特征多项式等于分空间的特征多项式的乘积,因此特征多项式就是$( x - \lambda_i )^{ a_i }$.

接下来我们定义代数重数:作为特征多项式的根的重数.几何重数:$\dim V_{ \lambda_i }$.

注意到代数重数$\geq$几何重数,等式成立当且仅当$V_{ \lambda_i } = V_{ [ \lambda_i ] }$.原因是$a_i = \deg \text{ char }_{ T_i } = \dim V_{ [ \lambda_i ] } \geq \dim V_{ \lambda_i }$.

由于可对角化等价于可以拆分成若干特征子空间的直和,我们之前又已经证明过可对角化的几何重数一定等于代数重数,于是可对角化当且仅当几何重数等于代数重数.

另外代数重数和几何重数的关系还可以从线性方程组的角度理解,因为特征空间的维度就是$\lambda_k I - A$的自由变量的数量.而$\lambda_k$的代数重数意味着$( x - \lambda_k )$这一项在对角线的出现次数,现在这些位置都变成了$0$,也就最多会有这么多个自由变量(还有可能上面某位置也有可以当主元的点顶替了他).那么几何重数就是实际上的自由量.

#### 特殊矩阵的特征值
###### Example
回忆道斐波那契数列应当转移矩阵为$T = \begin{bmatrix}0 & 1 \\ 1 & 1\end{bmatrix}$,如果能把这个东西对角化的话就会很舒服.直接解特征值能解出来$\text{ char }_T = x^2 - x + 1$,两个特征值$\lambda = \frac{ 1 \pm \sqrt{ 5 } }{ 2 }$.

然后需要去解特征向量再用特征向量取基,解出来$\vec{ v } = \left ( \begin{matrix}1 \\ \frac{ 1 \pm \sqrt{ 5 } }{ 2 }\end{matrix} \right )$,这样立刻就拿到了换基矩阵$P = [ \vec{ v }_1 , \vec{ v }_2 ]$和对角矩阵$D = \begin{bmatrix}\lambda_1 & 0 \\ 0 & \lambda_2\end{bmatrix}$,并且拿到$D = P^{ - 1 } TP$.

### 双线性形式
#### 线性映射
给定$F$上的线性空间$V_1 , \cdots , V_n , W$,我们定义一个**多重线性映射**$C : V_1 \times \cdots \times V_n \to W$.满足其对每一个分量都是线性的.也即固定其他$m - 1$个向量后,剩下的那一个向量的改变也会引起线性的改变.换言之:

$$
\begin{aligned}
C ( \cdots , t \vec{ v }_i , \cdots ) & = tC ( \cdots , \vec{ v }_i , \cdots ) \\
C ( \cdots , \vec{ v }_i + v_i ' , \cdots ) & = C ( \cdots , \vec{ v }_i , \cdots ) + C ( \cdots , \vec{ v }_i ' , \cdots )
\end{aligned}
$$

特别地,当$W = F$的时候也叫做**多重线性形式**.

容易见到交错形式无非是多重线性形式的某种特例.

还可以看到多重线性映射的集合$\text{ Mul } ( V_1 , \cdots , V_n ; W )$本身也是线性空间.

更常用的是双线性映射,一般记作$\text{ Bil } ( V_1 , V_2 ; W )$.矩阵乘法本身是双线性映射的一个典型例子,对偶空间本身也是双线性形式的一个典型例子.特别地,一般定义$V$和$\check{ V }$之间的**典范配对**为$\langle \_ , \_ \rangle : \check{ V } \times V \to F$.其也可以看作点乘或者$( 1 \times n ) \times ( n \times 1 )$的矩阵乘法.

留意到柯里化过程,设$\dim V = n , \dim W = m$,容易见到:

$$
\begin{aligned}
M_{ m \times n } & \cong \\
\text{ Hom } ( V , \check{ W } ) & \cong \text{ Bil } ( V , W ; F ) \cong \text{ Hom } ( W , \check{ V } ) \\
& \cong M_{ n \times m }
\end{aligned}
$$

证明是平凡的.

这就能看出$M_{ n \times m } \cong \text{ Bil } ( V \cong F^n , W \cong F^m ; F )$,留意到其实可以取$A : W \to V$以$( \vec{ v } , \vec{ w } ) \mapsto \check{ \vec{ v } } A \vec{ w }$.取二者的坐标表示,立刻见到$( \vec{ v } , \vec{ w } ) \mapsto \sum_{ i } \sum_j v_i a_{ i , j } w_j = ( \vec{ v } )^T A \vec{ w }$.留意到如果设$C ( \vec{ w } , \vec{ v } ) = B ( \vec{ v } , \vec{ w } )$,这个$C$对应的矩阵实际上就是对$B$对应的矩阵取了转置.

接下来考虑对其取直和,不妨考虑$B_1 : V_1 \times W_1 \to F , B_2 : V_2 \times W_2 \to F$,则令$B_1 \oplus B_2 : ( V_1 \oplus V_2 ) \times ( W_1 \oplus W_2 ) \to F$,其中$( ( \vec{ v }_1 , \vec{ v }_2 ) , ( \vec{ w }_1 , \vec{ w }_2 ) ) \mapsto B_1 ( \vec{ v }_1 , \vec{ w }_1 ) + B_2 ( \vec{ v }_2 , \vec{ w }_2 )$.容易见到这是一个双线性形式.并且其代表的矩阵当然是原本的$B_1 , B_2$对应的矩阵组成的分块对角矩阵.

考虑双线性形式$B : V \times V \to F$.称其为**对称的**当且仅当$B ( \vec{ v } , \vec{ w } ) = B ( \vec{ w } , \vec{ v } )$,称其为**反对称的**当且仅当$B ( \vec{ v } , \vec{ w } ) = - B ( \vec{ w } , \vec{ v } )$.容易看到对称的当且仅当其对应的矩阵为**对称矩阵**,即$A = A^T$,原因是$B ( \vec{ w } , \vec{ v } ) = ( \vec{ w } )^T A \vec{ v } = ( \vec{ v } )^T A^T \vec{ w }$.反对称的对应的矩阵成为**反对称矩阵**或者**斜对称矩阵**,即$- A = A^T$.

#### 非退化形式
对于双线性形式$B : V \times W \to F$,称$\vec{ v }$是$B$的一个**左根**当且仅当$B ( \vec{ v } , \_ ) \equiv 0$.同理可以定义**右根**.容易见到左根右根集合分别应当是$V , W$的子空间.将左根空间记作$^{ \bot } V$,右根空间记作$W^{ \bot }$.

当$V = W$并且$B$是对称的或者反对称的时候,此时左根和右根是一回事,一般将它们统称为$B$的根基.

称$B$是**非退化的**,当且仅当其左右根集合都是$\{ \vec{ 0 } \}$.

现在考虑对于典范配对$\langle \lambda , \vec{ v } \rangle \to \lambda \vec{ v }$,考虑其左根和右根,显然都只是各自空间里的$\vec{ 0 }$,原因是对于前者是一个映射,其映射所有都是$0$,那它当然是$0$映射本身.对于后者,考虑取$\vec{ v } \ne \vec{ 0 }$,扩充$\vec{ v }$为一组基$\vec{ v } , \vec{ v }_2 \cdots , \vec{ v }_n$,并取其对偶基$\check{ v } , \check{ v }_2 , \cdots , \check{ v }_n$,此时就当然满足$\langle \check{ v } , \vec{ v } \rangle = 1$,所以它是非退化的.

另一个例子是考虑迹映射$Tr : \text{ End } ( V ) \times \text{ End } ( V ) \to F , ( S , T ) \mapsto \text{ tr } ( ST )$,容易见到它是对称的.那么考虑其根基,只考虑左根,如果$\forall T , \text{ tr } ( ST ) = 0$,应当满足对于基矩阵$E_{ i , j }$满足$\text{ tr } ( SE_{ i , j } ) = 0$.容易见到$tr ( SE_{ i , j } ) = 0$,所以$S \equiv 0_{ n \times n }$.这意味着迹映射是非退化的.

回忆道双线性形式同构于线性映射$\psi \in \text{ Hom } ( V , \check{ W } )$或$\varphi \in \text{ Hom } ( W , \check{ V } )$,注意到左根实际上就是$\ker \psi$,右根实际上就是$\ker \varphi$.

不妨假设$\dim V > \dim W$,则$\dim \ker \psi = \dim V - \text{ rk } \psi \geq \dim V - \dim W > 0$,所以注意到$B$是非退化的一定要有$\dim V = \dim W$.

现在我们对于$B : V \times W \to F$并且满足$\dim V = \dim W$,我们声称以下三条性质等价:

1. $B$是非退化的.

2. 左根空间为$0$.

3. 右根空间为$0$.

显然只需说明(2)$\Leftrightarrow$(3).两个方向类似,下面只证明(2)$\Rightarrow$(3).而当$\dim \ker \psi = 0$时,由于两边维数相等,自然得到$\psi$是一个同构.接下来考虑$\vec{ w }$如果是一个右根,那么对此取一个空间$H = \{ \lambda \in \check{ W } \mid \lambda \vec{ w } = 0 \}$,见到$\psi ( V )$一定是$H$的一个子空间.但是$\dim V = \dim \psi ( V ) \leq \dim H \leq \dim W$,这就看出$\dim H = \dim V = n$.然而当$\vec{ w } \ne \vec{ 0 }$的时候,$\dim H$不可能取到满空间,原因是典范同态是非退化的.这就导出了右根空间也是$0$空间.

另一种看法是直接观察矩阵,右根实际上就是$\ker ( A )$,左根实际上就是$\ker ( A^T )$,那它们的$\dim$当然要相等,原因是它们的$\text{ rk }$相等.

此时也容易注意到,非退化的矩阵实际上也就等价于可逆矩阵.

如果对商空间比较敏感,不妨设左根空间为$L$,右根空间为$R$,我们应当见到$\bar{ B } : ( V / L ) \times ( W / R ) \to F , ( \vec{ v } + L , \vec{ w } + R ) \mapsto B ( \vec{ v } , \vec{ w } )$是非退化双线性形式.此时见到其实$\dim V - \dim L = \dim W - \dim R$.

适当推广上述结论,我们实际上可以定义任何子空间的$V_0 \subseteq V$的正交空间为$V_0^\bot = \{ \vec{ w } \in W \mid \forall \vec{ v }_0 \in V_0 , B ( \vec{ v }_0 , \vec{ w } ) = 0 \}$.同理定义$^\bot W_0$.容易见到$V_0 \subseteq^\bot ( V_0^\bot )$.

我们试图寻找一些更好的关系,不妨假设$B$是非退化的,那么立刻有$\dim V = \dim W$,我们下面证明$\dim V_0^\bot + \dim V_0 = \dim V$.

令$d = \dim V_0 , n = \dim V = \dim W$已经知道$B$会对应一个同构$\psi : V \cong \check{ W }$,那么取$V_0$在$\psi$下的像,并取其基$\check{ w }_1 , \cdots , \check{ w }_d$,此时$V_0^\bot \subseteq W$是满足$\langle \check{ w }_1 , \_ \rangle = \cdots = \langle \check{ w }_d , \_ \rangle = 0$的子空间.

不妨先考虑两个特殊情况并从中窥见归纳的办法.当$d = n$的时候,此时$V_0^\bot = V^\bot$,也就是右根集合,我们之前已经证明过了这是一个零空间.而当$d = 1$的时候,考虑$\check{ w }_1$,因为它不是$0$,所以它必然是$W \to F$的满射,此时意味着$\dim \text{ im } \ \check{ w }_1 = 1$,意味着此时$V_0^\bot = \ker ( \check{ w }_1 )$的维数应当是$n - 1$.

接下来从上面的结构中跳出来,直接观察$\check{ W }$上的情形,我们将上述的那一组基扩充为$\check{ W }$的基$\check{ w }_1 , \cdots \check{ w }_n$并考虑$W$的子空间列$^\bot \langle \check{ w }_1 \rangle \supseteq \cdots \supseteq^\bot \langle \check{ w }_1 , \cdots , \check{ w }_n \rangle$.不妨设$W_k =^\bot \langle \check{ w }_1 , \cdots , \check{ w }_k \rangle$.

然而此时可以见到$W_{ k + 1 } = W_k \cap{ ^\bot \langle \check{ w }_{ k + 1 } \rangle }$,用第二同构定理得到:

$$
\begin{aligned}
\dim ( W_{ k + 1 } ) & = \dim W_k + \dim^\bot \langle \check{ w }_{ k + 1 } \rangle - \dim ( W_k +^\bot \langle \check{ w }_{ k + 1 } \rangle ) \\
& \geq \dim W_k + \dim^\bot \langle \check{ w }_{ k + 1 } \rangle - n \\
& = \dim W_k - 1
\end{aligned}
$$

因此每加一个最多降一维(其实等价于把那个$\dim = 1$的空间商掉了),然而注意到上述空间列的首项是$n - 1$维,末项是$0$维,每次降一维的话就可以得到$\dim^\bot \langle \check{ w }_1 , \cdots , \check{ w }_d \rangle = n - d$,这就证明了上述的结论.

然而上述推断其实并不意味着$V = V_0^\bot + V_0$,也不意味着$V_0 \cap V_0^\bot = \{ \vec{ 0 } \}$,这两个结论都是得不到的.

然而好用的结论是我们之前已经提过$V_0 \subseteq^\bot ( V_0^\bot )$,此时我们注意到这个式子两边的维度实际上相等,因此$^\bot ( V_0^\bot ) = V_0$.再次声明此式子只在非退化的前提下成立.

#### 伴随映射
考虑两个双线性形式$B_1 , B_2$,其中$B_1 \in \text{ Bil } ( V_1 , V_1 ' , F )$,$B_2$类似.我们声称存在唯一的线性映射$\varphi : \text{ Hom } ( V_1 , V_2 ) \to \text{ Hom } ( V_2 ' , V_1 ' ) , T \mapsto T^*$满足$B_2 ( T \vec{ v }_1 , \vec{ v }_2 ' ) = B_1 ( \vec{ v }_1 , T^* \vec{ v }_2 ' )$.其中这个$T^*$称为$T$相对于$B_1$和$B_2$的**右伴随**.同理可以定义**左伴随**$^* T$满足$B_2 ( \vec{ v }_2 , T \vec{ v }_1 ' ) = B_1 (^* T \vec{ v }_2 , \vec{ v }_1 ' )$,并且假设$B_1$非退化,它对应的矩阵$A_1$可逆.

如何证明这个结论呢?只考虑右伴随的情况,对于某个固定的$\vec{ v }_2 '$,取出映射$B_2 ( T ( \_ ) , \vec{ v }_2 ' ) : V_1 \to F$.那它当然是$\check{ V_1 }$中的元素.而因为$B_1$非退化,其对应了一个$V_1 ' \to \check{ V }_1$的同构,因此应当存在唯一的$\vec{ v }_1 '$使得其映射到$\check{ V_1 }$的时候恰好满足$B_2 ( T ( \_ ) , \vec{ v }_2 ' ) = B_1 ( \_ , \vec{ v }_1 ' )$,这就给出了$T^* : \vec{ v }_2 ' \mapsto \vec{ v }_1 '$.换言之其唯一性取决于$A_1$的非退化性.取$\varphi_i : V_i ' \to \check{ V_i }$且$\varphi_1$可逆,回忆到之前过程可以知道这里的$\varphi_1$实际上就是$A_1$,下述图表的上半部分交换:

$$
\xymatrix{ V_2 ' \ar[r]^{ T^* } \ar[d]_{ A_2 } & V_1 ' \ar[d]^{ A_1 } \\
\check{ V_2 } \ar[r]^{ ^t T } & \check{ V_1 } \\
V_2 \ar[u] & V_1 \ar[u] \ar[l]_T }
$$

另外可以见到如果假设$B_2$也是非退化的,当线性空间本身满足条件的时候应该有$^* ( T^* ) = T = (^* T )^*$.还可以见到$( ST )^* = T^* S^* ,^* ( ST ) =^* T^* S$.这还顺便说明了$( T^{ - 1 } )^* = ( T^* )^{ - 1 } ,^* ( T^{ - 1 } ) = (^* T )^{ - 1 }$.

上面的结论如何用矩阵刻画?只考虑右伴随的部分,选定一组有序基后,将双线性形式对应到矩阵应当有:

$$
\begin{aligned}
B_2 ( T \vec{ v }_1 , \vec{ v }_2 ' ) & = ( \vec{ v }_1 )^T T^T A_2 \vec{ v }_2 ' = ( \vec{ v }_1 )^T A_1 ( A_1^{ - 1 } T^T A_2 ) \vec{ v }_2 ' \\
B_1 ( \vec{ v }_1 , T^* \vec{ v }_2 ' ) & = ( \vec{ v }_1 )^T A_1 T^* \vec{ v }_2 '
\end{aligned}
$$

比对就可以看到只需取$T^* = A_1^{ - 1 } T^T A_2$,这与上述交换图表的形式也是符合的.同理可以得到$^* T = ( A_2 TA_1^{ - 1 } )^T$.并从此看出当$B_1 , B_2$都非退化的时候,伴随不改变矩阵的秩,因为此时伴随矩阵与原矩阵的转置是相抵的.事实上可以直接写出$\ker ( T^* ) = ( \text{ im } T )^\bot$,原因是:

$$
\begin{aligned}
T^* \vec{ w } & = 0 \Leftrightarrow \forall \vec{ v } , ( \vec{ v } \mid T^* \vec{ w } ) = 0 \\
& \Leftrightarrow \forall \vec{ v } , ( T \vec{ v } \mid \vec{ w } ) = 0 \Leftrightarrow \vec{ w } \in ( \text{ im } T )^\bot
\end{aligned}
$$

而由同态基本定理,$\dim ( \text{ im } T )^\bot = \dim \ker ( T^* )$,由上可以见到当$\dim V = \dim W$时$\text{ rk } \ T = \text{ rk } \ T^*$.

当对应的双线性形式非退化的时候回忆到可以两边取$\bot$就拿到$\ker ( T^* )^\bot = ( \text{ im } T )$.

下面考虑$V_1 = V_2 = V_1 ' = V_2 ' = V$,并假设$B_1 = B_2$且二者要么对称要么反对称.此时注意到$T$的左伴随和右伴随是没有区别的,我们将它们统一称作伴随,一般采取右伴随的记号记作$T^*$并尝试通过上下文区分.特别地,如果$T = T^*$,那我们称其为**自伴的**,如果$T = - T^*$,则称其为**反自伴的**.

如果选取$A_1 = A_2 = I$,那么根据上面的分析见到$T$是自伴的当且仅当$T$是对称的,反自伴的当且仅当其是反对称的.

#### 分类问题
定义双线性形式上的同构:称$( V_1 , B_1 ) \cong ( V_2 , B_2 )$,其中$B_1 : V_1 \times V_1 \to F , B_2 : V_2 \times V_2 \to F$,当且仅当存在一个同构$\varphi : V_1 \cong V_2$,满足$B_2 ( \varphi ( \vec{ v } ) , \varphi ( \vec{ v } ' ) ) = B_1 ( \vec{ v } , \vec{ v } ' )$.容易验证这个同构满足等价条件的三条性质:反身性,对称性,传递性.并且双线性形式的几乎所有性质(左右根,根基,对称性,反对称性,退化性)都在同构关系下得到保持.这就意味着双线性形式应该可以以这种同构关系分类.

不失一般性,不妨直接假设$V = F^n$以简化描述.

我们定义两个$n \times n$的矩阵$A , A '$是**合同的**,当且仅当$\exists C \in ( M_{ n \times n } )^\times$使得$A = C^T A ' C$.容易验证这是一个等价关系.必须要强调这里的$C$是可逆矩阵.

接下来考虑两个双线性形式$B , B ' : F^n \times F^n \to F$,我们声称当且仅当它们对应的矩阵$A , A '$是合同的有$A = C^T A ' C$时是同构的.同构办法就是利用$C$所代表的线性映射.

对于充分性:只需检验是否有$B ( \vec{ v }_1 , \vec{ v }_2 ) = B ' ( C \vec{ v }_1 , C \vec{ v }_2 )$,而$B ' ( C \vec{ v }_1 , C \vec{ v }_2 ) = ( C \vec{ v }_1 )^T A ' ( C \vec{ v }_2 ) = ( \vec{ v }_1 )^T A \vec{ v }_2 = B ( \vec{ v }_1 , \vec{ v }_2 )$.

对于必要性:考虑所有的同构$\varphi$当然都可以写作可逆矩阵形式,设这个矩阵为$C$,立刻见到应当有$A = C^T A ' C$.

#### 二次型
考虑$\text{ char } \ne 2$的域$F$,原因是这种域上面存在$\frac{ 1 }{ 2 }$.为了方便讨论下面不妨直接假设$V = F^n$.

定义$n$元**二次型**为一个$F$上的齐次多项式$f = \sum_i a_{ i , i } x_i^2 + 2 \sum_{ i < j } a_{ i , j } x_i x_j$,并在考虑对称的情况下设$a_{ i , j } = a_{ j , i }$成立,上述自然有$f = \sum_{ i , j } a_{ i , j } x_i x_j$.容易见到$f$实际上可以表示为某一个对称的双线性形式,将这些$a$排列成矩阵就可以见到上述$f ( x ) = ( \vec{ x } )^T A \vec{ x } = B ( \vec{ x } , \vec{ x } )$.这种对应实际上是双射,原因是该多项式也可以反过去确定一个对称的双线性形式,原因是注意到首先$B ( \vec{ v } , \vec{ v } )$肯定是可以被确定的,其次由于$B ( \vec{ v }_1 + \vec{ v }_2 , \vec{ v }_1 + \vec{ v }_2 ) = B ( \vec{ v }_1 , \vec{ v }_1 ) + B ( \vec{ v }_2 , \vec{ v }_2 ) + 2 B ( \vec{ v }_1 , \vec{ v }_2 )$,于是立刻得到$B ( \vec{ v }_1 , \vec{ v }_2 ) = \frac{ 1 }{ 2 } ( f ( \vec{ v }_1 + \vec{ v }_2 ) - f ( \vec{ v }_1 ) - f ( \vec{ v }_2 ) )$,这就给出了一组一一对应的关系.从这里顺便应该能看出对称双线性形式应该由其所有的$B ( \vec{ v } , \vec{ v } )$的取值唯一确定.我们会在内积空间里再次讨论这个技巧,这被称为**配极化**.

将合同关系挪到二次型上就可以见到,两个二次型同构当且仅当它们可以通过一个可逆的线性变量替换而相互过渡.

既然如此,应该见到二次型某种意义上同构于矩阵,那么能不能把二次型作对角化呢,也即能否将二次型同构于形如$g = \sum b_i x_i^2$的二次型.这个当然可以用配方法解决,具体操作是数学归纳,每次将和一个变量有关的全部赛到一个里面去.

具体地,我们执行以下策略:

1. 如果当前存在一个$a_{ i , i } \ne 0$,不妨设其为$a_{ 1 , 1 }$,那就可以提出一个$a_{ 1 , 1 } ( x_1 + \frac{ 1 }{ a_{ 1 , 1 } } \sum_{ k \geq 2 } a_{ 1 , k } x_k )^2$.

2. 反之,则存在一个$a_{ i , j } \ne 0$,不妨在此时做变量替换$y_i = x_i - x_j$换掉$x_i$,这样$x_j^2$前的系数就非$0$,自然回到前一种情况.

上述过程不好机械化,能不能用一些更加形式化的方式呢?考虑配方当然等价于寻找一个可逆矩阵$C$使得对称矩阵$A$被写作$C^T AC = A '$,而可逆矩阵可以写作若干初等矩阵的乘积,不妨记作$U_1 \cdots U_k$,于是知道我们想要让$( U_1 \cdots U_k )^T A ( U_1 \cdots U_k )$.

这里有一个算法是将矩阵排列成$\begin{bmatrix}A \\ I\end{bmatrix}$的形式,然后对其做对称行列变换(这样列变换会影响到下面的$I$,但是行变换不会影响),这样最后会消成$\begin{bmatrix}A ' \\ C\end{bmatrix}$的形式,就会有$A ' = C^T AC$.

由此可以设$r = \text{ rk } ( f )$为对角化后剩余的元素个数.见到其根基的维数恰为$n - r$,原因是$\ker A$的维数就是根基的维数.见到其非退化当且仅当$n = r$.

如果我们做的域是一个代数闭域,那么我们上面的分类问题立刻得到了解决,原因是二次型当然可以进一步化简为$\sum_{ k = 1 }^r y_k^2$的形式,因此二次型完全由秩来决定.当然其实代数闭这个性质并没有用完全,实际上上述结论只需要求所有的$F$中元素都有平方根即可.

##### 实二次型
然而我们知道实数域并不满足以上性质,能不能把实数上的二次型进一步分类呢?虽然不能对$- 1$开根,但总之可以将二次型同构成$f = \sum_{ k = 1 }^p x_k^2 - \sum_{ j = p + 1 }^r x_j^2$.将此称作该二次型的**规范型**.

先对于实二次型引入一些新的定义:对于一个对称双线性形式,如果$\forall \vec{ v }$,$B ( \vec{ v } , \vec{ v } ) \geq 0$恒成立,则称其为**半正定的**,如果在此基础上当$\vec{ v } \ne 0$时$B ( \vec{ v } , \vec{ v } ) > 0$恒成立,则称其为**正定**的.同样可以定义**半负定的**和**负定的**概念.还可以定义**不定的**概念.容易见到如果一个对称双线性形式是正定的或者负定的时候它必然是非退化的,因为此时除了$\vec{ 0 }$其无根基.一定要注意这里的正定是在对称性的基础上定义的.

容易见到半正定当且仅当上述$f$中满足$p = r$,正定当且仅当满足$p = r = n$.到这里我们忽然疑问到同构的二次型的$p$是否相等.

##### 惯性(Sylvester)定理
即:对于两个同构的二次型,它们的$( p , r - p )$相同,一般将$p$称作**正惯性系数**而将$r - p$称作**负惯性系数**,两者的差$2 p - r$一般称作其**符号差**.

之所以会疑惑同构的二次型的$p$不一定相等,是因为同构只搬运了空间上的性质.因此如果我们能把$p$以某种基于空间上的形态,当然就可以显然看出两个同构的二次型的$( p , r - p )$相同.我们声明对于一个正惯性系数为$p$的二次型以下命题成立:

1. 存在一个$p$维的正定子空间.

2. 对于任何维数$> p$的子空间,其都不可能是正定的.

(1)显然,直接拿出其正部分对应的空间即可.

(2)的话无非是考虑将鸽笼原理用子空间之间的交写出来,设$N$为后$p + 1 , \cdots , n$这部分所代表的子空间,对于任何一个$V '$子空间满足$\dim V ' > p$,直接拿第二同构定理得到:

$$
\begin{aligned}
\dim ( V ' \cap N ) & = \dim ( V ' ) + \dim ( N ) - \dim ( V ' + N ) \\
& \geq n - p + \dim V ' - n > 0
\end{aligned}
$$

这意味着其不为空.

对负惯性系数当然可以类似做.这样惯性定理自然解决了实二次型的分类问题.

#### 辛空间
回到特征不为$2$的域$F$上,考虑反对称形式,注意到$B ( \vec{ v } , \vec{ v } ) = - B ( \vec{ v } , \vec{ v } )$,那此时它们就应该同时为$0$.这当然类似我们之前说的交错形式.

观察这类反对称形式的根基,取$R ( V ) = \{ \vec{ v } \in V \mid B ( \vec{ v } , \_ ) = 0 \}$.此时把补空间拿出来,取$V = R ( V ) \oplus V '$,我们断言$( V , B ) \cong ( R ( V ) , 0 ) \oplus ( V ' , B \mid_{ V ' \times V ' } )$,其中$B \mid_{ V ' \times V ' }$是$V '$上的非退化线性形式.这个断言当然是显然的,只需简单拆分空间即可.问题在于如何证明$B \mid_{ V ' \times V ' }$是$V '$上的非退化线性形式,只需证明$R ( V ' ) = \{ 0 \}$,我们考虑如果$\exists \vec{ v } ' \in V '$,$\forall \vec{ v }_1 ' \in V ' , B ( \vec{ v } ' , \vec{ v }_1 ' ) = 0$,由于根基当然和任何东西都会变零,因此这等价于$\forall \vec{ v }_1 \in V , B ( \vec{ v } ' , \vec{ v }_1 ) = 0$,这必然意味着$\vec{ v } ' \in R ( V )$,也就是$\vec{ v } ' \in R ( V ) \cap V ' = \{ 0 \}$.

因此其实反对称双线性形式更重要的是考虑其非退化的部分.将这些非退化反对称双线性形式称为**辛形式**,并将$( V , B )$称作辛空间.

考虑取一对基$\vec{ p } , \vec{ q }$形成空间$\langle \vec{ p } , \vec{ q } \rangle$并满足$B ( \vec{ p } , \vec{ q } ) = 1 = - B ( \vec{ q } , \vec{ p } )$,这当然唯一确定了一个辛形式$B$,并且其实际上对应了一个反对称矩阵$\begin{bmatrix}0 & 1 \\ - 1 & 0\end{bmatrix}$,这种二维辛空间一般还被称为**双曲辛平面**.

虽然研究范围被限制在了辛空间上,但仍然能看到辛形式中会因为正负相抵爆出大量的$0$.设$V_0$是$V$的子空间,若$B \mid_{ V_0 \times V_0 } \equiv 0$,我们称$V_0$是**全迷向子空间**,一个极大(真包含它的子空间全都不是全迷向的)的全迷向子空间称为**拉格朗日子空间**.

设$( V , B )$为辛空间,$L$是拉格朗日子空间,我们下面证明两个性质:

1. $L^\bot = L$.

2. $\dim V = 2 \dim L$.

3. 辛空间必定是偶数维的.

如果(1)得证,由于$\dim V = \dim L + \dim L^\bot$,(2)(3)也就显然了.所以问题在于如何证明(1).

首先全迷向性质自然等价于$L \subseteq L^\bot$.此时考虑若$\exists \vec{ v } \in L^\bot \setminus L$,则$L + \langle \vec{ v } \rangle$这个空间仍然是全迷向的(原因是$\vec{ v }$自身的部分会因为交错性质而成$0$),这就与其极大性矛盾.

我们试图想要找到更好的策略去勾勒$L$以及$V / L$这两部分.

##### 达布定理
该定理是说:给定一个拉格朗日子空间$L$,$L$的任何有序基$\vec{ p }_1 , \cdots , \vec{ p }_n$都能扩充为$V$的有序基$\vec{ p }_1 , \cdots \vec{ p }_n , \vec{ q }_n , \cdots , \vec{ q }_1$,使得$\forall 1 \leq i , j \leq n$:

1. $B ( p_i , p_j ) = B ( q_i , q_j ) = 0$.

2. $B ( p_i , q_j ) = - B ( q_j , p_i ) = \begin{cases}1 & i = j \\ 0 & i \ne j\end{cases}$.

这组有序基又被称为$V$的**辛基**.

考虑定义$n - 1$维子空间$L_i = \langle \vec{ p }_1 , \cdots , \vec{ p }_{ i - 1 } , \vec{ p }_{ i + 1 } , \cdots , \vec{ p }_n \rangle$.留意到此时会有$L = L^\bot \subsetneq L_i^\bot , \dim L_i^\bot = n + 1$.

接下来数学归纳构造这组$q$.

对于$k = 1$的情况,由于我们上面的分析,$\exists \vec{ q }_1 \in L_1^\bot \setminus L$,于是此时必然满足$B ( \vec{ p }_1 , \vec{ q }_1 ) \ne 0$而$\forall i \geq 2 , B ( \vec{ p }_i , \vec{ q }_1 ) = 0$.适当伸缩即可满足条件.

在此之后,假设$k < n$并且我们已经有了$\vec{ q }_1 , \cdots , \vec{ q }_k$使得条件成立,仍然可以取$\vec{ q }_{ k + 1 } ' \in L_{ k + 1 }^\bot \setminus L$,使得$B ( \vec{ p }_i , \vec{ q }_{ k + 1 } ' ) \begin{cases}1 & i = k + 1 \\ 0 & i \ne k + 1\end{cases}$满足.

此时注意到,如果用$\vec{ q }_{ k + 1 } = \vec{ q }_{ k + 1 } ' + \sum_i a_i \vec{ p }_i$后,上述条件实际上不受影响.然而,只需取$a_i = B ( \vec{ q }_i , \vec{ q }_{ k + 1 } ' )$就自动满足最后的条件.

接下来无非是要证明这些向量线性无关,对于$\sum a_i \vec{ p }_i + \sum b_i \vec{ q }_i = 0$,两边取$B ( \_ , \vec{ q }_i )$就可以得到$a_i = 0$,取$B ( \vec{ p }_i , \_ )$就可以得到$b_i = 0$,这样就做完了.

此时就可以见到,我们实际上是把原空间拆成了两个拉格朗日子空间的直和,并且这两个子空间之间以某种形式连接起来.此时其对应的矩阵就是:

$$
\begin{bmatrix}
& & & & & 1 \\
& & & & \cdots & \\
& & & 1 & & \\
& & - 1 & & & \\
& \cdots & & & & \\
- 1 & & & & &
\end{bmatrix}
$$

另外,如果$W$是$V$的子空间并且满足$W \cap W^\bot = \{ 0 \}$,我们称这样的子空间是**辛子空间**.只需套用拉格朗日子空间的过程就可以证明:

1. $W$是辛子空间当且仅当$B$在$W$上的限制非退化.

2. $W$是辛子空间则有直和分解$V = W \oplus W^\bot$.

#### 回看对偶空间
应当回忆我们将一个有限维空间对应到其对偶空间的时候所做的努力:先找到原空间的一组基,再对应地找到一组对偶基.然而,这种对应并非空间自身典范的对应,而是需要选定基的人为构造的定义.

回忆我们上述拿到的所有东西,应当可以见到起码以下资料均是典范的:

1. 典范配对$\langle \_ , \_ \rangle : V^\vee \times V \to F , \langle \lambda , \vec{ v } \rangle \mapsto \lambda \vec{ v }$.

2. 转置映射$^t T : W^\vee \to V^\vee , \lambda \mapsto \lambda T$.

##### 双重对偶
先分析典范配对的情形,考虑到典范配对会给出一个映射$ev_V : V \to ( V^\vee )^\vee , \vec{ v } \mapsto \langle \_ , \vec{ v } \rangle$.容易验证这当然是一个同构,而且中途没有用任何取基操作,所以这是一个典范的同构$\text{ ev }_V : V \cong ( V^\vee )^\vee$.更确切地来说,这个$\text{ ev }_V$具有某种函子性.事实上有以下交换图表:

$$
\xymatrix{ V \ar[r]^T \ar[d]_{ \text{ ev }_V } & W \ar[d]^{ \text{ ev }_W } \\
( V^\vee )^\vee \ar[r]^{ ( T^t )^t } & ( W^\vee )^\vee }
$$

虽属显然,但也可以小心地展开定义以确实证明$( \text{ ev }_W ) T = ( T^t )^t \text{ ev }_V$.策略只需任取$\check{ w }$,而观察:

$$
\begin{aligned}
& \langle ( ( T^t )^t \text{ ev }_V ) \vec{ v } , \check{ w } \rangle \\
= & \langle ( \text{ ev }_V ) \vec{ v } , T^t \check{ w } \rangle \\
= & \langle T^t ( \check{ w } ) , \vec{ v } \rangle \\
= & \langle \check{ w } , T \vec{ v } \rangle \\
= & \langle \text{ ev }_W ( T \vec{ v } ) , \check{ w } \rangle
\end{aligned}
$$

而$\check{ w }$可以任取,立刻证毕.

回忆到还有一件重要的事情需要解释,那就是柯里化$\text{ Hom } ( W , V^\vee ) \cong \text{ Bil } ( V , W ; F ) \cong \text{ Hom } ( V , W^\vee )$.从表面上看,它应当是典范的,可实则描述的时候却发现总需要标准内积结构来体现.为解决这个问题,我们下面证明上述同构映射将$\varphi \in \text{ Hom } ( W , V^\vee )$为$( \varphi^t ) \circ \text{ ev }_V \in \text{ Hom } ( V , W^\vee )$.

考虑将两边都映射到典范配对的结构上,由双线性形式的定义,$\varphi \in \text{ Hom } ( W , V^\vee )$必然映一组$\vec{ w } , \vec{ v }$为$\langle \varphi ( \vec{ w } ) , \vec{ v } \rangle$,这当然是显然的.

回看$( \varphi^t ) \circ \text{ ev }_V$,它做到的是将一组$\vec{ w } , \vec{ v }$映射到$\langle \text{ ev }_V ( \vec{ v } ) , \varphi ( \vec{ w } ) \rangle$,可只需用$\text{ ev }_V ( \vec{ v } )$的定义,上述两者当然是相同的.

再最后,我们终于回看基的选取,我们可以证明以下结论:设$V$是有限维的向量空间,$\vec{ v }_1 , \cdots , \vec{ v }_n \in V$是一组有序基,其对偶基记作$\check{ v }_1 , \cdots , \check{ v }_n$.我们声称:$V^\vee$中的一组元素$\text{ ev }_V ( \vec{ v }_1 ) , \cdots , \text{ ev }_V ( \vec{ v }_n )$是$\check{ v }_1 , \cdots , \check{ v }_n$的对偶基.也就是说中间不管怎么取对偶形态,最终总会回到一种典范的同构$V \cong V^\vee$上.

至于证明,无非只是:

$$
\langle \text{ ev }_V ( \vec{ v }_i ) , \check{ v }_j \rangle = \langle \check{ v }_j , \vec{ v }_i \rangle
$$

立属显然.

##### 核,余核与对偶映射
上述证明揭露了对偶空间的一角.之所以$T^t$会显示出伴随性质,实际上并不是代数形式上的巧合,而恰是$T^t$的定义本身就是典范配对上的伴随,而自然会以某种形式显示在新的伴随中.既如此,请以下内容再观$T : V \to W$以及其转置$T^t : W^\vee \to V^\vee$的性质.

首先,容易观察到的应该是$T$单则$T^t$满,$T$满则$T^t$单.此结论我们早就得出,策略是使用行秩等于列秩,而由同态基本定理得到$\dim V = \dim \ker T + \text{ rk } T$,如若$T$单,则$\dim \ker T = 0$,意味着$\text{ rk } T = \dim V$,也就意味着$\text{ rk } T^t = \dim V^\vee$,这就得到$T^t$是满的.反之亦然同理.

然而回看$\text{ rk } T^t = \text{ rk } T$这个结论的得出其实并不典范,我们推出此结论使用的策略是矩阵的相抵.而找到一种典范的证明此的策略便是重要的.而只需稍有对商操作的直觉就可以看到,要抛开矩阵,用一种真正典范的策略证明行秩等于列秩,必然离不开上述的引理.这就是我们下述真正想要做的事.

现在,设$T$是单射,对于任给的$\mu \in V^\vee$,尝试找到一个$\tilde{ \mu } \in W^\vee$使得$\tilde{ \mu } ( T ( x ) ) = \mu ( x )$,那根据对偶映射的定义,知道这里有$T^t ( \tilde{ \mu } ) = \mu$.如果这里$\mu$可以任选的话,换言之,对于任意的$\mu$我们都能构造出一个$\tilde{ \mu }$来实现上述过程,当然意味着$T^t$是满射.可既然这里是构造,我们终于可以放下"典范"的包袱,开始取基.

取$V$的一组基$\mathcal{ X }$,而$T ( \mathcal{ X } )$当然是$W$的线性无关子集,可以扩展为一组基$\mathcal{ Y }$,这样$\forall \vec{ w } \in W$都有唯一的展开$\sum_{ \vec{ y } \in \mathcal{ Y } } c_y \vec{ y }$,对于给定的$\mu \in V^\vee$可以定义$\tilde{ \mu } : W \to F$为:

$$
\tilde{ \mu } ( \sum_{ y \in \mathcal{ Y } } c_y \vec{ y } ) = \mu ( \sum_{ x \in \mathcal{ X } } c_{ T ( \vec{ x } ) \vec{ x } } )
$$

而当$T$满的时候,设$\tilde{ \mu } \in W^\vee$,此时如果$T^t ( \tilde{ \mu } ) = \tilde{ \mu } T = 0$,因为$T$是满的,这必然意味着$\tilde{ \mu } = 0$,所以$T^t$当然是单的.

换句话说,这里通过空间上的结构,用基强行凑出了一个$\ker T$的结构.然而取基总是会让我们忽略空间的结构,并给人一种使用文字游戏偷懒的感觉.或者说,构造性证明必然会有一种失掉典范性的感觉.

现在我们重新展现一下上述取出$\tilde{ \mu }$的合理性:设$T : V \to W$为线性映射,而$\mu \in V^\vee$,我们声称存在$\tilde{ \mu } \in W^\vee$使得$T^t ( \tilde{ \mu } ) = \mu$当且仅当$\mu |_{ \ker T } = 0$.

先证明充分性,当$\mu |_{ \ker T } = 0$的时候,不妨设$\twoheadrightarrow$是满射而$\hookrightarrow$是单射,如下交换图表当然成立:

$$
\xymatrix{ V \ar @{->>}[r]^T \ar[d]_\mu \ar @{->>}[dr]^\pi & \text{ im } \ ( T ) \\
F & V / \ker ( T ) \ar[l]^{ \bar{ \mu } } \ar @{^{(}->>}[u]_{ \bar{ T } } }
$$

左下三角形的成立性完全依赖于$\mu |_{ \ker T } = 0$的性质,而此时取一个$\tilde{ \mu_0 } = \bar{ \mu } \circ ( \bar{ T } )^{ - 1 } \in ( \text{ im } \ T )^{ \vee }$,而由于$\text{ im } T$自然嵌入$W$中,必有$\text{ im } ( T ) \hookrightarrow W$,那也就意味着$W^\vee \twoheadrightarrow \text{ im } ( T )^\vee$,因此$\tilde{ \mu_0 }$就可以延拓为$\tilde{ \mu } \in W^\vee$,交换图表给出:

$$
\tilde{ \mu } T = \tilde{ \mu_0 } T = \bar{ \mu } ( \bar{ T } )^{ - 1 } T = \bar{ m } u \circ \pi = \mu
$$

这就证明了充分性.至于必要性,由于$T^t ( \tilde{ \mu } ) = \tilde{ \mu } T$,而$\tilde{ \mu } T |_{ \ker T } = 0$当然是显然的.

接下来回忆到余核$\text{ coker } ( T ) = W / \text{ im } ( T )$这个东西,尝试看看它和$\ker T$之间的关系,回忆到我们应该有包含映射$i : \ker ( T ) \hookrightarrow V$和商映射$q : W \twoheadrightarrow \text{ coker } ( T )$,取对偶见到:

$$
\xymatrix{ \text{ coker } ( T )^\vee \ar @{^{(}->}[r]^{ q^t } & W^\vee \ar[r]^{ T^t } & V^\vee \ar @{->>}[r]^{ i^t } & \ker ( T )^\vee \\
\lambda \ar @{|->}[r] & \lambda q & & \\
& \tilde{ \mu } \ar @{|->}[r] & \tilde{ \mu } T & \\
& & \mu \ar @{|->}[r] & \mu i }
$$

这个当然没有问题,而我们试图借上面为辅助证明下面这张交换图表:

$$
\xymatrix{ \text{ coker } ( T )^\vee \ar @{^{(}->}[r]^{ q^t } \ar @{^{(}->>}[d] & W^\vee \ar[r]^{ T^t } & V^\vee \ar @{->>}[r]^{ i^t } \ar @{->>}[rd] & \ker ( T )^\vee \\
\ker ( T^t ) \ar @{^{(}->}[ur] & & & \text{ coker } ( T^t ) \ar @{^{(}->>}[u] }
$$

其中竖直方向的同构由左右两部分交换图表各自唯一确定.

先看左部分,我们尝试证明$\text{ coker } ( T )^\vee \hookrightarrow W^\vee$的像正好是$\ker ( T^t )$.

先证明$q^t ( \text{ coker } ( T )^\vee ) \subseteq \ker ( T^t )$,考虑$\lambda \in \text{ coker } ( T )^\vee$,那$q^t$会将其射到$\lambda q \in W^\vee$.留意到$T^t ( \lambda q ) = \lambda q T$,然而根据$\text{ coker }$的定义知道$qT = 0$,这必然意味着$T^t ( \lambda q ) = 0$.

再证明$\ker ( T^t ) \subseteq q^t ( \text{ coker } ( T )^\vee )$,设$\tilde{ \mu } \in \ker{ T^t }$,也就是说$T^t ( \tilde{ \mu } ) = \tilde{ \mu } T = 0$,我们要找到一个$\lambda$使得$q^t ( \lambda ) = \lambda q = \tilde{ \mu }$,那么必然见到$\lambda : \vec{ w } + \text{ im } ( T ) \mapsto \tilde{ \mu } ( \vec{ w } )$是满足条件的,只需验证$\tilde{ \mu } |_{ \text{ im } ( T ) } = 0$,这恰好由$T^t ( \tilde{ \mu } ) = \tilde{ \mu } T = 0$给出.

至于右半部分,就是我们上面所刻画的$\tilde{ \mu }$和$\mu$的关系了,换言之$\mu i = 0 \Leftrightarrow \exists \tilde{ \mu } , \mu = \tilde{ \mu } T$.

上述过程当然太过复杂了,尝试感性理解一下我们刚刚在做什么.我们的$\ker T$的意义是,$T : V \to W$这个过程中所损失的信息对吧,那我们反观$T^t : W^\vee \to V^\vee , \lambda \mapsto \lambda T$这个过程中所损失的,也就是何时,$\lambda T = 0$.由于这是两个映射,当然就等价于$\forall \vec{ v } \in V$,$\lambda T \vec{ v } = 0$.当然就等价于$\forall \vec{ u } \in \text{ im } ( T )$,$\lambda \vec{ u } = 0$,这必然等价于$\ker \lambda \supseteq \text{ im } ( T )$.所以这些$\lambda$都可以表示为一个$\tilde{ \lambda } q$的形式,其中$\ker q = \text{ im } ( T )$,取余核是刚刚好的.

##### 像自对偶
对于映射$T : V \to W$,我们下面将给出典范同构:

$$
\text{ im } ( T )^\vee \cong \text{ im } ( T^t )
$$

如上,商映射$q : W \to \text{ coker } ( T )$满足$\ker q = \text{ im } ( T )$,于是:

$$
\begin{aligned}
\text{ im } ( T )^\vee & \cong \text{ coker } ( q^t : \text{ coker } ( T )^\vee \to W^\vee ) \\
& \cong \text{ coker } ( \ker T^t \hookrightarrow W^\vee ) \\
& = W^\vee / \ker ( T^t ) \\
& \cong \text{ im } ( T^t )
\end{aligned}
$$

两边取$\dim$,得知行秩等于列秩.

### 实内积空间
考虑正定对称双线性形式$( \_ \mid \_ ) : V \times V \to F$,这样的资料$( V , ( \_ \mid \_ ) )$称为**内积空间(IPS)**.为了要一些$\mathbb{ R }$上的完备性质,我们下面主要讨论$F = \mathbb{ R }$的特殊情况.回忆道此时它应当满足的条件:

1. 双线性:各位有分配律以及标量乘法.

2. 对称性:$( \vec{ v } \mid \vec{ w } ) = ( \vec{ w } \mid \vec{ v } )$.

2. 正定性:$( \vec{ v } \mid \vec{ v } ) \geq 0$,并且等号成立当且仅当$\vec{ v } = \vec{ 0 }$.

回忆道二次型理论的时候我们曾经说过正定性是强于非退化的,因此内积一定是非退化的.

接下来定义$\vec{ v } \in V$的**长度**为$\Vert \vec{ v } \Vert = \sqrt{ ( \vec{ v } \mid \vec{ v } ) }$,并且如果$( \vec{ v } \mid \vec{ w } ) = 0$,则称它们**正交**,也写作$\vec{ v } \bot \vec{ w }$.同理可以定义正交空间$V_0^\bot$.还将满足$\Vert \vec{ v } \Vert = 1$的$\vec{ v }$称为**单位向量**.

#### 勾股定理
事实上可以写出所谓的**配极化**:$( \vec{ v }_1 \mid \vec{ v }_2 ) = \frac{ 1 }{ 2 } ( \Vert \vec{ v }_1 + \vec{ v }_2 \Vert^2 - \Vert \vec{ v }_1 \Vert^2 - \Vert \vec{ v }_2 \Vert^2 )$.

这里可以看出内积上的勾股定理:也就是当$\vec{ v } \bot \vec{ w }$时$\Vert \vec{ v } + \vec{ w } \Vert^2 = \Vert \vec{ v } \Vert^2 + \Vert \vec{ w } \Vert^2$.

#### 柯西不等式
即$( \vec{ v } \mid \vec{ w } )^2 \leq ( \vec{ v } \mid \vec{ v } ) ( \vec{ w } \mid \vec{ w } )$,或言$( \vec{ v } \mid \vec{ w } ) \leq \Vert \vec{ v } \Vert \cdot \Vert \vec{ w } \Vert$.

等式成立当且仅当$\vec{ v } , \vec{ w }$线性相关,此时也是容易证明的,只需要讨论$\vec{ v } = t \vec{ w }$以及反过来的情况即可.

当$\vec{ v } , \vec{ w }$线性无关时,那么$\vec{ v } \ne 0 , \vec{ w } \ne 0$,于是$\forall t , \vec{ v } + t \vec{ w } \ne 0$.这会导致:

$$
0 < ( \vec{ v } + t \vec{ w } \mid \vec{ v } + t \vec{ w } ) = t^2 ( \vec{ w } \mid \vec{ w } ) + 2 t ( \vec{ v } \mid \vec{ w } ) + ( \vec{ v } \mid \vec{ v } )
$$

将此看作关于$t$的二次多项式,那它就没有实根,于是观察判别式$\Delta = 4 ( \vec{ v } \mid \vec{ w } )^2 - 4 ( \vec{ v } \mid \vec{ v } ) ( \vec{ w } \mid \vec{ w } ) < 0$就见到柯西不等式成立.

用柯西不等式还可以定义两个向量之间的夹角,设其为$\angle ( \vec{ v } , \vec{ w } )$,并定义$\cos \angle ( \vec{ v } , \vec{ w } ) = \frac{ ( \vec{ v } \mid \vec{ w } ) }{ \Vert \vec{ v } \Vert \cdot \Vert \vec{ w } \Vert }$.

值得一提的是,柯西准则实际上保证了如果空间本身是连续的,那么内积就一定是连续的,原因是假设一列$( \vec{ x }_n , \vec{ y }_n ) \to ( \vec{ x } , \vec{ y } )$,无论以何种方式逼近,一定有:

$$
\begin{aligned}
& | ( \vec{ x }_n \mid \vec{ y }_n ) - ( \vec{ x } \mid \vec{ y } ) | \\
\leq & | ( \vec{ x }_n - \vec{ x } \mid \vec{ y }_n ) | + | ( \vec{ x } \mid \vec{ y }_n - \vec{ y } ) | \\
\leq & \Vert \vec{ x } - \vec{ x }_n \Vert \cdot \Vert \vec{ y }_n \Vert + \Vert \vec{ x } \Vert \cdot \Vert \vec{ y }_n - \vec{ y } \Vert
\end{aligned}
$$

这样就可以将其控制住.

#### 三角不等式
先用配极化,再用柯西不等式得到:

$$
\begin{aligned}
\Vert \vec{ v }_1 + \vec{ v }_2 \Vert^2 & = \Vert \vec{ v }_1 \Vert^2 + \Vert \vec{ v }_2 \Vert^2 + 2 ( \vec{ v }_1 \mid \vec{ v }_2 ) \\
& \leq \Vert \vec{ v }_1 \Vert^2 + \Vert \vec{ v }_2 \Vert^2 + 2 | ( \vec{ v }_1 \mid \vec{ v }_2 ) | \\
& \leq \Vert \vec{ v }_1 \Vert^2 + \Vert \vec{ v }_2 \Vert^2 + 2 \Vert \vec{ v }_1 \Vert \cdot \Vert \vec{ v }_2 \Vert \\
& = ( \Vert \vec{ v }_1 \Vert + \Vert \vec{ v }_2 \Vert )^2 \\

\end{aligned}
$$

#### 距离相关
如上可以定义**距离函数**$d : V \times V \to \mathbb{ R }_{ \geq 0 } , ( \vec{ v } , \vec{ w } ) \mapsto \Vert \vec{ w } - \vec{ v } \Vert$.容易见到其满足三角不等式$d ( \vec{ u } , \vec{ v } ) + d ( \vec{ v } , \vec{ w } ) \geq d ( \vec{ u } , \vec{ w } )$.

#### 正交向量族
选取$V$中的一组两两正交的非零元素,称其为**正交向量族**或者**正交子集**.特别地如果所有向量都是单位向量,则称其为**单位正交向量族**或者**单位正交子集**.容易见到正交向量族应该是线性无关的,原因是反证,如果$\sum a_k \vec{ v }_k = 0$,两边对$\vec{ v }_k$做内积就可以得到$a_k \Vert \vec{ v }_k \Vert^2 = 0$,根据正定性得到$a_k = 0$.

既然如此,我们就将由一个单位正交子集所给出的基称为**单位正交基(ONB)**.

如果我们能拿出一组$n$个元素的单位正交基$\langle \vec{ v }_1 , \cdots \vec{ v }_n \rangle$,那么考虑用这组有序基将$V \to \mathbb{ R }^n$.则注意到$\forall \vec{ v } \in V$,应当有:$\vec{ v } = \sum_k a_k \vec{ v }_k$.并且两边对$\vec{ v }_k$取内积就可以见到$a_k = ( \vec{ v } \mid \vec{ v }_k )$.我们可以证明在这组基的同构下原本的内积就同构于$\mathbb{ R }^n$上的标准内积(点乘).因此需要验证这个映射是否是保距的,事实上注意到:

$$
\begin{aligned}
& ( \sum_i a_i \vec{ v }_i \mid \sum_j b_j \vec{ v }_j ) \\
= & \sum_{ i , j } a_i b_j ( \vec{ v }_i \mid \vec{ v }_j ) \\
= & \sum_i a_i b_i
\end{aligned}
$$

#### Gram-Schmidt 正交化
问题现在在于单位正交基是否总是存在以及如何找到一个.我们先取一组向量$\vec{ v }_1 , \vec{ v }_2 , \cdots$线性无关(这里甚至允许可数无穷个向量),递归定义:

$$
\begin{aligned}
\vec{ w }_1 & = \vec{ v }_1 \\
\vec{ w }_k & = \vec{ v }_k - \sum_{ i = 1 }^{ k - 1 } \frac{ ( \vec{ w }_i \mid \vec{ v }_k ) }{ ( \vec{ w }_i \mid \vec{ w }_i ) } \vec{ w }_i
\end{aligned}
$$

就可以取出一组正交基,之后只需令$\vec{ u }_k = \frac{ \vec{ w }_k }{ \Vert \vec{ w }_k \Vert }$就可以转化为一组单位正交基.事实上还可以注意到$\langle \vec{ w }_1 , \cdots \vec{ w }_k \rangle = \langle \vec{ v }_1 , \cdots , \vec{ v }_k \rangle$.其构造思路是每次添加一个拥有两项的元素,一项要是前面的空间内的部分,另一部分不是,然而它们之和要与前面空间正交.

证明的话,注意到:$\vec{ w }_k \in \vec{ v }_k + \langle \vec{ w }_1 , \cdots , \vec{ w }_{ k - 1 } \rangle$,因此数学归纳证明$\langle \vec{ w }_1 , \cdots \vec{ w }_k \rangle = \langle \vec{ v }_1 , \cdots , \vec{ v }_k \rangle$:

$$
\begin{aligned}
& \langle \vec{ w }_1 , \cdots , \vec{ w }_k \rangle \\
= & \langle \vec{ w }_1 , \cdots , \vec{ w }_{ k - 1 } \rangle + \langle \vec{ w }_k \rangle \\
= & \langle \vec{ v }_1 , \cdots , \vec{ v }_{ k - 1 } \rangle + \langle \vec{ v }_k \rangle \\
= & \langle \vec{ v }_1 , \cdots , \vec{ v }_k \rangle
\end{aligned}
$$

此外上述的算法的优点在于如果$\vec{ v }_k$已经和前面的$\vec{ w }_1 , \cdots , \vec{ w }_{ k - 1 }$正交,自然有$\vec{ w }_k = \vec{ v }_k$.

而验证正交无非是两边同时对$\vec{ w }_j$做内积并继续归纳即可.

由上给出两个推论:

1. 任何有限维内积空间都有单位正交基.

2. 任何单位正交子集都可以扩充为一个单位正交基.

(1)找到一组基用上述算法即可,(2)的话可以先扩充成基,再按照上述算法,而前面原本已经正交的部分不会改变.

###### Example1(RU分解)
观察上述Gram-Schmidt正交化过程就可以看到,对于一个矩阵$A = ( \vec{ v }_1 , \cdots , \vec{ v }_n )$,对其作正交化,每次相当于右乘(列变换)一个上三角矩阵(没有交换列的操作),最终变换的形态则会是一个正交矩阵.因此,任何一个可逆矩阵都可以被分解成$RU$,其中$R$是一个正交矩阵,$U$是一个上三角矩阵.

###### Example2(Legendre多项式)
考虑$\mathbb{ R }$上的多项式组成的$\mathbb{ R } -$向量空间$\mathbb{ R } [ x ]$,定义内积$( f \mid g ) = \int_{ - 1 }^1 f ( x ) g ( x ) \text{ d } x$,容易见到其满足定义.此外,$\mathbb{ R } [ x ]$的一组自然的基是$\langle 1 , x , x^2 \cdots \rangle$.

是否可以对其进行正交化呢?考虑限制得到的单位正交基的最高次系数均为$1$,容易见到这组单位正交基如此便被唯一刻画,并且必然是上述做Gram-Schmidt正交化的产物.

既然如此,我们接下来尝试检验$P_0 = 1 , P_n = \frac{ 1 }{ 2^n n ! } ( ( x^2 - 1 )^n )^{ ( n ) }$就是一组正交基.我们声称以下命题成立:

1. $[ x^n ] P_n = \frac{ ( 2 n ) ! }{ 2^n ( n ! )^2 }$.

2. $P_n ( 1 ) = 1$.

3. 当$0 \leq k < n$时,$\int_{ - 1 }^1 t^k P_n ( t ) \mathrm{ d } t = 0$.作为此的一个推论,$n \ne m$的时候$\int_{ - 1 }^1 P_m ( t ) P_n ( t ) \mathrm{ d } t = 0$.

4. $P_n ( - t ) = ( - 1 )^n P_n ( t )$.

5. $\int_{ - 1 }^1 P_n ( t )^2 \mathrm{ d } t = \frac{ 2 }{ 2 n + 1 }$.

6. 递归式:$( n + 1 ) P_{ n + 1 } = ( 2 n + 1 ) x ( P_n ) - nP_{ n - 1 }$.

对于(1),讨巧的策略是直接看$\lim_{ x \to \infty } \frac{ P_n }{ x^n }$,而于此使用洛必达法则上下求导$n$次,立刻证毕.

对于(2),考虑$( x^2 - 1 ) = ( x - 1 ) ( x + 1 )$,用Leibniz律求导,得到$P_n ( 1 ) = \frac{ 1 }{ 2^n n ! } n ! ( 1 + 1 )^n = 1$.

对于(3),只需分部积分,每次将$P_n ( t )$给扔到$\rm d$里,这样前面的$t^k$就会被不断消耗,最终得到结果.

对于(4),考虑在求导前的部分当然都是一样的,因此只是求导的时候,$P_n ( t )$是$\frac{ 1 }{ ( \mathrm{ d } t )^n }$,而$P_n ( - t ) = \frac{ 1 }{ ( ( \mathrm{ d } ) ( - t ) )^n }$.

对于(5),把$P_n ( t )^2 = P_n ( t ) \times P_n ( t )$,然后使用分部积分,得到的结果当然就是:

$$
\begin{aligned}
& ( - 1 )^n \frac{ ( 2 n ) ! }{ ( 2^n n ! )^2 } \int_{ - 1 }^1 ( t^2 - 1 )^n \mathrm{ d } t \\
= & 2 ( - 1 )^n \frac{ ( 2 n ) ! }{ ( 2^n n ! )^2 } \int_{ 0 }^1 ( t^2 - 1 )^n \mathrm{ d } t \\
= & 2 \frac{ ( 2 n ) ! }{ ( 2^n n ! )^2 } \int_{ 0 }^1 ( 1 - t^2 )^n \mathrm{ d } t \\

\end{aligned}
$$

做换元$\cos \theta = t$,则:

$$
\begin{aligned}
& 2 \frac{ ( 2 n ) ! }{ ( 2^n n ! )^2 } \int_{ 0 }^1 ( 1 - t^2 )^n \mathrm{ d } t \\
= & 2 \frac{ ( 2 n ) ! }{ ( 2^n n ! )^2 } \int_{ 0 }^{ \frac{ \pi }{ 2 } } \sin ( t )^{ 2 n + 1 } \mathrm{ d } t \\
= & 2 \frac{ ( 2 n ) ! }{ ( 2^n n ! )^2 } \frac{ ( 2 n ) ! ! }{ ( 2 n + 1 ) ! ! } \\
= & 2 \frac{ ( 2 n ) ! }{ ( 2^n n ! )^2 } \frac{ ( 2^n n ! )^2 }{ ( 2 n + 1 ) ! } \\
= & \frac{ 2 }{ 2 n + 1 }
\end{aligned}
$$

于是证毕.

对于(6),考虑既然上述Legendre多项式是一组基,那么$xP_n$作为一个$n + 1$次多项式,理应可以被表示出来.也就是有:

$$
xP_n = \sum_{ k = 0 }^{ n + 1 } a_k P_{ k }
$$

两边对$P_{ k } , k \leq n - 2$做内积,那么左侧就是$\int_{ - 1 }^1 ( xP_k ( x ) ) P_n ( x ) \mathrm{ d } x$,此时考虑$xP_k ( x )$是一个$k + 1 < n$次多项式,所以左侧理应为$0$,于是右侧的$a_k = 0$.上式被我们简化为:

$$
xP_n = a_n P_{ n + 1 } + b_n P_{ n } + c_n P_{ n - 1 }
$$

考虑两边提取$[ x^{ n + 1 } ]$,立刻知道$a_n = \frac{ n + 1 }{ 2 n + 1 }$.

两边对$P_n$做内积,注意到$x ( P_n )^2$是一个奇函数,所以左边为$0$,所以$b_n = 0$.

接下来是$c_n$.考虑$( xP_n \mid P_{ n - 1 } ) = ( P_n \mid xP_{ n - 1 } )$,而由刚才的$a_n$,$xP_{ n - 1 } = \frac{ n }{ 2 n - 1 } P_n + P '$,因此立刻见到$c_n = \frac{ n }{ 2 n + 1 }$,这就得证了.

#### 正交算子
如果拿出两个内积空间,并能找到一个映射$\varphi : V \to W$使得保$\Vert \varphi ( \vec{ v } ) \Vert_W = \Vert \vec{ v } \Vert_V$,那么称其为**保距同构**.用配极化容易见到保距同构一定保持了内积.容易见到如果$\varphi$是同构,那么$\varphi^{ - 1 }$当然也是同构的.

考虑取两个有限维内积空间$V , W$.由于内积非退化并且对称,于是应当对于所有线性映射$T : V \to W$都有伴随$T^* : W \to V$使得$( T \vec{ v } \mid \vec{ w } )_W = ( \vec{ v } \mid T^* \vec{ w } )_V$并且$( T^* \vec{ w } \mid \vec{ v } )_V = ( \vec{ w } \mid T \vec{ v } )_W$.

接下来我们尝试证明:$T$是保距同构当且仅当$T^* = T^{ - 1 }$.

先证必要性:当$T$是保距同构的时候,见到$T^{ - 1 }$必然也是同构,这就意味着$( T \vec{ v } \mid \vec{ w } )_W = ( T^{ - 1 } T \vec{ v } \mid T^{ - 1 } \vec{ w } )_V = ( \vec{ v } \mid T^{ - 1 } \vec{ w } )_V$对于$\forall \vec{ v } , \vec{ w }$都成立,这当然意味着$T^* = T^{ - 1 }$.

再证充分性,当$T^* = T^{ - 1 }$时,考虑$( T \vec{ v }_1 \mid T \vec{ v }_2 )_W = ( \vec{ v }_1 \mid T^* T \vec{ v }_2 )_V = ( \vec{ v }_1 \mid \vec{ v }_2 )_V , \forall \vec{ v }_1 , \vec{ v }_2$,这就意味着其是保距同构.

另外还可以证明如果$\vec{ v }_1 , \cdots , \vec{ v }_n$是$V$的单位正交基时,$T$是保距同构当且仅当$T \vec{ v }_1 , \cdots , T \vec{ v }_n$是$W$的单位正交基.

必要性源于保距同构保持了关于内积的一切性质,因此显然.接下来考虑充分性,如果$T \vec{ v }_1 , \cdots , T \vec{ v }_n$是$W$的单位正交基,那么:

$$
\begin{aligned}
\Vert \sum_k a_k \vec{ v }_k \Vert_V^2 & = \sum_k a_k^2 \\
\Vert T ( \sum_k a_k \vec{ v }_k ) \Vert_W^2 & = \Vert \sum_k a_k ( T \vec{ v }_k ) \Vert^2_W = \sum_k a_k^2
\end{aligned}
$$

这意味着其保距.并且$\dim V = n = \dim W$意味着是同构.

接下来定义有限维内积空间的自同构称为$V$上的**正交变换**.现在不妨假设$V = \mathbb{ R }^n$并将视角转移到标准内积上(此时应当有$A^* = A^T$)尝试使用矩阵来描述该问题.容易见到以下命题等价,并将满足下列性质的矩阵称为**实正交矩阵**:

1. $A^{ - 1 } = A^T$.

2. $A$相对于标准内积是正交变换.

由此得到以下推论:

1. 单位矩阵是正交矩阵.

2. 如果$A$和$B$都是正交矩阵,那么$AB$亦然.

3. 如果$A$是正交矩阵,则$A^T , A^{ - 1 }$均亦然.

4. 正交矩阵的行列式为$\pm 1$.

5. 对于矩阵$A = ( \vec{ v }_1 , \cdots , \vec{ v }_n )$,$A$是正交矩阵当且仅当$\vec{ v }_1 , \cdots , \vec{ v }_n$是一组单位正交基.

6. 正交矩阵的特征值(无论实复)必然满足$| \lambda | = 1$.

(1)(2)(3)显然,(4)则是因为$( \det A )^2 = \det ( A^T ) \det A = 1$.

(5)的话原因是$\mathbb{ R }$下$\vec{ e }_1 , \cdots \vec{ e }_n$是一组标准正交基,而$\vec{ v }_k = A \vec{ e }_k$,因此根据前面提到的正交矩阵对标准正交基的转译性质即证毕.

(6)的话,考虑标准内积空间$( A \vec{ v } \mid A \vec{ v } )$,其中$\vec{ v }$是以$\lambda$为特征值的特征向量,那么首先,$( A \vec{ v } \mid A \vec{ v } ) = | \lambda |^2 ( \vec{ v } \mid \vec{ v } )$,另一方面,其又是$( ( A \vec{ v } )^t ) ( A \vec{ v } ) = ( \vec{ v } \mid \vec{ v } )$.这就完事了.

#### 正交补空间
取$S$为$V$的任意子集,则称$S^\bot = \{ \vec{ v } \in V \mid \forall \vec{ s } \in S , ( \vec{ s } \mid \vec{ v } ) = 0 \}$为$S$的**正交补空间**.

虽然在此定义下$S$无需是子空间,但仍见到$S^\bot$自动对加法和纯量乘法封闭,因此$S^\bot$仍是一个子空间.然而其实这里$S^\bot = ( \langle S \rangle )^\bot$,这当然是显然的,因此后面只关注$S = V_0$是$V$的子空间的情况.

虽然之前我们做双线性形式的时候正交空间并没有太好的性质,但此时对于内积的情况有,我们声称$V = V_0 \oplus V_0^\bot$.原因是任取一组$V_0$的单位正交基$\vec{ v }_1 , \cdots , \vec{ v }_m$,注意到$\vec{ v } = \sum_{ k } ( \vec{ v }_k \mid \vec{ v } ) \vec{ v }_k + ( \vec{ v } - \sum_{ k } ( \vec{ v }_k \mid \vec{ v } ) \vec{ v }_k )$,前者显然属于$V_0$,而后者只需逐个对$\vec{ v }_k$做内积就可以验证其属于$V_0^\bot$.不妨将前者称为$\vec{ v }$在$V_0$上的正交投影.并将$P : V \to V_0 , \vec{ v } \mapsto \vec{ v }_0$称作正交投影算子,其中$\vec{ v } = \vec{ v }_0 + \vec{ v }_1$,$\vec{ v }_0 \in V_0 , \vec{ v }_1 \in V_0^\bot$.另外,观察到伴随映射有$( \text{ im } \ T )^\bot = \ker T^*$,见到$V = \ker T^* + \text{ im } \ T$.

取$V_0 \subseteq V$,并且取$V = V_0 \oplus V_0^\bot$,考虑$P$映射是从$V$到$V_0$的投影,那么$\forall \vec{ v }$,$\min_{ \vec{ u } \in V_0 } \Vert \vec{ u } - \vec{ v } \Vert$在$\vec{ u } = P \vec{ v }$的时候取最小.原因是不妨设$\vec{ v } = \vec{ v }_0 + \vec{ v }_1$,使用勾股定理:

$$
\begin{aligned}
& \Vert \vec{ u } - \vec{ v } \Vert^2 \\
= & \Vert \vec{ u } - \vec{ v }_0 - \vec{ v }_1 \Vert^2 \\
= & \Vert \vec{ u } - \vec{ v }_0 \Vert^2 + \Vert \vec{ v }_1 \Vert^2 \\
\geq & \Vert \vec{ v }_1 \Vert^2
\end{aligned}
$$

此时仔细观察投影算子$P : V \to V_0$,应当可以将$P$视作$\text{ End } ( V )$中的某元素,我们下面证明$P$是正交投影算子当且仅当$P^* = P$和$P^2 = P$同时成立.

先证明必要性:仍选择将$\vec{ v } = \vec{ v }_0 + \vec{ v }_1$,其中$\vec{ v }_0 \in V_0$而$\vec{ v }_1 \in V_0^\bot$,此时$P \vec{ v } = \vec{ v }_0$,而$P^2 \vec{ v } = \vec{ v }_0 = P \vec{ v }$,因此$P^2 = P$总是成立.另外如果有$\vec{ v } ' = \vec{ v }_0 ' + \vec{ v }_1 '$,那么应当注意到:

$$
\begin{aligned}
& ( P \vec{ v } \mid \vec{ v } ' ) \\
= & ( \vec{ v }_0 \mid \vec{ v } ' ) = ( \vec{ v }_0 \mid \vec{ v }_0 ' ) = ( \vec{ v } \mid \vec{ v }_0 ' ) \\
= & ( \vec{ v } \mid P \vec{ v } ' )
\end{aligned}
$$

然后证明充分性,直接取$V_0 = \text{ im } ( P )$,那么$\forall \vec{ v } \in V$当然$\exists \vec{ u }$有$\vec{ v } = P \vec{ u } + \vec{ v }_1$.此时注意到$P \vec{ v } = P \vec{ u } + P \vec{ v }_1$.观察发现$( P \vec{ v }_1 \mid P \vec{ v }_1 ) = ( P^* P \vec{ v }_1 \mid \vec{ v }_1 ) = ( P \vec{ v }_1 \mid \vec{ v }_1 )$,然而$\vec{ v }_1 \in V_0^\bot$而$P \vec{ v }_1 \in V_0$,于是$( P \vec{ v }_1 \mid P \vec{ v }_1 ) = 0$,于是$P \vec{ v }_1 = 0$.因此$P \vec{ v } = P \vec{ u }$.

接下来定义一个正交投影算子的**镜像**为$2 P - \text{ id }_V$,其将$\vec{ v }_0 + \vec{ v }_1$映射到$\vec{ v }_0 - \vec{ v }_1$上.我们想要证明如果$P$是正交投影算子,那么$2 P - \text{ id }_V$是正交变换.容易检验$( 2 P - \text{ id }_V )^* = ( 2 P - \text{ id }_V )$,那就只需要验证:

$$
\begin{aligned}
& ( 2 P - \text{ id }_V )^* ( 2 P - \text{ id }_V ) \\
= & ( 2 P - \text{ id }_V ) ( 2 P - \text{ id }_V ) \\
= & 4 P^2 - 4 P + \text{ id }_V \\
= & \text{ id }_V
\end{aligned}
$$

还可以证明:如果$V_0$是$T$不变子空间,那么$( V_0 )^\bot$是$T^*$的不变子空间,证明的话只需考虑$( T^* \vec{ w } \mid \vec{ v } ) = ( \vec{ w } \mid T \vec{ v } ) = 0$,其中$\vec{ v } \in V_0$而$\vec{ w } \in ( V_0 )^\bot$.这将为我们把空间分解为$V_0 \oplus ( V_0 )^\bot$提供帮助.更详细的讨论将在自伴算子处讨论.

#### 投影矩阵
考虑向量空间$V$和$A_1 , \cdots , A_s \in \text{ End } ( V )$,满足$A_1 + \cdots + A_s = \text{ id }$,并且$\forall i \ne j , A_i A_j = 0$,而且$A_i^2 = A_i$.

此时令$V_i = \text{ im } ( A_i )$.我们下面证明以下三条事实:

1. $A_i ( \sum \vec{ v }_k ) = \vec{ v }_i$,其中$\vec{ v }_k \in V_k$.

2. $V = \bigoplus V_k$.

3. 若$A^2 = A$,则$V = \text{ im } ( A ) \oplus \text{ im } ( \text{ id } - A ) = \text{ im } A \oplus \ker A$.

(1)显然.

考虑(2)的证明,由于$V = \text{ im } ( \text{ id } ) \subseteq \sum V_k \subseteq V$,于是$\sum V_k = V$.只需再证明$\forall k , V_k \cap ( \sum_{ j \ne k } V_j ) = \{ \vec{ 0 } \}$即可.而$\forall \vec{ w } \in V_k$,考虑$A_k ( \vec{ w } ) = \vec{ w }$但$A_k ( \sum_{ j \ne k } V_j ) = 0$,这意味着$\vec{ w } \notin ( \sum_{ j \ne k } V_j )$,这就证毕了.

考虑(3)的证明,只需检验$A ( \text{ id } - A ) = 0$,这是显然的.

而在特征为$0$的域上,我们尝试证明更强一点的结论:当$\sum A_k = \text{ id }$的时候,以下三条命题等价:

1. 对于每个$i$都有$A_i^2 = A_i$.

2. $\sum \text{ rk } ( A_i ) = \dim V$.

3. $\forall i \ne j$,$A_i A_j = 0 = A_j A_i$.

(1)$\Rightarrow$(2)的话考虑对于单个$A_i$,我们上面已经证明过当$A_i^2 = A_i$的时候应当有直和分解$V = \text{ im } ( A_i ) \oplus \text{ im } ( \text{ id } - A_i )$,此时任取一组基限制在$\text{ im } ( A_i )$上都是恒等变换,于是$\text{ tr } ( A_i ) = \text{ rk } ( A_i )$.这个证明可能略有口胡,或者你直接观察$A_i$的极小多项式为$x ( x - 1 ) = 0$,立刻得到其可对角化并且对角上只有$1$和$0$,并且只划分了两个子空间$V_0$和$V_1$,再用迹对于相似不变就可以得到$\text{ tr } ( A_i ) = \text{ rk } ( A_i )$.于是$\sum \text{ rk } ( A_i ) = \sum \text{ tr } ( A_i ) = \text{ tr } ( \text{ id } ) = \dim V$.

(2)$\Rightarrow$(3)的话,考虑同上用$V = \text{ im } ( \text{ id } ) \subseteq \sum V_k \subseteq V$,所以$\sum V_k = V$.这就可以看到同态$\sigma : \bigoplus V_k \to V , ( \vec{ v }_i ) \mapsto \sum \vec{ v }_i$是满的,比较维数就知道$\dim \ker = 0$,所以这是同构.这意味着$\forall k , j , \text{ im } ( A_k ) \cap \text{ im } ( A_j ) = \{ \vec{ 0 } \}$.而我们知道$\text{ im } ( A_k ) \oplus \ker A_k \cong V$,在这里的直和应该直接表现为$\text{ im } ( A_k ) + \ker A_k = V$,这样就直接把$A_j$扔进了$\ker A_k$里面.

(3)$\Rightarrow$(1)是简单的,原因是$A_k = A_k ( \sum_{ j \ne k } A_j ) + A_k \times A_k$.

如果我们将满足$A^2 = A$的矩阵称为投影矩阵的话,可以发现一个矩阵是投影矩阵的必要条件是$\ker A + \text{ im } \ A = V$.然而并不充分,除非再加上$A ( I - A ) = 0$.

#### 自伴算子
对于一个内积空间$( V , ( \_ , \_ ) \ )$,其中$\dim V = n$.

称一个映射是**自伴的**,当且仅当$T = T^*$.我们下面尝试证明如果$T$是自伴的,那么$T$可以正交对角化.换言之存在正交映射$P$使得$P^{ - 1 } TP = P^* TP$是对角的.另外,如果$T$可以正交对角化,那么也仅当$T = T^*$.原因当然显然,因为只需两边取伴随即可见得.

对$n = \dim V$进行归纳,下面不妨假设$n \geq 2$.

回忆到这个内积空间应该可以同构于标准内积空间$( \mathbb{ R }^n , \cdot )$,因此将自伴算子同构于$A^T = A^* = A$的矩阵.假设可以找到$T$的某一个实特征值$\lambda_1$,我们就可以取其对应的特征向量$\vec{ v }_1 \ne 0$并且满足$T \vec{ v }_1 = \lambda_1 \vec{ v }_1$,而且满足$\Vert \vec{ v }_1 \Vert = 1$,并设$V_0 = \text{ span } ( \vec{ v }_1 )$,回忆到我们有正交直和分解$V = V_0 \oplus ( V_0^\bot )$.

此时观察,既然$V_0$是$T$不变子空间,我们之前证明了$V_0^\bot$是$T^* = T$的不变子空间,既然如此,$T$限制在$V_0^\bot$上就仍然自伴,这样就可以进行数学归纳.

那么怎么找到$T$的一个实特征值呢?为了证明这个定理,我们可能需要先跳出实数域而在复数域上做一些操作.在复数域上的好处在于可以定义共轭,具体地容易发现$\overline{ A + B } = \bar{ A } + \bar{ B }$以及$\overline{ A \times B } = \bar{ A } \times \bar{ B }$.

对于复数域上的矩阵,我们约定$A^\dagger = \bar{ A^T }$,注意到$\lambda^\dagger = \bar{ \lambda }$,并且理应有$( AB )^\dagger = B^\dagger A^\dagger$.此时取$\vec{ v } = ( z_1 , \cdots , z_n ) \in \mathbb{ C }^n$,注意到$( \vec{ v } )^\dagger ( \vec{ v } ) = \sum | z_k |^2 \in \mathbb{ R }_{ \geq 0 }$,并且其等于$0$当且仅当$\vec{ v } = 0$.

下面我们证明:当$A^\dagger = cA , c \in \mathbb{ C }$,那么$A$的所有特征值都满足$\bar{ \lambda } = c \lambda$.不过容易见到这里只能取$c = \pm 1$,原因是$c^2 = 1$.这个性质我们会在后面复内积空间中的hermite形式中进一步用到.

证明的话,考虑$\vec{ v } \ne 0$是以$\lambda \in \mathbb{ C }$为特征值的特征向量,那么我们考虑$( \vec{ v } )^\dagger ( A \vec{ v } ) = ( \vec{ v } )^\dagger ( \lambda ) \vec{ v }$.此时两边取$\dagger$,左边给出$( \vec{ v } )^\dagger A^\dagger \vec{ v } = c ( \vec{ v } )^\dagger A \vec{ v } = c \lambda ( \vec{ v } )^\dagger \vec{ v }$,而右边给出$\bar{ \lambda } ( \vec{ v } )^\dagger ( \vec{ v } )$,这样就证明了上述结论.

那么这样做的意义是什么呢?已经知道实数上的自伴算子满足$A^\dagger = A$,也就是在上面$c = 1$,那意味着$\lambda = \bar{ \lambda }$,意味着$\lambda$是实数.然而,由于在$\mathbb{ C }$是代数闭的,那么就一定可以找到至少一个复特征值,由上知道这些特征值还都是实数,那我们就补上了上述证明的最后一步.

于是,回忆到直和与分块矩阵的关系,将我们拿到的这些特征向量做正交化后,取$P$为这些单位正交基$\vec{ v }_1 , \cdots , \vec{ v }_n$为列向量的矩阵,那么$P^{ - 1 } AP = \begin{bmatrix}\lambda_1 & & \\ & \ddots & \\ & & \lambda_n\end{bmatrix}$.

或者直接来看,我们断言自伴算子对应的特征子空间一定互相正交,原因是设它们分别是$V_{ \lambda_1 } , V_{ \lambda_2 }$,那么$\lambda_1 ( \vec{ v }_1 \mid \vec{ v }_2 ) = ( T \vec{ v }_1 \mid \vec{ v }_2 ) = ( \vec{ v }_1 \mid T \vec{ v }_2 ) = \lambda_2 ( \vec{ v }_1 \mid \vec{ v }_2 )$.这就必然给出$( \vec{ v }_1 \mid \vec{ v }_2 ) = 0$.那只需先求特征向量,然后对此施加Gram-Schmidt正交化就赢了.

##### (实)Sylvester 判准
将一个矩阵的左上的$k \times k$的矩阵称为$A$的顺序主子式.

容易发现,实对称矩阵正定当且仅当其所有特征值皆正,原因是正交对角化后的结果.

由此可以得到Sylvester判准,也就是一个实对称矩阵正定当且仅当其所有顺序主子式皆正.

必要性显然,把线性空间限制在左上角的那个$k \times k$里,如果有某个顺序主子式是负的,那就存在负的特征值.

下面证明充分性:考虑数学归纳,当$n \geq 2$的时候,假设$A$的每个顺序主子式都是正的,我们下面试图证明其所有特征值都是正的.记其特征值为$\lambda_1 , \cdots , \lambda_n$,必定有等式$\lambda_1 \cdots \lambda_n = \det A > 0$.也就是说,如果$A$有负特征值则必然成对出现,不妨设其为$\lambda_1$和$\lambda_2$(当然它们有可能相等,但总之应该取不同的特征向量),此时不妨取它们的特征向量并做单位正交化后得到$\vec{ v }_1 , \vec{ v }_2$.此时$\forall \alpha , \beta \in \mathbb{ R }$,立刻有:

$$
\begin{aligned}
& ( \alpha \vec{ v }_1 + \beta \vec{ v }_2 )^t A ( \alpha \vec{ v }_1 + \beta \vec{ v }_2 ) \\
= & \alpha^2 \lambda_1 + \beta^2 \lambda_2 \leq 0
\end{aligned}
$$

其中$\alpha , \beta$可以任取,当然存在不全为$0$的一对$( \alpha , \beta )$使得$\alpha \vec{ v }_1 + \beta \vec{ v }_2$作为列向量的第$n$个坐标为$0$,此时对于左上角的$( n - 1 ) \times ( n - 1 )$的空间来说,由于进行了数学归纳,上面必然是正定的.这当然就矛盾了.

##### (实)正定矩阵的二次根
设$T \in \text{ End } ( V )$正定(或者半正定),那么就存在唯一的正定(或半正定)的$S \in \text{ End } ( V )$使得$S^2 = T$,这样我们记$S = \sqrt{ T }$.

(一定要记得正定蕴含着其对称啊,总是忘记这个定义)

存在性的话只需要对$T$做单位正交分解,然后把对角线上的特征值全部取根号就可以了.唯一性的话,由于$S$在每一个特征子空间上都应当表现为$\sqrt{ \lambda_i } \text{ id }$(原因是每一个特征子空间本身都是不变子空间,因此$S$的特征子空间也需要是$S^2 = T$的特征子空间,于是反之亦然),那$S$当然是唯一确定的.

###### Example1
设$A , B , A - B$都是正定矩阵,求证$\sqrt{ A } - \sqrt{ B }$正定.

反证,假设$\sqrt{ A } - \sqrt{ B }$并非正定,那就一定存在一个特征值$\lambda \leq 0$以及配套的特征向量$\vec{ v } \ne 0$,使得$( \sqrt{ A } - \sqrt{ B } ) \vec{ v } = \lambda \vec{ v }$,也就是$\sqrt{ B } \vec{ v } = \sqrt{ A } \vec{ v } - \lambda \vec{ v }$.此时见到:

$$
\begin{aligned}
& ( \vec{ v } )^t B \vec{ v } \\
= & ( \sqrt{ B } \vec{ v } )^t ( \sqrt{ B } \vec{ v } ) \\
= & ( \sqrt{ A } \vec{ v } - \lambda \vec{ v } )^t ( \sqrt{ A } \vec{ v } - \lambda \vec{ v } ) \\
= & ( \vec{ v } )^t A \vec{ v } + \lambda^2 \vec{ v }^t \vec{ v } - 2 \lambda ( \vec{ v } )^t \sqrt{ A } \vec{ v } \\
\geq & ( \vec{ v } )^t A \vec{ v }
\end{aligned}
$$

这就与$A - B$正定是矛盾的了.

###### Example2
设$A , B , A - B$都是正定矩阵,求证$A^{ - 1 } - B^{ - 1 }$正定.

直接扩到复数域,这样一定存在一个$C$,使得$( C^* ) BC = I$,此时由于合同是内积空间上的同构,所以$( C^* ) ( A - B ) C = C^* AC - I$仍然正定,这就将情况化约到$B = I$的情况.

此时只需对$A$做对角化即可见得了.

##### 极分解
对于内积空间,设$T \in \text{ End } ( V )$可逆,那么就存在唯一一对$R , U \in \text{ End } ( V )$使得$R$正定并且$U$是正交变换,$T = RU$.这其实类似于把一个复数拆成模长和辅角两个部分.

在此之前先证明一个引理:标准内积空间上$T^* T$是半正定的,并且如果$T$单,那么其是正定的.

首先其自伴性质是已知的,半正定的原因是$^t ( \vec{ v } ) ( T^* T ) ( \vec{ v } ) = ( T \vec{ v } \mid T \vec{ v } )$,而后者继承了内积空间上的半正定性.并且从此可以看出$T$单的话,也就是$\ker T ={ 0 }$就可以继承内积空间上的正定性.

注意到$TT^* = RUU^* R = R^2$,因此必定有$R = \sqrt{ TT^* }$,因此$R$是唯一且存在的且正定的(原因是$T$是可逆的,因此$TT^*$是正定的)而且$R$还自伴.另外,因为$R$可逆(正定性推出非退化性),因此$U = R^{ - 1 } T$就确定.问题在于证明$U$是否是正交变换,只需要证明$U^* = U^{ - 1 }$,而:

$$
\begin{aligned}
U^* U & = ( R^{ - 1 } T )^* ( R^{ - 1 } T ) \\
& = T^* R^{ - 2 } T = \text{ id }
\end{aligned}
$$

这就得证.

##### 最小二乘法
给定一个特定的$T \in \text{ Hom } ( V , W )$,现在对于一个$\vec{ w } \in W$,想要求一个$\vec{ v } \in V$使得$\Vert T \vec{ v } - \vec{ w } \Vert$最小.这个解被称为**最小二乘解**.

取$W_0 = \text{ im } T$,那么就可以将$\vec{ w } = \vec{ w }_0 + \vec{ w }_1$,其中$\vec{ w }_0 \in W_0 , \vec{ w }_1 \in ( W_0 )^\bot$.此时就可以见到:

$$
\begin{aligned}
\Vert T \vec{ v } - \vec{ w } \Vert^2 & = \Vert ( T \vec{ v } - \vec{ w }_0 ) - \vec{ w }_1 \Vert^2 \\
& = \Vert T \vec{ v } - \vec{ w }_0 \Vert^2 + \Vert \vec{ w }_1 \Vert^2
\end{aligned}
$$

只需让前面为$0$就行,从这也可以看出来一般而言$\vec{ v }$不是唯一的.然而$\vec{ v } + \ker T$总是唯一的,我们试图在其中找到$\Vert \vec{ v } \Vert$最小的一个作为代表.而$\vec{ v }$仍可以分解为$\ker T$和$( \ker T )^\bot$两部分,这就能见到$\Vert \vec{ v } \Vert$最小时$\vec{ v } \in ( \ker T )^\bot$.不妨将这个$\vec{ v }$定义为$S ( \vec{ w } ) = \vec{ v }$,应该见到$S$是一个线性映射.

我们还可以证明,其最小二乘解正好是$T^* T \vec{ v } = T^* \vec{ w }$的解.原因是最小二乘解其实也就是$T \vec{ v } - \vec{ w } \in ( \text{ im } T )^\bot = \ker ( T^* )$.

而注意到$T^* T$是自伴的,然而其还有更多的好性质:

1. $\text{ im } \ ( T^* T ) = \text{ im } \ ( T^* )$.

2. $\ker ( T^* T ) = \ker T$.

3. $\text{ rk } \ ( T^* T ) = \text{ rk } \ ( T ) = \text{ rk } ( T^* )$.

考虑(1), 首先显然有$\text{ im } \ ( T^* T ) \subseteq \text{ im } \ ( T^* )$.而反方向的话,对于$T^* \vec{ w } \in \text{ im } ( T^* )$,只需取其最小二乘解$\vec{ v } \in V$就给出了$T^* \vec{ w } = T^* T \vec{ v } \in \text{ im } ( T^* T )$.

考虑(2),首先显然有$\ker \ ( T^* T ) \subseteq \ker \ ( T )$.而反方向的话,若$T^* T \vec{ v } = 0$,应该能看到$( T \vec{ v } \mid T \vec{ v } ) = ( T^* T \vec{ v } \mid \vec{ v } ) = 0$,这意味着$T \vec{ v } = 0$.

(3)是(1)(2)的推论.顺便一提,这里推出的$\text{ rk } ( T ) = \text{ rk } ( T^* )$是行秩等于列秩的另一个证明.

#### 奇异值分解
取$V , W$为有限维实内积空间并采取标准内积,不妨设$m = \dim V , n = \dim W$,注意这里的字母使用与习惯略有差别.并取$T : V \to W$为线性映射.接下来我们证明,存在$V , W$的两组单位正交基,不妨记作$\mathcal{ B }_V = \{ \vec{ v }_1 , \cdots , \vec{ v }_m \}$和$\mathcal{ B }_W = \{ \vec{ w }_1 , \cdots , \vec{ w }_n \}$.记$p = \text{ rk } T$,以及存在一列非负实数$\sigma_1 \geq \cdots \geq \sigma_p$,使得$T \vec{ v }_i = \begin{cases}\sigma_i \vec{ w }_i & 1 \leq i \leq p \\ 0 & i > p\end{cases}$.我们称这列非负实数为$T$的**奇异值**,并将在下面证明其由$T$唯一确定.应当说明的是,下述中虽然有的时候会默认后面有一列$0$来避开讨论,但$\sigma_p \ne 0$.一般,我们也用非零奇异值的数量来判断$T$的秩.可以认为,奇异值是特征值的某种推广.

考虑矩阵,此时的$T$应当是一个$n \times m$的矩阵.考虑$V$的单位正交基组成的矩阵$P \in M_{ m \times m }$,以及$W$的单位正交基矩阵$Q \in M_{ n \times n }$.

此时不妨考虑$T$在标准基下表示为$A$,那考虑$Q^{ - 1 } AP$实际上就是在两个单位正交基内转化的过程.不妨令$\Sigma = Q^{ - 1 } AP$,只需证明其在前$p$个主对角线位置分别为$\sigma_1 , \cdots , \sigma_p$即可.

留意到$( \vec{ v }_i \mid T^* \vec{ w }_j ) = ( T \vec{ v }_i \mid \vec{ w }_j ) = \sigma_i ( \vec{ w }_i \mid \vec{ w }_j ) = \sigma_i \delta_{ i , j }$,其中$\delta_{ i , j } = [ i = j ]$.回忆到$T^*$是唯一的,并注意到如果干脆定义$T^*$满足$T^* \vec{ w }_j = \sigma_j \vec{ v }_j$上式依旧成立,因此$T^*$的确有此性质.进一步得到推论$T^* T \vec{ v }_i = \sigma_i^2 \vec{ v }_i$.所以$T^* T$的特征值恰为$\sigma_1^2 \geq \cdots$.这样的话其唯一性立刻见到了.不过由于是在$\mathbb{ R }$上,因此我们还要说明其存在性.考虑对$T^* T$做正交对角化.此时回忆到$\text{ rk } ( T^* T ) = \text{ rk } ( T ) = p$,又因为$T^* T$可对角化,$\text{ rk }$对应了非零特征值的数量.直接取$\sigma_i = \sqrt{ \lambda_i }$,其中$\lambda_i$是$T^* T$的特征值,当然都是可行的.

接下来要反推出$V , W$的两组单位正交基.任取一组$V$的特征向量组成的基并从其构造$W$的基,也就是取$\vec{ w }_i = \frac{ T \vec{ v }_i }{ \sigma_i }$(假设$m \geq n$的情况下,不然的话反之)并证明这也是一组单位正交基.

此时观察到$( \vec{ w }_i \mid \vec{ w }_j ) = \frac{ 1 }{ \sigma_i \sigma_j } ( T \vec{ v }_i \mid T \vec{ v }_j ) = \frac{ 1 }{ \sigma_i \sigma_j } ( \vec{ v }_i \mid T^* T \vec{ v }_j ) = \frac{ \sigma_j }{ \sigma_i } \delta_{ i , j }$,所以这当然也是一组单位正交基.

#### Moore-Penrose 广义逆
取域$F$上的有限维向量空间$V , W$以及线性映射$T : V \to W$.其中$T$未必可逆,但有的时候我们又需要$T$的逆的性质,我们的目标是去找到一个弱一点的替代品.

我们声明一定存在一个$S : W \to V$,满足以下性质:

1. $TST = T$.

2. $STS = S$.

3. $TS = ( TS )^*$.

4. $ST = ( ST )^*$.

容易见到,如果$T$可逆,它的逆当然是一个MP广义逆.事实上我们可以证明满足上述条件的MP广义逆是唯一的.

先来证明其存在性,$\forall \vec{ v } \in V$,做分解$\vec{ v } = \vec{ v } ' + \vec{ v } ' '$,其中$\vec{ v } ' \in \ker T$且$\vec{ v } ' ' \in ( \ker T )^\bot$.同样$\forall \vec{ w } \in W$,做分解$\vec{ w } = \vec{ w } ' + \vec{ w } ' '$,其中$\vec{ w } ' \in \text{ im } T$.

接下来应当见到,任取$\vec{ v }$使得$T \vec{ v } = \vec{ w }$,则$T^{ - 1 } ( \vec{ w } ' ) = \vec{ v } + \ker T$,而$\vec{ v } + \ker T$中的每个元素做投影后得到的$\vec{ v } ' '$都是相同的,于是我们定义$S \vec{ w } = \vec{ v } ' '$.容易验证$S$是线性映射,而且应当见到$ST$和$TS$其实都是正交投影,具体而言,$ST : V \to ( \ker T )^\bot$而$TS : W \to \text{ im } T$,也容易验证上述四条性质.也就是我们想法是,干脆考虑映射$V / \ker T \to \text{ im } \ T$必然是可逆的,直接在这个上面找逆而不顾其它.

接下来证明其唯一性,假设$T$有两个MP广义逆$S , R$,我们注意到:

$$
\begin{aligned}
TS & = ( TS )^* = S^* T^* = S^* ( TRT )^* \\
& = S^* T^* R^* T^* \\
& = ( TS )^* ( TR )^* \\
& = TSTR \\
& = TR
\end{aligned}
$$

同理可证明$ST = RT$,因此$S = STS = STR = RTR = R$,这就给出了唯一性的证明.

那么如何求出一个MP广义逆呢?考虑对于一个线性映射$T : V \to W$,将其视为标准基下的矩阵,做奇异值分解有$T = Q \Sigma P^{ - 1 }$.

容易见到满足条件的MP逆$S$应当满足$S \vec{ w }_j = \frac{ 1 }{ \sigma_j } \vec{ v }_j$,而写作矩阵形式的话就是$P \Pi Q^{ - 1 }$,其中$\Pi$就是$\Sigma$的非零对角线全部取倒数.验证此事实的策略,要么取检验MP广义逆的定义,要么取一组基并观察投影,但总之都是容易的.

如果想要进一步说明MP广义逆的合理性,不妨考虑设$C ( t ) = T^* T + t \cdot \mathrm{ { id } }_V$,我们证明MP广义逆实际上就是$S = \lim_{ t \to 0 , \det C ( t ) \ne 0 } C ( t )^{ - 1 } T^*$,证明无非也只是使用奇异值分解,考虑$T = Q \Sigma P^{ - 1 } , T^* = P \Sigma Q^{ - 1 }$,那么$C ( t ) = P ( \Sigma^2 + tI ) P^{ - 1 }$,容易检验其满足性质.

#### 极大化极小原理(Courant-Fischer定理)
对于实空间$V$,考察其标准内积$( \_ \mid \_ )$和任一对称双线性形式$B : V \times V \to \mathbb{ R }$.当然存在唯一的$S \in \text{ End } ( V )$使得$B ( \vec{ v }_1 , \vec{ v }_2 ) = ( \vec{ v }_1 \mid S \vec{ v }_2 )$恒成立,无非是把$B$所代表的矩阵拿过来而已.

此时应有$S = S^*$,对其施加正交对角化拿到单位正交基$\vec{ v }_1 , \cdots , \vec{ v }_n$和对应的一列特征值$\lambda_1 \geq \cdots \geq \lambda_n$.对于单位球面上的向量$\vec{ v }$,其应当满足$| \vec{ v } | = 1$,此时我们注意到$\lambda_1 = \max_{ | \vec{ v } | = 1 } B ( \vec{ v } , \vec{ v } ) , \lambda_n = \min_{ | \vec{ v } | = 1 } B ( \vec{ v } , \vec{ v } )$.原因只是取$\vec{ v } = \sum a_k \vec{ v }_k$,然后$B ( \vec{ v } , \vec{ v } ) = \sum_k a_k^2 \lambda_k$.上述当然成立.确定除此以外的其他特征值需要更精确地刻画,我们引入如下定理:

$$
\begin{aligned}
\lambda_k & = \min_{ U \subseteq V , \dim U = n - k + 1 } \left ( \max_{ \vec{ v } \in U , | \vec{ v } | = 1 } B ( \vec{ v } , \vec{ v } ) \right ) \\
\lambda_k & = \max_{ U \subseteq V , \dim U = k } \left ( \min_{ \vec{ v } \in U , | \vec{ v } | = 1 } B ( \vec{ v } , \vec{ v } ) \right ) \\

\end{aligned}
$$

将$S$用$- S$替换,则降序的特征值序列要翻转,立刻见到上述两条等价.下面只证明第一条.

取$W_k = \langle \vec{ v }_1 , \cdots , \vec{ v }_k \rangle$.应当见到:

$$
\begin{aligned}
\dim U \cap W_k & = \dim U + k - \dim ( U + W_k ) \\
& \geq \dim U + k - n = 1
\end{aligned}
$$

这意味着这两个空间的交非零空间,取出一个交集元素$\vec{ v } = \sum_{ i = 1 }^k a_i \vec{ v }_i \in U$且满足$| \vec{ v } | = 1$.应当见到$B ( \vec{ v } , \vec{ v } ) = \sum_k \lambda_k a_k^2 \geq \lambda_k$.这就意味着$\max_{ \vec{ v } \in U , | \vec{ v } | = 1 } B ( \vec{ v } , \vec{ v } ) \geq \lambda_k$.或言之$\inf \max_{ \vec{ v } \in U , | \vec{ v } | = 1 } B ( \vec{ v } , \vec{ v } ) \geq \lambda_k$.证明下界可取到只需取$U = \langle \vec{ v }_k , \cdots , \vec{ v }_n \rangle$即可取到(证明取到的策略呢,可以直接看基立刻得到,也可以采取更加严谨好说的方式即前后分别表示一下发现其既满足$\geq \lambda_k$又要$\leq \lambda_k$).

这个定理也可以用来求奇异值,半正定条件下,奇异值无非是特征值开根后的结果.

这个原理的重要意义是将奇异值,特征值这些东西全部挪到了空间本身的性质上(类似惯性定理).于是从此只要我们能拿到空间的同构当然就能断言奇异值,特征值全都同构,这是无可置疑的.

应当能看出这个定理更多有一种拓扑性质,事实上的确如此,如果能取最大值的话,这实际上对应了某种李氏连续性质.

#### Perron-Frobenius定理
约定$A \geq B$表示对于每一个位置$a_{ i , j } \geq b_{ i , j }$.于是约定$A \geq 0$意味着$A$中的任意元素都$\geq 0$.

容易见到以下平凡引理:

1. $A > 0 , x \geq 0 , x \ne 0 \Rightarrow Ax > 0$.

2. $A \geq 0 , x \geq 0 \Rightarrow Ax \geq 0$.

定义实矩阵$A$的**谱半径**为$\rho ( A ) = \max{ | \lambda | }$,也就是所有复特征值模长的极大值.

虽然这里用到了复特征值,但意义仅是使得多项式可裂,而矩阵仍然是实数域的.因此仍然认为该定理是实内积空间里的定理.

##### Collatz-Wielandt公式
对于实矩阵$A > 0$,我们考虑$S = \{ \vec{ x } \in \mathbb{ R }^n \mid | \vec{ x } | = 1 , \vec{ x } \geq 0 \}$,容易见到这是个紧集,考虑定义在其上的映射$L : S \to \mathbb{ R_{ + } } , \vec{ x } \mapsto \min \left ( \cfrac{ ( A \vec{ x } )_i }{ x_i } , x_i \ne 0 \right )$.于是命$\rho \in \mathbb{ R_+ }$为上述映射像中的极大值,我们下面证明两个事情:首先是这个$\rho$是$A$的一个特征值,其次是这个$\rho$就是$A$的谱半径.

首先证明其是一个特征值,而且对应的特征向量大于零,也就是$\exists \vec{ v } > 0 , A \vec{ v } = \rho \vec{ v }$.容易由定义见到$\exists \vec{ v } , A \vec{ v } \geq \rho \vec{ v }$.假若$A \vec{ v } \ne \rho \vec{ v }$,则上述的平凡引理使得$A ( A \vec{ v } - \rho \vec{ v } ) > 0$.这里我们就应当会思考一个事情是,能否进行微调来使得得到更大的$\rho$从而导出矛盾.因此这里的思路应当是,取一个$\epsilon$然后找到一个向量$\vec{ w }$,使得$A \vec{ w } > ( \rho + \epsilon ) \vec{ w }$从而导出矛盾.

回到我们拿到的条件,应该存在$\epsilon > 0$使得$A ( A \vec{ v } - \rho \vec{ v } ) > \epsilon A \vec{ v }$,留意到由于平凡引理,应当$A \vec{ v } > 0$,取适当的$t$来归一化,定义出$\vec{ w } = tA \vec{ v } \in S$,从而上式导出:

$$
\begin{aligned}
A ( A \vec{ v } ) & > ( \rho + \epsilon ) A \vec{ v } \\
A \vec{ w } & > ( \rho + \epsilon ) \vec{ w }
\end{aligned}
$$

立即导出矛盾.因此必有$A \vec{ v } = \rho \vec{ v }$.此外,引理告诉我们$A \vec{ v } > 0$恒成立,则$\vec{ v } = \rho^{ - 1 } A \vec{ v } > 0$亦然成立.

接下来证明这个$\rho$就是谱半径$\rho ( A )$.由定义见到$\rho$作为特征值应当满足$\rho \leq \rho ( A )$,接下来只需证明$\rho \geq \rho ( A )$即可.

对于所有的特征值$\mu$和相应的特征向量$\vec{ w } \ne 0$满足$A \vec{ w } = \mu \vec{ w }$,使用三角不等式,$\forall 1 \leq i \leq n$应当有:

$$
\begin{aligned}
| \mu | | w_i | & = | ( A \vec{ w } )_i | \\
& = | \sum_j a_{ i , j } w_j | \leq \sum_j a_{ i , j } | w_j |
\end{aligned}
$$

将$\vec{ w } ' = ( | w_1 | , \cdots , | w_n | ) \in \mathbb{ R }^n$,也就是将复向量强行转到实向量上,上式立刻给出$A \vec{ w } ' \geq | \mu | \vec{ w } '$.伸缩该向量使得$| \vec{ w } ' | = 1$,于是上式给出$\rho \geq L ( \vec{ w } ' ) \geq | \mu |$,这就给出了$\rho ( A ) \leq \rho$.于是证毕.

##### Perron定理
对于实矩阵$A > 0$,我们接下来声明如下定理:

1. $\rho ( A ) > 0$,$\exists \vec{ v } \in \mathbb{ R }^n , \vec{ v } > 0 , A \vec{ v } = \rho ( A ) \vec{ v }$.也即:谱半径的确是一个特征值.

2. 如果$\lambda \ne \rho ( A )$,则$| \lambda | < \rho ( A )$.也就是:谱半径只此实特征值能达到.

3. 谱半径的代数重数和几何重数均为$1$.

(1)也就是Collatz-Wielandt公式.

(2)的话考虑对Collatz-Wielandt公式作一些补充.假使复特征值$\mu$满足$| \mu | = \rho ( A )$,则不等式链$\rho ( A ) = \max L ( \vec{ v } ) \geq L ( \vec{ w } ' ) \geq | \mu | = \rho ( A )$必然全部三角不等式取等,中间的每一项$w_j$都必然落在复平面的同一条直线上,那就可以除去一个复数得到实向量$\vec{ w } ' '$亦然满足$A \vec{ w } ' ' = \mu \vec{ w } ' '$,由于该式子中除了$\mu$均为实数,因此$\mu$也必然是实数.而且$\mu$当然不可能是负数,这样就完成了(2)的证明.

(3)的话先考虑证明几何重数$\dim V_{ \rho ( A ) } = 1$,考虑$\vec{ v } , \vec{ v } '$都是$\rho ( A )$的特征向量,其中$\vec{ v }$由于上述讨论而满足$\vec{ v } > 0$,而$\vec{ v } '$至少有一个分量为正数.既然如此,应当可以取足够小的$\epsilon > 0$使得$\vec{ v } - \epsilon \vec{ v } ' \geq 0$,以至于可以取其中尽可能大的$\epsilon$使得$\vec{ v } - \epsilon \vec{ v } '$的某一个分量恰好为$0$.我们接下来声明此时$\vec{ v } - \epsilon \vec{ v } ' = 0$以说明$\vec{ v } '$和$\vec{ v }$线性相关

为说明此进行反证,假设$\vec{ v } - \epsilon \vec{ v } ' \ne 0$,用平凡引理,考虑$\vec{ v } - \epsilon \vec{ v } ' = \frac{ 1 }{ \rho ( A ) } A ( \vec{ v } - \epsilon \vec{ v } ' ) > 0$,然而我们已经说明其有一个分量为$0$,导出矛盾.

接下来考虑(3)的完全版本,利用$\dim V_{ \rho ( A ) } = 1$,如果我们能将整个空间拆成两部分不变子空间,其中一部分是$\langle \vec{ v } \rangle$,那就可以完成上面的部分,原因是此时另一部分不变子空间中不能有$\rho ( A )$作为根,否则与其维数为$1$矛盾.另一方面,原本的特征多项式就是两个不变子空间的特征多项式的乘积,这就证明了单根的性质.

于此,考虑以下操作:考虑$\text{ char } ( A ) = \text{ char } ( A^T )$,于是$\rho ( A ) = \rho ( A^T )$,这就意味着$\exists \vec{ u } > 0$使得$( A^T ) \vec{ u } = \rho ( A ) \vec{ u }$.取出它的正交补空间$\langle \vec{ u } \rangle^\bot$,它应当是$A$的不变子空间,原因是:

$$
\begin{aligned}
( \vec{ u } )^T ( A \vec{ x } ) & = ( A^T \vec{ u } )^T \vec{ x } \\
& = \rho ( A ) ( \vec{ u } )^T \vec{ x } = 0
\end{aligned}
$$

同时注意到$\vec{ v } > 0 , \vec{ u } > 0$,所以$\vec{ v } \notin \langle \vec{ u } \rangle^\bot$,这就完成了直和分解,于是上述命题证毕.

#### 实正交变换的标准型
(ps:虽然这里放在实内积空间里,然而大部分性质实际上是转移自复内积空间的结构,应当先看下面的章节,回头再来观察此节.)

回忆到取定$V$是实线性空间,并取上面的某种内积形式,对于$T \in \text{ End } ( V )$,若其满足$T^* = T^{ - 1 }$,则它是实正交变换.特别地,当取标准内积的时候,所对应的就是正交矩阵,则$A^t = A^{ - 1 }$.

回忆到此时$\det A = \pm 1$.

将正规算子的概念从复数域里面拿到实数域,即满足$( A^t ) A = A ( A^t )$.我们在下面做复数酉变换的时候证明过引理:$\exists k \geq 0 , T^k = 0 \Rightarrow T = 0$.容易见到正交算子一定是正规的.

接下来来观察不同维度的正交变换,假设$\dim_{ \mathbb{ R } } V = n$.

当$n = 1$的时候,正交变换显然是长度为$\pm 1$的伸缩变换,也就是$\pm \text{ id }_V$.

当$n = 2$的时候,不妨设矩阵为$\begin{bmatrix}\alpha & \beta \\ \gamma & \delta\end{bmatrix}$,用正交性质就知道:

$$
\begin{cases}
\alpha^2 + \gamma^2 = 1 \\
\beta^2 + \delta^2 = 1 \\
\alpha^2 + \beta^2 = 1 \\
\gamma^2 + \delta^2 = 1
\end{cases}
$$

做三角换元后考虑到行列式为$\pm 1$,在一些简单的确定后,立刻见到原矩阵要么是:$\begin{bmatrix}\cos \theta & - \sin \theta \\ \sin \theta & \cos \theta\end{bmatrix}$,要么是$\begin{bmatrix}\cos \theta & \sin \theta \\ \sin \theta & - \cos \theta\end{bmatrix}$,它们的行列式分别为$\pm 1$.

拿出行列式恰好为$+ 1$的前者:

$$
R ( \theta ) = \begin{bmatrix}
\cos \theta & - \sin \theta \\
\sin \theta & \cos \theta
\end{bmatrix}
$$

也就是通常所说的旋转矩阵.容易见到:

1. $R ( \theta ) R ( \psi ) = R ( \theta + \psi ) = R ( \psi ) R ( \theta )$.

2. 如果$P$是一个正交算子,则$P^{ - 1 } R ( \theta ) P = R ( ( \det P ) \theta )$.

(1)只需对$\vec{ e }$验证即可.

(2)的话,当$\det P = 1$的时候,当然有$P = R ( \psi ) , P^{ - 1 } = R ( - \psi )$,那显然成立了;当$\det P = - 1$的时候,只需在外面补一个$\begin{bmatrix}1 & 0 \\ 0 & - 1\end{bmatrix}$即可转化.

于此之前,考虑如果$T$是正交变换,它当然一定是正规的,那么它在复数上可正交对角化,特征值当然满足$| \lambda | = 1$,那么当然$T + T^{ - 1 }$自伴.而且它俩可以同步对角化,那$T + T^{ - 1 }$的特征值当然是$| \mu | = | \lambda + \lambda^{ - 1 } | \leq | \lambda | + | \lambda^{ - 1 } | = 2$.

接下来考虑一般的情况,我们声明,对于任意正交变换,都可以取基转化为下述形式:

$$
\begin{bmatrix}
I_{ a \times a } & & & \\
& - I_{ b \times b } & & & \\
& & R ( \theta_1 ) & & \\
& & & \ddots & \\
& & & & R ( \theta_k )
\end{bmatrix}
$$

其中$\theta_1 , \cdots , \theta_k$都并非$\pi$的整数倍.

证明考虑令$S = T^{ - 1 } + T$,容易见到其自伴,那原空间就可以拆成$S$的若干特征子空间的直和,容易发现$ST = TS$,这必然意味着$V_\lambda$是$T$不变的,于是下面可以只着眼于一个特征子空间,观测$V = V_\lambda$的情形,且$\lambda \in \mathbb{ R } , | \lambda | \leq 2$.

则此时观察此空间,应当有$T + T^{ - 1 } = \lambda I_V$.两边乘以$T$,得到$T^2 - \lambda T + I = 0$.

下面我们开始讨论,当$\lambda = \pm 2$的时候,则上述配方得到$( T \mp I )^2 = 0$,由于$T \mp I$是正规算子,于是$T \mp I = 0$,这就对应了上述矩阵中的$\pm 1$的分块.

那如果$\lambda \ne \pm 2$呢,那就必然有$| \lambda | < 2$,则$x^2 - \lambda x + 1$无实根,不可约,但带入$x = T$会得到$0$,因此它必然是$T$在此空间下的极小多项式.又因为$T$如果有实特征值,必然是$\pm 1$,则此时$\lambda = \pm 2$.因此,$T$没有实特征值.既然如此,任取一个向量$\vec{ v } \ne 0$,$\vec{ v }$与$T \vec{ v }$必然线性无关.

既然如此,使他们张成子空间$W = \langle \vec{ v } , T \vec{ v } \rangle$并作直和分解$V = W \oplus W^\bot$.由于$T^2 = \lambda T - I$,因此注意到$W$应当是$T$不变的.那么,$W^\bot$在$T^* = T^{ - 1 }$作用下当然也是不变的.然而,$T = \lambda I - T^{ - 1 }$,所以$T$是关于$T^{ - 1 }$的多项式,于是$W^\bot$在$T$下也是不变的.这样,我们完全把空间归纳下去了,每次都可以扔出去一个$\dim = 2$的空间.于是最后一步是检验$T$在归纳下去的空间上的性质.对于$\dim = 2$的空间,$x^2 - \lambda x + 1$当然仍然不可约,而它必然就是特征多项式,那$\det = 1$就自然成立.

最后应当简单解释上述形式是否由$T$唯一确定.然而应属显然了,因为每一次拆出的特征多项式当然都是原本特征多项式的一个不可约因子.

##### 欧拉角
考虑三维空间中的旋转,我们通过上述对实正交矩阵的分类得知,三维空间中的旋转立刻统合为$\begin{bmatrix}1 & \\ & R ( \theta )\end{bmatrix}$.

那么就需要两个参数来描述此,旋转轴$\vec{ u } = \vec{ v }_1$的坐标用以描述转轴以及一个参数$\theta$用来描述转角.不妨将这种旋转记作$R_{ u } ( \theta )$.然而于此之外,先要论证此旋转和另外两个单位正交基$\vec{ v }_2 , \vec{ v }_3$无关.然而容易见到不管如何选取这两个基,它们都会同样张成$\langle \vec{ u } \rangle^\bot$,因此它们都在同一个空间里,而且只差一个旋转.这个旋转前后当然可以抵消,这也是直觉所告知我们的.

由此就可以看到三维空间的及其好的性质,我们一般称呼二维空间中的旋转,它的转轴实际上是垂直于此平面的一个更高维的轴.而三维空间中的转轴一定落在三维空间中,不需要再到高维空间中找到转轴了.

综上,旋转总是将一个有序单位正交基转化为另一个有序单位正交基,三维空间中表现为$( \vec{ e }_1 , \vec{ e }_2 , \vec{ e }_3 ) \to ( \vec{ u }_1 , \vec{ u }_2 , \vec{ u }_3 )$.

对于指定基的旋转,我们应当通过一定的交换基的操作见到:

$$
\begin{aligned}
R_{ e_1 } ( \theta ) & = \begin{bmatrix}
1 & & \\
& \cos \theta & - \sin \theta \\
& \sin \theta & \cos \theta
\end{bmatrix} \\
R_{ e_2 } ( \theta ) & = \begin{bmatrix}
\cos \theta & & \sin \theta \\
& 1 & \\
- \sin \theta & & \cos \theta
\end{bmatrix} \\
R_{ e_3 } ( \theta ) & = \begin{bmatrix}
\cos \theta & - \sin \theta & \\
\sin \theta & \cos \theta & \\
& & 1
\end{bmatrix} \\

\end{aligned}
$$

为了表示使得任意基下的旋转,我们考虑先转好一个轴,然后再以此轴作旋转.那我们考虑构造一个$\vec{ f }_2 = \begin{cases}\vec{ e }_2 & \vec{ e }_3 / / \vec{ u }_3 \\ \vec{ e }_3 \times \vec{ u }_3 & otherwisse\end{cases}$.

换言之,此时我们选了一个轴$\vec{ f }_2$,它与$\vec{ u }_3 , \vec{ e }_3$均正交,那我们就可以这么转:

1. 将$( \vec{ e }_1 , \vec{ e }_2 , \vec{ e }_3 )$绕$\vec{ e }_3$转到$( \vec{ f }_1 , \vec{ f }_2 , \vec{ e }_3 )$.

2. 将$( \vec{ f }_1 , \vec{ f }_2 , \vec{ e }_3 )$绕$\vec{ f }_2$转到$( \vec{ g }_1 , \vec{ f }_2 , \vec{ u }_3 )$.

3. 将$( \vec{ g }_1 , \vec{ f }_2 , \vec{ u }_3 )$绕$\vec{ u }_3$转到$( \vec{ u }_1 , \vec{ u }_2 , \vec{ u }_3 )$.

综上,一个旋转矩阵总可以写成三个矩阵$R_{ u_2 } ( \varphi ) R_{ f_2 } ( \theta ) R_{ e_3 } ( \psi )$的乘积,我们称$T$是由**欧拉角**$( \varphi , \theta , \psi )$所确定的.

不过,上述的转轴并非我们选定的有序正交基$( \vec{ e }_1 , \vec{ e }_2 , \vec{ e }_3 )$,换言之,转轴不是以观察者视角的,而是以被旋转物体本身的视角而言的.可既然要转为矩阵表述,我们应当搞一个观察者版本的矩阵.这就要求转轴被控制为$( \vec{ e }_1 , \vec{ e }_2 , \vec{ e }_3 )$.我们下面尝试证明$R_{ u_2 } ( \varphi ) R_{ f_2 } ( \theta ) R_{ e_3 } ( \psi ) = R_{ e_3 } ( \psi ) R_{ e_2 } ( \theta ) R_{ e_3 } ( \varphi )$.

对于正交变换$P$,并设$\epsilon = \det P = \pm 1$,应当有$R_{ Pu } ( \epsilon \theta ) = PR_u ( \theta ) P^{ - 1 }$.其实到这里只有这个$\epsilon$的用法并非显然,但总之,用代数的语言可以看出$P$将$( \vec{ u } , \vec{ v } , \vec{ w } ) \mapsto ( P \vec{ u } , P \vec{ v } , P \vec{ w } )$,这当然仍是一组有序正交基.但为了使其的方向为正,可以改为$( P \vec{ u } , P \vec{ v } , \epsilon P \vec{ w } )$以避免讨论,然后去检验$PR_u ( \theta )$和$R_{ Pu } ( \epsilon \theta ) P$对基$( \vec{ u } , \vec{ v } , \vec{ w } )$的作用.总之容易验证.

既然如此,考虑$\vec{ f }_2 = R_{ e_3 } ( \psi ) \vec{ e }_2$,既然如此,$R_{ f_2 } ( \theta ) = R_{ e_3 } ( \psi ) R_{ e_2 } ( \theta ) R_{ e_3 } ( \psi )^{ - 1 }$.

同理,$\vec{ u }_3 = R_{ f_2 } ( \theta ) \vec{ e }_3 = R_{ f_2 } ( \theta ) R_{ e_3 } ( \psi ) \vec{ e }_3$,既然如此,得到$R_{ u_3 } ( \varphi ) = R_{ e_3 } ( \psi ) R_{ e_2 } ( \theta ) R_{ e_3 } ( \varphi ) R_{ e_2 } ( \theta )^{ - 1 } R_{ e_3 } ( \psi )^{ - 1 }$.这就证明了上述结论.

更具体地说,它表为:

$$
\begin{bmatrix}
1 & & \\
& \cos \psi & - \sin \psi \\
& \sin \psi & \cos \psi
\end{bmatrix} \begin{bmatrix}
\cos \theta & & \sin \theta \\
& 1 & \\
- \sin \theta & & \cos \theta
\end{bmatrix} \begin{bmatrix}
1 & & \\
& \cos \varphi & - \sin \varphi \\
& \sin \varphi & \cos \varphi
\end{bmatrix}
$$

##### 四元数
试图搞一个以$\mathbb{ R } \subsetneq \mathbb{ C } \subsetneq \mathbb{ H }$的东西.其中$\mathbb{ H }$是一个除环,也就是在域的基础上丢掉了交换律.

具体如何构造呢?考虑在实线性空间的基础上引入乘法,那就先要构造它的一组基$\{ 1 , i , j , k \}$.现在就只需要定义乘法,当然是$\mathbb{ H } \times \mathbb{ H } \to \mathbb{ H } , ( x , y ) \mapsto x \cdot y$,并且要求它对于$\mathbb{ R }$是双线性的.同时要求以下规则:

1. $1 \cdot x = x$.

2. $i^2 = j^2 = k^2 = - 1$.

3. $ij = k = - ji$.

4. $jk = i = - kj$.

5. $ki = j = - ik$.

那么双线性性质和上述约定当然搞定了一个乘法映射.为说明其是环,还应当验证以下性质:

1. 分配律.由双线性形式显然.

2. 结合律.只需验证上述四个元素$1 , i , j , k$的结合律即可.

这就搞定了环的性质.可以看到$\mathbb{ H }$可以看作$\mathbb{ R }$或者$\mathbb{ C }$上的向量空间,对于前者是因为$q = a + bi + cj + dk$,对于后者是因为任何一个$q$都可以写作$q = z + jw$,其中$z , w \in \mathbb{ C }$的形式,这里顺便一提此表述下$jw = \bar{ w } j$.这当然顺便也搞定了除环的性质.

回忆到环的中心$Z ( \mathbb{ H } ) = \{ z \in \mathbb{ H } \mid \forall q \in \mathbb{ H } , zq = qz \}$.我们声称$Z ( \mathbb{ H } ) = \mathbb{ R }$.由于$\mathbb{ H }$的特殊性,我们知道只需要对$\{ 1 , i , j , k \}$检查交换性即可,会发现只有$\mathbb{ R }$是合理的.

定义一个四元数$q = a + bi + cj + dk$的**共轭**$\bar{ q } = a - bi - cj - dk$,定义其**迹**$\mathrm{ { Tr } } ( q ) = q + \bar{ q } = 2 a$,再定义其**范数**$N ( q ) = q \bar{ q }$.有以下性质:

1. 共轭对$\mathbb{ R }$是线性映射.

2. $\overline{ ( \bar{ q } ) } = q$.

3. $\overline{ q_1 + q_2 } = \bar{ q_1 } + \bar{ q_2 }$.

4. $\overline{ q_1 q_2 } = \bar{ q_2 } \bar{ q_1 }$.

5. 迹对$\mathbb{ R }$是线性映射.

6. $\bar{ N ( q ) } = N ( q ) = N ( \bar{ q } )$.

7. $N ( a + bi + cj + dk ) = a^2 + b^2 + c^2 + d^2 \in \mathbb{ R }$.

8. $N ( q_1 q_2 ) = N ( q_1 ) N ( q_2 )$.

9. 如若$q \ne 0$,则$q^{ - 1 } = ( N ( q ) )^{ - 1 } \bar{ q }$.

10. $N ( q^{ - 1 } ) = N ( q )^{ - 1 }$

(1)(2)(3)是显然的.(4)的话由于乘法的双线性,只需验证$q_1 , q_2 \in \{ 1 , i , j , k \}$的情形.

(5)(6)是显然的.

(7)除了暴力验证以外,观察到(6),考虑$N ( a + bi + cj + dk ) = a^2 + b^2 + c^2 + d^2 + xi + yj + zk$,然而$\bar{ N ( q ) } = N ( q )$,这必然意味着后面均为$0$.

(8)(9)(10)只需展开检验即可.这就详细描述了除环的结构.

我们还可以证明四元数已经到达极限了,有Frobenius定理:如果$D$是一个除环且是一个$\mathbb{ R }$上的有限维线性空间,其乘法还对$\mathbb{ R }$满足双线性,那么$D$一定同构于$\mathbb{ R } , \mathbb{ C } , \mathbb{ H }$三者其一,再无别的情形.怎么证明?我不会,长大后再学习.

最后我们还可以验证$\mathbb{ H }$可以表为$M_{ 2 \times 2 } ( \mathbb{ C } )$的一个子环,考虑仍取$q = z + jw$,然后将其改写为$\begin{bmatrix}z & - \bar{ w } \\ w & \bar{ z }\end{bmatrix}$.这个矩阵结构可以符合加法,乘法,迹(对应到矩阵的迹),范数(对应到矩阵的行列式).这其实很像复数上把$a + bi \mapsto \begin{bmatrix}a & - b \\ b & a\end{bmatrix}$.那么我们可以:

$$
\begin{aligned}
1 & \mapsto \begin{bmatrix}
1 & \\
& 1
\end{bmatrix} \\
i & \mapsto \begin{bmatrix}
i & \\
& - i
\end{bmatrix} \\
j & \mapsto \begin{bmatrix}
& - 1 \\
1 &
\end{bmatrix} \\
k & \mapsto \begin{bmatrix}
& - i \\
- i &
\end{bmatrix} \\

\end{aligned}
$$

接下来考虑去掉环的性质,只看线性空间的性质,将目光着眼于$\mathbb{ R }^3$上并只取出$q = ai + bj + ck$,取这个子空间为$\mathbb{ H }_0$,在上面挪用$N ( q )$以刻画长度,具体而言只需取$\Vert q \Vert^2 = N ( q )$.

接下来,让$x \in \mathbb{ H }^\times$,我们声明:

1. $\forall q \in \mathbb{ H }$,$N ( xqx^{ - 1 } ) = N ( q )$.

2. 设$R_x \in \text{ End } ( \mathbb{ H }_0 ) , q \mapsto xqx^{ - 1 }$,则这是个正交变换.

3. $\det R_x = 1$.

4. $R_{ tx } = R_x$,其中$t \in \mathbb{ R }^\times$,且$R_x R_y = R_{ xy } , R_{ x^{ - 1 } } = ( R_x )^{ - 1 }$.而且$R_{ - 1 } = R_1 = \text{ id }_{ \mathbb{ H }_0 }$.

(1)是显然的.

(2)的话,不妨假设$N ( x ) = 1$,此时$x^{ - 1 } = \bar{ x }$.首先要证明这个映射确实是$\text{ End } ( \mathbb{ H }_0 )$里面的,观察$\overline{ xqx^{ - 1 } } = \bar{ x^{ - 1 } } \bar{ q } \bar{ x } = x ( \bar{ q } ) x^{ - 1 } = - xqx^{ - 1 }$,这就说明$xqx^{ - 1 } \in \mathbb{ H }_0$.

然后要检验它是线性映射,这里只需使用除环的性质简单验证.

最后由(1)说明其保距,这就说明了它是正交变换.

最后是(3),将$\mathbb{ H } \cong \mathbb{ R }^4$考虑映射$\mathbb{ R }^4 \setminus \{ 0 \} \to \{ \pm 1 \} , x \mapsto \det R_x$.这由于矩阵$R_x$的每个元当然是关于$x$的连续函数,因此整个映射都是连续的,而空间$\mathbb{ R }^4 \setminus \{ 0 \}$是联通的,因此整个映射必然只射到常数,而$\det R_1 = 1$,因此对于任意$x \in \mathbb{ H }^\times$都有$\det R_x = 1$.

(4)只需简单验证即可.

所以我们终于见到了,这个$R_x$应当就是三维上的正交旋转.现在最后的问题在于是否所有的旋转都可以表示为$R_x$.我们有下述定理:即对于任意在$\mathbb{ H_0 }$上的旋转$T$,$\exists x \in \mathbb{ H }$,$N ( x ) = 1$,使得$T = Rx$,并且$x$精确到绝对值是唯一的,换言之,恰有两个$x , - x$满足上述条件.

先证明存在性,回忆到欧拉角声明了$T$可以由三个绕轴旋转完成.也就是说,只要我们能证明绕轴旋转$R_{ e_l } ( \theta )$可以用四元数表示,那么任意旋转都可以用四元数表示了.

不同轴类似只算一个,当绕着$i$那条轴转的时候,取$x = \cos \theta + i \sin \theta$,则$x^{ - 1 } = \bar{ x } = \cos \theta - i \sin \theta$,见到:

$$
\begin{aligned}
xix^{ - 1 } & = i \\
xjx^{ - 1 } & = \cos ( 2 \theta ) j + \sin ( 2 \theta ) k \\
xkx^{ - 1 } & = - \sin ( 2 \theta ) j + \cos ( 2 \theta ) k
\end{aligned}
$$

综上见到$R_x$是:$\begin{bmatrix}1 & \\ & R ( 2 \theta )\end{bmatrix}$,这样就实现了对第一个坐标轴转$2 \theta$的情形.

类似的,就可以说明四元数可以表示所有的旋转.

最后要检验唯一性,如果$R_x = R_y$,其中$N ( x ) = N ( y ) = 1$,那就有$\text{ id }_{ \mathbb{ H }_0 } = R_x ( R_y )^{ - 1 } = R_{ xy^{ - 1 } }$.

于是最后转化为说,要证明如果$R_{ x } = \text{ id }_{ \mathbb{ H }_0 }$,那么$x = \pm 1$.那就说明$\forall q , xqx^{ - 1 } = q$,也就是$x \in Z ( \mathbb{ H } ) = \mathbb{ R }$.又由于$N ( x ) = 1$,所以$x = \pm 1$.

更具体来说,绕着$u$这条轴,且满足$N ( u ) = 1$来说,那么$R_u ( \theta ) = \cos \frac{ \theta }{ 2 } + u \sin \frac{ \theta }{ 2 }$.

首先要验证$x = \cos \frac{ \theta }{ 2 } + u \sin \frac{ \theta }{ 2 }$满足$N ( x ) = 1$,而:

$$
\begin{aligned}
N ( x ) & = x \bar{ x } \\
& = ( \cos \frac{ \theta }{ 2 } + u \sin \frac{ \theta }{ 2 } ) ( \cos \frac{ \theta }{ 2 } - u \sin \frac{ \theta }{ 2 } ) \\
& = \cos^2 \frac{ \theta }{ 2 } - u^2 \sin^2 \frac{ \theta }{ 2 } \\
& = \cos^2 \frac{ \theta }{ 2 } + N ( u ) \sin^2 \frac{ \theta }{ 2 } \\
& = 1
\end{aligned}
$$

而考虑总存在一个旋转$P = R_y$,使得$P ( i ) = u$,此时我们可以知道:

$$
\begin{aligned}
R_u ( \theta ) & = PR_i ( \theta ) P^{ - 1 } \\
& = R_y R_i ( \theta ) R_{ y^{ - 1 } } \\

\end{aligned}
$$

不妨设$R_u ( \theta ) = R_x$,则:

$$
\begin{aligned}
x & = y ( \cos \frac{ \theta }{ 2 } + i \sin \frac{ \theta }{ 2 } ) y^{ - 1 } \\
& = \cos \frac{ \theta }{ 2 } + \sin \frac{ \theta }{ 2 } yiy^{ - 1 } \\
& = \cos \frac{ \theta }{ 2 } + \sin \frac{ \theta }{ 2 } u
\end{aligned}
$$

用欧拉公式的话,有$R_u ( \theta ) = e^{ \frac{ \theta }{ 2 } u }$.

### 复内积空间
#### 共轭空间
这应当带给我们思考,既然远在定义复数的时候,我们就声称过$i$和$- i$无法区分,于是在此,应当研究一下共轭前后的两个空间分别的性质.

设$V$是$\mathbb{ C }$下的向量空间,它的复共轭$\bar{ V }$是按照以下方式确定的$\mathbb{ C }$向量空间:

1. 集合$V$与加法操作与原空间相同.

2. 纯量乘法$\odot : \mathbb{ C } \times V \to V$定义为$t \odot \vec{ v } = \bar{ t } \vec{ v }$.

显然$\overline{ ( \bar{ V } ) } = V$.半双线性映射无非是线性映射$\bar{ V } \to W$.当然,也可以等价说是$V \to \bar{ W }$.我们接下来验证复共轭的若干性质:

1. 映射$z \mapsto \bar{ z }$给出向量空间的共轭$\mathbb{ C } \cong \bar{ \mathbb{ C } } : z \mapsto \bar{ z }$.

2. $\bar{ V }_1 \oplus \bar{ V }_2 = \overline{ V_1 \oplus V_2 }$.

3. $\overline{ \text{ Hom } ( V_1 , V_2 ) } = \text{ Hom } ( \bar{ V }_1 , \bar{ V }_2 )$.

4. ${ \bar{ W } }^\vee \cong \overline{ W^\vee }$,其实是(3)的另一种表示方式.

应当检验上述映射,同构,甚至是完全挪移都是良定义的且半线性的,这里不做赘述.

相信对共轭空间最大的疑问在于,既然其改变了纯量乘法,原本的向量结构为什么不会被打乱呢?事实上,如果你考虑选定一组基的话,纯量乘法上的轻微改变只会使得将其它向量转化为基表示形式的过程中所提取出的纯量发生变化,然而共轭满足$\overline{ z_1 z_2 } = \bar{ z_1 } \bar{ z_2 }$.虽然取定基的做法并非典范,然而不同基之间的转化仍然可以提取纯量.或者说,我们可以先取$V \cong \mathbb{ C }^n \cong ( \bar{ \mathbb{ C } } )^n \cong \bar{ V }$.

或直接从复数的几何意义来说,共轭无非是将逆时针旋转改为顺时针旋转,这应当仍是合理的.

#### (复)半双线性形式
应当见到双线性形式在复空间中未必有很好的应用,原因是正定性难以得到满足.

定义**半线性映射**为$T : V \to W$满足:

1. $T ( \vec{ v }_1 + \vec{ v }_2 ) = T ( \vec{ v }_1 ) + T ( \vec{ v }_2 )$.

2. $T ( t \vec{ v } ) = \bar{ t } T ( \vec{ v } )$.

定义**半双线性映射**为$B : V \times W \to X$满足:

1. $B ( \vec{ v }_1 + \vec{ v }_2 , \vec{ w } ) = B ( \vec{ v }_1 , \vec{ w } ) + B ( \vec{ v }_2 , \vec{ w } )$.

2. 对第一个变量半线性:$B ( t \vec{ v } , \vec{ w } ) = \bar{ t } B ( \vec{ v } , \vec{ w } )$.

3. $B ( \vec{ v } , \vec{ w }_1 + \vec{ w }_2 ) = B ( \vec{ v } , \vec{ w }_1 ) + B ( \vec{ v } , \vec{ w }_2 )$.

4. 对第二个变量线性:$B ( \vec{ v } , t \vec{ w } ) = t B ( \vec{ v } , \vec{ w } )$.

至于复伴随映射以及其余的东西,无非只是在实数上类似的定义的照抄.将上述记作$\text{ Sesq } ( V , W ; X )$.特别地,当$X = \mathbb{ C }$的时候,将此称作半双线性形式.应当立刻见到$\text{ Sesq } ( V , W ; X ) = \text{ Bil } ( \bar{ V } , W ; X )$.

至于非退化等部分仍然是实数情况的照抄,对左右根,不妨设左根空间为$L$,右根空间为$R$,用柯里化过程见到该形式等价于$\psi \in \text{ Hom } ( W ,{ \bar{ V } }^\vee )$或者$\varphi \in \text{ Hom } ( \bar{ V } , W^\vee )$,那么$L = \ker \varphi , R = \ker \psi$,我们应当见到$B ' : ( V / L ) \times ( W / R ) \to F , ( \vec{ v } + L , \vec{ w } + R ) \mapsto B ( \vec{ v } , \vec{ w } )$是非退化双线性形式.此时见到其实$\dim V - \dim L = \dim W - \dim R$.

接下来要用矩阵形式予以表述,我们声称$M_{ m \times n } ( \mathbb{ C } ) \cong \text{ Sesq } ( \mathbb{ C }^m , \mathbb{ C }^n ) , A \mapsto B ( \vec{ v } , \vec{ w } ) = (^\dagger \vec{ v } ) A \vec{ w }$,原因是标准基下$\mathbb{ C }$的对偶空间其实就是共轭转置后的结果.

#### 伴随映射
与实数情况毫无差异,我们在此复述一遍当时的图表即可:

$$
\xymatrix{ V_2 ' \ar[r]^{ T^* } \ar[d]_{ A_2 } & V_1 ' \ar[d]^{ A_1 } \\
\check{ V_2 } \ar[r]^{ ^\dagger T } & \check{ V_1 } \\
V_2 \ar[u] & V_1 \ar[u] \ar[l]_T }
$$

同样,我们可以定义**自伴**性质:$T^* = T$以及**反自伴**性质$T^* = - T$.此处出现了复数与实数不同的地方:假设$c$是非零纯虚数,则$T$自伴当且仅当$cT$反自伴.这性质是容易验证的,也告知我们复数域上的反自伴并不需要像实数那样使用单独的辛空间来刻画.

类比实数,下面定理当然也是对的:

而注意到$T^* T$是自伴的,然而其还有更多的好性质:

1. $( \text{ im } T )^\bot = \ker ( T^* )$.

2. $\text{ im } \ ( T^* T ) = \text{ im } \ ( T^* )$.

3. $\ker ( T^* T ) = \ker T$.

4. $\text{ rk } \ ( T^* T ) = \text{ rk } \ ( T ) = \text{ rk } ( T^* )$.

证明与实数情形无差.

#### Hermite形式
设$V$是$\mathbb{ C }$下的向量空间,$\epsilon \in \{ - 1 , 1 \}$,若半双线性形式$B : V \times V \to \mathbb{ C }$满足$B ( \vec{ v } , \vec{ w } ) = \epsilon \overline{ B ( \vec{ w } , \vec{ v } ) }$的话,我们就称$B$是一个$\epsilon -$Hermite形式.如果$\epsilon = 1$,称这是一个**Hermite形式**;如果$\epsilon = - 1$,则称其为**反Hermite形式**.容易验证,如果$B$是Hermite的,那么$iB$是反Hermite的.

此定义直接导致$B$的左右根是一回事,我们后面会称其为$B$的根基.并容易见到等价于其对应的矩阵满足$A^\dagger = \epsilon A$.也就是说,Hermite形式对应的矩阵自伴,而反Hermite形式对应的矩阵反自伴.

##### 正规线性映射
给定某$\epsilon -$Hermite形式$B : V \times V \to \mathbb{ C }$,我们称满足$( T^* ) T = T ( T^* )$的$T$是**正规线性映射**.容易见到自伴和反自伴当然是正规的.事实上,取$T ' = \frac{ 1 }{ 2 } ( T + T^* ) , T ' ' = \frac{ 1 }{ 2 } ( T - T^* )$即可见到能将$T = T ' + T ' '$拆成一个自伴形式和一个反自伴形式.这种分解还是唯一的,原因是如果有两组形式满足$T_1 ' + T_1 ' ' = T_2 ' + T_2 ' '$,移项后得到$T_1 ' - T_2 ' = T_2 ' ' - T_1 ' '$,左侧是自伴的,右侧是反自伴的,因此它们都是$0$.

若进一步要求$T$是正规的,也就是$( T^* ) T = T ( T^* )$,还可以见到$T ' T ' ' = T ' ' T '$,只需简单验证即可.

##### 二次型
定义复数上的半双线性版本的**二次型**为$f ( x_1 , \cdots , x_n ) = \sum a_{ i , j } \bar{ x_i } x_j$.要证明其与$\epsilon -$Hermite形式同构,不妨先直接考虑$f ( \vec{ v } + \vec{ w } ) = B ( \vec{ v } + \vec{ w } , \vec{ v } + \vec{ w } )$,那只需考虑配极化技巧,定义$\Re$为提取实部,$\Im$为提取虚部,立刻得到:

$$
f ( \vec{ v } + \vec{ w } ) = \begin{cases}
f ( \vec{ v } ) + f ( \vec{ w } ) + 2 \Re B ( \vec{ v } , \vec{ w } ) & \epsilon = 1 \\
f ( \vec{ v } ) + f ( \vec{ w } ) + 2 i \Im B ( \vec{ v } , \vec{ w } ) & \epsilon = - 1
\end{cases}
$$

考虑$\Im B ( \vec{ v } , \vec{ w } ) = \Re B ( i \vec{ v } , \vec{ w } )$,所以上述立刻确定了$B$.

搬运实二次型上的性质即可看到**合同**在复数域上体现于$A = (^\dagger C ) A ' C$.对该多项式做对角化与实数情况无异.

至于复数上的**惯性定理**,证明与实数丝毫不差.

既然如此,我们下面会说明Hermite形式对应了标准内积下的自伴算子,其所有特征值均为实数.由此可以定义其**正定性**,当然也就是所有特征值均为正数.其余类似**半正定性**均可照搬原本性质.

于此,我们可以引出以下定理:

$T^* T$是半正定的.当$T$是单射时,其是正定的.

证明无非仍然是$( T^* T \vec{ v } \mid \vec{ v } )_V = ( T \vec{ v } \mid T \vec{ v } )_W$的直接结论.

#### 复内积空间
考虑正定Hermite形式$( \_ \mid \_ ) : V \times V \to \mathbb{ C }$,这样的资料$( V , ( \_ \mid \_ ) )$称为**复内积空间**,又称作**酉空间**.特别地,在$\mathbb{ C }^n$上定义标准Hermite内积为$( \vec{ x } \mid \vec{ y } ) = \sum \bar{ x_i } y_i$,应当见到其正定且是一个Hermite形式.

容易照搬大部分实内积空间的性质过来,这里简单列举:

1. $\Vert \vec{ v } \Vert = 0 \Leftrightarrow \vec{ v } = 0$.

2. $\Vert t \vec{ v } \Vert = | t | \Vert \vec{ v } \Vert^2$.

3. $\Im ( \vec{ v }_1 \mid \vec{ v }_2 ) = \Re ( i \vec{ v }_1 \mid \vec{ v }_2 )$.

4. 配极化,即$\Vert \vec{ v } + \vec{ w } \Vert^2 = \Vert \vec{ v } \Vert^2 + \Vert \vec{ w } \Vert^2 + 2 \Re ( \vec{ v } \mid \vec{ w } )$.

5. 正交单位向量,正交集合,单位正交基等概念原样照搬.

6. 勾股定理,即$\vec{ v } \bot \vec{ w }$时,$\Vert \vec{ v } + \vec{ w } \Vert^2 = \Vert \vec{ v } \Vert^2 + \Vert \vec{ w } \Vert^2$.

7. 柯西不等式,即$| ( \vec{ v } \mid \vec{ w } ) | \leq \Vert \vec{ v } \Vert \cdot \Vert \vec{ w } \Vert$.

8. 三角不等式,即$\Vert \vec{ v } + \vec{ w } \Vert \leq \Vert \vec{ v } \Vert + \Vert \vec{ w } \Vert$.

9. Gram-Schmidt正交化,策略仍是$\vec{ w }_k = \vec{ v }_k - \sum_{ i = 1 }^{ k - 1 } \frac{ ( \vec{ w }_i \mid \vec{ v }_k ) }{ ( \vec{ w }_i \mid \vec{ w }_i ) } \vec{ w }_i$.

(1)(2)(3)(4)(5)(6)的证明是平凡的.

(7)的证明与实数情景略有差别,仍设$t \in \mathbb{ C }$,并观察:

$$
\begin{aligned}
0 & \leq \Vert \vec{ v } + t \vec{ w } \Vert^2 \\
& = \Vert \vec{ v } \Vert^2 + 2 \Re ( t ( \vec{ v } \mid \vec{ w } ) ) + | t |^2 \Vert \vec{ w } \Vert^2 \\
& \leq \Vert \vec{ v } \Vert^2 + 2 | t | \cdot | ( \vec{ v } \mid \vec{ w } ) | + | t |^2 \Vert \vec{ w } \Vert^2
\end{aligned}
$$

对于(8),只需做一些微小的调整,具体为:

$$
\begin{aligned}
\Vert \vec{ v } + \vec{ w } \Vert^2 & = \Vert \vec{ v } \Vert^2 + 2 \Re ( \vec{ v } \mid \vec{ w } ) + \Vert \vec{ w } \Vert^2 \\
& \leq \Vert \vec{ v } \Vert^2 + 2 | ( \vec{ v } \mid \vec{ w } ) | + \Vert \vec{ w } \Vert^2 \\
& \leq \Vert \vec{ v } \Vert^2 + 2 \Vert \vec{ v } \Vert \cdot \Vert \vec{ w } \Vert + \Vert \vec{ w } \Vert^2 \\
& = ( \Vert \vec{ v } \Vert + \Vert \vec{ w } \Vert )^2
\end{aligned}
$$

(9)的话,值得注意的是在左右两边对$\vec{ w }_i$做内积其实并非完全一致的.做$( \vec{ w }_i \mid \_ )$当然显然是正确的,做$( \_ \mid \vec{ w }_i )$的话,由于是Hermite形式,当然也是对的.

##### 酉变换
复内积空间到自身的同构称为$V$上的**酉变换**.容易类比实数情况,以下命题等价:

1. $T$是酉变换.

2. $T^* = T^{ - 1 }$,特别地,取标准内积空间时,该条等价于$T^\dagger = T^{ - 1 }$,如果$T$是矩阵,我们称其是**酉矩阵**.

3. $\{ \vec{ v }_1 , \cdots , \vec{ v }_n \}$是一组单位正交基,则$\{ T \vec{ v }_1 , \cdots , T \vec{ v }_n \}$也是一组单位正交基.

##### 正规算子的酉对角化(谱定理)
我们下面证明,只要$T$是正规算子,那它就可以酉对角化.换言之$T$如果对应矩阵$A$,那么存在酉矩阵$P$满足$P^\dagger = P^{ - 1 }$而且$P^{ - 1 } AP = \begin{bmatrix}\lambda_1 & & \\ & \ddots & \\ & & \lambda_n\end{bmatrix}$.容易见到其逆命题仍然成立,原因是$P^{ - 1 } A ( A^\dagger ) P = P^{ - 1 } ( A^\dagger ) AP = \begin{bmatrix}| \lambda_1 |^2 & & \\ & \ddots & \\ & & | \lambda_n |^2\end{bmatrix}$.

接下来要证明该定理正命题成立,其证明策略有若干种,下面选取两种进行说明.

第一种策略是,照搬实数域上的情形对空间进行数学归纳.已经可以将一个正规算子分解为$T = T ' + T ' '$,其中$T '$自伴而$T ' '$反自伴.那我们可以类比实数域上对其进行归纳,从而说明$T ' , T ' '$均可以对角化.又因为当$T$正规的时候,$T ' T ' ' = T ' ' T '$,于是满足同步对角化的条件.

第二种策略较为麻烦,但能揭示更多正规算子的性质.假设$N \in \text{ End } ( V )$是正规算子,我们首先证明以下引理:

1. 对于$f \in \mathbb{ C } [ x ]$,$f ( N ) \in \text{ End } ( V )$也是正规的.

2. 假设特征向量$\vec{ v } \in V$使得$\exists \lambda \in \mathbb{ C }$有$N \vec{ v } = \lambda \vec{ v }$,则$N^* \vec{ v } = \bar{ \lambda } \vec{ v }$.

3. 若存在$k \geq 1$使得$N^k = 0$,那么$N = 0$.

4. 对于其任意两个不同的特征子空间$V_\lambda , V_\mu$,当$\lambda \ne \mu$的时候,$V_\lambda \bot V_\mu$.

(1)的话,考虑$( \sum a_i N^i )^* = \sum \bar{ a_i } ( N^* )^i$,其与任意$N$的多项式都交换.

(2)的话不妨直接取$M = N - \lambda I$,根据(1)这当然仍是正规的.取$M^* = N^* - \bar{ \lambda } I$,此时注意到$\ker M$就是$V_\lambda$这个特征子空间,然而见到$\ker ( M ) = \ker ( M^* M ) = \ker ( MM^* ) = \ker ( M^* )$,这就证毕了.当然这也意味着自伴算子的特征值一定都是实数,而反自伴算子的特征值一定都是纯虚数.

(3)考虑到$( N^k )^* = ( N^* )^k$,于是$N$的任意次幂都是正规的.既然如此,考虑$N^* N$当然是自伴的.要证$N = 0$,只需证明$\text{ rk } \ N = 0$,只需证明$\text{ rk } \ ( N^* N ) = 0$,也就只需要证明$N^* N = 0$即可.而如果$N^k = 0$,那么立刻有$( N^* N )^k = 0$,因此我们对$N^* N$,也就是自伴情形证明上述结论即可.

于是接下来不妨假设$N$自伴,其任意次幂都自伴.考虑$k = 1$显然成立,$k = 2$的时候,回忆到$\forall \vec{ v } , ( N \vec{ v } \mid N \vec{ v } ) = ( N^2 \vec{ v } \mid \vec{ v } ) = 0$,因此立刻有$N = 0$.

然后使用数学归纳,对于一般的$k > 2$,如果$k$是偶数,那$( N^{ \frac{ k }{ 2 } } )^2 = N^k = 0$意味着$N^{ \frac{ k }{ 2 } } = 0$,可以数学归纳;当$k$是奇数的时候,$N^{ k + 1 } = 0$,而$\frac{ k + 1 }{ 2 } < k$,因此仍然可以数学归纳.

(4)只需要考虑(2),立刻有:

$$
\begin{aligned}
\mu ( \vec{ v } \mid \vec{ w } ) & = ( \vec{ v } \mid \mu \vec{ w } ) = ( \vec{ v } \mid N \vec{ w } ) \\
& = ( N^* \vec{ v } \mid \vec{ w } ) = ( \bar{ \lambda } \vec{ v } \mid \vec{ w } ) = \lambda ( \vec{ v } \mid \vec{ w } )
\end{aligned}
$$

于是完事.

有了以上引理,我们就可以来揭示正规算子可对角化的性质.

使用复数域上分裂的性质,将其特征多项式$\text{ Char }_T = \prod ( x - \mu_i )^{ a_i }$.取$m = \prod ( x - \mu_i )$,只要我们能说明$m$就是极小多项式,由于其无重根则立即完事.那就只需要验证$m ( T ) = 0$,然而,$m ( T )$由于(1)是正规的,又因为$m ( T )^{ \max a_i } = 0$,用(3)立刻见到$m ( T ) = 0$,再用(4)和Gram-Schmidt法得到正交基,这就证明了该结论.

回忆到实数域上可正交对角化当且仅当自伴,那是因为实数域上的反自伴性质很差.而复数域上二者差别不大,想要它们能够同步对角化就需要有交换性,而正规算子恰好保证了交换性.

但总之,我们由上可以立即见到以下结论成立,当$T$是正规算子的时候:

1. $T$自伴当且仅当所有特征值都是实数.

2. $T$反自伴当且仅当所有特征值都是纯虚数.

3. $T$是酉变换当且仅当所有特征值都满足$| \lambda | = 1$.

原因是自伴性质是$A^\dagger = A$,反自伴性质是$A^\dagger = - A$,酉变换性质是$A^\dagger = A^{ - 1 }$.而只需选取单位正交基,立刻就发现$T$对应的$A = \begin{bmatrix}\lambda_1 & & \\ & \ddots & \\ & & \lambda_n\end{bmatrix}$,$T^*$对应的$A^\dagger = \begin{bmatrix}\bar{ \lambda }_1 & & \\ & \ddots & \\ & & \bar{ \lambda }_n\end{bmatrix}$.

##### 相关实内积空间定理推广
我们声明以下操作在复内积空间上都能做:

1. 二次根:设$T \in \text{ End } ( V )$正定(半正定),那么存在唯一的$S \in \text{ End } ( V )$使得$S$也正定(半正定),并且$S^2 = T$,将此$S$记作$\sqrt{ T }$.

2. 极分解:设$T \in \text{ End } ( V )$可逆,则存在唯一一对$R , U \in \text{ End } ( V )$使得$R$正定,$U$是酉变换,而且$T = RU$.具体地,$R = \sqrt{ TT^* }$.

3. 奇异值分解:对于线性映射$T : V \to W$,存在$V , W$分别的一组正交基和唯一一组$\sigma_1 \geq \cdots \geq \sigma_p$使得$T \vec{ v }_i = \begin{cases}\sigma_i \vec{ w }_i & 1 \leq i \leq p \\ 0 & i > p\end{cases}$.或者写作$A = Q \Sigma P^{ - 1 }$,其中$\Sigma = \begin{bmatrix}\sigma_1 & \\ & \ddots\end{bmatrix}$.

4. MP广义逆:对于$T : V \to W$,存在唯一的$S : W \to V$,使得其满足从前的广义逆结论.特别地,这里的$S$实际上可以用奇异值分解唯一刻画.

##### 复矩阵范数实例
考虑$( A , B ) \to ( A \mid B )_{ HS } = \text{ Tr } ( ( A^\dagger ) B )$,立刻使得$M_{ m \times n } ( \mathbb{ C } )$成为一个复内积空间.在此基础上定义**Hilbert-Schmidt范数**:$\Vert T \Vert_{ HS } = \sqrt{ ( T \mid T )_{ HS } }$.

这里还可以展现$\text{ Tr }$在空间上的体现.回忆到$V$上的内积结构$\vec{ v } \mapsto ( \vec{ v } \mid \_ )$

另外,还可以定义所谓**算子范数**为$\Vert T \Vert = \max_{ \Vert \vec{ v } \Vert_V = 1 } \Vert T \vec{ v } \Vert_W$.应当容易验证:

1. $\Vert tT \Vert = | t | \Vert T \Vert$.

2. $\Vert T_1 + T_2 \Vert \leq \Vert T_1 \Vert + \Vert T_2 \Vert$.

3. $\Vert T \Vert = 0 \Leftrightarrow T = 0$.

4. $\Vert ST \Vert \leq \Vert S \Vert \cdot \Vert T \Vert$.

5. $\Vert I \Vert = 1$.

只需回忆极小化极大原理就知道$\Vert T \Vert^2$取到了$T^* T$的模长最大的特征值,而$\Vert T \Vert_{ HS }^2$取到了$T^* T$的特征值之和,考虑$T : V \to W$,见到:

$$
\frac{ 1 }{ \sqrt{ \dim V } } \Vert T \Vert_{ HS } \leq \Vert T \Vert \leq \Vert T \Vert_{ HS }
$$

综上,就可以定义两种距离,容易验证它们的若干性质.

因此容易见到上面两种范数有互相等价的收敛性和极限概念,柯西列的概念也是等价的.

### 模
考虑将线性空间的定义从域挪到环上,对于一个环$R$,定义**左模**为一个资料$M$,其中$M$自带一个加法群(需要可交换)而其还有一个对环的左乘$R \times M \to M$使得$\forall x \in M , r_1 , r_2 \in R$,以下性质成立:

1. $r ( x + y ) = rx + ry$.

2. $( r_1 + r_2 ) x = r_1 x + r_2 x$.

3. $( r_1 r_2 ) x = r_1 ( r_2 x )$.

4. $1_R x = x$.

同理可以定义**右模**.这里可能又需要定义相反环$R^{ op }$,相当于把乘法顺序调转.当$R$是交换环的时候,$R = R^{ op }$,此时左模和右模无差别.下面无特殊说明默认为左模.

容易见到线性空间是模的特例.

还可以定义两个都定义在$R$上的模$M_1 , M_2$之间的同态$f : M_1 \to M_2$满足:

1. $f ( rx ) = rf ( x )$.

2. $f ( x_1 + x_2 ) = f ( x_1 ) + f ( x_2 )$.

容易见到$( f_1 + f_2 ) ( x ) = f_1 ( x ) + f_2 ( x )$成立并且$( f_1 + f_2 )$当然也是同态,然而$( rf ) ( x ) = rf ( x )$并不一定是同态,原因是:

$$
\begin{aligned}
rf ( r ' x ) & = rf ( r ' x ) = rr ' f ( x ) \\
& \ne r ' rf ( x ) = r ' ( rf ) ( x )
\end{aligned}
$$

因此并不一定能把里面的$r '$提到外面来,这就出问题了.模的同态集合并非是模.然而同态对复合操作总是构成幺半群,容易见到:

1. $( f_1 + f_2 ) \circ g = ( f_1 \circ g ) + ( f_2 \circ g )$.

2. $f \circ ( g_1 + g_2 ) = ( f \circ g_1 ) + ( f \circ g_2 )$.

那也可以定义同构,就是存在逆映射.还可以定义子模.容易见到任何一组子模的交仍然是子模.

还可以定义商模.假设$N$是$M$的子模,在加法群意义下存在一个商群$M / N$(原因是此加法群可交换),现在我们想将其升级为$R$的模.我们定义:$R \times ( M / N ) \to ( M / N ) : ( r , x + N ) \mapsto rx + N$作为模上的标量乘法,我们试图证明:

1. 此运算使得$M / N$成为模.

2. $q : M \to M / N$构成模同态,而且$\ker q = N$.

(1)只需要简单检验即可(由于$N$对于标量乘法封闭显然).

(2)则是由于$q$本身已经是加法群的同态,又有$q ( rx ) = rx + N = rq ( x )$,这就搞定.

然后定义直积,考虑一组定义在$R$上的模$( M_i )_{ i \in I }$,考虑它们本身都是一个加法群,因此先用加法群的定义拿到$\prod_i M_i$,然后将它升级为一个$R$模,方法是$( r , ( x_i )_{ i \in I } ) \mapsto ( rx_i )_{ i \in I }$.同理把其至多有限多个分量非零的子集定义为直和.内直和亦然同理.设$( M_i )_{ i \in I }$是$M$的一组子模,以下命题等价:

1. $\sum_{ i \in I } M_i = M$,而且$\forall i \in I , M_i \cap \sum_{ j \ne i } M_j = \{ 0 \}$.

2. $\forall x \in M$都能唯一写作$x = \sum_{ i \in I } x_i$的形式.

3. $\sigma : \bigoplus_{ i \in I } M_i \cong M$.

验证上述命题是简单的.

最后,如果存在一个$M$的子集$S$使得$M = \langle S \rangle$,我们称$M$是**有限生成**的.

#### 自由模
设$X$为集合,其上的**自由模**定义为直和$R^{ \oplus X }$,可以自然地将$X$嵌入$R^{ \oplus X }$,只需将$x \mapsto ( r_y )$,其中$r_y = \begin{cases}1 & x = y \\ 0 & x \ne y\end{cases}$.

对于任何一个$R$上的模$N$,我们可以将$\mathrm{ Hom }_R ( R^{ \oplus X } , N )$与$X \to N$的映射一一对应.从前者对应到后者只需将映射限制在$X$上即可(上述已经给出将$X$嵌入其中的策略),从后者到前者的对应只需逐分量映射亦可.

既然$X$可以如上嵌入$R^{ \oplus X }$,能否找到一个$R$上的模$M$,使得干脆使得$X \subseteq M$并且$M \cong R^{ \oplus X }$呢.其实这里可以生添一个定义,假设$X \subseteq M$并且$M$已经是$R$模,以下命题等价:

1. $X$生成$M$而且线性无关.

2. 每个$m \in M$都能唯一表示成有限和$m = \sum_{ x \in X } r_x x$.

3. $\varphi : R^{ \oplus X } \cong M$是同构.

的确容易见到上面命题等价.此时称模$M$是以$X$为**基**的自由模.从而见到所有的向量空间其实都是自由模.并且只要我能找到一个模的一组基,那这个模自然成为了自由模.当然也存在非自由模,Example1给出了一个很平凡的非自由模实例.

回忆到线性空间上的基的定义,事实上当$R$是交换环的时候,它的任意两组基$X , Y$都满足$| X | = | Y |$,从而自然地引出其**秩**为$\mathrm{ rk } ( M ) = | X |$的定义.然而这个证明有点艰难,因此我们尝试去证明一个弱些的版本:整环上的有限生成自由模.

此时对于一组基$X$,则$M \cong R^{ \oplus X }$.注意到如果$| X | = \infty$,则$R^{ \oplus X }$必然不是有限生成的.因此有限生成自由模的基一定满足$| X | < \infty$.

而之后的做法只需将$R$通过分式域$\mathrm{ Frac } ( R )$嵌入进向量空间就可以了.

我们还想要像线性空间一样,将$\mathrm{ Hom }_R ( R^n , R^m ) \cong M_{ m \times n } ( R )$,这里取右模是方便的(因为需要标量乘法),也就是:

$$
\begin{bmatrix}
x_1 \\
\vdots \\
x_n
\end{bmatrix} \mapsto A \begin{bmatrix}
x_1 \\
\vdots \\
x_n
\end{bmatrix}
$$

用右模的好处是标量乘法可以看作$1 \times 1$的矩阵乘法:

$$
\begin{bmatrix}
x_1 t \\
\vdots \\
x_n t
\end{bmatrix} \mapsto A \begin{bmatrix}
x_1 t \\
\vdots \\
x_n t
\end{bmatrix} = A \begin{bmatrix}
x_1 \\
\vdots \\
x_n
\end{bmatrix} t
$$

至于证明只需要化到基上即可.

###### Example1(非自由模)
取$R = \mathbb{ Z } , M = \mathbb{ Z } / n \mathbb{ Z }$,其中$n \ne 0 , \pm 1$.此时立刻见到$M$必定不是自由的,因为不存在一个$X$使得$M \cong \mathbb{ Z }^{ \oplus X }$,原因是左侧是有限的$n$个元素,而右侧要么是单个元素,要么是无穷多元素.

#### 线性映射和模结构
回忆到$F [ x ]$是一个自然的$F$向量空间.我们的问题是如若$V$是$F$上的向量空间,能否将其升级为一个$F [ x ]$上的模.

我们先指定一个$V$上的线性变换$T \in \mathrm{ End } ( V )$,考虑$\forall f \in F [ x ]$,$f ( T ) \in \mathrm{ End } ( V )$,直接将其作用在$V$上就得到了一个$F [ x ]$上的模.换言之就是将$x \cdot v \mapsto T ( v )$.我们现在想要证明所有可以使得$V$升级为$F [ x ]$模的办法,均可以转化为指定一个映射$T \in \mathrm{ End } ( V )$.

对此进行说明,将$F$上的向量空间$V$升级为$F [ x ]$上的模等价于说对所有$f \in F [ x ]$指定对应于纯量乘法的映射$\rho_f : V \to V$,使得:

1. $\rho_f ( v_1 + v_2 ) = \rho_f ( v_1 ) + \rho_f ( v_2 )$.

2. $\rho_{ f + g } ( v ) = \rho_f ( v ) + \rho_g ( v )$.

3. $\rho_{ fg } ( v ) = \rho_f ( \rho_g ( v ) )$.

4. 当$c \in F$的时候,应该有$\rho_c ( v ) = cv$,以保证原本的向量空间结构.

容易检验当我们指定映射$T$后的确能使其升级为$F [ x ]$上的模.下面我们说明将$V$升级为$F [ x ]$上的模的过程一定对应了一个映射$T \in \mathrm{ End }$.

假设$V$已经升级为$F [ x ]$模,那么考虑证明$\rho_x$必须是个线性变换即可.而由上面我们说的,$\rho_x ( v_1 + v_2 ) = \rho_x ( v_1 ) + \rho_x ( v_2 )$,并且有$\rho_x ( cv ) = \rho_{ xc } ( v ) = c \rho_x ( v )$,这就搞定了.

而如果考虑模之间的映射$\varphi : V \to V '$,则$\varphi$是模同态等价于:

1. $\varphi ( c_1 v_1 + c_2 v_2 ) = c_1 \varphi ( v_1 ) + c_2 \varphi ( v_2 )$.

2. $\varphi f ( T ) = f ( T ' ) \varphi$.

(1)使得$\varphi$必须为一个线性映射,既然如此(2)中就可以拆开$f ( T )$,也就变成了要求$\forall n \geq 0$,都有$( T ' )^n \circ \varphi = \varphi \circ T^n$.容易见到只需满足$T ' \varphi = \varphi T$即可数学归纳满足上面的性质.乍一看可能觉得这个等式比较奇怪,但我们可以将其写作交换图表:

$$
\xymatrix{ V \ar[r]^\varphi \ar[d]_T & V ' \ar[d]^{ T ' } \\
V \ar[r]_\varphi & V ' \\
 }
$$

如果$\varphi$是同构,那这当然等价于$T ' = \varphi T \varphi^{ - 1 }$.

我们现在想要做一些更深的操作,能不能干脆对所有$F [ x ]$上的模结构$( V , T )$进行分类呢?不妨设$\dim_F V = n < \infty$,既然如此就有$V \cong F^n$.既然如此指定$T$无非是指定矩阵$F^{ n \times n }$,因此在同构意义下分类$F [ x ]$上的模结构等价于分类共轭的$n \times n$的矩阵.换言之分类共轭的$n \times n$的矩阵也就相当于同构意义下分类$F [ x ]$上的模结构.回忆到$F [ x ]$是PID(主理想环),而$\dim V < \infty$导出$V$是有限生成的.

#### 主理想环上的有限生成模
考虑$R$是一个交换环,$I \subseteq R$是一个理想,考虑一个$R$上的模$M$,现在考虑定义$M [ I ] = \{ x \in M | \forall a \in I , ax = 0 \}$为$I$所零化的子模(容易检验封闭性).当$I = ( h ) = \{ hr | r \in R \}$的时候,我们记作$M [ I ] = M [ h ]$,原因是显然此时只需要$h$零化即可.进一步地,留意到$M [ I ] = \bigcap_{ h \in I } M [ h ]$.

考虑当$R$是整环的时候,如果$x \in M , \exists h \in R \setminus \{ 0 \} , x \in M [ h ]$,我们称$x$是一个**挠元**.容易见到自由模无挠.

接下来考虑所有挠元组成的集合称为**挠子模**$M_{ tors }$,我们下面来证明它是一个子模.

如若$x , y \in M_{ tors }$,则存在$r , s \in R \setminus \{ 0 \}$,$rx = sy = 0$,由于此时我们在看整环,所以$rs \ne 0$.留神到:$rs ( ax + by ) = 0 , \forall a , b \in R$.于是就拿到了封闭性.

定义**无挠商**为$M_{ tf } = M / M_{ tors }$.下面我们来证明其中的确没有挠元.

考虑$\bar{ x } \in M_{ tf } , \bar{ x } = x + M_{ tors }$.如若$\exists r \in R \setminus \{ 0 \}$使得$r \bar{ x } = 0$,意味着$rx + M_{ tors } = M_{ tors }$,这意味着$rx \in M_{ tors }$,既如此则$\exists s \in R \setminus \{ 0 \}$使得$s ( rx ) = ( sr ) x = 0 \Rightarrow x \in M_{ tors }$,所以我们这个的确是无挠的.

接下来看$R$是PID的情形,此时考虑任何一个$t$都可以被分解为$t \sim p_1^{ a_1 } \cdots p_m^{ a_m } , a_i \geq 0$.我们下面来证明此时$M [ t ] = \bigoplus_{ i = 1 }^m M [ p_i^{ a_i } ]$.

其实只需要证明$t = ab$,其中$a , b$互素就可以推出$M [ t ] = M [ a ] \oplus M [ b ]$就行了对吧.和我们之前证明极小多项式的结论时的策略完全一致:

先证明$M [ t ] = M [ a ] + M [ b ]$,考虑$x \in M [ t ]$,留意到存在$u , v$使得$au + bv = 1$,所以$\forall x \in t$,$x = aux + bvx \in M [ b ] + M [ a ]$.

在证明$M [ a ] \cap M [ b ] = \{ 0 \}$,设$x \in M [ a ] \cap M [ b ]$,则$x = aux + bvx = 0$.

接下来我们证明$M [ a ] \cap M [ b ] = M [ \gcd ( a , b ) ]$.裴蜀定理告诉我们存在一对$u , v$使得$\gcd ( a , b ) = au + bv$,因此$M [ \gcd ( a , b ) ] \supseteq M [ a ] \cap M [ b ]$,反之如若$x \in M [ \gcd ( a , b ) ]$,则$x \in M [ a ] , x \in M [ b ]$,这就搞定了.

接下来考虑$p$是PID里的一个素元,容易见到$M [ p ] \subseteq M [ p^2 ] \cdots$,定义$M [ p^\infty ] = \bigcup_{ i \geq 1 } M [ p^i ]$.如果$\exists t \in R , t \ne 0 , M = M [ t ]$.此时考虑对$t$作分解,应该得到$M = \oplus M [ p_i^{ a_i } ]$.事实上此时我们可以直接说$M = \oplus_{ p } M [ p^\infty ]$,原因是考虑当$n \geq p^{ a_p }$时候,有$M [ p^n ] = M [ p^n ] \cap M [ t ] = M [ p^{ a_p } ]$.

下面干脆假设$M = R / ( t )$,$p$是$R$中的素元,我们有以下结论:

1. 如果$p^a | t$,则$( R / ( t ) ) [ p^a ] \cong R / ( p^a )$.

2. 如果$p^a | t , p^{ a + 1 } \nmid t$,则$( R / ( t ) ) [ p^\infty ] \cong R / ( p^a )$.

对于(1),如若$p^a | t$,考虑$\bar{ x } = x + ( t ) \in R / ( t )$,如若满足$p^a \bar{ x } = 0 \Leftrightarrow \exists y , p^a x = ty$,此时取$s = \frac{ t }{ p^a }$,于是上述条件立刻等价于$x \in ( s )$.所以$( R / ( t ) ) [ p^a ] = ( s ) / ( t )$.然而我们有一个满射$R \to ( s ) / ( t ) : y \mapsto sy + ( t )$,观察其$\ker = \{ y \in R : t | sy \} = ( p^a )$,由同态定理知道$( R / ( t ) ) [ p^a ] \cong R / ( p^a )$.

对于(2),我们已经说过了$M [ p^\infty ] = M [ p^a ]$,于是由(1)就做完了.

##### 自由模与其子模
回忆到Noether性质:任何一个理想链最终都会稳定.那同理可以定义一个$R$上的模$M$是有Noether性质的,当它的任何一个子模链都会趋于稳定.回忆到如果$R$是一个PID,则它当然有Noether性质,同时它作为自己的模也具有Noether性质(它的子模其实就是子理想).

下面我们来证明,如果一个模$M$具有Noether性质,那它是有限生成的.

证明非常简单,考虑反证,如果其不是有限生成的,那我们考虑任意$x_1$,$Rx_1$不能生成$M$,取$x_2 \in M \setminus ( Rx_1 )$,则$Rx_1 + Rx_2$也不能生成$M$,以此类推,这就得到了一个无穷增大的子模链,不符合Noether性质.

接下来引入一个引理:设$R$是一个交换环,$M$是$R$上的一个模,而且$M '$是$M$的一个子模,如果$M ' ' = M / M '$,那么$M$有Noether性质等价于$M '$和$M ' '$都有Noether性质.

先来看必要性,假设$M$有Noether性质,则其子模$M '$的子模列当然也是$M$中的子模列,所以$M '$必定有Noether性质.而反之$M ' '$中的子模列当然可以取原像回到$M$中,必定也是Noetherian的.

再来看充分性,考虑$M$中的一列子模$M_1 \subseteq M_2 \subseteq \cdots$.取$M_i ' = M_i \cap M '$,$M_i ' ' = ( M_i + M ' ) / M '$.它们当然分别是$M '$和$M ' '$中的子模列,所以存在一个$i_0 , \forall i \geq i_0$,$M_i ' = M_{ i + 1 } ' , M_i ' ' = M_{ i + 1 } ' '$.下面我们试图证明当$i \geq i_0$的时候此时也有$M_i = M_{ i + 1 }$,这只需证明$M_i \supseteq M_{ i + 1 }$.

考虑$\forall x \in M_{ i + 1 } , x + M ' \in M_{ i + 1 } ' ' = M_i ' '$,从而我们知道一定存在一个$y \in M_i$使得$x + M ' = y + M '$,意味着$x - y \in M '$,而$x \in M_{ i } \subseteq M_{ i + 1 }$,所以$x - y \in M ' \cap M_{ i + 1 } = M_{ i + 1 } ' = M_i ' \subseteq M_i$,由于$y \in M_i$,自然导出$x \in M_i$.从而证毕.

其实用第二同构定理见到$M_i ' ' \cong M_i / M_i '$也就搞定了.

现在我们有了如上引理,我们可以得知以下推论:

1. 如果$M_1 , M_2$都是Noether模,则$M_1 \oplus M_2$也是Noether模.

2. 作为(1)的推论,如果$R$是一个PID,$E$是一个$R$上的自由模,如果$E$是有限生成的,则$E$是Noether模.

3. 作为(2)的推论,有限生成自由模的子模总是有限生成的.

##### Smith标准型
最后要做的是在模上引入矩阵符号,对于交换环$R$上的模$M$,假设$e_1 , \cdots , e_n \in M$且$x_1 , \cdots , x_m \in M$,取矩阵$A \in M_{ n \times m } ( R )$,此时记$( x_1 , \cdots , x_m ) = ( e_1 , \cdots , e_n ) A$,当且仅当$x_j = \sum_{ i = 1 }^n e_i a_{ i , j }$.容易检查以下结论:

1. $( e_1 , \cdots , e_n ) AB = ( e_1 , \cdots , e_n ) ( AB )$.

2. $( e_1 , \cdots , e_n ) I = ( e_1 , \cdots , e_n )$.

3. 如果$e_1 , \cdots , e_n$线性无关,则$A$若存在则唯一确定.

同时Cramer法则告诉我们,当$\det A \in R^\times$的时候,$A$一定存在逆矩阵$B$使得$AB = BA = I$,这也很显然是$A$存在逆矩阵的充要条件,因为$\det B = \frac{ 1 }{ \det A }$.还可以检查如下结论:

1. 如果$( y_1 ' , \cdots , y_n ' ) = ( y_1 , \cdots , y_n ) P$,其中$P$可逆,则$\sum Ry_i = \sum R y_i '$.

2. 设$( e_1 , \cdots , e_n )$为基,则$( e_1 ' , \cdots , e_n ' ) = ( e_1 , \cdots , e_n ) A$是另一组基,当且仅当$A$可逆.

现在考虑自由模$E$,其基为$e_1 , \cdots , e_n$,其有一个子模$N$,可以被$x_1 , \cdots , x_m$生成.考虑取一个唯一的矩阵$A$满足$( x_1 , \cdots , x_m ) = ( e_1 , \cdots , e_m ) A$,此时发现如果将$A$换成$AP$,其中$P$是一个可逆矩阵,则$( x_1 , \cdots , x_m )$会被改变.如果将$A$换成$QA$,则$( e_1 , \cdots , e_n )$会被改变.

由此我们有**Smith标准型**:当$R$是PID的时候,对于矩阵$A \in M_{ n \times m }$,总存在两个可逆矩阵$P , Q$,使得$d_1 | d_2 | \cdots \in R$使得$A = Q \begin{bmatrix}d_1 & & \\ & d_2 & \\ & & \ddots\end{bmatrix} P$.

上述结论对欧几里得整环是平凡结论,而且还可以计算.

那么对于PID怎么办呢?难点当然在于现在我们没有办法任意做欧几里得算法了,也许唯一的工具只有裴蜀定理.

考虑数学归纳,如果$A = 0$那当然已经搞定了对吧,否则我们总可以做点初等变换使得$A$的左上角元素不为$0$.

现在假设第一列第一行元素为$a \ne 0$,第一列第二行元素为$b \ne 0$,取$d = \gcd ( a , b ) , u = - \frac{ b }{ d } , v = \frac{ a }{ d }$,用裴蜀定理拿到$sa + tb = d$,然后用下述矩阵左乘:

$$
\begin{bmatrix}
s & t & & & \\
u & v & & & \\
& & 1 & & \\
& & & \ddots & \\
& & & & 1
\end{bmatrix}
$$

就可以让第一列第二行的元素变成$0$.不断做此过程,矩阵变为:

$$
\begin{bmatrix}
d_1 ' & \\
& A_1
\end{bmatrix}
$$

其中$d_1 '$是$A$的第一行第一列所有元素的$\gcd$,继续做此操作得到:

$$
\begin{bmatrix}
d_1 ' & & \\
& d_2 ' & \\
& & A_1
\end{bmatrix}
$$

如果$d_1 ' \nmid d_2 '$,那我们就把第二列加到第一列上,然后重复以上操作,从而使得左上角变成$\gcd ( d_1 ' , d_2 ' )$.这样我们就搞定了.

##### 结构定理
扔出一个**结构定理**:$R$是一个PID,$M$是一个$R$模,我们都可以将$M$拆成:

$$
M \cong ( R / I_1 ) \oplus \cdots \oplus ( R / I_k ) \oplus E
$$

其中$I_1 \supseteq \cdots \supseteq I_k \ne \{ 0 \}$是$R$上的一列真理想,而$E$是一个$R$上的自由模.

并且上述拆法是有唯一性的,也就是如果还存在另一组拆法:

$$
M \cong ( R / J_1 ) \oplus \cdots \oplus ( R / J_{ k ' } ) \oplus E '
$$

则$k = k ' , I_1 = J_1 , \cdots , I_k = J_{ k ' } , E \cong E '$.

回忆到$E$既然是整环上的有限生成自由模,那它可以同构于$R^{ rk ( E ) }$.因此我们可以在上述一列理想后面补$\mathrm{ rk } ( E )$个$\{ 0 \}$.这样就可以简写作:

$$
M \cong ( R / I_1 ) \oplus \cdots \oplus ( R / I_k )
$$

这组$I_1 \supseteq \cdots \supseteq I_k \supseteq \{ 0 \} \cdots$被称为$M$的一组**不变因子**.

此时我们回忆到PID中的自由模不能有挠元,而前面商掉的部分全是挠元,因此结构定理会给出$( R / I_1 ) \oplus \cdots \oplus ( R / I_k ) \cong M_{ tors } = M [ I_k ] = M [ f_k ]$.

其实这也可以写成结构定理的第二种形式:

$$
M_{ tors } \cong \bigoplus_p M_{ tors } [ p^\infty ] \cong \bigoplus_{ p | f_k , p \in \mathrm{ prime } } \bigoplus_i R / ( p^{ b_i ( p ) } )
$$

其中$\forall p , 0 \leq b_1 ( p ) \leq \cdots \leq b_k ( p )$.我们将这一列称为**初等因子**.

现在我们着手证明结构定理,首先给出如下引理:

设$E$为主理想环$R$上的自由模,秩为$n$,而$N$是其子模,我们下面证明存在$E$的一组基$f_1 , \cdots , f_n$以及$R$的一列元素$d_1 | \cdots | d_n$,令$r = \max \{ i | d_i \ne 0 \}$,则$d_1 f_1 , \cdots , d_r f_r$构成$N$的基.

回忆到自由模的子模一定是有限生成的而且秩不超过$n$,此时取$x_1 , \cdots , x_m$是$N$的一组生成元,则存在一个唯一的$A$使得$( x_1 , \cdots , x_m ) = ( e_1 , \cdots , e_n ) A$,由Smith标准型,式子变成:

$$
( x_1 , \cdots , x_m ) P^{ - 1 } = ( e_1 , \cdots , e_n ) Q \begin{bmatrix}
d_1 & & \\
& d_2 & \\
& & \ddots
\end{bmatrix}
$$

此时观察左侧的确给出了一组$N$的一组生成元,而右侧的$( e_1 , \cdots , e_n ) Q$给出了另一组基$( f_1 , \cdots , f_n )$.这就完事了.

此时我们有了如上引理,来证明结构定理的存在性.对于有限生成模$M$,由定义知道存在一组元素$\langle x_1 , \cdots , x_n \rangle = M$.既然如此考虑取$E = R^{ \otimes n }$,考虑模同态$E \to M , ( r_i )_{ i = 1 }^n \mapsto \sum_{ i = 1 }^n r_i x_i$,容易见到这显然是个满同态,因此取其$\ker = N$,自然有$M \cong E / N$.根据引理,可以拿到$N$的一组基$d_1 f_1 , \cdots , d_r f_r$,此时见到:

$$
M \cong \cfrac{ Rf_1 \oplus \cdots \oplus R f_n }{ Rd_1 f_1 \oplus \cdots Rd_r f_r }
$$

然而对于单个的$f_n$来说,自然有$Rf_n \cong R$,而且如果$N_1$是$M_1$的子模,$N_2$是$M_2$的子模,并且$M_1 \cap M_2 = \{ 0 \}$,容易检验$( M_1 \oplus M_2 ) / ( N_1 \oplus N_2 ) \cong ( M_1 / N_1 ) \oplus ( M_2 / N_2 )$,而$( Rf_i ) / ( Rd_i f_i ) \cong R / ( d_i )$,这就搞定了存在性的部分.

接下来来证明唯一性,假设:

$$
M \cong ( R / I_1 ) \oplus \cdots \oplus ( R / I_k ) \oplus E \cong ( R / J_1 ) \oplus \cdots \oplus ( R / J_{ k ' } ) \oplus E '
$$

然而我们已经知道$E \cong M_{ tf }$而且$E ' \cong M_{ tf }$,因此有$E \cong E '$.因此我们将问题化约到没有自由部分的特例.于此使用初等因子形式的证明,考虑$M [ p^\infty ]$自然是其中所有的$R / ( p^j )$结构,因此必然只涉及它们的部分会同构,此时化约到已知为:

$$
R / ( p^{ b_1 } ) \oplus \cdots \oplus R / ( p^{ b_k } ) \cong R / ( p^{ b_1 ' } ) \oplus \cdots \oplus R / ( p^{ b_{ k ' } ' } )
$$

为证明这些项对应相同,需要将这些东西等同于某种空间上的结构,我们先来着手证明$k = k '$.回忆到$M [ p ] = \bigoplus_{ i } ( R / ( p^{ b_i } ) ) [ p ] = \bigoplus_{ i } R / ( p )$,此时见到$M [ p ]$实际上也可以表示为$R / ( p )$上的向量空间(回忆到主理想环商掉素理想后得到了一个域),而且$\dim_{ R / ( p ) } M [ p ] = k$,因此必然有$k = k '$.

类似上面,考虑对于一个固定的$a \geq 1$,考察$p^c M = \bigoplus_{ i , b_i > c } p^c R / p^{ b_i } R = \bigoplus_{ i , b_i > c } R / p^{ b_i - c } R$,此时观察$( p^c M ) [ p ]$,它的维度就是满足$b_i > c$的$i$的个数,即可继续见到两遍逐个相同.

##### 有限生成交换群的分类
我们先将交换群$A$等同于$\mathbb{ Z }$上的模.下述用加法群,则上面有自然的倍数运算$\mathbb{ Z } \times A \to A : na \mapsto a + a + \cdots + a$.因此交换群立刻等价于$\mathbb{ Z }$模.

然而模上的结构定理立刻给出:

$$
A \cong \mathbb{ Z } / d_1 \mathbb{ Z } \oplus \cdots \oplus \mathbb{ Z } / d_k \mathbb{ Z } \oplus \mathbb{ Z }^{ \oplus m }
$$

其中$d_1 | \cdots | d_k$,这就搞定.

#### 有理标准型
取定$R = F [ x ]$,我们可以将一个真理想写作$I_i = ( f_i )$,其中$f_i \in F [ x ] \setminus F$,并且如果进一步要求其首项为一则$f_i$唯一.

现在来看对于一个固定的多项式$f$,此时$F [ x ] / ( f )$必然也是一个$F [ x ]$模,那在上面的$x$必然也对应了一个矩阵,设$\deg f = n , f = x^n + c_{ n - 1 } x^{ n - 1 } + \cdots + c_0$,取其有序基为$\{ 1 + ( f ) , x + ( f ) , \cdots , x^{ n - 1 } + ( f ) \}$.容易见到$x$对应的矩阵应该形如:

$$
C_f = \begin{bmatrix}
0 & 0 & \cdots & 0 & - c_0 \\
1 & 0 & \cdots & 0 & - c_1 \\
0 & 1 & \cdots & 0 & - c_2 \\
\vdots & & \ddots & & \vdots \\
0 & 0 & \cdots & 1 & - c_{ n - 1 } \\

\end{bmatrix}
$$

这个被称为**友矩阵**.按照最后一列展开见到其特征多项式就是$f$.

结构定理告诉我们,对于上述的一个模(我们之前说过可以唯一对应一个矩阵$A$),可以找到唯一的一组首项为一的多项式$f_1 | \cdots | f_k , k \geq 1$,其中$\sum \deg f_i = n$,则$A$可以对应于它们所对应的友矩阵所构成的分块对角矩阵,称为**有理标准型**.

上述的具体细节是次数相关的讨论.对于$A$对应的$F [ x ]$上的模$M$,我们之前已经用结构定理搞定说:

$$
M \cong F [ x ] / ( f_1 ) \oplus \cdots \oplus F [ x ] / ( f_k ) \oplus E
$$

然而回忆到$\dim_F M = n , \dim_F F [ x ] = + \infty$,所以必然有$E = \{ 0 \}$.既然如此$\sum \dim_F ( F [ x ] / ( f_i ) ) = \sum \deg f_i = n$.

有理标准型的强大之处在于蕴含了我们之前讲过的很多事情,并且给出了更加简单的描述:我们下面证明:

1. 极小多项式有$\mathrm{ Min }_A = f_k$.

2. 特征多项式有$\mathrm{ Char }_A = \prod_{ i = 1 }^k f_i$.

先证明(1):如果多项式$h$满足$h ( A ) = 0$,由于我们此时已经将$A$的作用等价于$x$的作用,因此$h ( x )$必然作用于每个商模上都是$0$,那就必然意味着$\forall i , f_i | h \Leftrightarrow f_k | h$.

再证明(2):首先我们知道友矩阵的特征多项式满足$\mathrm{ Char }_{ C_f } = f$,此时发现证完了,因为直和直接就是每一分模的特征多项式乘积.

回忆到结构定理存在第二种表示方法,我们考虑将$\mathrm{ Min }_A = f_k = p_1^{ a_1 } \cdots p_k^{ a_k }$,那我们就可以将$A$同构于若干分块对角矩阵$A_1 , \cdots , A_m$组成的分块对角矩阵,其中每一个$A_i$亦然可以分解为若干友矩阵$C_{ p_j^{ b } }$之类的东西.

接下来看如何计算一个有理标准型,考虑$V$是一个$F$向量空间,而$T \in \mathrm{ End } ( V )$,从而使得$V$从一个$F$向量空间升级为一个$F [ x ]$模.

如果我们能拿到一个满射$\varphi : F [ x ]^{ \oplus n } \to V$,那考虑$\ker \varphi$当然是$F [ x ]^{ \oplus n }$的一个子模,既然如此,存在$F [ x ]^{ \oplus n }$的一组基$f_1 , \cdots , f_n$,以及一组非零多项式$d_1 | \cdots | d_n$,使得$\ker \varphi = \bigoplus_i F [ x ] d_i f_i$,从而使得$V \cong \bigoplus_i F [ x ] / ( d_i )$.

策略当然是简单的,考虑$F [ x ]^{ \oplus n }$的一组基$e_1 , \cdots , e_n$,以及$V$作为向量空间的一组基$v_1 , \cdots , v_n$,我们要求$\varphi ( e_i ) = v_i , \varphi ( x ) = T$,从而$\varphi ( \sum_i r_i e_i ) = \sum_i r_i ( T ) v_i$,立刻见到这的确是满射,疑问只在于如何求其$\ker$.

定义$y_j = xe_j - \sum_{ i = 1 }^n a_{ i , j } e_i$.取$N = \sum_{ j = 1 }^n F [ x ] y_j$,其中$a_{ i , j }$是$T$再这组基下对应的矩阵元素.此时立刻见到$\varphi ( y_j ) = Tv_j - \sum_{ i } a_{ i , j } v_i = 0$,因此$\ker \varphi \supseteq N$.

反之,观察到$xe_j = \sum_i a_{ i , j } e_i + y_j \in \sum_i a_{ i , j } e_i + N$,反复做此操作,得知$\forall w \in F [ x ]^{ \bigoplus n } , w \in \sum_{ i } c_i e_i + N$,于此做$\varphi$得到$\varphi ( w ) = 0 \Leftrightarrow \sum_i c_i v_i = 0$,从而得知所有的$c_i = 0$.这就证明了$\ker \varphi = N$.

观察到$( x_1 , \cdots , x_n ) = ( e_1 , \cdots , e_n ) ( xI - A )$,而$( xI - A )$的确是一个定义在$F [ x ]$上的矩阵,对此用Smith标准型得到$( e_1 , \cdots , e_n ) Q \begin{bmatrix}d_1 & & \\ & \ddots & \\ & & d_n\end{bmatrix} P$,这就搞定了.

###### Example1
考虑$V = F^3$,$T = A = \begin{bmatrix}- 1 & - 2 & 6 \\ - 1 & 0 & 3 \\ - 1 & - 1 & 4\end{bmatrix}$.

首先要做的是计算$xI - A$,对此:

$$
\begin{aligned}
xI - A & = \begin{bmatrix}
x + 1 & 2 & - 6 \\
1 & x & - 3 \\
1 & 1 & x - 4
\end{bmatrix} \\
& \to \begin{bmatrix}
0 & - x + 1 & - x^2 + 3 x - 2 \\
0 & x - 1 & - x + 1 \\
1 & 1 & x - 4
\end{bmatrix} \\
& \to \begin{bmatrix}
1 & 1 & x - 4 \\
0 & x - 1 & - x + 1 \\
0 & - x + 1 & - ( x - 1 ) ( x - 2 )
\end{bmatrix} \\
& \to \begin{bmatrix}
1 & 0 & 0 \\
0 & x - 1 & - x + 1 \\
0 & - x + 1 & - ( x - 1 ) ( x - 2 )
\end{bmatrix} \\
& \to \begin{bmatrix}
1 & 0 & 0 \\
0 & x - 1 & - x + 1 \\
0 & 0 & - ( x - 1 )^2
\end{bmatrix} \\
& \to \begin{bmatrix}
1 & 0 & 0 \\
0 & x - 1 & 0 \\
0 & 0 & - ( x - 1 )^2
\end{bmatrix} \\

\end{aligned}
$$

这样我们就得到了不变因子,得到$T \sim \begin{bmatrix}1 & & \\ & 0 & - 1 \\ & 1 & 2\end{bmatrix}$.

#### Jordan标准型
对环上元素$r \in R$,如果$\exists d \geq 1 , r^d = 0$,我们称它是**幂零的**.而最小的$d \geq 1$满足$r^d$称为$r$的**幂零指数**.回忆到我们定义过广义特征子空间$V_{ [ \lambda ] } = \ker ( T - \lambda )^\infty$.回忆到$\mathrm{ End } ( V )$是一个环,$T \in \mathrm{ End } ( V )$是其中的一个元素,我们说以下命题等价:

1. $T \in \mathrm{ End } ( V )$是幂零的.

2. $\exists k , \mathrm{ Min }_T = x^k$.

3. $\mathrm{ Char }_T = x^n$.

4. $V = V_{ [ 0 ] }$.

(1)$\Rightarrow$(2)只需考虑如果$T^k = 0$,则$\mathrm{ Min }_T | x^k$.

(2)$\Rightarrow$(3)是显然的,因为极小多项式和特征多项式共根.

(3)$\Rightarrow$(4)直接是定义.

(4)$\Rightarrow$(1),考虑$V_{ [ 0 ] }$作为子空间,扩张一定是有限次的,所以也就搞定.

接下来我们定义上三角块$J_d ( \lambda ) = \begin{bmatrix}\lambda & 1 & & \\ & \ddots & \ddots & \\ & & \lambda & 1 \\ & & & \lambda\end{bmatrix}$以及下三角块$( J_d )^t ( \lambda )$.容易见到$J_d ( \lambda ) = \lambda I + J_d ( 0 )$,我们还可以发现下三角块$( J_d )^t ( 0 )$恰好是多项式$x^d$的友矩阵.并且容易见到对于$( J_d )^t ( \lambda )$而言,其$\mathrm{ Min } = \mathrm{ Char } = ( x - \lambda )^d$.同理取其转置可以知道$J_d ( \lambda )$亦然有$\mathrm{ Min } = \mathrm{ Char } = ( x - \lambda )^d$.

接下来我们证明,如果$A$幂零,则存在唯一的正整数列$1 \leq b_1 \leq \cdots \leq b_r$使得$\sum_{ i = 1 }^r b_i = n$而$A$共轭于分块对角矩阵$\begin{bmatrix}J_{ b_1 } ( 0 ) & & \\ & \ddots & \\ & & J_{ b_r } ( 0 )\end{bmatrix}$,并且如果上述陈述中的$J_{ b_i }$全换成下三角$( J_{ b_i } )^t$,结论亦然成立.

这个其实是显然的,因为此时直接取其有理标准型满足$f_1 | \cdots | f_r$,并且$f_r = x^d$,设$f_i = x^{ b_i }$,自然得到下三角的结论,转置后得到上三角的结论.

接下来假设特征多项式在$F$上分裂(这当然是必要的,因为Jordan标准型是上三角矩阵,而一个矩阵可上三角化的充要条件是特征多项式在$F$上分裂),我们设其相异根为$\lambda_1 , \cdots , \lambda_m \in F$,存在$V$的有序基,使得$T$表为分块对角矩阵$diag ( A_1 , \cdots , A_m )$,其中$A_j = \begin{bmatrix}J_{ b_1 , j } ( \lambda_j ) & & \\ & \ddots & \\ & & J_{ b_{ r_j } , j } ( \lambda_j )\end{bmatrix}$.而且每个$j$对应地正整数数列由$T$唯一确定,上述矩阵被称为$T$的Jordan标准型.

证法呼之欲出,取对$T$的广义特征子空间分解$V = V_{ [ \lambda_1 ] } \oplus \cdots \oplus V_{ [ \lambda_m ] }$,限制在$V_{ \lambda_j }$上,$T_j - \lambda_j$幂零,根据上面的结论就做完了.

现在的问题可能在于如何计算Jordan标准型,一种方法是途径有理标准型,另一种方法是依赖于秩的计算.让我们依旧从幂零情形入手,假设$T$幂零,我们断言:

1. 在$T$的Jordan标准型中,Jordan块的总数为$n - \mathrm{ rk } ( T )$个.

2. 对于每个$d \geq 1$,标准型中的$d \times d$Jordan块的个数$N ( d )$满足:$N ( d ) = \mathrm{ rk } ( T^{ d + 1 } ) + \mathrm{ rk } ( T^{ d - 1 } ) - 2 \mathrm{ rk } ( T^d )$.

(1)是显然的,因为一个$d \times d$的Jordan块的秩为$d - 1$.

而容易见到$\mathrm{ rk } ( J_b ( 0 )^k ) = \max ( 0 , b - k )$,从而:

$$
\begin{aligned}
\mathrm{ rk } ( T^{ d + 1 } ) - \mathrm{ rk } ( T^d ) & = \sum_{ j : b_j \geq d + 1 } ( - 1 ) \\
\mathrm{ rk } ( T^{ d } ) - \mathrm{ rk } ( T^{ d - 1 } ) & = \sum_{ j : b_j \geq d } ( - 1 ) \\

\end{aligned}
$$

两式相减即可得到答案.

至于对其余情形,只需观察到当$\lambda_i \ne \lambda_j$的时候,$A_i - \lambda_j I$肯定可逆,因此:

$$
N_j ( d ) = \mathrm{ rk } ( T - \lambda_j I )^{ d + 1 } + \mathrm{ rk } ( T - \lambda_j I )^{ d - 1 } - 2 \mathrm{ rk } ( T - \lambda_j I )^d
$$

便数平凡.

可以见到Jordan标准型的每一个块,如果特征值为$0$,实际上是把基向前挪动.

##### 加性Jordan-Chevalley分解
由上述可以见到,对于一个特征多项式可分裂的矩阵$T$,存在唯一一个可对角化矩阵$S$和一个幂零矩阵$N$使得$T = S + N$而且$SN = NS$.

由Jordan标准型知道的确存在解(因为每一个分块对角上的对角矩阵都是$\lambda I$的形式),现在问题在于唯一性.

既然$S$可对角化,我们取其相异的特征值$\mu_1 , \cdots , \mu_l$.对其做特征子空间分解$V = V_{ \mu_1 } \oplus \cdots \oplus V_{ \mu_l }$,回忆到$NS = SN$导出$ST = TS$,这意味着每一个特征子空间都是$T$不变的.

此时观察$T |_{ V_j }$,观察到此时$( T - \mu_j I ) |_{ V_j } = N |_{ V_j }$是幂零的,所以$V_j$被包含于$T$的广义特征子空间$V ' [ \mu_j ]$,然而广义特征子空间必然也给出了一个空间的分解,因此它们也不可能再往外扩张了,这必然意味着$S$的特征空间$V_j$恰好就是$T$的广义特征子空间$V ' [ \mu_j ]$.所以$S$限定在$V ' [ \mu_j ]$上就是$\mu_j I$,这就保证了$S$是唯一的,从而$N$也就唯一.

此外,我们还可以证明存在多项式$f , g$使得$S = f ( T ) , N = g ( T )$.首先可以取$g = x - f$,下面来搞定$f$是如何取定的.

考虑用中国剩余定理拿到一个$f$满足$f \equiv \mu_i \pmod{ ( x - \mu_i )^n }$,留神到$f ( T ) |_{ V_j } = \mu_j$,原因是$( T - \mu_j )^n |_{ V_j } = 0$,那按照我们刚才对唯一性的论证,这必然说明了$f ( T ) = S$.

##### 乘性Jordan-Chevalley分解
对于一个特征多项式可分裂的可逆(等价于特征多项式无零根)的$T \in \mathrm{ End } ( V )$,则存在唯一一对可逆的$S , U$使得:

1. $S$可对角化.

2. $U - I$幂零.

3. $SU = T = US$.

4. 存在多项式$f , g$,$S = f ( T ) , U = g ( T )$.

对于存在性,取道加性分解,设$T = S + N = S ( I + S^{ - 1 } N )$,直接取$U = I + S^{ - 1 } N$,由于$SN = NS$,则$U - I = S^{ - 1 } N$因为$N$幂零而幂零,同时$SU = T = US$也满足了.

对于唯一性,假设存在$T = SU = S + S ( U - I )$,由于$SU = US$,所以$S ( U - I )$幂零,这就给出了一个加性分解,从而导出了唯一性.

最后来看如何取多项式.加性版本给出了$S = f ( T )$.要求出$g$的话需要满足$g ( T ) f ( T ) = T$.

于此考虑证明$f$和$\mathrm{ char }_T$是互素的,原因是反证法:如果不互素,则存在$T$的特征值$\lambda$使得$( x - \lambda ) | f$,则对于对应的特征向量$\vec{ v }$总有$S \vec{ v } = f ( T ) \vec{ v } = 0$,可是$S$可对角化而且特征值无$0$,这就矛盾了.

既然如此,用裴蜀定理立刻拿到一对$g , h \in F [ x ]$使得$gf + h \mathrm{ char }_T = x$,带入$T$得到$g ( T ) f ( T ) = T$,这个$g ( T ) = TS^{ - 1 } = U$.

### 张量积
给定两个$F$上的向量空间$V , W$,我们想要通过它们构造新的向量空间$V \otimes W$(这里并非直积,只是用这个符号).我们希望这个配对满足:

1. $V \times W \to V \otimes W$是双线性的.

2. 构造应该是自然的:不依赖基的选取.

此外,我们希望其满足一些更好的泛性质,考虑资料$( L , B )$,其中$L$是一个向量空间,$B$是一个$V \times W \to L$的一个双线性映射,我们希望有一个尽可能泛的资料$( L_{ \mathrm{ univ } } , B_{ \mathrm{ univ } } )$,使得对于任何满足条件的$( L , B )$,存在唯一的$\varphi$以下图表交换:

$$
\xymatrix{ V \times W \ar[r]^{ B_{ \mathrm{ univ } } } \ar[dr]_B & L_{ \mathrm{ univ } } \ar[d]^\varphi \\
& L }
$$

此外我们还希望上述资料$( L_{ \mathrm{ univ } } , B_{ \mathrm{ univ } } )$是唯一的,我们希望将其记作$V \otimes W$.

我们可以先尝试构造一下,用最粗暴的方法,干脆定义$E = F^{ \oplus ( V \times W ) }$,其元素表为有限线性组合$\sum_i c_i ( v_i , w_i )$,考虑以下元素生成的子空间$N$:

1. $( v + v ' , w ) - ( v , w ) - ( v ' , w )$.

2. $( v , w + w ' ) - ( v , w ) - ( v , w ' )$.

3. $( tv , w ) - t ( v , w )$.

4. $( v , tw ) - t ( v , w )$.

接下来取$L_{ \mathrm{ univ } } = E / N$,连同映射$B_{ \mathrm{ univ } } : ( v , w ) \mapsto ( v , w ) + N$,容易见到我们的确强行定义了双线性形式的性质.

接下来考虑其泛性质,对双线性映射$B : V \times W \to L$,其可以确定唯一的线性映射$\Phi : E \to L , \sum_i c_i ( v_i , w_i ) \mapsto \sum_i c_i B ( v_i , w_i )$.

然而由于$B$的双线性性,立刻得到$B ( N ) = \{ 0 \}$,从而可以导出唯一的$\varphi : L_{ \mathrm{ univ } } \to L , x + N \mapsto \Phi ( x )$.这立刻得到了$\varphi$的唯一性,原因是$\varphi ( ( v , w ) + N ) = B ( v , w )$是总有的.

唯一性应当是显然的,只需见到如果有两个$L_{ \mathrm{ univ } }$和$L_{ \mathrm{ univ } } '$,则应当有:

$$
\xymatrix{ & L_{ \mathrm{ univ } } \ar[d]^\varphi \\
V \times W \ar[ru]^{ B_{ \mathrm{ univ } } } \ar[r]^{ B_{ \mathrm{ univ } } ' } \ar[dr]_{ B_{ \mathrm{ univ } } } & L_{ \mathrm{ univ } } ' \ar[d]^\psi \\
& L_{ \mathrm{ univ } } }
$$

外圈当然也是交换的,然而取$id$也必然导出外圈交换,由图表唯一性立刻得到$\varphi \psi = id$,对称性得出$\psi \varphi = id$,这就搞定了$L_{ \mathrm{ univ } } \cong L_{ \mathrm{ univ } } '$.

容易见到上述性质保证了同构:$\mathrm{ Hom } ( V \otimes W , L ) \cong \mathrm{ Bil } ( V , W ; L ) , \varphi \mapsto \varphi B_{ \mathrm{ univ } }$.

容易将上述结论推广到多重线性映射的情形.还可以将张量积的符号推广到线性映射上,我们断言总存在唯一的$\varphi$使得下图交换:

$$
\xymatrix{ V_1 \times \cdots \times V_n \ar[r]^{ ( f_1 , \cdots , f_n ) } \ar[d] & W_1 \times \cdots \times W_n \ar[d] \\
V_1 \otimes \cdots \otimes V_n \ar[r]_{ \varphi } & W_1 \otimes \cdots \otimes W_n \\
 }
$$

从而可以自然地将$\varphi$记作$f_1 \otimes \cdots \otimes f_n$.

于此只需考虑上半部分,我们知道$( v_1 , \cdots , v_n ) \mapsto f_1 ( v_1 ) \otimes \cdots \otimes f_n ( v_n )$是一个多重线性映射,所以当然存在唯一一个映射打过来.从而得到:

$$
( f_1 \otimes \cdots \otimes f_n ) ( v_1 \otimes \cdots \otimes v_n ) = f_1 ( v_1 ) \otimes \cdots \otimes f_n ( v_n )
$$

其实我们上面证明的就是下面这个同构$\bigotimes_{ i = 1 }^n \mathrm{ Hom } ( V_i , W_i ) \cong \mathrm{ Hom } ( \bigotimes_i V_i , \bigotimes_i W_i )$,好吧起码这个有限维情况下是同构,无穷维的话lww好像也有点不确定.

有以下性质自然成立(另一种看法是,线性映射也在一个线性空间里,所以上面也会自带一种张量积,恰为这个):

1. $id_{ V_1 } \otimes \cdots \otimes id_{ V_n } = id_{ V_1 \otimes \cdots \otimes V_n }$.

2. $( f_1 \otimes \cdots \otimes f_n ) ( g_1 \otimes \cdots \otimes g_n ) = f_1 g_1 \otimes \cdots \otimes f_n g_n$.

3. $( ( \alpha f_1 + \alpha ' f_1 ' ) \otimes \cdots \otimes f_n ) = \alpha ( f_1 \otimes \cdots \otimes f_n ) + \alpha ' ( f_1 ' \otimes \cdots \otimes f_n )$.

而考虑多重线性映射的$\mathrm{ Mul } ( V_1 , \cdots , V_n ; M )$,如果$V_1 , \cdots , V_n$中任意一个是零空间,那由于多重线性性质,立刻导出这个映射只有零映射(原因是此时$a = a + a$),从而使得$\mathrm{ Hom } ( V_1 \otimes \cdots \otimes V_n ; M )$中只有零映射(对所有$M$),那其$id$映射也是零映射,自然意味着$V_1 \otimes \cdots \otimes V_n$是零空间.

于此之外,如果想要跳过张量积的构造而直接探究张量积的性质,可能需要先判断$\{ v_1 \otimes \cdots \otimes v_n \}$集合是否的确生成了$V_1 \otimes \cdots \otimes V_n$.于此只需要取$K = \langle v_1 \otimes \cdots \otimes v_n | \forall v_i \in V_i \rangle$,然后判断$M = ( V_1 \otimes \cdots \otimes V_n ) / K$是否为零空间即可.此时$\forall \varphi$作为商映射,当然都有$\varphi ( v_1 \otimes \cdots \otimes v_n ) = 0$,我们想说明$\varphi$的确是零映射,这首先需要保证即使只有$v_1 \otimes \cdots \otimes v_n$上的限制也可以有类似外延公理的结论.

换言之,如果两个在$V_1 \otimes \cdots \otimes V_n$映射$f , g$满足$\forall v_i , f ( v_1 \otimes \cdots \otimes v_n ) = g ( v_1 \otimes \cdots \otimes v_n )$则$f \equiv g$,只需回忆到这上面的映射与多重线性映射一一对应,而多重线性映射上用外延公理就搞定了.

这就看到,用与多重线性映射一一对应的性质同样是张量积的某种泛性质.

我们还可以看到,如果$f_1 , \cdots , f_n$都是满的,则$f_1 \otimes \cdots \otimes f_n$亦然,这是显然的.

可能稍不显然的是如果$f_1 , \cdots , f_n$都是单的,则$f_1 \otimes \cdots \otimes f_n$也是单的.此时$W_i$可以分解出一个与$V_i$同构的$\mathrm{ im } f_i$部分,干脆记作$W_i \cong V_i \oplus V_i '$,并假设$f_i : V_i \to W_i$是自然的嵌入映射.回忆到张量积在同构下有交换律和对直和的分配律(下面会证),见到$W_1 \otimes \cdots \otimes W_n \cong ( V_1 \otimes \cdots \otimes V_n ) \oplus V '$,此时$f_1 \otimes \cdots \otimes f_n$就是自然的嵌入映射.你可能会想欸,不对啊,$V '$里也会有一些$V_i$项啊.可是,回忆到如果某一分量为零,则其在张量积中拿到的结果就是$0$,所以$f_1 \otimes \cdots \otimes f_n$只会对前面有意义.

#### 张量积与直和
接下来考虑证明以下结论:

1. 结合律:$V_1 \otimes ( V_2 \otimes V_3 ) \cong V_1 \otimes V_2 \otimes V_3 \cong ( V_1 \otimes V_2 ) \otimes V_3$.

2. 幺元:$F \otimes V \cong V \cong V \otimes F$.

3. 交换律:$V \otimes W \cong W \otimes V$.

4. 零元:$\{ 0 \} \otimes V \cong \{ 0 \} \cong V \otimes \{ 0 \}$.

5. 分配律:设$V$带有直和分解$V = \bigoplus_{ i \in I } V_i$,则$V \otimes W \cong \bigoplus_{ i \in I } ( V_i \otimes W )$.

考虑(1),下面证明$V_1 \otimes ( V_2 \otimes V_3 ) \cong V_1 \otimes V_2 \otimes V_3$,其关键是构造以下线性映射:

$$
\xymatrix{ V_1 \otimes ( V_2 \otimes V_3 ) \ar[r]^{ \alpha } & V_1 \otimes V_2 \otimes V_3 \ar[r]^{ \beta } & V_1 \otimes ( V_2 \otimes V_3 ) \\
v_1 \otimes ( v_2 \otimes v_3 ) \ar @{|->}[r] & v_1 \otimes v_2 \otimes v_3 \ar @{|->}[r] & v_1 \otimes ( v_2 \otimes v_3 ) }
$$

如果我们的确造出了这个映射,由于我们有外延公理,这必然意味着$\alpha \circ \beta = id$,从而导出$V_1 \otimes ( V_2 \otimes V_3 ) \cong V_1 \otimes V_2 \otimes V_3$.

对于$\beta$,立刻就能看出$V_1 \times V_2 \times V_3 \to V_1 \otimes ( V_2 \otimes V_3 )$是三重线性的.因此泛性质诱导出$\beta$映射.

至于$\alpha$映射,只需先做$V_1 \times ( V_2 \times V_3 ) \cong V_1 \times V_2 \times V_3$就行.

考虑(2),和(1)一样,我们想要搞一个映射链$V \to F \otimes V \to V$,对于前者只需要取映射$\lambda_V : F \otimes V \to V , t \otimes v \mapsto tv$,对于后者只需要取$\tau_V : V \to F \otimes V , v \mapsto 1 \otimes v$.回忆到$1 \otimes tv = t \otimes v$,因此上述映射均是合理的,而且复合之后是$id$.

考虑(3),我想答案呼之欲出:$V \otimes W$和$W \otimes V$都是$V \times W$诱导出来的,而$V \times W \cong W \times V$.

(4)我们之前已经证明了.

考虑(5),回忆到如果$V = \bigoplus_{ i \in I } V_i$,则$V^\vee = \prod_{ i \in I } ( V_i )^\vee$.同理应该能拿到$\mathrm{ Hom } ( V , L ) = \prod_{ i \in I } \mathrm{ Hom } ( V_i , L )$,进一步地有:

$$
\mathrm{ Bil } ( V , W ; L ) \cong \prod_{ i \in I } \mathrm{ Bil } ( V_i , W ; L ) \cong \prod_{ i \in I } \mathrm{ Hom } ( V_i \otimes W , L ) \cong \mathrm{ Hom } \left ( \bigoplus_{ i \in I } ( V_i \otimes W ) , L \right )
$$

从而可以使我们诱导出原本的同构.

基于(5),我们可以知道如果$V$有基$( v_i )_{ i \in I }$而$W$有基$( w_j )_{ j \in J }$,则$( v_i \otimes w_j )_{ ( i , j ) \in I \times J }$也是$V \otimes W$的基,从而得到$\dim ( V \otimes W ) = ( \dim V ) \times ( \dim W )$.这给出了张量积的Kronecker基形式.

最后我们还可以引入张量幂$V^{ \otimes n }$,从上面的讨论我们可以知道$( V^{ \otimes a } )^{ \otimes b } \cong V^{ \otimes ab }$以及$V^{ \otimes a } \otimes V^{ \otimes b } \cong V^{ \otimes ( a + b ) }$.

#### Kronecker积
对于矩阵$X \in F^{ n \times m } , Y \in F^{ p \times q }$,我们定义它们的克罗内多积为:

$$
X \otimes Y = \begin{bmatrix}
x_{ 1 , 1 } Y & x_{ 1 , 2 } Y & \cdots & x_{ 1 , m } Y \\
x_{ 2 , 1 } Y & x_{ 2 , 2 } Y & \cdots & x_{ 2 , m } Y \\
\vdots & \vdots & \ddots & \vdots \\
x_{ n , 1 } Y & x_{ n , 2 } Y & \cdots & x_{ n , m } Y
\end{bmatrix} \in F^{ ( np ) \times ( mq ) }
$$

克罗内多积显然不满足交换律.

事实上,设$X , Y , Z$在下列运算中有意义,克罗内多积满足以下性质:

1. 结合律:$X \otimes Y \otimes Z = X \otimes ( Y \otimes Z )$.

2. 左分配律:$X \otimes ( Y + Z ) = X \otimes Y + X \otimes Z$.

3. 右分配律:$( X + Y ) \otimes Z = X \otimes Z + Y \otimes Z$.

4. $( X \otimes Y )^T = X^T \otimes Y^T$.

5. $( X \otimes Y ) ( U \otimes V ) = ( XU ) \otimes ( YV )$.

6. $X^{ - 1 } \otimes Y^{ - 1 } = ( X \otimes Y )^{ - 1 }$.

7. $\det ( X \otimes Y ) = \det ( X )^m \det ( Y )^n , X \in F^{ n \times n } , Y \in F^{ m \times m }$.

8. $rank ( X \otimes Y ) = rank ( X ) \times rank ( Y )$.

上述命题稍不显然的是(4),我们将在下面讨论对偶空间的时候得出它.

克罗内多积的性质有一个很有用的特例是:

$$
\begin{aligned}
A \otimes B \otimes C & = ( A \otimes I_n \otimes I_n ) ( I_n \otimes B \otimes I_n ) ( I_n \otimes I_n \otimes C ) \\
A , B , C & \in F^{ n \times n }
\end{aligned}
$$

#### 张量积与对偶空间
回忆到典范配对,如今我们可以用张量积将其表示为线性映射$V^\vee \otimes V \to F , \lambda \otimes v \mapsto \langle \lambda , v \rangle = \lambda ( v )$,这被称为**缩并**映射.

考虑映射$\Theta : V^\vee \otimes W \to \mathrm{ Hom } ( V , W ) , \sum_i \lambda_i \otimes w_i \mapsto \sum_i \langle \lambda_i , \_ \rangle w_i$.这个映射总是单射,当维数是有限维的时候则是满的.

也许还该简单验证此结论,首先检查$V^\vee \times W \to \mathrm{ Hom } ( V , W )$,这的确是一个双线性映射,于是由泛性质立刻诱导出映射$V^\vee \otimes W \to \mathrm{ Hom } ( V , W )$.

接下来证明其是单射,只需要证明其$\ker = \{ 0 \}$即可.而其$\ker$中的元素总形如$\sum_{ i = 1 }^k \lambda_i \otimes w_i$,不妨假设$w_1 , \cdots , w_k$是线性无关的(可以把相关的项整理到$\lambda_i$里面).

此时由于其在$\ker$里,所以$\forall v \in V$,总有$\sum_{ i = 1 }^k \langle \lambda_i , v \rangle w_i = 0$,然而$w_i$线性无关,所以$\langle \lambda_i , v \rangle \equiv 0$,可是$v$是任取的,这必然意味着$\lambda_i \equiv 0$.

接下来考虑证明当$V , W$其一是有限维的时候上述映射是满的,容易发现$\Theta$的像总是有限维的(因为左边是有限求和).当$V , W$其一是有限维的,$T \in \mathrm{ Hom } ( V , W )$就是有限秩的,此时就可以取其像的基$w_1 , \cdots , w_n$,对所有$v \in V$,$T ( v )$都可以唯一表述为$c_1 w_1 + \cdots + c_n w_n$,然而这里面每一个$c_i$对$v$都是线性的,从而它们都可以表示为$V^\vee$中的元素而与$v$的选取无关,这就搞定了.

万事俱备,再看$\mathrm{ End } ( V )$上,既然$V^\vee \otimes V$带有一个典范的缩并映射,在有限维的情况下,那与之同构的$\mathrm{ End } ( V )$也该带有一个到$F$的映射.我们断言这个映射就是$\mathrm{ Tr }$映射.有下述图表:

$$
\xymatrix{ V^\vee \otimes V \ar[rd]_{ \text{ contraction } } \ar[rr]^{ \sim } & & \mathrm{ End ( V ) } \ar[ld]^{ \mathrm{ Trace } } \\
& F & }
$$

如何检验这个结论,我们之前刻画Trace的办法只有取对偶基$\check{ v }_1 , \cdots , \check{ v }_n$,然后将$v = \sum c_i v_i$和$\lambda = d_i \check{ v }_i$,那典范配对$\Theta ( \lambda \otimes v ) = \sum_{ i , j } \langle \check{ v }_i , \_ \rangle v_j$,也就是说其将基向量$v_i$映射到了$\sum_j c_j d_i$,从而见到其$\mathrm{ Tr } = \sum_i c_i d_i = \langle \lambda , v \rangle$.

如果写成矩阵形式的话,$\lambda \in V^\vee$可以写成一个$1 \times n$的矩阵,$w \in W$可以写成一个$m \times 1$的矩阵,那其实$\Theta ( \lambda \otimes w )$就是$w \lambda$这个$m \times n$的矩阵.

最后来看曾经想要拿到的那个结论,下面我们引入一个典范的同构$\Psi : V_1^\vee \otimes \cdots \otimes V_n^\vee \to ( V_1 \otimes \cdots \otimes V_n )^\vee$,策略是观察$V_1^\vee \times \cdots \times V_n^\vee \to ( V_1 \otimes \cdots \otimes V_n )^\vee$自然是一个线性映射,其映射自然是:

$$
( \check{ v }_1 , \cdots , \check{ v }_n ) \mapsto \left [ v_1 \otimes \cdots \otimes v_n \mapsto \prod_{ i = 1 }^n \langle \check{ v }_i , v_i \rangle \right ]
$$

我们希望当每个$V_i$都是有限维的时候,上述映射是同构.由于两遍维数相同,所以单性蕴含着满性.而其单性只要取左侧$V_1^\vee \otimes \cdots \otimes V_n^\vee$的基$\check{ v_{ 1 , i_1 } } \otimes \cdots \otimes \check{ v_{ n , i_n } }$,将每个$\lambda$按照这组基展开,$\psi ( \lambda )$在$v_{ 1 , i_1 } \otimes \cdots \otimes v_{ n , i_n }$处的值就是该基在$\lambda$中的系数.因此$\ker = \{ 0 \}$导出单性.

#### 张量代数
设$F$是一个域,一个$F$上的**代数**需要以下资料:

1. 一个环$A$,其需要带有$F$向量空间的结构,使得环的加法等价于向量空间的加法.

2. 环的乘法$A \times A \to A$需要是双线性的.

请沿之前引入代数结构的常规讨论把代数的结构一块搞定,其实大部分在环那里已经做完了,代数结构更强,因此只需要限定同态必须也态射到代数就行.也许还需要检验理想对标量乘法封闭,只需要观察到$tI = ( t 1_A ) I \subseteq I$就行.

对于$F$向量空间$V$,下面定义$T ( V ) = \bigoplus_{ n \geq 0 } V^{ \otimes n }$,其自然带有向量空间的结构,只需加上双线性乘法就能构成一个代数,策略是:

$$
\begin{aligned}
V^{ \otimes a } \otimes V^{ \otimes b } & \to V^{ \otimes ( a + b ) } \\
( v_1 \otimes \cdots \otimes v_a ) \otimes ( v_1 ' \otimes \cdots \otimes v_b ' ) & \mapsto v_1 \otimes \cdots \otimes v_a \otimes v_1 ' \otimes \cdots \otimes v_b '
\end{aligned}
$$

它的幺元自然选作$1 \in F = V^{ \otimes 0 }$.我们还可以自然地在上面定义$\psi^{ \otimes a }$之类的结构,不再赘述.

来进入对称代数与外代数,考虑$C \in \mathrm{ Mul } ( V , \cdots , V ; M )$,任意置换$\sigma$,定义:

1. 如果$C ( \cdots , x , y , \cdots ) = C ( \cdots , y , x , \cdots )$,则称其为**对称的**.对称性等价于说$C ( v_1 , \cdots , v_n ) = C ( v_{ \sigma ( 1 ) } , \cdots , v_{ \sigma ( n ) } )$.

2. 如果说$C ( v_1 , \cdots , v_n ) = \mathrm{ sgn } ( \sigma ) C ( v_{ \sigma ( 1 ) } , \cdots , v_{ \sigma ( n ) } )$,则称其为**反对称的**.

3. 如果$C ( \cdots , x , x , \cdots ) = 0$,则称其为**交错的**.

当$\mathrm{ char } F \ne 2$的时候,交错和反对称是等价的.不然,起码交错的一定是反对称的.下面定义:

1. $I_{ Sym }$为形如$x \otimes y - y \otimes x$的元素生成的理想.

2. $I_{ \wedge }$为形如$x \otimes x$生成的理想.

这样就可以定义相应的**对称代数**$Sym ( V ) = T ( V ) / I_{ Sym }$,**外代数**$\bigwedge ( V ) = T ( V ) / I_\wedge$.还可以定义$Sym^m ( V ) = V^{ \otimes m } / I_{ \mathrm{ sym } }^m$,同理定义$\bigwedge^m ( V )$,容易发现$Sym^0 ( V ) = F = \bigwedge^0 ( V ) , Sym^1 ( V ) = V = \bigwedge^1 ( V )$.

在其中的$\otimes$运算被自然变成了新的运算,设前者仍是通常的乘法符号,而后者符号采取$\wedge$.

外代数里一个重要的性质是如果$\omega \in \bigwedge^p ( V ) , \eta \in \bigwedge^q ( V )$,则$\omega \eta = ( - 1 )^{ pq } \eta \omega$,原因是只需要假设$\omega = x_1 \wedge \cdots \wedge x_p , \eta = y_1 \wedge \cdots \wedge y_q$,其中$x_i , y_i \in V$,而外代数的定义已经给出$x \wedge y = - y \wedge x$,这就完事了.

既然我们可以将一个对称代数或者外代数分成若干层,那考虑对于任意线性映射$\psi : V \to W$,我们都可以通过在各个分量上诱导从而拿到:$Sym ( \psi ) : Sym ( V ) \to Sym ( W ) , x_1 \cdots x_m \mapsto \psi ( x_1 ) \cdots \psi ( x_m )$,其中$\psi ( x_m )$是限制在$V^{ \otimes m }$上的.这应该是平凡的.

最后我们想要说明对称代数与外代数确实对应着我们的多重线性映射.也就是$\mathrm{ Hom } ( Sym^m , M )$同构于所有的对称$m$重线性映射.外代数类似.而这些依照理想的定义的确如此.

接下来来展现二者的结构,不妨设$\dim V = n$,取其一组基$v_1 , \cdots v_n$,我们说:

1. 当$m > n$的时候,$\bigwedge^m ( V ) = \{ 0 \}$.

2. 当$0 \leq m \leq n$的时候,$\dim \bigwedge^m ( V ) = \binom{ n }{ m }$.更确切地说,任意一组下标$1 \leq i_1 < \cdots < i_m \leq n$,$v_{ i_1 } \wedge \cdots \wedge v_{ i_m }$构成$\bigwedge^m ( V )$的基.

3. 作为(2)的推论,$\dim \bigwedge ( V ) = 2^n$.

(1)比较显然,因为当$m > n$的时候,任何一个元素被生成的方式$v '_{ 1 } \wedge \cdots \wedge v '_{ m }$一定线性相关,所以自然为$0$.

接下来看(2),我们早在行列式的讨论处就证明过了当$m = n$的时候,交错形式是一维的,那当然导出$\dim ( \bigwedge^n ( V ) ) = 1$.而且若$\psi \in \mathrm{ End } ( V )$,则当然有$\bigwedge^n ( \psi ) = det ( \psi ) \mathrm{ id }_{ \wedge^n ( V ) }$.

而由之前张量积的结论,$v_{ i_1 } \wedge \cdots \wedge v_{ i_m }$这些元素肯定构成了一组生成元,只需要证明它们的确线性无关即可.只需证明:

$$
\sum_{ 1 \leq j_1 < \cdots < j_m \leq n } c_{ j_1 , \cdots , j_m } ( v_{ j_1 } \wedge \cdots \wedge v_{ j_m } ) = 0 \Rightarrow c_{ j_1 , \cdots , j_m } \equiv 0
$$

做法呼之欲出,取$V ' = \langle v_{ i_1 , } , \cdots , v_{ i_m } \rangle$,定义$\psi : V \to V '$,方法是$\psi ( v_j ) = \begin{cases}v_j & j \in \{ i_1 , \cdots , i_m \} \\ 0 & \text{ otherwise }\end{cases}$.此时其诱导出了$\bigwedge^m ( \psi ) : \bigwedge^m ( V ) \to \bigwedge^m ( V ' )$.然后两边做映射.如果$\{ j_1 , \cdots , j_m \} \ne \{ i_1 , \cdots , i_m \}$那就会变成零,最后就会导出$c_{ i_1 , \cdots , i_m } ( v_{ i_1 } \wedge \cdots \wedge v_{ i_m } ) = 0$,然而左边是在$\bigwedge^m ( V ' )$这个$\dim = 1$的空间中,所以一定能导出$c_{ i_1 , \cdots , i_m } = 0$.

上述是外代数的结构,我们还可以刻画对称代数的结构,可以证明如果$\dim V = n$,取定$V$的基$v_1 , \cdots , v_n$,可以将$Sym ( V ) \cong F [ X_1 , \cdots , X_n ]$.其实就是二次型理论对吧,应该比较简单.

最后我们可以来定义交错矩阵:$A \in M_{ n \times n } ( R )$,若$a_{ i , j } = - a_{ j , i }$而且$a_{ i , i } = 0$.在$2 \in R^\times$的交换环上,这等价于$A^t = - A$.

可以证明当$A \in M_{ n \times n } ( F )$,对应了双线性形式$B \in \mathrm{ Bil } ( F^n , F^n ; F )$为$B ( \vec{ x } , \vec{ y } ) = ( \vec{ x } )^t A \vec{ y }$.则$B$是交错形式当且仅当$A$是交错矩阵.必要性很显然,只需带入$a_{ i , j } = ( \vec{ e }_i )^t A \vec{ e }_j = B ( \vec{ e }_i , \vec{ e }_j )$;充分性也很显然,考察$B ( \vec{ x } , \vec{ x } ) = ( \vec{ x } )^t A \vec{ x } = 0$即可.

最后来看特征多项式如何表为外代数形式.考虑$V$是域$F$上的$n$维向量空间,$T \in \mathrm{ End } ( V )$,则我们断言:

$$
\det ( \lambda I - T ) = \sum_{ k = 0 }^n ( - 1 )^k \mathrm{ Tr } ( \bigwedge_k T ) \lambda^{ n - k }
$$

我们之前已经解释过了:

$$
\begin{aligned}
& \det ( \lambda I - T ) e_1 \wedge \cdots \wedge e_n \\
= & ( \lambda I - T ) e_1 \wedge \cdots \wedge ( \lambda I - T ) e_n \\
= & \sum_{ k = 0 }^n ( - 1 )^k \lambda^{ n - k } \sum_{ | I | = k } e_1 \wedge \cdots Te_{ i_1 } \cdots \wedge e_n
\end{aligned}
$$

也就是设只有$i_k \in I$这个指标集选到了$Te$而其它的还是$e$,不妨设$J = \{ 1 , \cdots , n \} \setminus I$.现在考虑$\{ e_{ i_1 } \wedge \cdots \wedge e_{ i_k } \}_{ I }$会构成$\bigwedge^k ( V )$的一组基,$T$施加到上面得到:

$$
\begin{aligned}
( \bigwedge^k T ) ( e_{ i_1 } \wedge \cdots \wedge e_{ i_k } ) & = Te_{ i_1 } \wedge \cdots \wedge T e_{ i_k } \\

\end{aligned}
$$

对上面这个式子两边做$\wedge ( e_{ j_1 } \wedge \cdots \wedge e_{ j_{ n - k } } )$,现在看右侧,如果$Te_{ i_1 }$变出了一些带有$e_{ j }$的项,就会变成$0$.因此右边最后剩下来的只会是形如$a_I ( e_{ i_1 } \wedge \cdots \wedge e_{ i_k } ) \wedge ( e_{ j_1 } \wedge \cdots \wedge e_{ j_{ n - k } } )$的东西.两边再换回去,得到:

$$
\begin{aligned}
& \det ( \lambda I - T ) e_1 \wedge \cdots \wedge e_n \\
= & \sum_{ k = 0 }^n ( - 1 )^k \lambda^{ n - k } \sum_{ | I | = k } a_I
\end{aligned}
$$

而最后那个东西恰好是$\mathrm{ Tr } ( \bigwedge^k T )$.

##### 另一种构造
当$\mathrm{ char } ( F ) = 0$的时候,我们可以脱离商空间的结构,而直接将对称代数与外代数定义在$T ( V )$的子空间上.

对称代数与外代数(此时已经等价于反对称代数)都依赖于交换导出的结果,因此首要要做的就是让置换群$S_n$作用域$V^{ \otimes n }$,为了保证左作用,下面的设计可能略有一些不合理,但好在我们后面也不会涉及具体的运算.假设$\sigma \in S_n$,保持:

1. $\sigma ( ax + by ) = a ( \sigma x ) + b ( \sigma y )$.

2. $\sigma ( v_1 \otimes \cdots \otimes v_n ) = v_{ \sigma^{ - 1 } ( 1 ) } \otimes \cdots \otimes v_{ \sigma^{ - 1 } ( n ) }$.

3. $( \sigma \tau ) ( x ) = \sigma ( \tau x )$.

(2)的奇异设计是为了让(3)合理.这个时候你肯定要问:wcnmd,这个为啥不对啊.你别急,我们设$w_i = v_{ \tau_{ - 1 } ( i ) }$,则最后得到的结果是$w_{ \sigma^{ - 1 } ( i ) } = w_{ \tau^{ - 1 } \sigma^{ - 1 } ( i ) } = w_{ ( \sigma \tau )^{ - 1 } ( i ) }$.

原因很简单,因为对于$v_i$这个东西,我们想要把它送到$v_{ \sigma ( i ) }$那里去,这样看就合理了.

现在我们着手取子空间,拿出$V_{ Sym }^{ \otimes n } = \{ x \in V^{ \otimes n } | \forall \sigma \in S_n , \sigma x = x \}$以及$V_{ \wedge }^{ \otimes n } = \{ x \in V^{ \otimes n } | \forall \sigma \in S_n , \sigma x = \mathrm{ sgn } ( \sigma ) x \}$.同理可以施加$\bigoplus_n$从而拿到$T ( V )$的子空间$V_{ Sym }$和$V_\wedge$.

回忆到我们之前定义过商映射$q_{ \mathrm{ sym } } : T ( V ) \to Sym ( V )$和$q_\wedge : T ( V ) \to \bigwedge ( V )$.我们下面断言,如果$n ! \in F^\times$(当$\mathrm{ char } F = 0$的时候自动满足),则上述映射限制在子空间上变为同构$( V^{ \otimes n } )_{ Sym } \cong Sym^n ( V )$和$( V^{ \otimes n } )_{ \wedge } \cong \bigwedge^n ( V )$.

上述两条类似,我们下面只证明$Sym$的情形.定义映射$Avg \in \mathrm{ End } ( V^{ \otimes n } ) , x \mapsto \frac{ 1 }{ n ! } \sum_{ \sigma \in S_n } \sigma x$.留神到$Avg |_{ V_{ \mathrm{ sym } }^{ \otimes n } } = id$,而$\mathrm{ im } ( Avg ) = V_{ \mathrm{ sym } }^{ \otimes n }$.从而我们知道$Avg$事实上给出了一个$V^{ \otimes n } \to V_{ \mathrm{ sym } }^{ \otimes n }$的映射,有同构$V^{ \otimes n } / \ker ( Avg ) \cong V_{ \mathrm{ sym } }^{ \otimes n }$.

来考虑$q_{ \mathrm{ sym } }^n$限制在$V_{ \mathrm{ sym } }^{ \otimes n }$上的结果,记作$\pi$.如果我们想证明$\pi$是满射,其实就是要证明$\ker Avg \subseteq \ker q_{ \mathrm{ sym } }^n$.可是$q_{ \mathrm{ sym } }^n$的定义保证了$q_{ \mathrm{ sym } }^n ( \sigma x ) = q_{ \mathrm{ sym } }^n ( x )$,因此$q_{ \mathrm{ sym } }^n \circ Avg = q_{ \mathrm{ sym } }^n$,导出$\ker Avg \subseteq \ker q_{ \mathrm{ sym } }^n$.

要证明单射则只需要证明$V_{ Sym }^{ \otimes n } \cap \ker q_{ \mathrm{ sym } }^n = \{ 0 \}$.我想这应该是显然的,假设它们的交不为$0$,取出其中之一,由于其在$\ker q_{ \mathrm{ sym } }^n$中,所以它必然可以写成若干个$\cdots ( x \otimes y ) - ( y \otimes x ) \cdots$的组合,而这些都会在$Avg$作用下变成$0$.从而我们证明了上述结论.

不过还没有完,我们想要刻画它的代数结构,还差一个乘法.

考虑取上述映射的逆映射,定义$p_\mathrm{ sym }^n : Sym^n ( V ) \cong V$,我们想要研究一下乘法,断言当$( a + b ) ! \in F^\times$的时候,对所有$x \in ( V^{ \otimes a } )_\mathrm{ sym } , y \in ( V^{ \otimes b } )_\mathrm{ sym }$:

$$
p_\mathrm{ sym }^{ a + b } ( q_\mathrm{ sym }^{ a } ( x ) q_\mathrm{ sym }^{ b } ( y ) ) = \frac{ a ! b ! }{ ( a + b ) ! } \sum_{ \sigma \in S_{ a + b } / ( S_a \times S_b ) } \sigma ( x \otimes y )
$$

对于$\bigwedge$有类似的版本,有:

$$
p_{ \wedge }^{ a + b } ( q_{ \wedge }^{ a } ( x ) q_{ \wedge }^{ b } ( y ) ) = \frac{ a ! b ! }{ ( a + b ) ! } \sum_{ \sigma \in S_{ a + b } / ( S_a \times S_b ) } \mathrm{ sgn } ( \sigma ) \sigma ( x \wedge y )
$$

考虑上述断言的证明,选取$\bigwedge$版本进行证明,首先应该有:

$$
\begin{aligned}
q_{ \wedge }^{ a } ( x ) q_{ \wedge }^{ b } ( y ) & = q_{ \wedge }^{ ab } ( x \wedge y ) \\
& = q_{ \wedge }^{ ab } \left ( \frac{ 1 }{ ( a + b ) ! } \sum_{ \sigma \in S_{ a + b } } \mathrm{ sgn } ( \sigma ) \sigma ( x \wedge y ) \right )
\end{aligned}
$$

考虑$\sigma ( x \wedge y )$,其对于$( \tau , \eta ) \in S_a \times S_b$的时候,考虑:

$$
\begin{aligned}
( \tau , \eta ) ( x \wedge y ) & = \tau ( x ) \wedge \eta ( y ) \\
& = \mathrm{ sgn } ( \tau ) \mathrm{ sgn } ( \eta ) x \wedge y \\
& = \mathrm{ sgn } ( ( \tau , \eta ) ) ( x \wedge y )
\end{aligned}
$$

这就会和前面抵消.所以$S_a \times S_b$可以被商掉,这些元素的数量是$a ! b !$个,从而我们确实知道了它的乘法结构.

至于$S_{ a + b } / ( S_a \times S_b )$,其实就是选出$a + b$中的$a$个元素扔到前面,把剩下的$b$个元素扔到后面,而且不在意它们的顺序.这就是为什么它的大小为$\binom{ a + b }{ a }$.

##### 又看对偶空间
假设$\dim V = n < \infty$,$n ! \in F^\times$.我们断言有以下典范(从左到右)同构成立:

1. $\mathrm{ Sym }^n ( V^\vee ) \cong \mathrm{ Sym }^n ( V )^\vee$.

2. ${ \bigwedge }^n ( V^\vee ) \cong{ \bigwedge }^n ( V )^\vee$.

原因是以下同构:

$$
\xymatrix{ ( V^\vee )^{ \otimes n } \ar[r]^{ \cong }_{ \Psi } & ( V^{ \otimes n } )^\vee \ar[r]^{ \cong }_{ \Phi } & \mathrm{ Mul } ( V , \cdots , V ; F ) \\
\lambda_1 \otimes \cdots \otimes \lambda_n \ar @{|->}[rr] & & [ C ( x_1 , \cdots , x_n ) = \prod_{ i = 1 }^n \langle \lambda_i , x_i \rangle ] }
$$

如果我们在左侧施加$\sigma$后,右侧会如何改变呢?当然是会变成$C ( x_{ \sigma ( 1 ) } , \cdots , x_{ \sigma ( n ) } )$.如果左侧是$( V^\vee )^{ \otimes n }_\mathrm{ sym }$,那右侧就会对应到$\mathrm{ Mul }_\mathrm{ sym } ( V , \cdots , V ; F ) = ( V^{ \otimes n } )^\vee_\mathrm{ sym }$.

###### Example1
设$U , W$分别是$V$的$p$维子空间,$U$有基$x_1 , \cdots , x_p$,$V$有基$y_1 , \cdots , y_p$,求证:$U = V$当且仅当$x_1 \wedge \cdots \wedge x_p$和$y_1 \wedge \cdots \wedge y_p$成比例.

必要性显然,只需要把其中一组基用令一组基表示即可.

对于充分性,考虑反证,如果$U \ne W$,取$w \in U \setminus V$,两边对$\wedge w$操作.则$x_1 \wedge \cdots \wedge x_p$会变成$0$可$y_1 \wedge \cdots \wedge y_p$不会,这就矛盾了.

###### Example2
对于$\omega \in \bigwedge^p ( V ) \setminus \{ 0 \}$,我们定义如果存在$v_1 , \cdots , v_p \in V$,使得$\omega = v_1 \wedge \cdots , \wedge v_p$,则称$\omega$可**分解**.这当然是我们关注的一个重点,下面我们尝试去推导出关于可分解的部分结论.

设$\omega \in \bigwedge^p ( V ) \setminus \{ 0 \}$,其中$p \leq n = \dim V$.

考虑如果$\omega \wedge v = 0$,我们断言$\omega$一定可以写作$v \wedge \eta$的形式.策略是将$\{ v \}$扩展成一组基$e_1 = v , e_2 \cdots , e_n$.此时$\omega = \sum_{ | I | = p } a_I e_{ i_1 } \wedge \cdots \wedge e_{ i_k }$.如果$\omega \wedge v = 0$,必然意味着如果$1 \notin I \Rightarrow a_I = 0$.从而导出上述结论.

定义$\mathrm{ ann } ( \omega ) = \{ v \in V | \omega \wedge v = 0 \}$,容易发现这的确是$V$的一个子空间.以数学归纳的策略可以证明其$r = \dim \leq p$,策略是取其一组基$e_1 , \cdots , e_r$.必然有$\omega \wedge e_1 = 0$,从而导出$\exists \eta \in \bigwedge^{ p - 1 } ( V ) \setminus \{ 0 \}$,$\omega = e_1 \wedge \eta$.从而导出$e_1 \wedge e_2 \wedge \eta = 0$.因为$e_1 \wedge \eta \ne 0$并且$e_1 \wedge e_2 \ne 0$,这意味着$e_2 \wedge \eta = 0 , \cdots e_n \wedge \eta = 0$,从而可以数学归纳.

现在我们尝试断言:$\omega$可分解的充要条件是$\dim \mathrm{ ann } ( \omega ) = p$.

先来看必要性,如果$\omega = x_1 \wedge \cdots \wedge x_p$,因为$\omega \ne 0$,所以$x_1 , \cdots , x_p$线性无关.显然$\langle x_1 , \cdots , x_p \rangle \subseteq \mathrm{ ann } ( \omega )$,然而左侧维数是$p$,右侧维数$\leq p$.比较维数知道两边相等.

再看充分性,假设$\mathrm{ ann } ( \omega )$的基是$e_1 , \cdots , e_r$,我们已经断言过存在$\eta \in \bigwedge^{ p - r } ( V )$使得$\omega = t ( e_1 \wedge \cdots \wedge e_r \wedge \eta )$,当$p = r$的时候得到$\omega$可分解.

还可以证明以下两个结论:

1. 所有的$\omega \in \bigwedge^{ n - 1 } ( V )$都是可分解的.

2. 设$\mathrm{ char } ( F ) \ne 2 , n \geq 2$,则$\omega \in \bigwedge^2 ( V )$可分解当且仅当$\omega \wedge \omega = 0$.

对于(1),考虑选取一组基$v_1 , \cdots , v_n$,则$\omega \wedge v = c ( v ) v_1 \wedge \cdots \wedge v_n$,其中$c : V \to F$显然是线性映射.既然如此,$\dim \ker c = n - 1$,从而导出$\mathrm{ ann } ( \omega ) = \ker c$的维数也是$n - 1$,这就搞定了.

对于(2),首先必要性显然.对于充分性,一个朴素的证明是基于归纳法的:

对$\dim V$的大小进行归纳,取一组基$e_1 , \cdots e_n$,令$V_1 = \langle e_2 , \cdots , e_n \rangle$则$\omega = e_1 \wedge v + \eta$的形式,其中$v \in V_1 , \eta \in \bigwedge^2 ( V_1 )$,根据归纳假设其可分解,设$\eta = x \wedge y$.

考虑:

$$
\begin{aligned}
0 & = \omega \wedge \omega \\
0 & = ( e_1 \wedge v + x \wedge y ) \wedge ( e_1 \wedge v + x \wedge y ) \\
0 & = e_1 \wedge v \wedge x \wedge y \\
0 & = v \wedge x \wedge y \\

\end{aligned}
$$

既然如此,说明$v , x , y$线性相关.如果$x = 0$或者$y = 0$当然就做完了,反之必有$v = ax + by$.那么:

$$
\begin{aligned}
\omega & = e_1 \wedge ( ax + by ) + x \wedge y \\
& = ( x + be_1 ) \wedge ( y - ae_1 )
\end{aligned}
$$

另一个策略是考虑$\bigwedge^2 ( V ) \cong \mathrm{ Bil } ( V^\vee , V^\vee ; F )$,而且右侧是反对称的.然而我们对辛形式早有研究,回忆到取基使得$B ( p_i , q_i ) = 1$而当$i \ne j$的时候$B ( p_i , q_j ) = 0$.用辛形式的分类得知存在一组基$u_1 , \cdots , u_r , v_1 , \cdots v_r$,其中$2 r \leq n$使得$\omega = \sum_{ i = 1 }^r u_i \wedge v_i$.立刻见到$\omega \wedge \omega = 0 \Rightarrow r \in \{ 0 , 1 \}$.

##### 简单应用
对于外代数来说,其最显然的应用是考虑$V = \mathbb{ R }^3$的情形,此时$\bigwedge^2 ( V )$有$\vec{ e }_1 \wedge \vec{ e }_2 , \vec{ e }_1 \wedge \vec{ e }_3 , \vec{ e }_2 \wedge \vec{ e }_3$三个基,并且:

$$
\begin{aligned}
& ( x \vec{ e }_1 + y \vec{ e }_2 + z \vec{ e }_3 ) \wedge ( x ' \vec{ e }_1 + y ' \vec{ e }_2 + z ' \vec{ e }_3 ) \\
= & ( xy ' - x ' y ) ( \vec{ e }_1 \wedge \vec{ e }_2 ) + ( yz ' - y ' z ) ( \vec{ e }_2 \wedge \vec{ e }_3 ) + ( zx ' - z ' x ) ( \vec{ e }_3 \wedge \vec{ e }_1 )
\end{aligned}
$$

这其实就是叉积对吧.

#### 域的变换
对于域$F \subseteq E$,如果我们有一个定义在$F$上的向量空间$V$,我们想要一种策略将其典范地推广到$E$上的向量空间$V_E$.

将$E$视作一个$F$向量空间.乘法映射$E \times E \to E$当然是双线性映射,从而诱导出$E \otimes E \to E , x \otimes y \mapsto xy$的映射.

考虑一个$F$向量空间$V$,考虑下述线性映射:

$$
\begin{aligned}
E \times ( E \otimes_F V ) & \to E \otimes_F ( E \otimes_F V ) \cong ( E \otimes_F E ) \otimes_F V \to E \otimes_F V \\
( x , y \otimes_F v ) & \mapsto x \otimes_F ( y \otimes v ) \mapsto ( x \otimes_F y ) \otimes_F v \mapsto ( xy ) \otimes_F v
\end{aligned}
$$

于是$E \otimes_F V$成为了一个$E$线性空间.

另一个重要的是检验如果$V , W$都是$F$向量空间,则上述拓展后$\mathrm{ Hom } ( V_E , W_E )$亦然构成$E$向量空间.而且我们还要检验如果原本的$f : V \cong W$,则$f_E : V_E \cong W_E$.

取$V$的一组基$\{ v_i \}_{ i \in I }$,则:

$$
V_E = E \otimes_F ( \bigoplus_{ i \in I } F v_i ) \cong \bigoplus_{ i \in I } ( E \otimes_F F ) \cong E^{ \oplus I }
$$

从而我们知道$V_E$有一组基:$\{ \tilde{ v }_i = 1_E \otimes v_i \}_{ i \in I }$,推论是$\dim_F V = \dim_E V_E$.

接下来将$\mathrm{ Hom }_F ( F^n , F^m )$等同于矩阵$A \in M_{ m \times n } ( F )$,我们下面验证$A_E = 1_E \otimes A$.只需发现:

$$
\begin{aligned}
A_E ( \tilde{ e }_i ) & = ( 1_E \otimes A ) ( 1_E \otimes e_i ) \\
& = 1_E \otimes ( Ae_i ) \\
& = 1_e \otimes ( \sum_{ k = 1 }^n a_{ k , i } e_k ) \\
& = \sum_{ k = 1 }^n a_{ k , i } ( 1_E \otimes e_k ) \\
& = \sum_{ k = 1 }^n a_{ k , i } \tilde{ e_k }
\end{aligned}
$$

考虑$i : V \to V_E , v \mapsto 1_E \otimes v$可以作为一个$F$线性映射,方法是将$V_E$限制成一个$F$向量空间.这显然是一个单射.

下面假设$V$是一个$F$向量空间,$\tilde{ V }$是一个$E$向量空间,则我们下面来证明最后的结论:

$$
\begin{aligned}
\mathrm{ Hom }_E ( V_E , \tilde{ V } ) & \cong \mathrm{ Hom }_F ( V , \tilde{ V } ) \\
\tilde{ f } & \mapsto \tilde{ f } \circ i
\end{aligned}
$$

先证明这的确是一个单射,考虑:

$$
\tilde{ f } ( t \otimes v ) = \tilde{ f } ( t \cdot i ( v ) ) = t \cdot ( \tilde{ f } \circ i ) ( v )
$$

由于$i$是一个单射,所以这里如果非单,则存在两个$\tilde{ f }_1 , \tilde{ f_2 }$满足$\forall v$,$( \tilde{ f }_1 \circ i ) ( v ) = ( \tilde{ f }_2 \circ i ) ( v )$,必然导出$\forall t \otimes v , \tilde{ f }_1 ( t \otimes v ) = \tilde{ f }_2 ( t \otimes v )$.

接下来考虑其满射,对于$f \in \mathrm{ Hom }_F ( V , \tilde{ V } )$,考虑$E \times V \to \tilde{ V } , ( t , v ) \mapsto t f ( v )$当然是一个$F$上的双线性映射,其对应了一个$\tilde{ f } : E \otimes_F V \to \tilde{ V } , t \otimes v \mapsto tf ( v )$.这就论证了满性.

好吧,我们上面论证的一切都太过无趣,但请来总结,我们实际上是想要这么做:假设我有$F$上的向量空间$V$,$E$上的向量空间$\tilde{ V }$以及它们之间的$F$线性映射$f$.关于这个$f$的看法,如果我们将$\tilde{ V }$拉到$F$上当然是可以的对吧,那能不能将$V$升上$E$呢?我们想要一个资料$( \tilde{ V }_{ \mathrm{ univ } } , f_{ \mathrm{ univ } } )$使得下述图表交换:

$$
\xymatrix{ V \ar[r]^{ f_{ \mathrm{ univ } } } \ar[rd]_{ f } & \tilde{ V }_{ \mathrm{ univ } } \ar[d]^{ \tilde{ f } } \\
& \tilde{ V } }
$$

我们上面所证明的就是这个资料$( \tilde{ V }_{ \mathrm{ univ } } , f_{ \mathrm{ univ } } )$可以取作$( E \otimes_F V , i )$.


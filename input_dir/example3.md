---
title: 数学分析
categories: 大学课程
tags: [数学] 
mathjax: true
---
<!-- toc -->

<!-- more -->



### 实数

首先我们拿到了有理数,然后我们按照以下步骤定义无理数:

首先我们注意到有理数是不连续的.问题在于我们要将其中不连续的部分给补上.首先我们应当去发现一下如何去发现有理数中的一个断点:

设$S$是一个全序非空集合,我们找到它的两个非空子集$A,B$,若满足:

1. $A\cap B=\empty,A\cup B=S$.
2. $\forall a\in A,b\in B,a<b$.
3. $A$中无最大元素.

则称这是$S$的一个**分划**,记作$(A|B)$.接下来我们先考虑$S=\mathbb Q$的情况.

我们注意到一些事,比如说注意到$B$中有可能没有最小元素.举个例子的话,我们令$S=\mathbb Q,A=\{x|x\leq 0\lor (x>0\and x^2<2)\}$,显然$A$中无最大元素,此时取$B=S\setminus A$,则显然$B$中无最小元素(反证法),假设存在最小元素则可以调整到更小的元素,因为容易证明不存在一个有理数的平方为$2$.

通过上面的铺垫我们知道,有理数并非连续的,因此如果$B$中有最小元素,则称这是一个**有理分划**;如果$B$中无最小元素,则称这是一个**无理分划**.容易发现有理分划与有理数一一对应.我们将无理分划也去对应到一些数上,这就产生了无理数的定义.只需比较$A$集合的大小包含关系就可以比较两个实数的大小.

那我们还需要做的一件事是去证明无理数也有四则运算法则.不妨直接拿分划去验证四则运算法则:

1. 加法:对于$(A_c|B_c)+(A_d|B_d)$,我们取$A_{c+d}=\{x+y|x\in A_c,y\in A_d\}$即可.
2. 减法:只需定义加法逆元.如果是无理分划的话,直接把$A,B$中的元素全部取反再交换即可;如果是有理分划的话,需要注意取反再交换后$A$中出现了最大元素,只需要在这里把那个元素给提出来就行.
3. 乘法:两个正数相乘可以直接模仿加法,如果有负数的话就把负号提出来再把绝对值作乘法即可.
4. 除法:只需定义乘法逆元,这个有点麻烦需要判断$A$和$B$哪个集合是同号的,但总之这个是平凡的.

#### 戴德金分割定理

对$\R$的任一分划$(A|B)$,$B$中必定有最小元素.

这个怎么证明呢?对于分划$(A|B)$,我们直接取$A'=A\cap \Q,B'=B\cap \Q$,不难发现$(A'|B')$一定是$\Q$的一个分划,也就是说$(A'|B')$对应了一个实数$x$.

既然如此就可以考虑$x$应该属于哪个集合.我们注意到:如果$x\in A$,那么$x$必定是$A$中的最大元素;如果$x\in B$,那么$x$必定是$B$中的最小元素.

两部分是类似的,只考虑前一部分.考虑反证,如果$\exists y>x,y\in A,x\in A$,那么$A'\subsetneq A_y$,那么$A_y$就一定不是$A$的子集.换言之,$\exists w\in A_y$,$\forall z\in A,w>z$.但我们有$y>w$,这必然意味着$y\notin A$,与假设不符.

#### 确界存在定理

若$M$有上(下)界,则必然有上(下)确界.

设$M$有上界$c$,由于它是一个实数,它应该可以写作$(A_c|B_c)$的形式.$\forall \alpha\in M,A_\alpha \subset A_c$.

于是我们取$A=\bigcup_{\alpha\in M}A_\alpha,B=\Q\setminus A$,我们来说明$(A|B)$是一个分划.

首先由于有上界,显然$A,B\ne \empty,A\cap B=\empty,A\cup B=\Q$.

如果 $\exists b\in B,a\in A,b<a$,这是不可能的,因为如果$b<a$,那么$b\in A,b\notin B$.

因为所有的$A_\alpha$中都没有最大元素,反证即可说明$A$中无最大元素.

那么$(A|B)$就是$M$的上确界.

###### Example1

证明对于单增函数$f:[0,1]\rightarrow [0,1]$,$f([0,1])\subseteq (0,1)$,则$f$与$y=x$一定有交点.

取$A=\{x|f(x)<x\}$,则$1\in A,A\subseteq [0,1]$.取$a=\inf A$.

此时我们断言$f(a)=a$,否则:

如果$f(a)<a$,取$\epsilon=a-f(a)$,注意到$f(a-\frac{\epsilon}{2})<f(a)<a-\frac{\epsilon}{2}$,因此$(a-\frac{\epsilon}{2})\in A$,与$a$是下确界矛盾.

如果$f(a)>a$,取$\epsilon=f(a)-a$.考虑由于$a$是下确界,因此$\exists b\in A,b\in (a,a+\frac{\epsilon}{2})$.那么$b<a+\frac{\epsilon}{2}<f(a)<f(b)$,这与$b\in A$矛盾.

#### 阿基米德性质

以下命题等价,任取其一均可作为阿基米德性质:

1. $\forall y\in \R$,$\exists n\in \N,n>y$.
1. $\forall x\in \R_+,\exist n\in \N$,$nx>1$.
1. $\{\frac{1}{n}\}$有聚点.
1. $\lim_{n\rightarrow \infty}\frac{1}{n}$存在.
1. $\lim_{n\rightarrow \infty}\frac{1}{n}=0$.
1. $\lim_{n\rightarrow \infty}\frac{1}{2^n}=0$.

(1)$\Rightarrow $(2)$\Rightarrow$(3)$\Rightarrow$(4)$\Rightarrow (5)$是显然的.又考虑$\{\frac{1}{2^n}\}$是$\{\frac{1}{n}\}$的子列,所以$(5)\Rightarrow (6)$.而取一个$\frac{1}{2^k}<\frac{1}{|y|}$即可(6)$\Rightarrow$(1).

阿基米德性质可以由确界存在定理推来,简单来说考虑$x\in \R_+,A=\{nx|nx<1\}$.反证,如果$A$是无限集合,那么根据确界存在定理一定存在最小上确界$\alpha\leq 1$有$\forall n,nx<\alpha,(n+1)x<\alpha,nx<\alpha-x$,这就说明$\alpha-x$是一个更小的上界,这就矛盾了.

### 实数集上的一元函数

#### 性质

##### 奇偶性

首先需要定义域关于原点对称.

然后注意到取$f(x)=\frac{f(x)+f(-x)}{2}+\frac{f(x)-f(-x)}{2}$,可将任意定义域关于原点对称的函数写作奇偶函数之和.

事实上,对于一个可以求任意次导数的偶函数$f$,是否总存在一个可以求任意次导数的函数$g$使得$f(x)=g(x^2)$.

这个问题的难点肯定在于$0$附近对吧.

这个证明要用到泰勒展开.
$$
f(\sqrt{x})=\sum_{k=0}^n\frac{f^{(k)}(0)}{k!}x^\frac{k}{2}+\frac{f^{(n+1)}(\xi_n)}{(n+1)!}x^\frac{n+1}{2}
$$
两边求导,应该有:
$$
(f(\sqrt{x}))^{(t)}=\sum_{k=0}^n\frac{f^{(k)}(0)\prod\limits_{i=0}^{t-1}(\frac{k}{2}-i)}{k!}x^{\frac{k}{2}-t}+\frac{f^{(n+1)}(\xi_n)\prod\limits_{i=0}^{t-1}(\frac{n+1}{2}-i)}{(n+1)!}x^{\frac{n+1}{2}-t}
$$
我们知道$f$任意阶导数存在,所以在闭区间$[-1,1]$上任意阶导数有界.这就意味着,对于固定的$t$.只要取足够大的$n=2t+2$满足$\frac{n+1}{2}-t>1$,就可以让余项趋于$0$.

现在来数学归纳,用多项式的特殊情况,假设$g^{(t)}(0)=\frac{t!}{(2t)!}f^{(2t)}(0)$,考虑:
$$
\begin{aligned}
g^{(t+1)}(0)=\frac{(f(\sqrt x))^{(t)}-\frac{t!}{(2t)!}f^{(2t)}(0)}{x}\\
=\cfrac{\sum_{k=0}^{2t+2}\frac{f^{(k)}(0)\prod\limits_{i=0}^{t-1}(\frac{k}{2}-i)}{k!}x^{\frac{k}{2}-t}-\frac{t!}{(2t)!}f^{(2t)}(0)}{x}\\
=\cfrac{\sum_{k=0}^{2t+2}\frac{f^{(k)}(0)\prod\limits_{i=0}^{t-1}(\frac{k}{2}-i)}{k!}x^{\frac{k}{2}}-\frac{t!}{(2t)!}f^{(2t)}(0)x^t}{x^{1+t}}\\
\end{aligned}
$$
一片混乱,考虑一下下面这项:
$$
\frac{f^{(k)}(0)\prod\limits_{i=0}^{t-1}(\frac{k}{2}-i)}{k!}x^{\frac{k}{2}}\\
=\frac{f^{(k)}(0)\prod\limits_{i=0}^{t-1}(k-2i)}{k!2^t}x^{\frac{k}{2}}\\
$$
这个时候我们注意到,当$k$是奇数的时候应该有$f^{(k)}(0)=0$,当$k$是偶数并且$k\leq t-1$的时候应该有$\prod\limits_{i=0}^{t-1}(k-2i)=0$,这意味着这个求和最后只会剩下$\frac{f^{(2t)}(0)t!}{(2t)!}x^{t}+\frac{f^{2t+2}(0)(t+1)!}{(2t+2)!}x^{t+1}$,然而前半部分和后面的刚好消掉了,这就数学归纳成立了.

##### 有界性

##### 单调性

##### 周期性

周期函数不一定有最小正周期,比如狄利克雷函数.

还有个思考题:

如果$f,g$是定义域为$R$的最小正周期分别为$T_1,T_2$的周期函数,求问:

1. 若$\frac{T_1}{T_2}\in\Q$,则$f+g$是周期函数.
2. 若$\frac{T_1}{T_2}\in \R\setminus \Q$,结论如何.
3. 若$f,g$均连续,结论如何.

(1)显然,(3)必定不是周期函数,原因是考虑反证:

如果是的话,必定有:
$$
f(x+T)+g(x+T)=f(x)+g(x)\\
f(x+T)-f(x)=g(x)-g(x+T)
$$
令$h(x)=f(x+T)-f(x)=g(x)-g(x+T)$,则$h(x)$以$T_1,T_2$为周期,那么其以$pT_1+qT_2$为周期,可逼近这个无理数使得其周期趋近于$0$,用$h$的连续性,于是$h(x)$为常函数,不符题意.

(2)怎么办呢?有没有一种构造方式使得它是周期函数呢?

构造$d_{p,q}(x)=\begin{cases}1&\exists m,n\in \Z,mp+nq=x\\0&\forall m,n\in \Z,mp+nq\ne x\end{cases}$.

取$f(x)=d_{1,\sqrt 2}(x)-d_{\sqrt 2,\sqrt 3}(x),g(x)=d_{\sqrt 2,\sqrt 3}(x)-d_{1,\sqrt{3}}$,容易见到$f+g$的周期为$1$,下面证明$f$的最小正周期是$\sqrt 2$,而$g$的最小正周期是$\sqrt 3$.

二者相似,只考虑证明前者,如若存在一个$T$,使得$f(x)=f(x+T)$,则有:

$$
d_{1,\sqrt 2}(x)-d_{\sqrt 2,\sqrt 3}(x)=d_{1,\sqrt 2}(x+T)-d_{\sqrt 2,\sqrt 3}(x+T)\\
d_{1,\sqrt 2}(x)-d_{1,\sqrt 2}(x+T)=d_{\sqrt 2,\sqrt 3}(x)-d_{\sqrt 2,\sqrt 3}(x+T)
$$

取$x=\sqrt 3$得到:

$$
d_{\sqrt 2,\sqrt 3}(\sqrt 3+T)-d_{1,\sqrt 2}(\sqrt 3+T)=1
$$

这就意味着:

$$
\begin{cases}
d_{\sqrt 2,\sqrt 3}(\sqrt 3+T)=1\\
d_{1,\sqrt 2}(\sqrt 3+T)=0\\
\end{cases}
$$

从而$T=n\sqrt 2+m\sqrt 3$,其中$m\ne -1$.再取$x=0$得到:

$$
d_{1,\sqrt 2}(T)=d_{\sqrt 2,\sqrt 3}(T)=1
$$

这就必然意味着$T=n\sqrt 2$,从而$f$的最小正周期恰好就是$\sqrt 2$.

#### 基本初等函数

1. 常值函数
2. 指数函数
3. 对数函数
4. 幂函数
5. 三角函数
6. 反三角函数

其中有一些函数的定义其实有一些缺陷,我们将在讨论完连续性后给出.

#### 初等函数

由基本初等函数经过有限次四则运算和复合得到的函数.

举个逆天例子:
$$
f(x)=\begin{cases}\sqrt{-x}&x<0\\0&x\in[0,1]\\\ln x&x>1\end{cases}
$$
注意到$f(x)=\sqrt{\frac{\sqrt {x^2}-x}{2}}+\ln\frac{x+\sqrt{(x-1)^2}+1}{2}$.

#### 代数函数

存在多项式$F(x,y)$满足$F(x,f(x))\equiv 0$的函数$f(x)$称为代数函数.

### 数列极限

#### 定义

设$\{a_n\}$是一个给定的序列,若$\exists l$,$\forall \epsilon>0$,$\exists N\in\mathbb{N}$,$\forall n> N,|a_n-l|< \epsilon$.我们称$l$是这个序列的**极限**,记作$\lim_{n\rightarrow \infty}a_n=l\\$.

#### 性质

1. 若数列存在极限,则极限唯一.
2. 有极限的数列有界.
3. 数列的极限可以四则运算.
4. 设序列$\{a_n\},\{b_n\}$有极限并且分别为$l_a,l_b$,若$\exists N\in\mathbb{N}$,$\forall n>N,a_n\geq b_n$,则$l_a\geq l_b$.
5. 设序列$\{a_n\},\{b_n\}$有极限并且分别为$l_a,l_b$,如果$l_a>l_b$,则$\exists N\in\mathbb{N}$,$\forall n>N,a_n>b_n$.
6. 夹逼定理:设$\{a_n\},\{b_n\},\{c_n\}$是三个序列,且$\exists N_0,\forall n>n_0,a_n\leq b_n\leq c_n$.那么如果$\lim_{n\rightarrow \infty}a_n=\lim_{n\rightarrow \infty}c_n=l\\$,则$\lim_{n\rightarrow \infty}b_n=l\\$.
8. 对于原序列,若它有极限$l$,取出它的无穷项子序列,那这个子序列的极限必然也是$l$.(如果一个序列能取出两个无穷项子序列使得极限不相同,那么原序列必然无极限)



大部分的证明都很显然,只需要套用极限定义即可,这里举几个证明为例.

(3):

设序列$\{a_n\},\{b_n\}$有极限并且分别为$l_a,l_b$,则要证$\lim_{n\rightarrow \infty}(a_n\pm b_n)=l_a\pm l_b\\$,$\lim_{n\rightarrow \infty}(a_nb_n)=l_al_b\\$,若$l_b\ne 0$,$\lim_{n\rightarrow \infty}(\frac{a_n}{b_n})=\frac{l_a}{l_b}\\$.

先证明加减法,取$\frac{1}{2}\epsilon$然后合并起来就行.

乘法也类似,首先有:
$$
l_a-\epsilon_1<a_n<l_a+\epsilon_1\\
l_b-\epsilon_2<b_n<l_b+\epsilon_2
$$
于是自然有:
$$
|a_nb_n-l_al_b|=|(a_n-l_a)b_n+l_a(b_n-l_b)|
\\\leq |b_n||a_n-l_a|+|l_a||b_n-l_b|\\
<|b_n|\epsilon_1+|l_a|\epsilon_2
\\<|l_b\pm \epsilon_2|\epsilon_1+|l_a|\epsilon_2
$$
不妨令$\epsilon_1=\epsilon_2$,尝试构造一组解使得$\forall \epsilon>0,\exists \epsilon_1>0$,$|l_b\pm\epsilon_1|\epsilon_1+|l_a|\epsilon_1<\epsilon$.限制一下$\epsilon<1,\epsilon_1<1$,取$\epsilon_1=\frac{\epsilon}{1+|l_b|+|l_a|}$即可.

接下来证明除法,实际上只需要证明若$l_b\ne 0$,$\lim_{n\rightarrow \infty}(\frac{1}{b_n})=\frac{1}{l_b}\\$.

考虑:$l_b\lim_{n\rightarrow \infty}b_n=l_b^2>0\\$,这意味着当$\exists N_0,\forall n>N_0,l_bb_n>0\\$,或者更强一点,$\exists N_0,\forall n>N_0,l_bb_n>\frac{1}{2}l_b^2\\$.

不妨设$\epsilon'>0,\exists N_1,\forall n>N_1,|l_b-b_n|<\epsilon'$

取$n>\max(N_0,N_1)$此时自然有:
$$
|\frac{1}{b_n}-\frac{1}{l_b}|=\frac{|l_b-b_n|}{|b_nl_b|}\leq \frac{2}{l_b^2}\epsilon'
$$
取$\epsilon'=\frac{l_b^2}{2}\epsilon$即可得证.

(4):

反证,设$l_a<l_b$,那么取$\epsilon=\frac{l_b-l_a}{2}$,立刻有:
$$
|a_N-l_a|<\epsilon\\
a_N< \frac{l_a+l_b}{2}<b_N
$$
这是不可能的.

#### 单调收敛准则

单调有界无穷数列有极限.

不妨设$\{a_n\}$是一个单调有界无穷数列(不妨假设其单增,不然可以取反变成单增),且$\forall n,a_n\leq l_a$.

考虑集合$A=\{x_n|n\in \N_+\}$,根据确界存在定理,其一定有上确界.令$a$是它的上确界,我们来证明$a$一定是数列的极限.

由于$a$是上确界,这意味着$\forall \epsilon>0,a-\epsilon$不是上确界,所以$\forall \epsilon>0$,$\exists N$,$\forall n>N,a-\epsilon<a_n\leq a<a+\epsilon$,这就是极限的定义.

#### 自然对数的底数

定义$e=\lim_{n\rightarrow \infty}(1+\frac{1}{n})^n\\$.如何证明它存在极限呢?考虑单调有界无穷序列必定存在极限,定义$x_n=(1+\frac{1}n)^n$,于是先证明它有界:
$$
(1+\frac{1}{n})^n=\sum_{k=0}^n\frac{n^{\underline{k}}}{k!}\frac{1}{n^k}\\
<\sum_{k=0}^n\frac{1}{k!}\\
<2+\sum_{k=2}^n\frac{1}{k(k-1)}\\
=2+\sum_{k=2}^n(\frac{1}{k-1}-\frac{1}{k})
<3
$$
接下来证明单调,只需证明:
$$
\sqrt[n+1]{(1+\frac 1 n)^n}<\frac{n+2}{n+1}\\
\sqrt[n+1]{1(1+\frac 1 n)^n}<\frac{1+n(1+\frac{1}{n})}{n+1}
$$
由均值不等式显然.

如果我们考虑另一个极限$y_n=(1+\frac{1}{n})^{n+1}$,注意到:
$$
\frac{1}{y_n}=(\frac{n}{n+1})^{n+1}\times 1< (\frac{n+1}{n+2})^{n+2}=\frac{1}{y_{n+1}}
$$
发现$y_n$单调递减且$y_n>x_n$,并且$\lim_{n\rightarrow \infty}y_n-x_n=0$,这就意味着二者必然同时趋向于一个常数的两侧,定义为$e=2.7182818284\cdots$.

这里还可以推出一些结论.因为我们知道:
$$
x_n<e<y_n\\
n\ln(1+\frac{1}{n})<1<(n+1)\ln (1+\frac{1}{n})\\
\frac{1}{n+1}<\ln(n+1)-\ln (n)<\frac{1}{n}\\
H_{n+1}-1<\ln (n+1)<H_n
$$
用这个可以注意到定义$z_n=H_n-\ln n$,这个东西肯定$\geq 0$且$\leq 1$,而且注意到$z_{n+1}-z_n=\frac{1}{n+1}-\ln \frac{n+1}{n}<0$,所以单调有界,这个极限就是欧拉常数$\gamma=0.57\cdots$.

接下来证明$e$是无理数.

考虑$e=\lim_{n\rightarrow \infty}(1+\frac{1}{n})^n$,取二项式定理,取$x_k=\sum_{k=0}^n\frac{n^{\underline{k}}}{k!n^k}\leq \sum_{k=0}^n\frac{1}{k!}=U_n$.

而又注意到$x_n=\sum_{k=0}^n\frac{n^{\underline{k}}}{k!n^k}$,取定一个确定的上界$m\leq n$,则$x_n\geq \sum_{k=0}^m\frac{n^{\underline{k}}}{k!n^k}=y_n$.由于$m$有限,所以当$n\rightarrow \infty$时,$y_n\rightarrow U_n$的前$m$项,此时再令$m\to \infty$,根据夹逼定理,立刻得到$x_n\rightarrow U_n$.换言之,$m$增大引起的误差总可以被后面的$n$的增大抹平.

考虑一下计算误差,设$r_n=e-U_n$,则$r_n=\sum_{k\geq n+1}\frac{1}{k!}=\frac{1}{(n+1)!}\sum_{k\geq 0}\frac{1}{(n+2)^{\overline{k}}}<\frac{1}{(n+1)!}\sum_{k\geq 0}\frac{1}{(n+2)^k}=\frac{1}{(n+1)!}\frac{n+2}{n+1}<\frac{1}{n\times n!}$.

如果$e$是有理数,设$e=\frac{m}{n}\in \Q$,考虑:
$$
0<r_n=\frac{m}{n}-U_n<\frac{1}{n\times n!}\\
0<m\times (n-1)!-U_n\times n!<\frac{1}{n}
$$
这立刻导出矛盾,因为$U_n\times n!$一定是整数.

#### 开区间与闭区间

定义一个集合的内部点:$x,\exists \delta>0,(x-\delta,x+\delta)\subseteq A$.我们将一个集合的所有内部点组成的集合记作$A^{\degree}$.

这样就可以定义一个集合是内部为空的,就是其没有内部点.反之,如果每个点都是其内部点,那么称其为稠密的.

定义一个集合的边界点:$x,\forall \delta>0,(x-\delta,x+\delta)\cap A\ne \empty,(x-\delta,x+\delta)\cap A^C\ne \empty$.

一个集合称为开集合,当且仅当$A=A^{\degree}$.

一个集合称为闭集合,当且仅当对于集合中的任何一个数列,数列极限也存在于这个集合中.

当然也可以用补集来用开集合定义闭集合(补集是开集合的集合是闭集合)或者反之.

由此我们知道开集的并是开集,有限个开集的交是开集;闭集的交是闭集,有限个闭集的并是闭集.

容易发现包含于$A$的所有开集的并是$A^{\degree}$.

我们类似定义$A$的闭包是$\bar A$.

而且容易发现如果$f$是连续函数,那么如果$U$是开集,那么$f^{-1}(U)$(也就是$U$的原像)也是开集.

#### 区间套定理

设$I_n=[a_n,b_n]\ne \empty,I_{n+1}\subseteq I_{n}$,则$I=\bigcap_{n\geq 1}I_n$存在且是一个非空闭区间.

怎么证明?考虑$a_n$单增,$b_n$单减并且$a_1\leq a_n\leq b_n\leq b_1$,所以$a_n$存在极限,不妨设为$l_a$,同理$b_n$也存在极限,设为$l_b$.容易验证$I=[l_a,l_b]$.

类似地,设$I_n=(a_n,b_n)\ne \empty,I_{n+1}\subseteq I_{n}$,并且$a_n,b_n$不会从某一项开始往后全都是常数(换言之其极限不等于其每一项,且不会上下波动),则$I=\bigcap_{n\geq 1}I_n$存在且是一个非空闭区间.

特别地,当$\lim |b_n-a_n|=0$的时候,上述给出一个单点.

###### Example1

设$x_1=a,x_2=b,a\ne b,x_{n+2}=\frac{x_{n+1}+x_n}{2}$,问$\lim_{n\rightarrow \infty}x_n$是否存在.

可以设$c_n=\min\{x_n,x_{n+1}\},d_n=\max\{x_n,x_{n+1}\}$,讨论$x_{n+1}$和$x_n$的大小关系(二者显然不相等)可以知道$[c_n,d_n]$满足区间套定理的条件,又注意到$|I_{n+1}|=|x_{n+2}-x_{n+1}|=\frac{1}{2}|I_n|$,所以区间长度趋近于$0$,这样的话$\lim_{n\rightarrow\infty}c_n=\lim_{n\rightarrow\infty}x_n=\lim_{n\rightarrow\infty}d_n$.

###### Example2

设$a_1>b_1>0$,$a_{n+1}=\frac{a_n+b_n}{2},b_{n+1}=\sqrt {a_nb_n}$.求证$a_n,b_n$极限都存在且相等.

还是区间套定理,容易验证满足区间套定理的条件,那么要注意到$|I_{n+1}|=\frac{a_n+b_n}{2}-\sqrt {a_nb_n}<\frac{a_n+b_n}{2}-b_n=\frac{1}{2}|I_n|$,所以$\lim_{n\rightarrow\infty}|I_n|=0$,这就证明了二者存在且相等.

其实先用单调有界数列有极限,再对着递推式两边求下极限就可以了.

#### 有限覆盖定理

覆盖:$\Sigma:$由一些开区间组成的集合.若$M\subset \R$,称$M$被$\Sigma$覆盖,当且仅当$M\subseteq \bigcup_{\sigma\in \Sigma }\sigma$.

有限覆盖定理是说,如果$M=[a,b]$是一个有界闭区间,且$M$被$\Sigma$覆盖,则一定存在一个$\Sigma$的有限子集$\Sigma_1$,使得$M$能被$\Sigma_1$覆盖.

换言之,有界闭区间若能被开区间覆盖,则一定能被有限个开区间覆盖.

反证:如果$M$不能被有限覆盖,那么$[a,\frac{a+b}{2}]$和$[\frac{a+b}{2},b]$至少有一个不能被有限覆盖(如果它俩都能那$[a,b]$肯定也能).不妨取出二者中任意一个不能被有限覆盖的区间,设为$[a_1,b_1]$,再进行上面的操作就可以得到$[a_2,b_2]$,以此类推,如此我们得到了一个满足区间套定理的区间序列而且均不能被有限覆盖,根据区间套定理,可以取出它们交集的那个元素$c$.由于$\{c\}\in M$,所以$\exists \sigma\in \Sigma,c\in \sigma$.因为$\sigma$是开区间,所以$\exists \epsilon>0,[c-\epsilon,c+\epsilon]\in \sigma$,而显然可以找到一个$N>0$,$\forall n\geq N$,$[a_n,b_n]\in [c-\epsilon,c+\epsilon]$,这就矛盾了.

###### Example1

证明:有界闭区间上的局部有界(对于每一个点,它都存在一个开区间邻域是有界的)函数是整体有界函数.

设定义域为$[a,b]$,考虑$\forall x_0\in [a,b],\exists \sigma>0$,$f$在$(x_0-\sigma,x_0+\sigma)$上有界,注意到设$I_{x_0}=(x_0-\sigma,x_0+\sigma)$,那么$[a,b]$一定被$I$这个区间集合开覆盖,那么就能被有限开覆盖,取出那有限个区间对应的值域并起来就行.

###### Example2

证明:有界闭区间上的局部增(对于每一个点,它都存在一个开区间邻域是增的)函数是整体增函数.

证明和上面一样,合并两个开区间的时候找任意一个交集元素即可.

#### 外测度与零测集

定义**外测度:**对于$E\subseteq \R$,定义$m^*(E)$为$E$的外测度,$m^*(E)=\inf\{(\sum |I_k|)\mid \bigcup I_k\supseteq E\}$,也就是找一列区间使得其并起来能包含$E$.

定义**零测集**:外测度为$0$的集合.容易发现$m^*(\bigcup E_k)\leq \sum m^*(E_k)$.于是可数个零测集的并仍然是零测集.

如果一个集合的势可数,那它当然是零测集,原因是可以构造$I_n=(x_n-\delta,x_n+\delta),\delta=\frac{1}{2^{n+1}}$.由这个构造$\epsilon_k=\frac{\epsilon}{2^{k+1}}$可以说明可数个零测集的并仍然是零测集.

反之,一个集合是零测集不一定代表其可数,例如康托三分集:先把$[0,1)$上扣去$[\frac{1}{3},\frac{2}{3})$,然后再对剩下的线段也做类似的操作,每次扣掉中间一段.算一下每次扣去的长度,发现长度恰好为$1$,于是当然是零测集.但注意到这对应着一个三进制小数并且每一位只能是$0,2$,那这当然势仍然是$2^\N$,于是不可数.

#### Vitali覆盖引理

当$E\subseteq R$,称一族区间$\Gamma=\{I_\alpha\}$是$E$的一个Vitali覆盖,若对$\forall x\in E,\epsilon>0$,$\exists I_{\alpha}\in \Gamma,|I_\alpha|<\epsilon,x\in I_\alpha$.也就是对于任何一个点都存在足够小的区间把它盖住.

Vitali覆盖引理是说,如果$E$是一个有界集(或者推广到外测度有限的集合),$\Gamma$是$E$的一个Vitali覆盖,则$\forall \epsilon>0$,$\Gamma$中可以拿出有限个两两不交的区间$I_j\in \Gamma$,满足$m^*(E\setminus (\bigcup I_j))<\epsilon$.注意这里取区间的方式依赖于$\epsilon$的选取.

取一个开集$G\supseteq E,m^*(G)<\infty$,不妨设$\Gamma$中的每个区间都$\subseteq G$(原因在于Vitali覆盖保证了任意小的区间覆盖,而开集的性质使得总有区间在其中).在此基础上,由于上面的$\Gamma$是任意一个Vitali覆盖,所以我们不妨假设$\Gamma$中的每个区间都是闭区间(也就是把端点加进去,那当然仍然是Vitali覆盖而且更难满足限制,因为闭区间更难以两两不交),这当然仍是可以的.

现在我们归纳取区间,我们任取一个$I_1\in \Gamma$,开始不断往上添区间.设$I_1,\cdots,I_k$已经被我们拿到,而且两两不交.

如果此时$E\subseteq \bigcup_{j=1}^k I_j$,当然直接满足条件.

反之,令$\delta_k=\sup\{(|I|)\mid I\in \Gamma,I\cap I_j=\empty,1\leq j\leq k\}$.容易发现$\delta_k<\infty$.那我们取$I_{k+1}$使得$\forall 1\leq j\leq k,I_{k+1}\cap I_j=\empty,|I_{k+1}|>\frac{1}{2}\delta_k$,这当然做得到.

如果一直做下去都没有使得$E\subseteq \bigcup I_j$,那我们就可以拿到一个区间列.接下来我们考虑$\sum |I_k|$,此时由于$I_k$两两不交,当然有$\sum|I_k|\leq m^*(G)<\infty$,那么根据单调收敛准则,$\sum |I_k|$趋于一个极限,于是$|I_k|$当然趋近于$0$,那么$\delta _k$趋近于$0$.在此基础上,我们当然可以取一个$N>0$,$\forall n\geq N,\sum_{k\geq n+1}|I_k|<\frac{1}{5}\epsilon$.

此时$\forall x\in E\setminus (\bigcup_{k\leq N} I_k)$.由于$I_k$均为闭区间,所以$E\setminus (\bigcup_{k\leq N} I_k)$必定是开区间,这也就是说$x$存在一个小邻域$I'\in \Gamma$并且$I'\subseteq E\setminus (\bigcup_{k\leq N} I_k)$.那当然有$I'\cap (\bigcup_{k\leq N}I_k)=\empty$,也就是$I'\cap I_k=\empty,1\leq k\leq N$.由于$|I'|$有限且不为$0$,而$\delta _n\rightarrow 0$.一定$\exists n\geq N$,使得$I'\cap (\bigcup_{k\leq n}I_k)=\empty,I'\cap I_{n+1}\ne \empty$.发现此时$|I'|\leq \delta_n<2|I_{n+1}|$.

此时会出个什么事呢?由于$|I'|<2|I_{n+1}|$,所以我们把$I_{n+1}$这个区间扩大五倍(中心不变,半径扩大到原本的五倍)就一定能盖住$I'$.

那会怎么样呢?那$m^*(E\setminus (\bigcup_{k\leq N} I_k))\leq 5\sum_{k\geq N+1}|I_k|<\epsilon$.于是证毕.

#### 聚点原理

假设$M$是一个实数集,称$x_0$是$M$的聚点,当且仅当$\forall \epsilon>0$,$((x_0-\epsilon,x_0)\cup(x_0,x_0+\epsilon))\cap M\ne \empty$,或说,$\forall \epsilon>0,(x_0-\epsilon,x_0+\epsilon)\cap M$是一个无限集.可以理解为,$x_0$不是一个孤点.

引理:$x_0$为$M$的聚点,当且仅当$\exists \{x_n\}\in M\setminus \{x_0\},\lim_{n\rightarrow \infty}x_n=x_0$.充分性显然,必要性的话只需取$\epsilon=2^{-n}$然后从上面的交集拿点就行.

聚点原理:有界无穷集$M$必有聚点.

我们可以找一个$I_0=[a,b],M\subset I_0$,我们找$[a,\frac{a+b}{2}],[\frac{a+b}{2},b]$,其中至少有一个和$M$的交集为无穷集,我们取其为$[a_1,b_1]$,不断这么做又会得到一个闭区间套列,交起来得到的那个元素$c$就是聚点,因为$\forall \epsilon>0,\exists N>0,\forall n\geq N,|b_n-a_n|<\epsilon$,那么$[a_n,b_n]\subseteq (c-\epsilon,c+\epsilon)$,立刻证毕.

#### Baire纲定理

1. 设$\{F_n\}$是一列内部为空(没有除空集以外的开子集)的闭集,则$\bigcup_{n=1}^{\infty}F_n$内部为空.
2. 设$\{G_n\}$是一列稠密(补集内部为空)的开集,则$\bigcap_{n=1}^{\infty}G_n$稠密.

(1)(2)等价,下面只证明(1):

反证,假设$F=\bigcup_{n=1}^{\infty}F_n$内部不为空,也就是其存在一个内部开集$(x_0-\delta_0,x_0+\delta_0)\subseteq F$.由于$F_1$是一个闭集,因此$(x_0-\delta_0,x_0+\delta_0)\setminus F_1$当然是一个开集,于是其就存在一个子区间$[x_1-\delta_1,x_1+\delta_1]\subseteq (x_0-\delta_0,x_0+\delta_0)$并且$[x_1-\delta_1,x_1+\delta_1]\cap F_1=\empty$,同理可以找到一列闭区间$\{[x_k-\delta_k,x_k+\delta_k]\}$使得$[x_k-\delta_k,x_k+\delta_k]\cap \bigcup_{j=1}^k F_j=\empty$.

容易见到我们可以任意调小$\delta_k$的大小,调整使得$\delta_k<\frac{1}{k}$总成立就得到了一列闭区间套,必然拿到了一个单独的点$\xi$,那么$\xi\in (x_0-\delta_0,x_0+\delta_0)\subseteq F$,然而对于任意区间都有$\xi\notin F_n$,因此$\xi \notin F$,这就导出了矛盾.



###### Example1

证明:如果$f(x)$连续,并且$\forall x>0,\lim_{n\rightarrow \infty}f(nx)=0$,则$\lim _{x\rightarrow \infty}f(x)=0$.

对于一个$\epsilon>0$,考虑构造内部为空的闭集$F_n=\{x|\forall k\geq n,f(kx)\leq \epsilon\}$(为什么这是闭集呢?因为当你设$F_{n,k}$满足上述定义,那$F_{n,k}$肯定是闭集,然后$F_n$就是上述这些的交,当然也是闭集).容易发现$F_n$的并是$\R_+$,那就一定至少有一个闭集内部不为空,我们就可以取出其中的一个区间设为$(a,b)$,然后用这个区间在若干倍后覆盖后半部分数轴,原因是当$n$足够大的时候,一定有$nb>(n+1)a$存在.

#### 致密性定理

有界无穷数列有收敛的子列.

考虑数列值域$M=\{x|x=x_n,n\in \N_+\}$.

如果$M$是有限集合,那$\exists a$,$I=\{n|x_n=a\}$是无限集合,把这些数拿出来就是一个无限的常数列.

如果$M$是无限集,那其存在聚点,设$a$为其中一个聚点.则$M_k=((a-\frac{1}{k},a+\frac{1}{k})\setminus \{a\})\cap M,k\in \N_+$是个无限集.

那只需要取$x_{n_k}$为$M_k$中的某个$x_{n'}$满足$n'>n_{k-1}$即可,由于这是无限集,所以显然可以取出.拿出来的这个数列显然以$a$为极限.

#### 柯西收敛准则

称一个数列为柯西数列,当且仅当$\forall \epsilon>0,\exists N>0,\forall n,m\geq N,|x_n-x_m|<\epsilon$.

一个数列收敛的充要条件是他是一个柯西数列.

先证必要性:

$\forall \epsilon>0,\exists N>0,\forall n,m\geq N,|a-x_n|<\frac{\epsilon}{2},|a-x_m|<\frac{\epsilon}{2},|x_n-x_m|\leq |x_n-a|+|a-x_m|<\epsilon$.

再证充分性:

考虑取$\epsilon=1$,则$\exists N_1,\forall n\geq N_1,|x_n-x_{N_1}|<1$,于是$|x_n|<|x_{N_1}|+1$,于是$\forall n,|x_n|<\max_{k=1}^{N_1}\{|x_k|\}+1$,于是这个数列有界,那么存在无穷子列有极限$a$.

那么$\forall \epsilon>0,\exists N>0,\forall n_k,m\geq N,|x_{n_k}-x_m|<\epsilon$,其中$\{x_{n_k}\}$以$a$为极限,于是可以取足够远的$x_{n_k}$.接下来就是简单说明的环节了.

柯西收敛准则的充分性还可以用上下极限证明:

取$\forall \epsilon>0,\exists N,\forall n\geq N,x_n\in (x_N-\epsilon,x_N+\epsilon)$,那么上下极限之差$\leq 2\epsilon$.

于是引出了实数的第二种定义方式:我们定义两个柯西列等价,有$\{x_n\}\sim \{y_n\}\Leftrightarrow \lim_{n\rightarrow \infty}|x_n-y_n|=0$.那么所有有理数组成的柯西列的等价类构成实数集合.不难证明这样定义出来的实数和戴德金分割等价.

这个定义方式只依赖于距离的定义.而且我们可以证明任何一次扩充后得到的空间一定是封闭的.用这个例子就是说你对于一个实数的柯西列,证明它一定收敛到一个实数.那么就可以把这个柯西列的每一个数(作为实数,实际上对应了一个柯西列)中抽出一个数来,然后证明这些数组成的新的柯西列与刚才那个柯西列等价,这样就证明了其封闭性.其实只要第$k$个数把其代表的任意一个柯西列中选第$k$个元素构造即可.

#### 上下极限

我们推广广义极限,也就是一个数列可以以$+\infty$或者$-\infty$为极限.这个容易用$\epsilon-N$语言写出来.这样的话我们立刻得到单调数列有极限.

那我们取任意一个数列从某一项往后的上确界$h_n=\sup_{k\geq n}\{x_k\}$和下确界$l_n=\inf_{k\geq n}\{x_k\}$,那么注意到$\{l_n\}$单调不降,$\{h_n\}$单调不增,所以二者都存在极限.由此可以定义上下极限,并且任意数列都一定存在上下极限.

注意到$\lim_{n\rightarrow \infty} l_n=\sup_{n\geq 1}\inf_{k\geq n}\{x_k\},\lim_{n\rightarrow \infty} h_n=\inf_{n\geq 1}\sup_{k\geq n}\{x_k\}$,于是$\varliminf_{n\rightarrow \infty} x_n= \lim_{n\rightarrow \infty}l_n\leq \lim_{n\rightarrow\infty}h_n=\varlimsup_{n\rightarrow \infty} x_n$.

那么$x_n$存在极限当且仅当$\varlimsup_{n\rightarrow \infty}x_n=\varliminf_{n\rightarrow \infty} x_n$.

充分性很好证明,只需要使用夹逼定理即可.

必要性也很好说如果极限不是$\infty$,$\forall \epsilon>0,\exists N,\forall n\geq N,a-\epsilon<x_n<a+\epsilon$,那么$a-\epsilon<l_n\leq h_n<a+\epsilon$.自然得到两者极限相等.是$\infty$也很好说.

上下极限同样有保序性,相反数有$\varlimsup (-x)=-\varliminf x$,但是不能简单的四则运算,而是有$\varliminf x_n+\varliminf y_n\leq \varliminf (x_n+y_n)\leq \varlimsup(x_n+y_n)\leq \varlimsup x_n+\varlimsup y_n$,我们这里先不讨论出现$\infty-\infty$的情况.

证明的话只需注意到$\inf_{k\geq n}x_k+\inf_{k\geq n}y_k\leq x_m+y_m,\forall m\geq n$,因此对右边可以取下确界,然后两边一起取极限即可.

另外还有结论是$\varliminf (x_n+y_n)\leq \varlimsup x_n+\varliminf y_n$.这个又如何证明呢?只需证明:
$$
\varliminf (x_n+y_n)-\varlimsup x_n\leq \varliminf y_n\\
\varliminf (x_n+y_n)+\varliminf (-x_n)\leq \varliminf y_n\\
$$
于是证毕.

乘法的话需要规定$x_n,y_n>0$,然后和加法的不等式形式是类似的(同样先不考虑$0\times \infty$的情况).

如果$\{x_n\},\{y_n\}$其中一个存在极限,那么上面的不等式就可以取等了(当然仍然不讨论出现$\infty-\infty$和$0\times \infty$的情况).如果$x_n=y_n>0$的话上面的乘法不等式也可以取等,换言之$x_n>0$的时候$\varlimsup x_n^2=(\varlimsup x_n)^2$成立.

###### Example1

对于数列$\{a_n\}$,满足$a_n>0$,$a_{n+2}=\frac{2}{a_{n+1}+a_n}$.

求证:

1. $\{a_n\}$有界.
2. $\lim_{n\rightarrow \infty}a_n=1$.
3. $\exists \lambda\in(0,1),c>0$,$\forall n,|a_n-1|<c\lambda^n$.
4. 对所有满足条件的数列,求$\min(\lambda)=\frac{\sqrt 2}{2}$.

对于(1),取$M=\max(\max(a_1,a_2),\frac{1}{\min(a_1,a_2)})$,数学归纳可证明$a_n\in [\frac{1}{M},M]$.

两边取一下上极限和下极限,立刻得到$1\leq \overline{a}\underline{a}\leq 1$,于是$\overline{a}\underline{a}=1$.

后面不会了(乐).

#### Stolz定理

如果$\lim_{n\rightarrow \infty}\frac{a_{n+1}-a_n}{b_{n+1}-b_n}=A$,并且$\{b_n\}$单调递增且发散,那么$\lim_{n\rightarrow \infty}\frac{a_n}{b_n}=A$.

这个怎么证明呢?先取$c_n=a_n-Ab_n$,考虑$\forall \epsilon>0,\exists N>0,\forall n\geq N$,有:
$$
c_{n+1}-c_n\leq  |c_{n+1}-c_n|<\epsilon(b_{n+1}-b_n)
$$
两边求和,立刻有:
$$
c_{n}-c_N< \epsilon(b_{n}-b_N)\\
\frac{c_n-c_N}{b_n-b_N}< \epsilon\\
\frac{\frac{c_n}{b_n}-\frac{c_N}{b_n}}{1-\frac{b_N}{b_n}}<\epsilon
$$
由于$b_n$发散,对左边取极限就得到$\lim_{n\rightarrow \infty}\frac{c_n}{b_n}=0$,也就证明了原结论.

如果将上述条件改为:

1. $\{b_n\}$单调递减且$\lim b_n=0$.
2. $\lim a_n=0$.

上述定理也是成立的.

证明仍然是上面那个过程,只是做一些小修改:

$$
c_{N}-c_n< \epsilon(b_{N}-b_n)\\
\frac{c_N-c_n}{b_N-b_n}< \epsilon\\
\frac{\frac{c_N}{b_N}-\frac{c_n}{b_N}}{1-\frac{b_n}{b_N}}<\epsilon
$$

两边取极限.

把Stolz定理推广到上下极限上.换言之,当:

1. $\{b_n\}$单调上升且趋近于无穷.

或:

1. $\{b_n\}$单调递减且$\lim b_n=0$.
2. $\lim a_n=0$.

此时应当有:

$$
\varliminf\frac{a_{n+1}-a_n}{b_{n+1}-b_n} \leq \varliminf \frac{a_n}{b_n}\leq \varlimsup \frac{a_n}{b_n}\leq \varlimsup \frac{a_{n+1}-a_n}{b_{n+1}-b_n}
$$

上述命题的一个平凡推论是:

$$
\varliminf (a_{n+1}-a_n)\leq \varliminf\frac{a_n}{n}\leq \varlimsup \frac{a_n}{n}\leq \varlimsup(a_{n+1}-a_n)
$$

### 实数的完备性基本定理

我们将以下定理称作实数的完备性的基本定理:

0. 戴德金分割定理
1. 确界存在定理
2. 单调收敛准则
3. 闭区间套定理(+阿基米德性质)
4. 有限覆盖定理
5. 聚点原理
6. 致密性定理
7. 柯西收敛准则(+阿基米德性质)

这七条定理(除了3和7,它们需要与阿基米德性质配合使用)全部与戴德金分割定理等价.

我们上面已经论证了其中的部分推导关系,我们在下面声称它们分为若干组推导链条,分别代表了实数的不同性质.

### 函数极限

#### 定义

函数的极限类型比较多:

1. $x$是从一点$a$的右侧趋向于$a$,这时记作$x\rightarrow a+0$.
2. $x$是从一点$a$的左侧趋向于$a$,这时记作$x\rightarrow a-0$.
3. $x$同时从一点$a$的两侧趋向于$a$,这时记作$x\rightarrow a$.
4. $x$无限制增大,记作$x\rightarrow +\infty$.
5. $x$无限制减小,记作$x\rightarrow -\infty$.
6. $|x|$无限制增大,$x$同时增大或减小,记作$x\rightarrow \infty$.

序列极限的定理在函数极限的条件下几乎也全都成立.

##### 单侧极限

即$x\rightarrow a+0$和$x\rightarrow a-0$,这两种是对称的,我们只讨论前一种.

若$\exists l,\forall \epsilon>0,\exists \delta>0,\forall 0<x-a<\delta,|f(x)-l|<\epsilon$.我们称$l$是$x\rightarrow a+0$时$f(x)$的**右极限**,记作$\lim_{x\rightarrow a+0}f(x)=l\\$,同理可以定义**左极限**.

##### 双侧极限

若左右极限都存在且相等,那么双侧极限也存在并且$\lim_{x\rightarrow a}f(x)=\lim_{x\rightarrow a+0}f(x)=\lim_{x\rightarrow a-0}f(x)\\$.

##### 自变量趋于无穷时的极限

设函数$f(x)$在$(a,+\infty)$上有定义,若$\exists l$,$\forall \epsilon,\exists A\geq a,\forall x>A,|f(x)-l|<\epsilon$,则记$\lim_{x\rightarrow +\infty}f(x)=l\\$,同理定义$\lim_{x\rightarrow -\infty}$,若二者均存在且相等,定义$\lim_{x\rightarrow \infty}=\lim_{x\rightarrow +\infty}=\lim_{x\rightarrow -\infty}$.

#### 上下极限

定义去心邻域$U_0(x_0,\delta)=(x_0-\delta,x_0+\delta)\setminus \{x_0\}$.

定义$\varlimsup_{x\rightarrow x_0}f(x)=\lim_{\delta\rightarrow +0}\sup_{x\in U_0(x_0,\delta)}f(x)$,下极限同理.

显然$\lim_{x\rightarrow x_0}f(x)=A\Leftrightarrow \varlimsup_{x\rightarrow x_0}f(x)=\varliminf_{x\rightarrow x_0}f(x)=A$.

#### 第一可数公理

函数极限当然满足四则运算法则,保序性,夹逼定理,单调收敛准则,柯西收敛准则.

事实上函数极限和数列极限满足关系:

$\forall \{x_n\}\in U_0(x_0)$,如果$\lim_{n\rightarrow \infty}x_n=x_0$,那么$\lim_{n\rightarrow \infty}f(x_n)=\lim_{x\rightarrow x_0}f(x)$,前提是这两个极限都存在.

证明是堆$\delta-\epsilon$语言,不再赘述.

而它们之间关系的本质其实是:$x_0$处的"任意小邻域"其实可以用某列邻域替代.

我们定义邻域基:一个由$x_0$的邻域组成的集合$U$,如果对于任意小邻域,$U$中都存在一个邻域比这个邻域更小,那么就称$U$是一个邻域基.$\R$的特殊之处在于任意一个点都有可数邻域基,这被称为第一可数公理($C_1$公理).

另外$\R$是可分的,也就是它存在可数稠密集($\Q$).

#### 无穷大量和无穷小量

如果$x\rightarrow x_0$时,$f(x)$有界,称$f(x)=O(1)$.

如果$x\rightarrow x_0$的时候,$f(x)\rightarrow 0$,称$f(x)=o(1)$.

如果$f,g=o(1)$,$\frac{f}{g}\rightarrow 1$,那么称二者为**等价无穷小**;如果$\frac{f}{g}=c\ne 0$,则称二者为同阶无穷小(可记作$f\sim g$);如果$\frac{f}{g}=0$,则称$f$为$g$的**高阶无穷小**,可记作$f=o(g)$.

同阶无穷小在做乘除法的时候是可以替换的.

如果$\frac{f}{g}$是有界的,那么可以写作$f=O(g)$.

###### Example1

$\lim_{x\rightarrow 0}\frac{\sin x}{x}=1$.

因为其左右对称,考虑取$x\in (0,\frac{\pi}{2})$我们有$\sin x<x<\tan x$(用单位元的面积证明).

那么考虑$\frac{\sin x}{x}<1$,又注意到:

$$
\frac{\sin x}{x}>\frac{\sin x}{\tan x}=\cos x
$$

夹逼定理一下就行.

类似可以证明$\lim_{x\rightarrow 0}\frac{\tan x}{x}=1,\lim_{x\rightarrow 0}\frac{1-\cos x}{x^2}=\frac{1}{2},$

###### Example2

$\lim_{x\rightarrow 0}\frac{x-\sin x}{x^3}=\frac{1}{6}$.

下面有个三次方,考虑套用一下三倍角公式,我们知道:

$$
f(x)=\frac{x-\sin x}{x^3}=\frac{x-(3\sin \frac{x}{3}-4\sin^3\frac{x}{3})}{x^3}\\
=\frac{3(\frac{x}{3}-\sin \frac{x}{3})+4\sin ^3\frac{x}{3}}{(\frac{x}{3})^3}\frac{1}{27}\\
=\frac{1}{9}f(\frac{x}{3})+\frac{4}{27}(\frac{\sin\frac{x}{3}}{\frac{x}{3}})^3
$$

看上去两边取上下极限就做完了对吧!但是还差一点,因为我们要证明上下极限都不是无穷,起码得说明它有界.

注意到$\frac{x-\sin x}{x^3}<\frac{\tan x-\sin x}{x^3}=\frac{\sin x}{x}(\frac{1-\cos x}{x^2\cos x})=\frac{\sin x}{x}(\frac{2\sin^2\frac{x}{2}}{x^2\cos x})$,这样两边限定一下范围再取上极限可以说明原式上极限有界.

###### Example3

求$\lim_{x\rightarrow 0}\frac{\sin x-\tan x}{x^3}$.

考虑$\frac{\sin x-\tan x}{x^3}=\frac{\sin x}{x}\frac{1}{x^2}\frac{\cos x-1}{\cos x}=\frac{\tan x}{x}\frac{\cos x-1}{x^2}=-\frac{1}{2}$.

###### Example4

$\lim_{t\rightarrow 0}\frac{\log_{a}(1+t)}{t}=\frac{1}{\ln a}$.
$$
\lim_{t\rightarrow 0}\frac{\log_{a}(1+t)}{t}=\log_a\lim_{t\rightarrow 0}(1+t)^{\frac{1}{t}}=\log_ae=\frac{1}{\ln a}
$$

###### Example5

$\lim_{t\rightarrow 0}\frac{a^t-1}{t}=\ln a$.

令$s=a^t-1$,则原式等于$\lim_{s\rightarrow 0}\frac{s}{\log_a(s+1)}=\ln a$.

###### Example6

$\lim_{t\rightarrow 0}\frac{(1+t)^\mu-1}{t}=\mu$.

令$s=(1+t)^\mu-1$,那么原式等于$\lim_{t\rightarrow 0,s\rightarrow 0}\mu\frac{s}{\ln(1+s)}\frac{\ln(1+t)}{t}=\mu$.

#### 连续函数

不妨设$f(x)$在$(a,b)$上有定义,若对于$x_0\in(a,b)$,$f(x)$在$x_0$处有双侧极限且$\lim_{x\rightarrow x_0}f(x)=f(x_0)$,称其在$x_0$处**连续**,$x_0$称为**连续点**,若$\forall x_0\in (a,b)$,$f(x)$在$x_0$处连续,则称其在$(a,b)$上**连续**.连续性相当于说:取极限和$f$可以交换顺序,换言之$\lim_{x\rightarrow x_0}f(x)=f(\lim_{x\rightarrow x_0}x)$.

我们也可以类似上面定义**左连续**和**右连续**,若$f(x)$在$[a,b]$上有定义,且在$(a,b)$上连续,在$a$处右连续,在$b$处左连续,称其在$[a,b]$上连续.

如果$f(x)$在$x_0$附近有定义且不连续,则称$x_0$是一个**间断点**,间断点有以下几种:

1. 如果$\lim_{x\rightarrow x_0+0}f(x)$和$\lim_{x\rightarrow x_0-0}f(x)$都存在但不相等或者不等于$f(x_0)$,则称为**第一类间断点**.如果是后者情况,可以通过修改$f(x_0)$的定义来去掉这个间断点,因而一般被称为**可去间断点**.
2. 反之,如果$\lim_{x\rightarrow x_0+0}f(x)$和$\lim_{x\rightarrow x_0-0}f(x)$至少有一个不存在,则称为**第二类间断点**.

使用单调收敛准则,可证明单调函数没有第二类间断点.并且单调函数的间断点只有可数个(几乎处处连续).

这个怎么证明呢?考虑设$M$是函数$f$的第一类间断点集,取$x\in M,I_x=(f(x_0-0),f(x_0+0))$,由于函数单调,所以这些$I_x$两两不交.考虑映射$M\rightarrow \Q,x\mapsto \min(I_x\cap \Q)$,也就是考虑每一个$I_x$中都有若干个有理数,且互不相交,那么显然这个映射是个单射,$M$可数.

事实上有更强的结论:任意函数的第一类间断点集都是可数的.

设$M$是函数$f$的第一类间断点集,取$M_n=\{x\in M\mid |f(x+0)-f(x-0)|>\frac{1}{n}\},M'=\{x\in M|f(x+0)=f(x-0)\ne f(x)\}$.

注意到$M=(\bigcup_{n\geq 1}M_n)\cup M'$.

我们定义孤立点:如果$x\in A,\exists \sigma,(x-\sigma,x+\sigma)\nsubseteq A$,则称$x$是$A$中的一个孤立点.类似刚才证明可数性,我们每个孤立点都可以取一段邻域,领域中一定存在有理数,所以任意一个集合中的孤立点个数一定是可数的.

又发现所有的$x\in M_n$,$x$都是孤立点,原因很显然因为在$x$的两侧小区间都在趋近于一个极限,所以它们肯定不是第一类间断点.所以所有的$M_n$都是可数集.

那怎么证明$M'$是可数集呢?考虑取$M'_n=\{x\in M| |f(x)-f(x-0)|>\frac{1}{n}\}$,现在$M'=\bigcup_{n\geq 1}M'_n$,而$M'_n$一定是孤立点集,所以$M'$可数.

综上就可以看出$M$是一个可数集.

而第二类间断点则是可以有任意多个,例如对于迪利克雷函数,其处处都是第二类间断点.



由于连续性由极限定义,这意味着在$(a,b)$上的连续函数的有定义的四则运算必定得到的还是在$(a,b)$上的连续函数.那么,如果我们对于一个区间$I$,定义$C(I)$为所有在$I$上连续的函数组成的集合,$C(I)$就是一个线性空间.且不难发现,连续函数的复合仍然是连续函数.这其实对应着$\lim_{x\rightarrow a}g(f(x))=g(\lim_{x\rightarrow a}f(x))=g(f(a))\\$.

另外,如果一个连续函数有单值反函数,则反函数必连续.

不难证明,所有的初等函数在定义域上都是连续的.



另外:连续周期非常值函数一定有最小正周期.

考虑设正周期的集合是$\{t\}$,取$T=\inf \{t\}$.

当$T=0$的时候,取一个正周期的数列$\{t_n\}\rightarrow 0$,然后$\forall x<y,x_n=x+\lfloor\frac{y-x}{t_n}\rfloor t_n,|y-x_n|\leq t_n\rightarrow 0$,于是$\lim_{n}f(x_n)=f(y)$,而恒有$f(x)=f(x_n)$于是这是一个常值函数.

反之,则取$\{t_n\}\rightarrow T,f(x+T)=\lim_{n}f(x+t_n)=f(x)$.

##### 一致连续

假设$f:I\rightarrow \R$,若$\forall \epsilon>0,\exists \delta>0,\forall x_1,x_2\in I,|x_1-x_2|<\delta,|f(x_1)-f(x_2)|<\epsilon$,则称其**一致连续**.一致连续性是一个比连续性强得多的性质,例如$f(x)=x^2$就不是一致连续的.

##### 李氏连续

假设$f:I\rightarrow \R$,若$\exists L,\forall x_1,x_2\in I,|f(x_1)-f(x_2)|\leq L|x_1-x_2|$,则称其**李氏连续**.李氏连续当然强于一致连续.

#### 闭区间下的连续函数

##### 介值定理(零点存在定理)

设$f(x)$在$[a,b]$上有定义且连续,$f(a)\ne f(b)$,则$\forall \eta\in(f(a),f(b)),\exists c\in (a,b),f(c)=\eta$.这当然等价于$f(a)f(b)<0\Rightarrow \exists c\in (a,b),f(c)=0$.

考虑取$f(\frac{a+b}{2})$,如果$f(\frac{a+b}{2})=0$那就完事了.不然的话考虑$f(a)f(\frac{a+b}{2})$和$f(\frac{a+b}{2})f(b)$中必定有恰好一个小于$0$,这当然给出一个闭区间套,不妨设最后套出了一个单点$c$.此时取极限立刻有$0\geq \lim f(a_n)f(b_n)=f(c)^2$,于是$f(c)=0$.

也有一个存在性证明:反证,如果$\forall x\in [a,b]$,$f(x)\ne 0$.那么$\forall x\in [a,b],\exists \delta(x)>0$,$f$在$(x-\delta(x),x+\delta(x))$与$f(x)$同号.根据有限覆盖定理,我们可以拿出有限个开区间来覆盖整个区间.此时可以将这些开区间排序,相邻两个区间两两有交,自然导出矛盾.

介质定理还有两个推论:

1. **介值定理的推论1**:设$f(x)$在$[a,b]$上有定义且连续,那么$[f(a),f(b)]\subseteq$其值域.
2. **介值定理的推论2**:单调连续函数的反函数仍然是单调连续函数.

推论(1)显然,考虑推论(2)的证明:

考虑对于连续单增函数$f(x)\in C(a,b),\alpha=f(a+0),\beta=f(b-0)$.先要证明其存在反函数$\varphi\in C(\alpha,\beta)$.

这里有一点小问题是我们要用介值定理要把定义域控制在闭区间,但这当然是好做的,对于任何一个$c\in (a,b)$,你一定可以取一个$a_1,b_1$使得$f(a_1)< f(c)< f(b_1)$,这样$f(x)\in C[a_1,b_1]$.

这样的话容易说明$f$是$(a,b)\rightarrow (\alpha,\beta)$的双射,于是反函数是存在的.反函数的单调性是好证的.

对于反函数的连续性,反证,假设其在某一点$c\in (\alpha,\beta)$处不连续,必然意味着$a<\varphi(c-0)<\varphi(c+0)<b$.此时容易说明$\varphi(x)\in (a,b)\setminus (\varphi(c-0),\varphi(c+0))\cup\{\varphi(c)\}$,那只需要取$w\in (\varphi(c-0),\varphi(c+0))\setminus\{\varphi(c)\}$,$\nexists x,\varphi(x)=w$,这当然与我们上述结论不符.

###### Example1

对于连续函数$f\in C[a,b],\forall x_0\in [a,b]\setminus \Q,\varlimsup _{\Delta x\rightarrow 0+0}\frac{f(x_0+\Delta x)-f(x_0)}{\Delta x}\geq 0$,求证:$f$单调递增.

先考虑把条件改成$\forall x_0\in [a,b]$怎么做.

一个想法是反证,如果不单调的话,找一个点使得它的右上导数小于$0$.最自然的想法是找最大值,但最大值有可能只能证明右上导数$\leq 0$.所以有一个想法是拿一条稍微斜一点(也就是斜率为较小的负数)的直线去截整个曲线.

下面反证,如果其不单增,总能找到两个点$a'<b'$,使得$f(a')>f(b')$.此时任意取一个$\epsilon_0<f(a')-f(b')$,然后取一个点$B=(b',f(b')+\epsilon_0)$.当然可以再取一个$\epsilon>0$使得$\epsilon+\epsilon_0<f(a')-f(b')$,然后取一个点$A=(a',f(b')+\epsilon_0+\epsilon)$,这样$AB$这条直线确定了一条斜率$k<0$的一次函数$g(x)$,不妨设$h(x)=f(x)-g(x)$,容易发现$h(a')>0,h(b')<0$,所以可以取一个变号零点$x_\epsilon=\sup\{x|h(x)\geq 0,x\in [a',b']\}$.当$\epsilon$取值不一样的时候当然可以取不同的而且是不可数个$x_\epsilon$,此时可以取出一个无理数$x_0$,那考虑$\varlimsup _{\Delta x\rightarrow 0+0}\frac{f(x_0+\Delta x)-f(x_0)}{\Delta x}=\varlimsup_{\Delta x\rightarrow 0+0} \frac{h(x_0+\Delta x)}{\Delta x}+k\leq k<0$,这就矛盾了.

##### 有界定理

设$f(x)$在$[a,b]$上有定义且连续,$\exists A,B$,$\forall x_0\in[a,b],A\leq f(x_0)\leq B$.

可以用之前的有限覆盖定理所证明的闭区间上的局部有界函数是有界函数,然后还有另一个证明:

假设无界,则存在一个数列$\{x_n\}$使得$|f(x_n)|\rightarrow +\infty$.由于$\{x_n\}$是有界数列,其一定有收敛子列$\{x'_n\}$收敛于$x'$,当然$|f(x')|=\lim_{n\rightarrow \infty}|f(x_n)|=+\infty$,这当然产生了矛盾.

还有一个闭区间套的证明方式:你考虑每次区间分两半,一定有其中一半是无界的,然后做闭区间套.

##### 最值定理

设$f(x)$在$[a,b]$上有定义且连续,$\exists x_1,x_2\in[a,b]$,$\forall x_0\in[a,b],f(x_1)\leq f(x_0)\leq f(x_2)$.

考虑先用有界定理得知$f(x)\in C[a,b]$有界,然后就可以拿到其上下确界$m\leq f(x)\leq M$,只需证明其上下确界均能取到就行.

反证,考虑若$M$取不到,那考虑定义$g(x)=\frac{1}{M-f(x)}>0$,根据有界定理$g(x)$有上界$C$,于是$\frac{1}{M-f(x)}\leq C\Rightarrow f(x)\leq M-\frac{1}{C}$,矛盾.

或者我们知道$M$是上确界则$M-\frac{1}{n}$不是上界,取出$\{x_n\}$使得$M-\frac{1}{n}<f(x_n)\leq M$,那么$\{x_n\}$必然有收敛子列,设其收敛于$x_0$,那么根据夹逼定理必然能得到$f(x_0)=M$.

##### 康托尔定理

有界闭区间上的连续函数必然一致连续.

考虑反证,反命题是$\exists \epsilon_0>0,\forall n>0,\exists x'_n,x''_n,|x'_n-x''_n|<\frac{1}{n},|f(x'_n)-f(x''_n)|\geq \epsilon_0$.

用致密性定理,取出$\{x'_n\}$的收敛子列,可以拿到$\{x''_n\}$与其对应的数列,二者必然同时收敛于一个值$x_0$,那么$|f(x'_n)-f(x''_n)|=0\geq \epsilon_0$,当然矛盾.

如果是开区间呢?如果$f(x)\in C(a,b]$,其在开区间$(a,b]$上一致连续,这需要等价于$\forall x_n\in (a,b),x_n\rightarrow a$,$\{f(x_n)\}$是柯西列(也就是$f(a+0)$极限存在),由此看出开区间上的一致连续函数必然能延拓称闭区间上的连续函数.

#### 不动点和周期点

定义$f(x)$的**不动点**$\text{fix} f(x)\Leftrightarrow f(x)=x$,**周期点**$P_n=\{x|f^{[n]}(x)=x\and \forall 0<m<n,f^{[m]}(x)\ne x\}$,其中$f^{[n]}(x)$表示将$f$复合$n$次.

##### 压缩映照原理

设$f(x)$在$[a,b]$上有定义并且$f([a,b])\subseteq [a,b]$,并且满足$\exists 0\leq q<1,\forall x,y\in [a,b],|f(x)-f(y)|\leq q|x-y|$,那么$[a,b]$上存在唯一的不动点$c$.

任取一个点$x_0\in [a,b]$,考虑$x_{n+1}=f(x_n)$,然后尝试判断这个数列的极限,注意到:
$$
|x_{n+1}-x_n|=|f(x_n)-f(x_{n-1})|\leq q|x_n-x_{n-1}|\leq q^{n}|x_1-x_0|\\
|x_{n+p}-x_n|\leq \sum_{k=1}^p|x_{n+k}-x_{n+k-1}|\\\leq \sum_{k=0}^{p-1}q^{n+k}|x_1-x_0|=|x_1-x_0|q^n\frac{1-q^{p}}{1-q}
$$
所以$\{x_n\}$是柯西列,其极限存在,取$c=\lim x_n$,容易发现:
$$
|f(x_n)-f(c)|\leq q|x_n-c|
$$
两边取极限得知$f(c)=c$,于是$c$是一个不动点.如果还存在一个不动点$c'\ne c$,那么:
$$
|c-c'|=|f(c)-f(c')|\leq q|c-c'|
$$
不符.

##### 李-约克定理

对于一个$f(x)\in C[a,b]$,如果$P_3\ne \empty$,那么$\forall n\in \N_+$,$P_n\ne \empty$.

###### 引理一

设$J$是实区间,$G:J\rightarrow \R$是连续函数,$\forall I=[a,b]\subseteq G(J)$,$\exists Q=[a',b']\subseteq J,G(Q)=I$.

根据介值定理,当然存在一对点$p,q$,$I=[G(p),G(q)]$.不妨设$p<q$,那么$a'=\sup \{x\in [p,q]|G(x)=G(p)\},b'=\inf\{x\in [a',q]|G(x)=G(q)\}$即可.

换言之,我们可以拿到一个区间使得它的像正是我们想要的区间.感性理解的话我们可以对一个区间求原像并且原像当然也是一个区间.那我们自然可以对区间进行迭代.这样迭代的区间有什么性质么?

###### 引理二

设$J$是实区间,$F:J\rightarrow J$是连续函数,取一列闭区间$I_n$,满足$\forall n\geq 0,I_n\subseteq J,I_{n+1}\subseteq F(I_n)$.不妨把这种区间列记作$I_0\Rightarrow I_1\Rightarrow I_2\Rightarrow \cdots$.

那么我们可以找到一列闭区间$I_0=Q_0\supseteq Q_1\supseteq Q_2\cdots$,$\forall n\geq 0$,有$F^{[n]}(Q_n)=I_n$.

考虑数学归纳,假设对于$\leq n$的都已经满足,现在有$I_{n+1}\subseteq F(I_n)=F^{[n+1]}(Q_n)$,令$G=F^{[n+1]}$然后用引理一就可以.

这个引理的意义在于,$\{I_n\}$当然不必是一条链,但是只要其满足一定的性质,我们就可以在它们中找到一条链,也就是每迭代一次都可能会将射出去的那个集合变小,或者是射出需要的集合所需要的初始集合变小.

这个当然也是给我们求原像的机会.我们尝试感性理解一下这个在干什么:

如果一个函数操作完一个区间后会把这个区间变小,那显然我如果想求$F^{[-n]}(I)$,$n$越大得到的原像就越大对吧.反之,如果会把这个区间变大,那$n$越大得到的原像就越小对吧.

这个讨论显然是不好的,我们得想办法刻画一下这个东西.那当然要在这里蕴含一个$F$本身.

于是用上面的定义会发现,即使$F$会把这个区间扩张到奇怪的大小,只要我们能把它缩回去,那不断取原像的过程照样是可行的.

###### 引理三

设$J$是实区间,$G:J\rightarrow \R$是连续函数,$I\subseteq J$是闭区间,并且$I\subseteq G(I)$或者$G(I)\subseteq I$,都可以推出$\exists p\in I,G(p)=p$.

只需构造函数$H(x)=x-G(x)$然后用介值定理即可.

###### 李-约克定理

设$q_3\in P_3$,则$F(q_3),F(F(q_3))\in P_3$,容易看出,这三个点中必然有一个点$a\in P_3$,使得$a,b=F(a),c=F(F(a))$这三个数单调递增或单调递减.不妨设$a<b<c$.

下面证明对于任意$k>0$,存在$k$周期点.

我们接下来取$K=[a,b],L=[b,c]$,立刻由介值定理发现$F(K)\supseteq L$以及$F(L)\supseteq [a,c]=K\cup L$.

接下来我们开始构造一列满足引理二的闭区间,我们考虑:

1. 当$k=1$的时候,我们直接取$\forall n,I_n=L$.
2. 当$k>1$的时候,取$I_n=\begin{cases}K&n\equiv 0\pmod k\\L&otherwise\end{cases}$

现在我们可以用引理二立刻拿到区间列$\{Q_n\}$,我们发现$Q_k\subseteq Q_0=I_0=I_k=F^{[k]}(Q_k)$,根据引理三立刻得到$\exists q\in Q_k,F^{[k]}(q)=q$.

那么这个$q$有没有可能存在更小的周期$d$呢?如果有的话,不妨取其中最小的那个$d$,辗转相除一下自然有$d|k$,那么当然要有$F^{[k]}(q)=F^{[k-d]}(q)=q$.

此时我们发现$q\in Q_k\subseteq Q_{k-d}$,$q\in I_{k}\cap I_{k-d}=K\cap L=\{b\}$,于是$F^{[k]}(q)=b,F^{[k+1]}(q)=c,F^{[k+2]}(q)=a$.

这说明此时$d=3$.

而当$k\geq 4$的时候,$F^{[k+2]}(q)=a\notin L$,这当然与$I_{k+2}=L$矛盾.

仔细观察一下上面的过程,我们到底依赖于什么东西得到了这个?

首先找到$q$使得$F^{[k]}(q)=q$这个其实反而并不是最主要的,因为只要$F(q)=q$那么显然$F^{[k]}(q)=q$,真正离谱的是竟然要让这个点在之前从未跳到过$q$.这也是这个证明的最精髓的部分,那就是证明任何一个$d<q$都不是周期.

那我们考虑取一个区间(也就是$K$),在$K$中取一个合法的$q$,然后我们只要找到一个闭区间(也就是$Q_k\subseteq K$),使得它当且仅当复合$k$次的时候才会打到$K$上,在其它时候都得打在$K$的外面(在这里是$L$)上,那自然是可行的了对吧.而数字$3$恰好完美地给出了这个构造.

如果我们再总结一下上面的过程,我们需要找到两个区间$K,L$,找到一个$Q_k\subseteq K$,使得$F^k(Q_k)$可以打到$K$上但是对于$d<k$,$F^d(Q_k)$打到$L$上.于是根据引理三其存在$k$周期点,但是几乎不可能存在$d$周期点(这里就需要简单判断一下$L\cap K$的部分了)

这样的$K$和$L$怎么构造呢?如果我们构造$F(K)$打到$L$上,$F(L)$打到$K$上显然是吧不合理的,那就可以一个打到另一个,另一个打到全集上再用引理一缩一下,这个看上去合理多了.这当然就是$F(K)\supseteq L$以及$F(L)\supseteq [a,c]=K\cup L$.

##### Sharkovskii定理

定义如下的一种序关系$\prec$(称为Sharkovskii序):
$$
3\prec 5\prec 7\prec \cdots \\
\prec 2\times 3\prec 2\times 5\prec \cdots \\
\cdots\\
\prec 2^k\times 3\prec 2^k\times 5\prec \cdots \\
\cdots\\
\cdots\prec 2^n\prec 2^{n-1}\prec \cdots\prec 4\prec 2\prec 1
$$
对于$f(x)\in C[l,r],f:[l,r]\rightarrow [l,r]$.那么如果$m\prec n,P_m\ne \empty\Rightarrow P_n\ne \empty$.

(首先发现当然存在不动点)

这咋办呢?我们刚才用了$3$的最重要的性质就是$3$个点可以排成有序的一列,如果有类似的引理,那我们就可以尝试推广上面的结论.

###### 引理

设$f:I\rightarrow I$是连续函数,$f$有$2n+1$周期点但无更小非一奇数阶周期点,我们取出其一个周期轨$x_0\rightarrow x_1\rightarrow \cdots x_{2n}$,必有以下两者成立其一:

1. $x_{2n}<x_{2n-2}<\cdots <x_2<x_0<x_1<\cdots <x_{2n-3}<x_{2n-1}$.
2. $x_{2n}>x_{2n-2}>\cdots >x_2>x_0>x_1>\cdots >x_{2n-3}>x_{2n-1}$.

这个引理怎么证明呢?考虑把周期轨排一下序然后归纳,我们不妨假设排好序后的结果是$y_0<y_1<\cdots<y_{2n}$.

如果引理成立,(第一种情况下)取$I_0=[x_0,x_1]$,则$I_0\Rightarrow I_0$.容易发现其实只有$I_0$有这个性质,那我们不妨来看看满足$[y_k,y_{k+1}]\Rightarrow [y_k,y_{k+1}]$的情况是什么样子的,尝试去满足$[f(y_{k+1}),f(y_{k})]\supseteq [y_k,y_{k+1}]$.这个时候注意到我们可以把所有点分成两类:满足$f(y_i)\geq y_{i+1}$(称为$\alpha$类点,容易见到这里也就等价于$f(y_i)>y_i$)和$f(y_i)\leq y_{i-1}$(称为$\beta$类点)的.那我们就是要找到一个$k$使得$y_k$是$\alpha$类点并且$y_{k+1}$是$\beta$类点.

由于$f(y_0)\geq y_1,f(y_{2n})\leq y_{2n-1}$,这两类点显然都有,那就一定存在交界的地方,这样我们就可以拿出这个分界的地方记作$I_0=[y_k,y_{k+1}]$,其中$y_k$是$\alpha$类点,$y_{k+1}$是$\beta$类点.

我们不妨定义一个集合$S$的张成区间$A$为$A=[\inf S,\sup S]$,那我们现在取$S_0=\{y_k,y_{k+1}\}$,设周期轨为$O_f=\{x_0,\cdots x_{2n}\}$,我们递归定义$S_{k+1}=f(A_k)\cap O_f$,容易根据上面对$S_0$的构造发现$S_0\subsetneq S_1$,并且$S_k$中因为取了一段极大的区间(但并没有包含所有周期点),所以一定能跳出这个区间(不然这个区间内部就可以自己射自己),所以$S_k\subsetneq S_{k+1}$.

那由此我们当然可以得到一条链:
$$
S_0\subsetneq S_1\subsetneq \cdots\subsetneq S_t=O_f
$$
按理来说应该有$t=2n-1$(这就会给出每次$S$扩张只会多一个数),让我们先来证明这个结论:

反证,假设$t<2n-1$,尝试找到一个更小的奇数周期来推出矛盾,不妨尝试找到一个$2n-1$的周期(当然,如果$2n+1=3$的话,这个结论我们在李约克定理已经证明了,所以下面假设$2n+1\geq 5$).

我不说你也知道我要干啥,我们考虑在末尾补$2n-t-1$个$A_t$,并把最后一个缩成$A_0$,当然有:
$$
A_0\Rightarrow A_1\Rightarrow A_2\Rightarrow\cdots\Rightarrow A_t\Rightarrow A_t\supseteq A_0
$$
但这样并不太行,因为这些区间的交不是空的,我们起码得把其中的一个区间给挪出去,不妨考虑改掉最后一个$A_t$,试图取一个$l\ne k$使得$[y_l,y_{l+1}]\Rightarrow A_0=[y_k,y_{k+1}]$.如果我们结合一下下面的主定理的证明部分,其实我们可以发现这里的$[y_l,y_{l+1}]$也就是$I_{2n-1}$.

我们上面已经保证了$f(y_{k+1})\leq y_k$,如果有$f(y_{k+2})\geq y_{k+1}$,那当然可以取$l=k+1$.也就是如果我们的这一列点类别中出现了$\alpha\beta\alpha$这样的结构就完事了.如果找不到,那说明$f(y_{k+2})\leq y_{k+1}$.我们继续往后找,如果可以遇到第一个$f(y_i)\geq y_{k+1} $,那$[y_{i-1},y_i]$自然是满足条件的.如果一直都找不到,说明始终有$f(y_i)< y_{k+1}\leq y_{i-1},i\in [k+2,2n]$,那说明后面全是$\beta$点.

同理如果出现了$\beta\alpha\beta$这样的结构也找到了.不然类似同理,如果一直找不到就说明前面全是$\alpha$点,前面的点始终满足$f(y_i)\geq y_{k+1},i\in [0,k]$.

这已经推导出了矛盾,因为我们发现前面的点射到后面,后面的点射到前面,问题在于前面有$k+1$个点,后面有$2n-k$个点,于是$k+1= 2n-k\Rightarrow 2k+1=2n$,这导出了矛盾.

所以确实可以找到这样的$l\ne k,[y_l,y_{l+1}]\Rightarrow [y_k,y_{k+1}]$,那我们用上面的区间列自然可以推导出存在一个$2n-1$周期点(或者其因子周期点,不过那也导出矛盾了).

于是$t=2n-1$,这表示我们拿到的这条链$S_0\subsetneq S_1\subsetneq \cdots\subsetneq S_t=O_f$每次只增加一个元素.如果$S_0$到$S_1$是只增加了一个元素,当然说明$f(y_k)=y_{k+1}$或者$f(y_{k+1})=y_k$.两者的证明是一样的,不妨设$f(y_k)=y_{k+1}$.

此时我们取$x_0=y_k,x_1=y_{k+1}$,此时其实也可以发现$x_2<x_0<x_1$.并且根据上面的说法,我们知道$A_{k}\subsetneq A_{k+1}$,那这个扩张一定是每次在边界上往外跑一个,也就是每次取一个新的迭代出来的结果扔到左边或者扔到右边.

网络上有关于这一点的证明往往是反证,假设不按规则走的话然后去尝试构造三周期点推到矛盾,但我们这里给出另一个证明方法:

考虑由于每次只增大一个数,那么如果我们能证明这一列$y$形如$\alpha\cdots\alpha\beta\cdots\beta$的形式就做完了,这等价于$I_0$的唯一性.

假设还有另一个区间也满足要求,对于这个新的区间,我们设其是$[y_l,y_{l+1}]$,我们再拿一个周期点$y_{l+2}$出来,假设这三个数按照顺序是$a<b<c$,那么根据上面的结论一定有$b\rightarrow c\rightarrow a$或者$b\rightarrow a\rightarrow c$成立,这就说明$b$被夹在了中间.而我们从一个$I_0$扩张当然是逐步的,会将$x_3,x_4\cdots$逐个加入,这必然意味着在$b$加入之前,$a,c$中已经有一个加进去了,但这是不可能的,只有$b$加入后才能把$a,c$给加进去.

###### sharkovskii定理

有了引理就可以开始类比了,不妨假设$x_{2n}<x_{2n-2}<\cdots <x_2<x_0<x_1<\cdots <x_{2n-3}<x_{2n-1}$.

现在我们要取区间列了,显然有一些奇偶对跳的现象,事实上我们取$I_0=[x_0,x_1],I_{2k-1}=[x_{2k},x_{2k-2}],I_{2k}=[x_{2k-1},x_{2k+1}]$显然就是满足条件的.而且这些区间除了端点外完全不相交(而且任意三个区间的交都是空集).这一列一共有$2n$个区间.我们事实上可以取$J=[x_{0},x_{2n-1}]=\bigcup I_{2k}$放到整个区间的最后.

接下来我们用这些东西去对于多种情况来对跳.

**Case1**

(假设$2n+1$是最小的非空的非一奇数周期)首先证明$P_{2n+1}\ne \empty\Rightarrow \forall 0<k,P_{k+2n}\ne \empty$.

在前面补$k$个$I_0$,在最后也补一个$I_0$,构造区间列:
$$
I_0\Rightarrow I_0\Rightarrow \cdots\Rightarrow I_0\Rightarrow I_1\Rightarrow I_2\Rightarrow \cdots\Rightarrow I_{2n-1}\Rightarrow I_0
$$
也就是在前面补$I_0$直到整个区间列的数量为$k+2n+1$,此时当然存在$x_0\in Q_{k+2n}$使得$f^{[k+2n]}(x_0)=x_0$,只需证明$\nexists d|(k+2n)$,$f^{[d]}(x_0)=x_0$即可.这当然好证,因为$d\leq \frac{k+2n}{2}\leq \max(k,2n)$,因为你只需要把它卡在那一堆$I_0$和$I_k$的分界线上就行.

**Case2**

(假设$2n+1$是最小的非空的非一奇数周期)接下来我们来证明$P_{2n+1}\ne \empty\Rightarrow \forall 0<m,P_{2m}\ne \empty$.

当$2m>2n+1$的时候我们已经证完了,接下来考虑$2m<2n+1$的情况.

由于$\forall k,I_{2n-1}\Rightarrow I_{2k}$,我们考虑取出$2m+1$个区间列,恰好有:
$$
I_{2n-2m}\Rightarrow I_{2n-2m+1}\Rightarrow \cdots\Rightarrow I_{2n-1}\Rightarrow I_{2n-2m}
$$
接下来就是简单说明的过程了.

**Case3**

接下来考虑证明:$P_{4}\ne \empty\Rightarrow P_2\ne \empty$.

这个证明就比较无聊了,我们进行暴力讨论,不妨假设$a<b<c<d$,那么:

1. $a\rightarrow b\rightarrow c\rightarrow d\rightarrow a$.

此时$[a,b]\Rightarrow [b,c]\Rightarrow [c,d]\Rightarrow [a,d]\supseteq [a,b]$,于是可证明有三周期点,当然有二周期点.

2. $a\rightarrow b\rightarrow d\rightarrow c\rightarrow a$.

此时$[a,b]\Rightarrow [b,d]\Rightarrow [c,d]\Rightarrow [a,c]\supseteq [a,b]$,于是可证明有三周期点,当然有二周期点.

3. $a\rightarrow c\rightarrow b\rightarrow d\rightarrow a$.

此时$[a,b]\Rightarrow [c,d]\Rightarrow [a,b]$,当然有二周期点.

4. $a\rightarrow c\rightarrow d\rightarrow b\rightarrow a$.

此时$[a,c]\Rightarrow [c,d]\Rightarrow [b,d]\Rightarrow [a,b]\Rightarrow [a,c]\supseteq [b,c]\Rightarrow [a,d]\supseteq [a,c]$.于是有五周期点,于是有二周期点.

5. $a\rightarrow d\rightarrow b\rightarrow c\rightarrow a$.

此时$[a,b]\Rightarrow [c,d]\Rightarrow [a,b]$,当然有二周期点.

6. $a\rightarrow d\rightarrow c\rightarrow b\rightarrow a$.

此时$[b,c]\Rightarrow [a,b]\Rightarrow [a,d]\supseteq [b,c]$,于是有二周期点.

**Case4**

接下来考虑证明:$P_{2^{n+1}}\ne \empty\Rightarrow P_{2^n}\ne \empty$.

考虑上面命题等价于$f^{[2^{n-1}]}$有四周期点,则其有二周期点.

**Case5**

接下来考虑证明:$P_{2^{n}p}\ne \empty\Rightarrow P_{2^nq}\ne \empty$,其中$p,q\in \text{odd},p<q$.

考虑$f^{[2^n]}$有$p$周期点,当然也就有$q$周期点.

**Case6**

接下来考虑证明:$P_{2^{n}p}\ne \empty\Rightarrow P_{2^mq}\ne \empty$,其中$p,q\in \text{odd},m>n$.

考虑$f^{[2^n]}$有$p$周期点,当然也就有$q2^{m-n}$周期点.

**Case7**

接下来考虑证明:$P_{2^{n}p}\ne \empty\Rightarrow P_{2^m}\ne \empty$,其中$p\in \text{odd}$.

选取$N=\max(n+1,m+1)$,则考虑$f^{[2^n]}$有$p$周期点,则其有$2^{N-n}$周期点,$P_{2^N}\ne \empty$.于是$P_{2^m}\ne \empty$.

### 导数

设函数$f(x)$在一个区间$(a,b)$上有定义,对于给定的$x_0\in(a,b)$,考虑增量$\Delta x\ne 0\and x_0+\Delta x\in(a,b)$,则$\Delta y=f(x_0+\Delta x)-f(x_0)$称为关于$\Delta x$的**增量**.若极限$\lim_{\Delta x\rightarrow 0}\frac{\Delta y}{\Delta x}\\$存在则称这个函数在$x_0$处**可导**,并称这个值为这个函数在$x_0$处的**导数**或者**微商**,记作$f'(x_0)$或$\frac{\text df}{\text dx}|_{x=x_0}$.同理可以定义出**左导数**和**右导数**.如果某个函数的导数构成一个函数,我们称这个函数为其对应的**导函数**.

显然右可导能推出右连续,左可导能推出左连续.换言之,可导一定连续.

但是连续不一定可导,相当漂亮的一个反例是$f(x)=\begin{cases}0&x=0\\x\sin \frac{1}{x}&otherwise\end{cases}$,其在零点没有左右导数.

如果$f$的$n-1$次导数存在,记作$f^{(n-1)}(x)$,如果该函数可导则其导数称作$f^{(n)}(x)$.

如果$f^{(n)}(x)$在$I$上连续,则称$f\in C^{n}(I)$.如果其任意阶可导,则称$f\in C^{\infty}(I)$.

特别地,如果称一个函数是$\alpha$次可导,即$|f(x)-f(y)|\leq|x-y|^\alpha$.其实就是Holder连续.

#### 导数基本运算

导数也可以类似求出四则运算以及复合的法则:

1. $[cf(x)]'=cf'(x)$.
2. $[f(x)\pm g(x)]'=f'(x)\pm g'(x)$.
3. $[f(x)g(x)]'=f'(x)g(x)+g'(x)f(x)$.
4. (莱布尼茨公式):$[f(x)g(x)]^{(n)}=\sum_{k=0}^n\binom{n}{k}f^{(k)}(x)g^{(n-k)}(x)$.
5. $[\frac{f(x)}{g(x)}]'=\frac{f'(x)g(x)-g'(x)f(x)}{g^2(x)},g(x)\ne 0$.
6. $[g(f(x))]'=g'(f(x))f'(x)$.

(1)(2)比较平凡.

(3)(4)的证明的话,考虑:
$$
\frac{f(x+\Delta x)g(x+\Delta x)-f(x)g(x)}{\Delta x}\\
=\frac{f(x+\Delta x)-f(x)}{\Delta x}g(x+\Delta x)+f(x)\frac{g(x+\Delta x)-g(x)}{\Delta x}
$$
取一下极限就行.

(5)的话,其实求出$(\frac{1}{g(x)})'$就行了,我们来看:
$$
(\frac{1}{g(x)})'=\lim_{\Delta x\rightarrow 0}\frac{\frac{1}{g(x+\Delta x)}-\frac{1}{g(x)}}{\Delta x}\\
=\lim_{\Delta x\rightarrow 0}\frac{\frac{g(x)-g(x+\Delta x)}{g(x+\Delta x)g(x)}}{\Delta x}\\
=\frac{-g'(x)}{g^2(x)}
$$
对于(6),不妨设$f(x_0)=y_0$.

当$\Delta y\ne 0$的时候,我们有:
$$
\lim_{\Delta y\rightarrow 0}\frac{g(y_0+\Delta y)-g(y_0)}{\Delta y}=g'(y_0)\\
\lim_{\Delta y\rightarrow 0}[\frac{g(y_0+\Delta y)-g(y_0)}{\Delta y}-g'(y_0)]=0
$$
令$\eta(\Delta y)=\frac{g(y_0+\Delta y)-g(y_0)}{\Delta y}-g'(y_0)=\frac{\Delta z}{\Delta y}-g'(y_0),\Delta y\ne 0$,那么$\lim_{\Delta y\rightarrow 0}\eta(\Delta y)=0$.

此时有:
$$
\frac{\Delta z}{\Delta x}=\frac{\Delta z}{\Delta y}\frac{\Delta y}{\Delta x}=\eta(\Delta y)\frac{\Delta y}{\Delta x}+g'(y_0)\frac{\Delta y}{\Delta x}\\
$$
带上极限,前者为$0$.

另一种写法是,类似上面的讨论,我们这里有一些无穷小量,而且(除了$\frac{\Delta z}{\Delta x}$以外)也不会出现无穷小量之间的除法,用一下当然有,$g(y+\Delta y)-g(y)=(g'(y)+o(1))\Delta y,f(x+\Delta x)-f(x)=(f'(x)+o(1))\Delta x$.

考虑:
$$
\Delta z=(g'(y)+o(1))\Delta y\\
=(g'(y)+o(1))(f'(x)+o(1))\Delta x\\
$$
两边除一下$\Delta x$两边求极限就行.

不难发现上面的过程等价于啥呢?等价于:
$$
\frac{\text d z}{\text d x}=\frac{\text dz}{\text dy}\frac{\text dy}{\text dx}
$$



#### 反函数的导数

我们首先声称:如果$x=\varphi(y),\varphi'\ne 0\Rightarrow $$\varphi$有反函数$f=\varphi^{-1}$.这当然是显然的

然后我们声称:如果$x=\varphi(y)\in C(c,d)$,并且其严格单调且连续.如果$(x_0,y_0)$处其导数存在,则$f'(x_0)=\frac{1}{\varphi'(f(x_0))}$.

这个可以用极限证明,我们有:
$$
f'(x_0)=\lim_{x\rightarrow x_0}\frac{f(x)-f(x_0)}{x-x_0}\\
=\lim_{y\rightarrow y_0}\frac{y-y_0}{\varphi(y)-\varphi(y_0)}\\
=\lim_{y\rightarrow y_0}\frac{1}{\frac{\varphi(y)-\varphi(y_0)}{y-y_0}}\\
=\frac{1}{\varphi'(y_0)}
$$


现在我们证明了反函数有导数,立即有:
$$
f(g(x))=x\Rightarrow f'(x)g'(f(x))=1
$$

#### 隐函数求导

这里我们不加证明地给出隐函数求导法则,也就是暂且把$y$当成$x$的函数.

###### Example1

举例的话,我们来看方程:
$$
x=y+\epsilon\sin y
$$
两边对$x$求导:
$$
1=y'+y'\epsilon \cos y\\
y'=\frac{1}{1+\epsilon\cos y}
$$


#### 参数方程的求导

如果$\begin{cases}x=x(t)\\y=y(t)\end{cases}$,考虑求导$\frac{\text d y}{\text d x}$.

这个做法就是反函数+链式法则.当然要保证下面需要的地方导数都不是$0$.

不妨设$t=t(x)$,当然有$y=y(t(x))$,那么$\frac{\text d y}{\text d x}=y'(t(x))t'(x)=\frac{y'(t)}{x'(t)}$.

###### Example1

极坐标的求导:
$$
\begin{cases}x=r(\theta)\cos\theta\\y=r(\theta)\sin\theta\end{cases}
$$
当然$\frac{\text d y}{\text d x}=\frac{r'\sin\theta+r\cos\theta}{r'\cos\theta-r\sin\theta}=\frac{\tan\theta+\frac{r}{r'}}{1-\tan\theta\frac{r}{r'}}=\tan(\theta+\arctan(\frac{r}{r'}))=\tan\alpha$.

而容易发现$\tan\alpha$其实是切线和$x$轴正半轴的夹角.

#### 初等函数的导数

下面给出若干初等函数的导数.

##### 常函数

常函数的连续性显然.

导数有:
$$
f(x)=c\Rightarrow f'(x)=0
$$

证明显然.

##### 三角函数

三角函数的连续性比较有意思,考虑只需证明$\sin x$连续即可,使用和差化积得到$|\sin x-\sin x_0|=|2\sin \frac{x-x_0}{2}\cos\frac{x+x_0}{2}|\leq 2|\sin \frac{x-x_0}{2}|\leq |x-x_0|$.

导数有:
$$
\sin'(x)=\cos(x)\\
\cos'(x)=-\sin(x)
$$

只给出前者的证明,后者类似,或者换个元做诱导公式也行.

我们有和差化积:
$$
\sin(x+\Delta x)-\sin (x)\\
=2\sin(\frac{\Delta x}{2})\cos(x+\frac{\Delta x}{2})
$$
自然有:
$$
\lim_{\Delta x\rightarrow 0}\frac{\Delta y}{\Delta x}=\lim_{\Delta x\rightarrow 0}\frac{2\sin(\frac{\Delta x}{2})\cos(x+\frac{\Delta x}{2})}{\Delta x}\\
=\lim_{\Delta x\rightarrow 0}\frac{\sin(\frac{\Delta x}{2})}{\frac{\Delta x}{2}}\cos(x+\frac{\Delta x}{2})\\
=\cos(x)
$$

进一步,我们可以考虑高阶导数:
$$
(\sin x)^{(n)}=\sin(x+\frac{n\pi}{2})\\
(\cos x)^{(n)}=\cos(x+\frac{n\pi}{2})
$$

而考虑$(\tan x)'$,我们有:
$$
(\tan x)'=\lim_{\Delta x\rightarrow 0}\frac{\frac{\sin (x+\Delta x)}{\cos(x+\Delta x)}-\frac{\sin x}{\cos x}}{\Delta x}\\
=\lim_{\Delta x\rightarrow 0}\frac{\sin \Delta x}{\Delta x}\frac{1}{\cos x\cos (x+\Delta x)}\\
=\frac{1}{\cos ^2 x}
$$


##### 反三角函数

使用反函数的导数公式:
$$
\arctan'(x)=\frac{1}{1+x^2}\\
\arccos'(x)=\frac{-1}{\sqrt{1-x^2}}\\
\arcsin'(x)=\frac{1}{\sqrt{1-x^2}}
$$

反三角函数的高阶导数比较困难,我们来看个例子:

考虑$y=\arctan x$,$y'=\frac{1}{1+x^2}=\frac{1}{1+\tan^2 y}=\cos^2 y$,那么$y''=-2\cos y\sin y\times y'=-\cos^2y\sin 2y$,事实上容易用归纳法证明$y^{(n)}=(n-1)!\cos^n y\sin (n(y+\frac{\pi}{2}))$.

不过还有一种办法,那就是我们考虑$y'=-\frac{1}{2i}(\frac{1}{i+x}+\frac{1}{i-x})$,用这个归纳立刻得到$y^{(n)}=-\frac{(n-1)!}{2i}((i-x)^{-n}-(-1)^n(i+x)^{-n})$.

然而$y=\arcsin x$要麻烦得多.下面我们来看一下:
$$
y'=\frac{1}{\sqrt{1-x^2}}\\
(y')^2(1-x^2)=1\\
(1-x^2)2y'y''-2x(y')^2\equiv 0\\
(1-x^2)2y''-2xy'\equiv 0
$$
两边求$n-2$次导,利用莱布尼茨公式,化简后得到:
$$
(1-x^2)y^{(n)}+(3-2n)xy^{(n-1)}-(n-2)^2y^{(n-2)}\equiv 0
$$

##### 指数函数

我们需要首先拿到指数为有理数的东西.

首先整数次方是有定义的,我们需要拿到形如$f(x)=a^{\frac{1}{x}}$的东西,这个考虑取一下反函数即可.然后二者嵌套一下得到形如$a^{\frac{q}{p}}$的幂函数,取反一下得到形如$a^{-\frac{q}{p}}$的幂函数.

然后要定义无理数指数幂,类似戴德金分割,定义$a^x=\sup_{q\leq x,q\in Q}a^q$.

当然我们要验证那些指数的运算法则,还有一些单调性(两个无理数之间插入两个有理数就可以拆成单调的,这里还需要讨论一下和$1$的大小关系),这些当然都是对的.

然后我们就可以拿出指数函数,用反函数定义对数函数.

指数函数的连续性:考虑对于$f(x)=a^x$,$\forall \epsilon>0$,$\exists N>0,a<(1+\epsilon)^N$,此时取$q_1<x_0<q_2$,$q_2-q_1<\frac{1}{N}$,这当然能做得到.考虑$a^{x_0}\leq f(x_0+0)\leq f(q_2)$,于是$1\leq \frac{a^{q_2}}{a^{q_1}}\leq a^{\frac{1}{N}}\leq 1+\epsilon$.

先来看最特殊的指数函数:
$$
f(x)=e^x\Rightarrow f'(x)=e^x
$$
事实上我们还有:
$$
f(x)=a^x\Rightarrow f'(x)=a^x\ln a,a>0
$$

下面来看下为啥:
$$
f'(x)=\lim_{\Delta x\rightarrow 0}\frac{a^{x+\Delta x}-a^x}{\Delta x}\\
=a^x\lim_{\Delta x\rightarrow 0}\frac{a^{\Delta x}-1}{\Delta x}\\
=a^x\ln a
$$

而高阶导数有:
$$
f(x)=e^{ax}\Rightarrow f^{(n)}=a^ne^{ax}
$$


##### 对数函数

先看:
$$
f(x)=\log_a(x)\Rightarrow f'(x)=\frac{1}{x\ln a}
$$
证明的话考虑:
$$
f'(x)=\lim_{\Delta x\rightarrow 0}\frac{\log_a(1+\frac{\Delta x}{x})}{\Delta x}\\
=\lim_{\Delta x\rightarrow 0}\frac{\log_a(1+\frac{\Delta x}{x})}{\frac{\Delta x}{x}}\frac{1}{x}\\
=\frac{1}{x\ln a}
$$
再看:
$$
f(x)=\ln |x|\Rightarrow f'(x)=\frac{1}{x}
$$
考虑$f(x)=\ln |x|=\frac{1}{2}\ln x^2$.

考虑对数函数的高阶导数:
$$
f(x)=\ln x\Rightarrow f^{(n)}(x)=(-1)^{n-1}(n-1)!x^{-n}
$$


##### 幂函数

我们只需要定义$x^{\alpha}=e^{\alpha\ln x}$即可.

导数有:
$$
f(x)=x^\mu\Leftrightarrow f'(x)=\mu x^{\mu-1}
$$

证明的话,考虑$x=0$的时候根据定义显然,如果$x\ne 0$,那么:
$$
f'(x)=\lim_{\Delta x\rightarrow 0}\frac{(x+\Delta x)^\mu-x^\mu}{\Delta x}\\
=\lim_{\frac{\Delta x}{x}\rightarrow 0}\frac{(1+\frac{\Delta x}{x})^\mu-1}{\frac{\Delta x}{x}}x^{\mu-1}\\
=\mu x^{\mu -1}
$$

容易发现其高阶导数是:
$$
f(x)=x^\alpha\Rightarrow f^{(n)}(x)=\alpha^{\underline{n}}x^{\alpha-n}
$$

##### 一般初等函数

定理:初等函数在其定义域上除了至多一个孤立点集外,可求导并且导函数也是初等函数.

#### 单调函数的导数

定义右上导数$D^+ f(x_0)=\varlimsup_{h\rightarrow 0+0}\frac{f(x_0+h)-f(x_0)}{h}$,同理定义右下导数$D_+$,左上导数$D^{-}$,左下导数$D_-$.那么$f'(x_0)$存在的充要条件当然是$D_+=D^+=D_-=D^-$并且有限.

下面证明单调(不妨设为单增)函数$f:[a,b]\rightarrow \R$几乎处处有导数.

我们接下来尝试证明$E_1=\{x\in [a,b]\mid D^+f(x)>D_- f(x)\},E_2=\{x\in [a,b]\mid D^-f(x)>D_+f(x)\}$两个集合是零测集.此时$E_1\cup E_2$就是零测集,而$\R\setminus(E_1\cup E_2)$中的点都满足$D^{+}f(x)\leq D_-f(x)\leq D^-f(x)\leq D_+f(x)\leq D^+f(x)$,于是它们都相等(可能等于$+\infty$).

二者类似,我们下面挑选前者进行证明:

考虑设$A_{r,s}=\{x|D^+f(x)>r>s>D_-f(x)\}$,其中$r,s$都是正有理数(因为是单调递增).这当然是可数个集合,所以只需要证明其中任意一个是零测集就可以.我们取$A=A_{r,s}$,下面证明$A$是零测集.

反证,先假设$A$不是零测集,$m^*(A)>0$且有限(最大是$b-a$).

首先删除$A$中的孤立点集.由于孤立点集是零测集,这当然不会对答案有什么影响,也不会删成空集.这样下面可以直接假设$A$是开集.

根据定义,存在任意小的$h>0$使得$\frac{f(x-h)-f(x)}{-h}<s\Rightarrow f(x)-f(x-h)<sh$,那么我们取出这些$I_{x,h}=[x-h,x]$,并且满足$I_{x,h}\subseteq A$.它们当然构成$A$的一个Vitali覆盖.

根据Vitali覆盖引理我们有$\exists I_{k}=[x_k-h_k,x_k]$两两不交并且满足$m^*(A\setminus(\bigcup I_k))<\epsilon$.而考虑$m^*(A\setminus(\bigcup I_k))+m^*(\bigcup I_k)\geq m^*(A)$,不妨令$B=\bigcup I_k$于是我们得到了$m^*(B)>m^*(A)-\epsilon$.

这里我曾经以为可以直接写$m^*(B)=m^*(A)$,但实际上是不行的,因为我们$B$的构造依赖于$\epsilon$.

同时由于所有的$I_k\subseteq A$并且两两不交,我们还有$\sum_j h_j\leq m^*(A)<(1+\epsilon)m^*(A)$.

而我们发现我们又有$f(x_j)-f(x_j-h_j)<sh_j$,两边求和就有$\sum f(x_j)-f(x_j-h_j)<sm^*(A)$.

而接下来考虑$B$,$B$当然仍然是一个开集.对于$\forall y\in B$,当然应该存在一个任意小的$k>0$使得$f(y+k)-f(y)>rk$.取出这些区间$[y,y+k]$当然仍然构成$B$的一个Vitali覆盖.类似上面,我们当然有结论$\sum f(y_j+k_j)-f(y_j)>rm^*(B)>r(m^*(A)-\epsilon)$.但考虑$[y_j,y_j+k_j]$一定被包含在某个$[x_j-h_j,x_j]$中,又根据函数的单调性,自然可以知道$sm^*(A)>\sum f(x_j)-f(x_j-h_j)\geq \sum f(y_j+k_j)-f(y_j)>r(m^*(A)-\epsilon)$.于是$sm^*(A)\geq rm^*(A)$,这当然导出$m^*(A)=0$,与条件不符!

接下来的修补工作是证明$E=\{x|f'(x)=+\infty\}$的集合是零测集.这意味着对于$\forall N>0,\exists  \Delta x>0,f(x+\Delta x)-f(x)>N\Delta x$,取$[x,x+\Delta x]$当然仍然是$E$的一个Vitali覆盖.同样根据Vitali覆盖引理得知$\sum \Delta x>m^*(E)-\epsilon$,如果$m^*(E)\ne 0$,做Vitali覆盖后对$f(x+\Delta x)-f(x)>N\Delta x$左右两边分别求和,左边是有界,上界当然是$f(b)-f(a)$,但右边无界.这就给出矛盾了.

理解一下上面的过程的话就是,由于该单调函数定义在一个闭区间上,那么这个单调函数应该有某种最值性.这必然会限制其大小,而如果哪里的大小崩坏了导致没有导数,这些地方的外测度一定要是$0$,不然整体的最值性无法保证.

#### 李氏连续函数的导数

注意到李氏连续的定义$|f(x)-f(y)|\leq L|x-y|$,注意到$f(x)=f(x)-Lx+Lx$.

如果钦定$x<y$,那李氏连续等价于$f(y)-f(x)\leq |f(x)-f(y)|\leq L(y-x),f(y)-Ly\leq f(x)-Lx$,于是$f(x)-Lx$是单调降函数,当然几乎处处可导,而$Lx$作为初等函数当然处处可导.

李氏连续几乎等价于导函数有界.

下面我们证明:如果$f(x)$在$[a,b]$上可导,那么$|f'|\leq L\Leftrightarrow |f(x_1)-f(x_2)|\leq L|x_1-x_2|$.

右推左是显然的,除一下然后取极限即可.

左推右也很显然,只需用一下拉格朗日中值定理即可.

#### 微分

设$y=f(x)$在$x_0$处有定义,假设有一个常数$A$使得$f(x_0+\Delta x)-f(x_0)=A\Delta x+o(\Delta x),\Delta x\rightarrow 0$,称$f(x)$在$x_0$处**可微**,并把$\text df=\text dy=A\Delta x$称为$f(x)$在$x_0$处的**微分**,由于后半部分是一个更高阶的无穷小量,我们说微分是函数改变量的线性主要部分.



##### 微分与导数(微商)

结合导数极限的定义,就可以得到$\text d y=f'(x)\text d x$,$f'(x)=\frac{\text d y}{\text d x}\\$.这就是我们将导数称作**微商**的原因.换言之,一阶可微一定一阶可导,反之亦然.但容易发现,可微的定义要比可导好得多,可微可以往外拓展到平面等拓扑结构,但可导不行.

一阶微分具有**形式不变性**.换言之就是,我们在求导的时候是需要选定一个自变量的,当选定的自变量是$y$的时候,根据上面自然会有$z=g(y),\text d z=g'(y)\text d y$.

但是当选定的自变量不是$y$的时候,上面的形式是同样成立的.我们下面证明这个结论,令$y=f(x)$:
$$
[g(f(x))]'=g'(f(x))f'(x)\\
\text d z=g'(f(x))f'(x)\text d x\\
\text d z=g'(y)\text d y\\
$$
必须提出高阶微分不存在形式不变性,换句话说,$z=g(y)$的二阶微分的形式不等价于$z=g(y=f(x))$的二阶微分,你不能乱换元.

一般将高阶微分记作$\frac{\text d^n y}{\text d x^n}$.

#### 导函数的性质

##### 费马定理

如果$f(x)$在邻域$U(\xi,\delta)$上有定义,$\xi$是其的一个极值点,在$\xi$处存在导数,那么$f'(\xi)=0$.

不妨考虑最大值情况,$f'(\xi)=\lim_{\Delta x\to +0}\frac{f(\xi +\Delta x)-f(\xi)}{\Delta x}=\lim_{\Delta x\to -0}\frac{f(\xi+\Delta x)-f(\xi)}{\Delta x}$,前者$\leq 0$,后者$\geq 0$,此时当然$f'(\xi)=0$.

##### 罗尔中值定理

如果$f\in C[a,b]$且可导,并且$f(a)=f(b)$.则$\exists \xi \in (a,b)$,$f'(\xi)=0$.

即:可导函数两个零点间一定有导数的零点.

设$M$为$f[a,b]$上的最大值,$m$是最小值,当二者相等即常值函数的时候,当然成立.

当$M>m$的时候,则$M>f(a)$和$m<f(a)$至少有一个成立.不妨设$f(M)>f(a)=f(b)$.

根据费马定理,这个极值点的导数为$0$.

由此可以推论:函数的零点数量$|\{x|f(x)=0\}|\leq 1+|\{x|f'(x)=0\}|$.

这当然可以推出$n$次多项式至多$n$个实根.

事实上可以证明更强的结论,当$c_i$不全为$0$的时候,取$n$个不同实数$\lambda_k$,$f(x)=\sum_{i=1}^n c_ix^{\lambda_i}$至多有$n-1$个正根.

归纳法,$n=1$的时候显然成立.$n-1$时成立的话,考虑$f(x)=0\Leftrightarrow x^{-\lambda_n}f(x)=0$,而后者有一项是常数项,求导后少一项.

##### 拉格朗日中值定理

$f\in C[a,b]$,并且在$(a,b)$上可导，$\exists \xi\in (a,b),f'(\xi)=\frac{f(b)-f(a)}{b-a}$.

考虑先把函数放平,设$g(x)=f(x)-\frac{f(b)-f(a)}{b-a}(x-a)-f(a)$,容易发现$g(a)=g(b)=0$,根据罗尔中值定理,$\exists \xi,g'(\xi)=0=f'(\xi)-\frac{f(b)-f(a)}{b-a},f'(\xi)=\frac{f(b)-f(a)}{b-a}$.

###### Example1

证明:如果$f'(x)\equiv 0\Rightarrow f(x)\equiv C$.

考虑$\forall x,y,x\ne y$,$\forall \xi,f'(\xi)=\frac{f(y)-f(x)}{y-x}=0\Rightarrow f(x)=f(y)$.

###### Example2

证明:如果$f'(x)\equiv g'(x)$,那么$f(x)=g(x)+C$.

考虑$h(x)=f(x)-g(x),h'(x)\equiv 0\Rightarrow h(x)=C$.

###### Example3

考虑对抛物线用拉格朗日中值定理,设$f(x)=px^2+qx+r$,则$\frac{f(b)-f(a)}{b-a}=\frac{p(b^2-a^2)+q(b-a)}{b-a}=p(b+a)+q=f'(\frac{a+b}{2})$.

反过来,如果恒有$\frac{f(b)-f(a)}{b-a}=f'(\frac{a+b}{2})$,事实上也可以推出$f(x)$是抛物线.

此时我们有$f(x+y)-f(x-y)=2yf'(x)$,取$y=1$,知道$f(x)\in C^{\infty}$,于是可以两边对$y$求两次导数,得到$f''(x+y)-f''(x-y)\equiv 0$,取$y=x$,这当然意味着$f''(x)$是常函数.

##### 柯西中值定理

$f,g\in C[a,b]$,并且均在$(a,b)$上可导,其中$g'(x)\ne 0$.则$\exists \xi\in (a,b)$,$\frac{f(b)-f(a)}{g(b)-g(a)}=\frac{f'(\xi)}{g'(\xi)}$.

此时注意到$g(b)\ne g(a)$,原因是根据拉格朗日中值定理,如果$g(b)=g(a),\exists \xi \in (a,b),g'(\xi)=0$,这与$g'(x)\ne 0$矛盾.

令$F(x)=f(x)(g(b)-g(a))-g(x)(f(b)-f(a))$.

接下来注意到上述命题等价于$\exists \xi\in (a,b),F'(\xi)=0$.

而注意到$F(a)=f(a)g(b)-g(a)f(b)=F(b)$,根据罗尔中值定理立刻得到答案.

然而这个东西竟然还能推广,我们有:

如果$f,g,h\in C[a,b]$并且在$(a,b)$上可导,$\exists \xi\in (a,b),$有:
$$
\left |\begin{matrix}f'(\xi)&g'(\xi)&h'(\xi)\\f(a)&g(a)&h(a)\\f(b)&g(b)&h(b)\end{matrix}\right |=0
$$
证明的话考虑构造:
$$
F(x)=\left |\begin{matrix}f(x)&g(x)&h(x)\\f(a)&g(a)&h(a)\\f(b)&g(b)&h(b)\end{matrix}\right |
$$
注意到$F(a)=F(b)=0$.

##### 广义微分中值定理

###### 广义罗尔中值定理

如果$f\in C[a,b]$且$\forall x\in (a,b),f'_\pm(x)$均存在,并且$f(a)=f(b)=0$.则$\exists \xi \in (a,b)$,$f_+'(\xi)f_-'(\xi)\leq 0$.

证明和罗尔中值定理没区别,仍然是找最值.

###### 广义拉格朗日中值定理

如果$f\in C[a,b]$且$\forall x\in (a,b),f'_\pm(x)$均存在,$\eta=\frac{f(a)-f(b)}{a-b}$,则$\exists \xi \in (a,b)$,$(f_+'(\xi)-\eta) (f_-'(\xi)-\eta)\leq 0$.

证明差不多,仍然是把函数放平然后用广义罗尔中值定理就行.

##### 导函数与间断点

导函数不存在第一类间断点.

证明的话,考虑证明当$f(x)$在$x_0$处可导的时候,$\exists x_n<x_0<y_n$,使得$x_n,y_n\rightarrow x_0$,并且满足$\lim f'(x_n)=\lim f'(y_n)=f'(x_0)$.

原因根据拉格朗日中值定理,$\exists x_n\in (x_0-\frac{1}{n},x_0)$,$f'(x_n)=\frac{f(x_0)-f(x_0-\frac{1}{n})}{\frac{1}{n}}$,直接这么取当然就是可行的.

##### 函数的升降性

$f\in C[a,b]$并且在$(a,b)$上可导,$f$单调不减$\Leftrightarrow $$f'(x)\geq 0$.

左推右当然是导数定义,右推左可以用拉格朗日微分中值定理.

另外由于保号性,$f'(x)>0\Rightarrow f\uparrow$,但反之不成立.

##### 函数的极值

若$f$在$(x_0-\delta,x_0+\delta)$上$n-1$次可导,$\forall 1\leq k\leq n-1,f^{(k)}(x_0)=0$.而且$f$在$x_0$处存在$n$阶导数并有$f^{(n)}(x_0)\ne 0$.我们有以下结论:

1. $n$是奇数的时候,$x_0$不是极值点.
2. $n$是偶数且$f^{(n)}(x_0)<0$时,$x_0$是极大值点.
3. $n$是偶数且$f^{(n)}(x_0)>0$时,$x_0$是极小值点.

证明考虑泰勒公式:
$$
f(x)=f(x_0)+\sum_{k=1}^n\frac{f^{(k)}(x_0)}{k!}(x-x_0)^k+o((x-x_0)^n)\\
f(x)-f(x_0)=(x-x_0)^n(\frac{f^{(n)}(x_0)}{n!}+o(1))
$$
于是取极小邻域并且$n$是偶数的时候$f(x)-f(x_0)$和$f^{(n)}(x_0)$同号,上述命题立见.

##### 达布定理

也即:导函数拥有介值性.

如果$f$在$[a,b]$上可导,$f'(a)f'(b)<0$,那么$\exists \xi\in (a,b),f'(\xi)=0$.

不妨设$f'(a)>0>f'(b)$,那么$f$在$a$处附近递增,在$b$处附近递减.那我们取一下$f(\xi)=\max_{[a,b]}\{f(x)\}$,那么$\xi \ne a,\xi \ne b$,于是$\xi\in (a,b)$,$f'(\xi)=0$.

那我们加个常数就可以把这个推广到导函数的介值性.

然而这个介值性比连续性弱得多,例如下面这个函数的导函数就在闭区间$[0,1]$上有界但没有最值:
$$
f(x)=\begin{cases}-x^2(1+\sin{\frac{1}{x}})&x\ne 0\\0&x=0\end{cases}
$$
其构造思路是让其在$0$附近的导数在$(-1,1)$上下波动,这样上确界为$1$,但$0$处跳出去,所以取不到$1$.

下面这个函数更极端,其导函数在闭区间$[0,1]$上无界:
$$
f(x)=\begin{cases}x^2\sin{\frac{1}{x^2}}&x\ne 0\\0&x=0\end{cases}
$$
其构造思路应当是让其在$0$处相当厉害地波动.

##### 函数的凹凸性

定义一个函数是下凸的,若对$\forall x_1,x_2\in I,\forall\lambda\in [0,1],f(\lambda x_1+(1-\lambda)x_2)\leq \lambda f(x_1)+(1-\lambda)f(x_2)$.

如果一个函数是下凸的,那么其充要条件是$\forall x_1,x_2,x_3\in I$满足$x_1<x_2<x_3$,$\left|\begin{matrix}1&x_1&f(x_1)\\1&x_2&f(x_2)\\1&x_3&f(x_3)\end{matrix}\right|\geq 0$.只需设$x_2=\lambda x_1+(1-\lambda)x_3$然后简单变形即可.

不妨设$k(x_1,x_2)=\frac{f(x_1)-f(x_2)}{x_1-x_2}$也就是这里的割线斜率.还可以证明如果一个函数是下凸的,那么其充要条件是$\forall x_1,x_2,x_3\in I,x_1<x_2<x_3$,斜率有$k(x_1,x_2)\leq k(x_1,x_3)\leq k(x_2,x_3)$,即所谓三弦引理.这个性质容易证明,而且该性质立刻说明开区间上的有界下凸函数一定是李氏连续函数.这意味着其应当几乎处处可导.另外三弦引理还可以改为$k(x_1,x_2)\leq k(x_2,x_3)$,原因是中间项一定是两项的带权平均.

另外显然的是:

1. 如果$f$可导,那么$f$下凸$\Leftrightarrow$$f'$不降.
2. 如果$f$可二阶导,那么$f$下凸$\Leftrightarrow$$f''\geq 0$.

如果$f(x)$在$x_0$附近连续并且在$x_0$左右小邻域内凸性相反,称其为$f(x)$的一个**拐点**.

容易说明$f''(x_0)=0$是拐点的必要条件,原因是拐点处$f'(x)$的增减性相反,$x_0$必定是$f'(x)$的极值点.由此还可以套用之前的极值点结论:

若$f$在$(a,b)$上$n$次可导,$\exists x_0\in (a,b),\forall 1\leq k\leq n-1,f^{(k)}(x_0)=0$.并且$\forall x\in (a,b),f^{(n)}(x)>0$我们有以下结论:

1. $n$是奇数的时候,$f(x)$是严格单增函数.
2. $n$是偶数的时候,$f(x)$是严格下凸函数.

但上述条件疑似有点过强,能不能放弱一点呢?

可以发现一个在开区间$(a,b)$上的函数是下凸的充要条件应当是以下条件同时成立:

1. $\forall x\in (a,b)$,$f'_\pm(x)$均存在,并且$f'_-(x)\leq f'_+(x)$.
2. $\forall x_1,x_2\in (a,b),x_1<x_2$,$f'_+(x_1)\leq f'_-(x_2)$.

必要性几乎是显然的,只需用三弦引理然后在每个点都卡一下极限就行.

充分性略有麻烦.考虑反证,只需反推三弦引理即可.设其不是凸的,那就$\exists x_1,x_2,x_3\in (a,b),x_1<x_2<x_3,k(x_1,x_2)>k(x_2,x_3)$.不妨设$\eta_1=k(x_1,x_2),\eta_2=k(x_2,x_3)$那么根据广义拉格朗日中值定理,$\exists \xi_1\in(x_1,x_2),\xi_2\in (x_2,x_3)$,使得$(f_+'(\xi_1)-\eta_1)(f'_-(\xi_1)-\eta_1)\leq 0,(f_+'(\xi_2)-\eta_2)(f'_-(\xi_2)-\eta_2)\leq 0$.

立刻得到$f'_+(\xi_1)\geq \eta_1\geq f_-'(\xi_1),f'_+(\xi_2)\geq \eta_2\geq f_-'(\xi_2)$,由于$\eta_1>\eta_2$,所以$f'_+(\xi_1)\geq \eta_1>\eta_2\geq f'_-(\xi_2)$,但$\xi_1<\xi_2$,这就出现了矛盾.

还可以证明下凸函数除了可数个点以外都可导,并且几乎处处有二阶导数.

这个怎么证明呢?考虑由(1)(2),可以发现$f'_+(x)$和$f_-'(x)$都是单调函数,那么除了可数个点以外它们均连续,而它们在连续处必定可导,原因是$f_+'(x_0-\epsilon)\leq f'_-(x_0)\leq f'_+(x_0)$,夹逼一下就可以了.而这样的话其导数单调,于是处处二阶可导.

由此还可以发现:$f$在开区间$I$上,是下凸的等价于:

1. $\forall x_0\in I,f'_\pm(x_0)$存在.
2. $\forall x\in I,f(x)\geq f(x_0)+f'_\pm(x_0)(x-x_0)$.

必要性:

考虑已知凸函数,只需证明$f(x)-f(x_0)\geq f'_\pm(x_0)(x-x_0)$.不妨假设$x>x_0$(另一方向同理),于是只需证明$\frac{f(x)-f(x_0)}{x-x_0}\geq f'_\pm(x_0)$,用广义拉格朗日微分中值定理立刻得见此成立.

充分性:

任取两点$x_1,x_2\in I,x_1<x_2$,有$\frac{f(x_2)-f(x_1)}{x_2-x_1}\geq f_-'(x_1)$,那么左边那个东西取极限得到$f'_+(x_1)$,这样就说明了任何一个点都有$f'_+(x)\geq f'_-(x)$.

接下来仍然容易得到:
$$
f(x_2)\geq f(x_1)+f'_+(x_1)(x_2-x_1)\\
f(x_1)\geq f(x_2)+f'_-(x_2)(x_1-x_2)
$$
由上述式子立刻得到$f_+'(x_1)'\leq f_-'(x_2)$.这样就证明了充分性.

#### 经典不等式

##### 琴生不等式

如果$f$在区间$I$上是下凸的,则$\forall x_i\in I,\lambda_i\in [0,1],\sum \lambda_i=1$,那么$f(\sum \lambda_ix_i)\leq \sum \lambda_i f(x_i)$.从几何角度来看,这意味着在$f(\sum \lambda_ix_i)$处作的切线位于原函数下方.所以函数如果是上凸的话不等号要反向.

当所有$x_i$均相等的时候显然成立.

不然考虑直接取$x_0=\sum \lambda_ix_i$,此时发现$x_0$一定在区间内部.于是我们知道$f(x_i)\geq f(x_0)+f'_+(x_0)(x_i-x_0)$,求和得到$\sum \lambda_if(x_i)\geq f(x_0)+f_+'(x_0)\sum \lambda_i (x_i-x_0)=f(x_0)$,这样就证毕了.

容易将琴生不等式推广到积分形式,当$\varphi(x)$是下凸连续函数的时候,当然有$\varphi(\int_0^1 f\text d x)\leq \int_0^1\varphi(f(x))\text d x$,证明只需对黎曼和求极限.

###### Example1

证明:圆内接$n$边形以正$n$边形面积和周长最大.

不妨设其为单位元,用圆心向顶点连边,会得到一圈圆心角$\alpha_1,\ldots,\alpha_n$,它们之和应当是$2\pi$并且范围均在$(0,\pi]$里(原因是圆心肯定得包含在正$n$边形里,不然肯定不算大),此范围内$\sin $是下凸函数,那么$S=\frac{1}{2}\sum \sin \alpha_i\leq \frac{n}{2}\sin\frac{\sum \alpha_i}{n}=\frac{n}{2}\sin \frac{2\pi}{n}$,$L=2\sum \sin \frac{\alpha_i}{2}\leq 2n\sin\frac{\sum \alpha_i}{2n}=2n\sin \frac{\pi}{n}$.

同理还可以证明圆外接$n$边形以正$n$边形面积和周长最小,证明可以考虑取切线然后用$\tan$的凸性.

##### 均值不等式

当$x_i>0$时:
$$
\frac{n}{\sum \frac{1}{x_i}}\leq (\prod x_i)^{\frac{1}{n}}\leq \frac{\sum x_i}{n}
$$
取$f(x)=\ln x$,$f(x)$是上凸的,于是$\ln \frac{\sum x_i}{n}\geq \frac{1}{n}\sum \ln x_i$,右边得证,取倒数得到左边.

##### 柯西-赫尔德不等式

当$a_i,b_i> 0$并且$\frac{1}{p}+\frac{1}{q}=1$时,有$\sum a_ib_i\leq (\sum a_i^p)^{\frac{1}{p}}(\sum b_i^q)^{\frac{1}{q}}$.其特例给出柯西不等式.

套用琴生不等式,取$f(x)=x^{\frac{1}{q}},\lambda_i=\frac{a_i^p}{\sum_j a_j^p},x_i=\frac{b_i^q}{a_i^p}$,容易发现$f(x)$上凸.

这样就会得到:
$$
f(\sum \lambda_ix_i)\geq \sum \lambda_i f(x_i)\\
(\sum_i \frac{b_i^q}{\sum_j a_j^p})^{\frac{1}{q}}\geq \sum_i \frac{a_ib_i}{\sum_j a_j^p}\\
$$
化简一下就可以了.

在这里也可以将此形式拓展到积分形式,注意到当$\frac{1}{p}+\frac{1}{q}=1$时,$a^{\frac{1}{p}}b^{\frac{1}{q}}=e^{\frac{\ln a}{p}+\frac{\ln b}{q}}\leq \frac{a}{p}+\frac{b}{q}$,原因是琴生不等式.

此时令$F=(\int_a^b |f(x)|^p\text d x)^{\frac{1}{p}},G=(\int_a^b |g(x)|^q\text d x)^{\frac{1}{q}}$,再令$a=\frac{|f(x)|^p}{F^p},b=\frac{|g(x)|^q}{G^q}$,根据上面的不等式立刻得到$\frac{|f(x)g(x)|}{FG}\leq\frac{a}{p}+\frac{b}{q}$,此时做两边积分,注意到$\int a=\int b=1$,立刻得到$(\int_a^b |f(x)|^p\text d x)^{\frac{1}{p}}(\int_a^b |g(x)|^q\text d x)^{\frac{1}{q}}\geq \int_a^b|f(x)g(x)|\text d x$.

##### 闵可夫斯基(Minkovski)不等式

$x_i,y_i\geq 0,p\geq 1$,则$(\sum(x_i+y_i)^p)^{\frac{1}{p}}\leq (\sum x_i^p)^{\frac{1}{p}}+(\sum y_i^p)^{\frac{1}{p}}$.这其实揭示了某种度量下的三角形不等式.

套用琴生不等式,取$f(x)=(1-x^{\frac{1}{p}})^p,x\in (0,1)$.求导得到$f'(x)=-(1-x^{\frac{1}{p}})^{p-1}x^{\frac{1}{p}-1}$,再来一次化简得到$f''(x)=(1-\frac{1}{p})(1-x^{\frac{1}{p}})^{p-2}x^{\frac{1}{p}-2}\geq 0$.于是$f(x)$下凸.

取$\lambda_i=\frac{(x_i+y_i)^p}{\sum_j (x_j+y_j)^p}$,$u_i=(\frac{x_i}{x_i+y_i})^p$.这样得到:
$$
(1-\frac{(\sum x_i^p)^{\frac{1}{p}}}{(\sum_j (x_j+y_j)^p)^{\frac{1}{p}}})^p\leq \sum_i \frac{(x_i+y_i)^p}{\sum_j (x_j+y_j)^p}(\frac{y_i}{x_i+y_i})^p\\
((\sum (x_i+y_i)^p)^{\frac{1}{p}}-(\sum x_i^p)^{\frac{1}{p}})^p\leq \sum y_i^p
$$
化简一下就做完了.

#### 洛必达法则

来看几个抽象的问题:

1. $\infty-\infty$.
2. $0\times \infty$.
3. $\frac{0}{0}$.
4. $\frac{\infty}{\infty}$.
5. $0^0$.
6. $1^\infty$.
7. $\infty^0$.

(1)可以写成$\infty(1-\frac{\infty}{\infty})$,这样就转化成了(4)和(2).

(2)可以写成$\frac{0}{\frac{1}{\infty}}$,就转化成了(3).

(5)可以写成$e^{0\ln 0}$,就转化成了(2).

(6)可以写成$e^{\infty\ln 1}$,就转化成了(2).

(7)可以写成$e^{0\ln \infty}$,就转化成了(2).

这样的话我们只需要解决俩问题就行了,也就是$\frac{0}{0}$和$\frac{\infty}{\infty}$.

##### 0/0型

如果$f,g$在$U_0(a,\delta)$上可导,$g'(x)\ne 0$,$\lim_{x\rightarrow a}f(x)=\lim_{x\rightarrow a}g(x)=0$.那么若$\lim_{x\rightarrow a}\frac{f'(x)}{g'(x)}=k\Rightarrow \lim_{x\rightarrow a}\frac{f(x)}{g(x)}=k$.

这个只需要定义$f(a)=g(a)=0$,这样这俩函数就连续了.然后直接跑柯西中值定理就可以对$\frac{f(x)-f(a)}{g(x)-g(a)}=\frac{f'(\xi)}{g'(\xi)}$.

这里可以看出洛必达法则是单向的,因为我们取得$\xi$并不是连续变化的,只是取了一个$\rightarrow 0$的$\xi$的子列.

而如果$a=\infty$,那就考虑取$t\rightarrow 0$,然后把$x$换成$\frac{1}{t}$,做复合函数求导即可,结论无差异.

##### $\infty$/$\infty$型

如果$f,g$在$U_0(a,\delta)$上可导,$g'(x)\ne 0$,$\lim_{x\rightarrow a}g(x)=\infty$.那么若$\lim_{x\rightarrow a}\frac{f'(x)}{g'(x)}=k\Rightarrow \lim_{x\rightarrow a}\frac{f(x)}{g(x)}=k$.

这个证明麻烦一点,先来讨论$k\ne \pm \infty$的情况.

考虑$\forall \epsilon>0$,$\exists \delta >0,\forall \xi\in (a,a+\delta)$,应当有$k-\epsilon<\frac{f'(\xi)}{g'(\xi)}<k+\epsilon$,那么$\forall x\in (a,a+\delta)$,取$x_1=a+\delta$当然有:
$$
\frac{f(x)-f(x_1)}{g(x)-g(x_1)}=\frac{f'(\xi)}{g'(\xi)}\\
\frac{f(x)}{g(x)}=\frac{f'(\xi)}{g'(\xi)}(1-\frac{g(x_1)}{g(x)})+\frac{f(x_1)}{g(x)}
$$
两边取极限就做完了.

当$k=\pm \infty$的时候容易发现上面那个还能用.

#### 泰勒公式

设$f(x)$在$x_0$处有$n$阶导数,则**泰勒公式**声称$x\rightarrow x_0$时$f(x)=\sum_{k=0}^nf^{(k)}(x_0)\frac{(x-x_0)^k}{k!}+o((x-x_0)^n)$,其中$o((x-x_0)^n)$称作**皮亚诺余项**.特别地,当$x_0=0$的时候,称其为**麦克劳林公式**.

要证明这个式子只需证明$\lim_{x\rightarrow x_0}\frac{f(x)-\sum_{k=0}^nf^{(k)}(x_0)\frac{(x-x_0)^k}{k!}}{(x-x_0)^n}=0$.对上下分别用$n-1$次洛必达法则后,那么只需要证明$\lim_{x\rightarrow x_0}\frac{f^{(n-1)}(x)-f^{(n-1)(x_0)}}{(x-x_0)}-f^{(n)}(x_0)=0$.而这当然就是导数的定义,于是就证毕了.

##### 拉格朗日余项

设$f(x)$在$(a,b)$上有$n+1$阶导数,并且$f(x)\in C^n[a,b]$,泰勒公式有$f(x)=\sum_{k=0}^nf^{(k)}(x_0)\frac{(x-x_0)^k}{k!}+\frac{f^{(n+1)}(\xi)}{(n+1)!}(x-x_0)^{n+1}$,其中$\xi\in (x_0,x)$.一般将$\frac{f^{(n+1)}(\xi)}{(n+1)!}(x-x_0)^{n+1}$称作**拉格朗日余项**.

证明的话,考虑对于一个固定的$x$,构造$F(t)=f(x)-\sum_{k=0}^nf^{(k)}(t)\frac{(x-t)^k}{k!}$,$G(t)=(x-t)^{n+1}$.这里之所以固定$x$而移动$x_0$的原因是,如果反过来固定则求导就不能实现下面的错项相消.不难发现$F(x)=G(x)=0$,此时考虑必定有一个$\xi\in (x_0,x)$满足$\frac{F'(\xi)}{G'(\xi)}=\frac{F(x_0)-F(x)}{G(x_0)-G(x)}=\frac{F(x_0)}{G(x_0)}$.

这个时候研究一下他们的导数,注意到$G'(t)=-(n+1)(x-t)^{n}$,$F'(t)=-\sum_{k=0}^nf^{(k+1)}(t)\frac{(x-t)^k}{k!}+\sum_{k=1}^nf^{(k)}(t)\frac{(x-t)^{k-1}}{(k-1)!}=-f^{(n+1)}(t)\frac{(x-t)^n}{n!}$.

于是$\frac{F(x_0)}{G(x_0)}=\frac{f^{(n+1)}(\xi)}{(n+1)!}$,$F(x_0)=\frac{f^{(n+1)}(\xi)}{(n+1)!}(x-x_0)^{n+1}$.

下面给出一些常见的泰勒展开公式(其中$0<\theta<1$):

1. $e^x=\sum_{k=0}^n\frac{x^k}{k!}+\frac{e^{\theta x}x^{n+1}}{(n+1)!}=1+\frac{x}{1!}+\frac{x^2}{2!}+\frac{x^3}{3!}+o(x^3)$.
2. $\sin x=\sum_{k=0}^{n}\frac{(-1)^kx^{2k+1}}{(2k+1)!}+(-1)^{n+1}\frac{\cos\theta x}{(2n+3)!}x^{2n+3}=x-\frac{1}{3!}x^3+o(x^4)$.
3. $\cos x=\sum_{k=0}^n\frac{(-1)^kx^{2k}}{(2k)!}+(-1)^{n+1}\frac{\cos\theta x}{(2n+2)!}x^{2n+2}=1-\frac{1}{2!}x^2+\frac{1}{4!}x^4+o(x^5)$.
4. $\tan x=x+\frac{1}{3}x^3+\frac{2}{15}x^5+o(x^6)$.
5. $\ln (1+x)=\sum_{k=1}^n\frac{(-1)^{k-1}x^k}{k}+(-1)^n\frac{x^{n+1}}{(n+1)(1+\theta x)^{n+1}}=x-\frac{1}{2}x^2+\frac{1}{3}x^3+o(x^3)$.
6. $(1+x)^\alpha=\sum_{k=0}^n\frac{\alpha^{\underline k}}{k!}x^k+\frac{\alpha^{\underline{n+1}}}{(n+1)!}(1+\theta x)^{\alpha-n-1}x^{n+1}$.

#### 解析函数

不妨设$f_n(x,x_0)=\sum_{k=0}^nf^{(k)}(x_0)\frac{(x-x_0)^k}{k!}$.我们称$f$在$x_0$处解析,若$\exists \delta>0$,$\lim_{n\rightarrow \infty}f_n(x,x_0)=f(x)$,也即泰勒展式在一个邻域上可以逼近原函数.

如果其在$(a,b)$上任何一点都解析,那么称其在$(a,b)$上解析,记作$f\in C^\omega(a,b)$.

解析函数应当有如下性质:

1. $f,g$在$x_0$处解析,那么$f,g$的四则运算在$x_0$处也解析.
2. $f,g$均解析,并且$f\circ g$有意义,那么$f\circ g$也解析.
3. $f$在$x_0$处解析,那么$f^{-1}$在$f(x_0)$处解析.
4. 初等函数基本是解析函数.

对于(1):

$f\pm g$当然是平凡的,至于$fg$和$\frac{f}{g}$如何证明解析性,我们回头再说.

对于(2):

回头再说.

#### 拉格朗日插值

构造多项式$P_n=\sum_{i=0}^{n}y_i(\prod_{j=0\and j\ne i}^{n}\frac {x-x_j}{x_i-x_j})\\$.显然当$x=x_i$时,该多项式的答案为$y_i$.

用这个在一个$n+1$次可导的函数$f\in C(a,b)$上打$n+1$个点并且取出这个多项式$P_n$,考虑它们的误差.取$\omega (x)=\prod_{k=0}^n(x-x_k)$,事实上有:$\exists \xi \in(a,b),f(x)=P_n(x)+\frac{f^{(n+1)}(\xi)}{(n+1)!}\omega(x)$.

取$h(t)=f(t)-P_n(t)$,考虑对于一个固定的$x\ne x_i $,取$g(t)=\frac{h(x)}{\omega(x)}\omega(t)-h(t)$.注意到$g(x)=0,g(x_i)=0$,所以$g$有$n+2$个零点,所以$g^{(n+1)}$一定有一个零点,记为$\xi$.

那么我们就有$g^{(n+1)}(\xi)=\frac{h(x)}{\omega(x)}(n+1)!-f^{(n+1)}(\xi)=0$.上述命题就得证了.

那么怎么样能让$\omega(x)$这个东西的最大值尽可能小呢?

我们定义所谓$n$次切比雪夫多项式$f$,使得对于任何一个首项为$1$的多项式$T_n$,$\max _{(a,b)}|T_n|\geq \max_{(a,b)}|f|$.

事实上$[-1,1]$上的切比雪夫多项式可以用递推关系描述:
$$
P_n=\begin{cases}1&n=0\\x&n=1\\x^2-\frac{1}{2}&n=2\\xP_{n-1}-\frac{1}{4}P_{n-2}&otherwise\end{cases}
$$
还可以写成$P_n=\frac{1}{2^{n-1}}\cos(n\arccos x)$.

#### 函数的作图

##### 渐近线

如果对于一个函数$y=f(x)$,存在一条直线$l$使得函数上一点$P(x,f(x))$距离原点距离$\to +\infty$的时候,$(x,f(x))$与$l$的距离$\to 0$.

大概可以有以下几类渐近线:

1. 水平渐近线:$f(+\infty)=C$或$f(-\infty)=C$.
2. 垂直渐近线:$f(x_0-0)=\infty$或$f(x_0+0)=\infty$.
3. 斜渐近线:$l=kx+b$,那么$\lim_{x\to+\infty}\frac{f(x)}{x}=a,\lim_{x\to \infty}(f(x)-ax)=b$.

##### 作图

1. 确定定义域和值域.
2. 研究其奇偶性,周期性.
3. 研究$f'(x)=0$的根以确定极值点和升降区间.
4. 研究$f''(x)=0$的根找出拐点.
5. 求出渐近线.

#### 牛顿迭代

目的是找到一个函数$f$的零点,尝试取上面任何一点并作切线,找到切线与$x$轴的交点.或者说任取$x_0$,取$x_{n+1}=x_n-\frac{f(x_n)}{f'(x_n)}$.

我们需要证明这个做法的正确性,事实上:

如果$f',f''$在$[a,b]$上非零,$\exists c\in [a,b],f(c)=0$,那我们取$x_0\in [a,b]$满足$f(x_0)f''(x_0)>0$,那么取$x_{n+1}=x_n-\frac{f(x_n)}{f'(x_n)}$的数列$\{x_n\}\rightarrow c$.事实上:

1. 当$f'f''>0$,$x_n$单调下降趋近于$c$.
2. 当$f'f''<0$,$x_n$单调上升趋近于$c$.

并且$|x_n-c|\sim O(2^{-2^n})$.

上述讨论情况有点多,我们下面只讨论其中一种进行证明:

当$f'>0,f''>0$的时候,$f$是一个单调上升的下凸函数.容易发现这个数列单调下降并且应当大于$c$,那它就有极限,两边取极限得知$\lim_{n\to\infty}f(x_n)=0$.

接下来看一下逼近速度,泰勒公式展开一下知道$0=f(c)=f(x_n)+f'(x_n)(c-x_n)+\frac{f''(\xi)}{2}(c-x_n)^2$.从上面的结果容易看出$n\geq 1$的时候,$|x_{n+1}-c|=\frac{|f(x_n)+f'(x_n)(c-x_n)|}{|f'(x_n)|}=\frac{f''(\xi)}{2|f'(x_n)|}(c-x_n)^2$.前面那个系数是有界的,可以看作一个常数.于是这一次的误差应当是上一次误差的平方级别.

### 积分

#### 不定积分

如果存在可导函数$F$满足$F'(x)=f(x)$,则称一个函数$f$是**可积**的,$F(x)$是$f(x)$的**原函数**,或称其为$f(x)$的不定积分.

我们之前已经证明过导数相等的两个函数最多只差一个常数,因此一个函数的原函数是有唯一的$F(x)+C$的形式的.

##### 不定积分的运算

只给出以下两个运算法则:
$$
\int(f(x)\pm g(x))\text d x=\int f(x)\text d x\pm \int g(x)\text d x\\
\int \alpha f(x)\text d x=\alpha\int f(x)\text d x
$$

##### 积分表

1. $\int x^\alpha \text d x=\frac{x^{\alpha+1}}{\alpha+1}+C,\alpha\ne -1$.
2. $\int \frac{\text d x}{x}=\ln |x|+C$.
3. $\int e^x\text d x=e^x+C$.
4. $\int \cos x \text d x=\sin x+C$.
5. $\int \sin x \text d x=-\cos x+C$.
6. $\int \frac{\text d x}{\cos^2 x}=\tan x+C$.
7. $\int \frac{\text d x}{\sin^2 x}=-\cot x+C=-\frac{1}{\tan x}+C$.
8. $\int \frac{\text d x}{1+x^2}=\arctan x+C$.
9. $\int \frac{\text d x}{\sqrt {1-x^2}}=\arcsin x+C$.
10. $\int \frac{\text d x}{\sqrt{x^2\pm a^2}}=\ln |x+\sqrt{x^2\pm a^2}|+C$.
11. $\int \sqrt{a^2-x^2}\text d x=\frac{1}{2}(x\sqrt {a^2-x^2}+a^2\arcsin\frac{x}{a})+C$.
12. $\int \sqrt{x^2\pm a^2}\text d x=\frac{1}{2}(x\sqrt{x^2\pm a^2}\pm a^2\ln|x+\sqrt {x^2\pm a^2}|)+C$.

##### 换元法

第一换元法:设$\int g(t)\text d t=G(t)+C$,$w(x)$可导,则$\int g(w(x))w'(x)\text d x=G(w(x))+C$.证明只需对右边求导即可.

第二换元法:设$x(t)$可导,且有反函数$t(x)$,那么如果$\int f(x(t))x'(t)\text d t=G(t)+C$,则$\int f\text d x=G(t(x))+C$.

###### Example1

求$I_n=\int \tan^n x\text{d}x$.

注意到:
$$
I_n=\int \tan^{n-2}(\frac{1}{\cos^2 x}-1)\text dx\\=\int \tan^{n-2}x(\text d\tan x)-I_{n-2}\\=\frac{\tan ^{n-1}x}{n-2}-I_{n-2}
$$
边界条件的话$I_0=x+C,I_1=\int \frac{\sin x}{\cos x}\text d x=-\int\frac{1}{\cos x}(\text d\cos x)=-\ln (\cos x)+C$.

##### 分部积分

我们应当有$(uv)'=u'v+v'u$,两边积分得到$uv=\int u'v\text dx+\int v'u\text d x$,变形得到$\int v\text d u=uv-\int u\text d v$.

###### Example1

求$I_n=\int \cos^n x\text d x$.

注意到:
$$
I_n=\int \cos^{n-1}x(\text d \sin x)\\=\cos^{n-1}x\sin x+(n-1)\int \sin^2 x\cos ^{n-2}x\text d x\\
=\cos^{n-1}x\sin x+(n-1)\int (1-\cos^2 x) \cos ^{n-2}x\text d x\\
=\cos^{n-1}\sin x+(n-1)I_{n-2}-(n-1)I_n
$$
整理得到$I_n=\frac{1}{n}\cos^{n-1}x\sin x+\frac{n-1}{n}I_{n-2}$.

边界条件当然是$I_0=x+C,I_1=\sin x+C$.

###### Example2

求$I_n=\int \frac{\text d x}{\cos^n x}$.

注意到:
$$
I_n=\int \frac{\text d \tan x}{\cos^{n-2} x}\\
=\frac{ \tan x}{\cos^{n-2} x}-\int \tan x(\text d \cos^{2-n}x)\\
=\frac{ \tan x}{\cos^{n-2} x}+(2-n)\int \cos^{-n} x\sin^2 x\text d x\\
=\frac{ \tan x}{\cos^{n-2} x}+(2-n)\int \cos^{-n} x(1-\cos^2 x)\text d x\\
=\frac{ \tan x}{\cos^{n-2} x}+(2-n)I_n-(2-n)I_{n-2}
$$
整理得到$I_n=\frac{1}{n-1}\frac{\tan x}{\cos^{n-2}x}+\frac{n-2}{n-1}I_{n-2}$.

###### Example3

求$I_{m,n}=\int \sin^m x\cos (nx)\text d x$,其中$m<n$.

注意到:
$$
I_{m,n}=\int \sin^m x\cos(nx)\text d x\\
=\frac{1}{n}\int \sin^m x\text d \sin (nx)\\
=\frac{1}{n}\left(\sin^m x\sin (nx)-\frac{m}{n}\int \sin (nx)\sin^{m-1}x\cos x\text d x\right)
$$
然后观察到:
$$
\int \sin (nx)\sin^{m-1}x\cos x\text d x\\
=-\frac{1}{n}\int \sin^{m-1} x\cos x\text d \cos(nx)\\
=-\frac{1}{n}(\cos (nx)\sin^{m-1}x\cos x-\int \cos (nx)(\text d\sin^{m-1}x\cos x))
$$
观察到:
$$
\int \cos (nx)(\text d\sin^{m-1}x\cos x)\\
=\int \cos(nx)(-\sin^{m}+(m-1)\sin^{m-2}x\cos^2x)\text d x\\
=\int \cos(nx)((m-1)\sin^{m-2}x-m\sin^m x))\text d x\\
=(m-1)I_{m-2,n}-mI_{m,n}
$$
往回倒腾倒腾,边界条件是:
$$
I_{0,n}=\int \cos(nx)\text dx=\frac{\sin(nx)}{n}+C\\
I_{1,n}=\int \sin x\cos(nx)\text d x\\=\frac{1}{2}\int(\sin((n+1)x)-\sin((n-1)x))\text d x\\
=\frac{1}{2}(\frac{-\cos((n+1)x)}{n+1}+\frac{\cos ((n-1)x)}{n-1})+C
$$

###### Example4

求$I_n=\int \frac{\text d x}{(x^2+a^2)^n}$.

注意到:
$$
I_n=\frac{x}{(x^2+a^2)^n}-\int x\text d \frac{1}{(x^2+a^2)^n}\\
=\frac{x}{(x^2+a^2)^n}+2n\int \frac{x^2+a^2-a^2}{(x^2+a^2)^{n+1}}\text d x\\
=\frac{x}{(x^2+a^2)^n}+2nI_{n}-2a^2nI_{n+1}
$$
边界是$I_1=\frac{1}{a}\arctan \frac{x}{a}+C$.

###### Example5

求$I=\int x \cos ^3 x\text d x$.

考虑:

$$
I=\int x \cos ^3 x\text d x=\int x\cos^2 x\text d(\sin x)\\
=x\cos^2 x\sin x-\int \sin x(\cos^2 x\text d x-2x\cos x\sin x)\\
=x\cos^2 x\sin x+\int \cos^2 x (\text d \cos x) +2\int x\cos x\sin^2 x\text d x\\
=x\cos^2 x\sin x+\frac{\cos^3 x}{3} +2\int x\cos x(1-\cos^2 x)\text d x\\
=x\cos^2 x\sin x+\frac{\cos^3 x}{3} +2\int x\cos x\text d x-2I\\
=x\cos^2 x\sin x+\frac{\cos^3 x}{3} +2\int x(\text d \sin x)-2I\\
=x\cos^2 x\sin x+\frac{\cos^3 x}{3} +2(x\sin x+\cos x)-2I+C\\
$$
也就是$3I=x\cos^2 x\sin x+\frac{\cos^3 x}{3} +2(x\sin x+\cos x)+C$.

##### 有理分式的不定积分

考虑真分式$R(x)=\frac{P(x)}{Q(x)}$,其中$\deg P(x)<\deg Q(x)$.至于其他分式可以做长除法变成真分式.

定义最简真分式形如$\frac{A}{(x-a)^m},\frac{Bx+C}{(x^2+px+q)^n}$.

先证明一个引理:实数域上,任何一个多项式可以分解为若干不可约的一次多项式和二次多项式的乘积.

原因是复根一定成对出现,如果$z$是复根,那么$\bar z$一定也是复根,原因是$\overline{zw}=\bar z\bar w$,对一个多项式两边逐项取共轭就可以发现上述结论.那只需要把成对的复根扔到一个二次多项式里就行.

再证明一个引理:实数上,任何一个真分式都可以转化为若干最简真分式的线性组合.

先拆一次项,如上将$Q(x)=(x-a)^mR(x)$,其中$R(a)\ne 0$,那我们考虑:
$$
\frac{P(x)}{Q(x)}-\frac{A_1}{(x-a)^m}=\frac{P(x)-A_1R(x)}{(x-a)^mR(x)}\\
$$
此时取$A_1=\frac{P(a)}{R(a)}$,立刻见到$(x-a)|(P(x)-A_1R(x))$,于是上下至少少一次,这样就可以继续分解了.

再拆二次项,如上将$Q(x)=(x^2+px+q)^mR(x)$,其中$(x^2+px+q)\nmid R(x)$,那:
$$
\frac{P(x)}{Q(x)}-\frac{A_1x+B_1}{(x^2+px+q)^m}=\frac{P(x)-(A_1x+B_1)R(x)}{(x^2+px+q)^mR(x)}
$$
令$T(x)=P(x)-(A_1x+B_1)R(x)$,类似上面应当要确定$A_1,B_1$使得上面那部分是$x^2+px+q$的倍数就可以.直接取$x^2+px+q$的某个复根$\alpha$,则只需$T(\alpha)=0$即可,这是两个方程(实部虚部),而我们有两个未知数,理应可以解出答案.具体解得过程其实也可以直接把$P(x)$和$R(x)$直接先用多项式长除法杀一杀然后剩的部分判一下方程.

这样最后就只需要对拆出来的东西逐项积分就行.意味着有理函数的原函数是初等函数.

##### 有理三角函数的不定积分

即$\sin x,\cos x$经过有限步四则运算得到的函数.

我们定义二元有理函数$R(u,v)$,那么有理三角函数实际上是$R(\cos x,\sin x)$.

下面给出以下命题:

1. 如果其对于$u$是奇函数,也即$R(u,v)=-R(-u,v)$,那么$R(u,v)=uR_1(u^2,v)$,其中$R_1$是另一个二元有理函数.如果对于$v$是奇函数则同理.
2. 当$R(-u,-v)=R(u,v)$时,那么$R(u,v)=R_1(\frac{u}{v},v^2)$.

证明只需要拆做$R(u,v)=\frac{P(u,v)}{Q(u,v)}$,其中$P,Q$是两个多项式,然后只需简单书写即可证明.

对于(1)形式的有理三角函数,注意到:
$$
\int R(\cos x,\sin x)\text d x\\
=\int \cos xR(\cos^2x,\sin x)\text d x\\\\
=\int R_1(1-\sin ^2 x,\sin x)\text d(\sin x)\\
=\int R_1(1-t^2,t)\text d t
$$
对于(2)形式的有理三角函数,我们有:
$$
\int R(\cos x,\sin x)\text d x\\
=\int R_1(\cos^2 x,\tan x)\text d x\\
=\int R_1(\frac{1}{1+\tan^2x},\tan x)\text d x\\
=\int R_1(\frac{1}{1+t^2},t)\frac{\text d t}{1+t^2}
$$
有了这两种形式后呢?我们注意到任意的有理三角函数都可以表示为以上两种函数之和,原因是:
$$
R(u,v)\\=\frac{R(u,v)-R(-u,v)}{2}\\+\frac{R(-u,v)-R(-u,-v)}{2}\\+\frac{R(-u,-v)+R(u,v)}{2}
$$
或者,令$t=\tan \frac{x}{2}$,那么$\sin x=\frac{2t}{1+t^2},\cos x=\frac{1-t^2}{1+t^2},\text d x=2\frac{\text d t}{1+t^2}$.

##### 无理函数的不定积分

我们考虑以下几类无理函数:

第一类是$R(x,\sqrt[m]{\frac{ax+b}{cx+d}})$的形式.只需做换元$t^m=\frac{a_1x+b_1}{a_2x+b_2}$,立刻得到原式其实就是$R(\frac{b_2t^m-b_1}{a_1-a_2t^m},t)$,并且$\text d x$也可以转化到$\text d t$.

第二类是形如$\int x^m(a+bx^n)^p\text d x$,其中$m,n,p\in \mathbb Q$.

令$t=x^n$,上式就变成了$\frac{1}{n}\int t^{\frac{m+1}{n}-1}(a+bt)^p\text d t$.令$q=\frac{m+1}{n}-1$.我们只需要做$\frac{1}{n}\int t^q(a+bt)^p\text d t=\frac{1}{n}\int t^{p+q}(\frac{a+bt}{t})^p$.事实上,下面三种情况可以积出来:

1. $p\in \Z$.
2. $q\in \Z$.
3. $p+q\in \Z$.

其它的都无初等形式.

第三类是形如$R(x,\sqrt {ax^2+bx+c})$,不妨假设$b^2-4ac\ne 0,a\ne 0$.

1. $b^2-4ac>0$,则有两个实根$\alpha>\beta$,此时提一下实根就可以知道原式等于$R(x,(x-\beta)\sqrt{\frac{a(x-\alpha)}{x-\beta}})$,这就能做了.
2. $b^2-4ac<0$,则无实根,不妨假设$a>0$.则做$t-\sqrt a x=\sqrt{ax^2+bx+c}$并两边平方就可以反解出$x=\frac{t^2-c}{b+2\sqrt a t}$,原本的积分就变成了关于$t$的有理函数的积分.

##### 双曲换元

引入双曲三角函数$\sinh (x)=\frac{e^x-e^{-x}}{2},\cosh=\frac{e^x+e^{-x}}{2}$以及$\tanh(x)=\frac{\sinh(x)}{\cosh(x)}$.留意到$\cosh^2-\sinh^2=1$.以及:

1. $\frac{\text d }{\text d x}\sinh(x)=\cosh(x)$
2. $\frac{\text d }{\text d x}\cosh(x)=\sinh(x)$
3. $\frac{\text d }{\text d x}\tanh(x)=\frac{1}{\cosh^2(x)}=1-\tanh^2(x)$

应当发现:

1. $\sinh(2x)=2\sinh(x)\cosh(x)$
2. $\cosh(2x)=\cosh^2(x)+\sinh^2(x)=2\cosh^2(x)-1=1+2\sinh^2(x)$

另外其反函数应当是:

1. $\text{arsinh}(x)=\ln(x+\sqrt{x^2+1})$
2. $\text{arcosh}(x)=\ln(x+\sqrt{x^2-1})$

应当发现:

1. $\frac{\text d }{\text d x}\text{arsinh}(x)=\frac{1}{\sqrt{x^2+1}}$
2. $\frac{\text d }{\text d x}\text{arcosh}(x)=\frac{1}{\sqrt{x^2-1}}$
3. $\frac{\text d }{\text d x}\text{artanh}(x)=\frac{1}{1-x^2}$

###### Example1

求$I=\int \frac{\text d x}{\sqrt{x^2\pm a^2}}$,其中$a>0$.

于是$\int \frac{\text d x}{\sqrt{x^2\pm a^2}}=\ln |x+\sqrt{x^2\pm a^2}|+C$.

###### Example2

求$I=\int \sqrt{a^2-x^2}\text d x$.

考虑三角换元,应当小心符号,这里注意到$x\in [-a,a]$,因此直接取$x=a\sin t$,$t\in [-\frac{\pi}{2},\frac{\pi}{2}]$,此时$\cos t \geq 0$恒成立,则有:

$$
I=a^2\int \cos^2 t\text d t\\
=a^2\int \frac{1+\cos(2t)}{2}\text d t\\
=\frac{a^2t}{2}+\frac{a^2}{4}\sin (2t)+C\\
=\frac{1}{2}(x\sqrt {a^2-x^2}+a^2\arcsin\frac{x}{a})+C
$$

###### Example3

求$I=\int \sqrt{x^2\pm a^2}\text d x$.

使用双曲换元,先考虑$\int \sqrt{x^2+ a^2}\text d x$的情况,此时换$x=a\sinh t$,$\cosh t\geq 0$,根号是平凡开出的,应当有:

$$
I=a^2\int \cosh^2 t\text d t\\
=a^2\int\frac{1+\cosh (2t)}{2}\text d t\\
=\frac{a^2t}{2}+\frac{a^2}{4}\sinh (2t)+C\\
=\frac{1}{2}(x\sqrt {x^2+a^2}+a^2\text{arsinh}\frac{x}{a})+C
$$

另一个同理,仍然使用双曲换元,结果是$\frac{1}{2}(x\sqrt {x^2-a^2}+a^2\text{arcosh}\frac{x}{a})+C$

##### 椭圆积分

以下是几个常见的无初等形式的积分:

1. $\int \frac{e^x}{x}\text d x$.
2. $\int \frac{\text d x}{\ln x}$.
3. $\int \frac{\sin x}{x}\text d x$.
4. $\int \frac{\cos x}{x}\text d x$.
5. $\int e^{x^2}\text d x$.

另外还有一类椭圆积分也无初等形式:设$R(x,y)$是二元有理函数,其中$y=\sqrt {P_3(x)}$或$\sqrt {P_4(x)}$,也即根号下放了个三次或者四次多项式,那么$\int R(x,y)\text d x$可能很难有初等形式.这样的积分可以被归结为三类椭圆积分.

首先说明$R(x,\sqrt {P_3(x)})$可以转化,原因是$P_3(x)=0$一定有至少一个实数解,任意取出一个设为$\lambda$,则$P_3(x)=(x-\lambda)P_2(x)$,令$t^2+\lambda=x$,带入可以转化为$R(t^2+\lambda,\sqrt {P_2(t^2+\lambda)})=R_1(t,\sqrt {P_4(t)})$.这就意味着其实根号下四次多项式是更本质一些的.

而四次多项式可以分解为$P_4(x)=a(x^2+p_1x+q_1)(x^2+p_2x+q_2)$.不妨假设其无重根(如果有重根的话,那么根号下就可以开出去一部分,那就可积了),设其四个根(可能有复根)分别为$\alpha_1,\beta_1,\alpha_2,\beta_2$,并且$\alpha_1,\beta_1$是上述分解中前半部分的根,特别地,如果这是四个实根,则不妨调整它们的顺序使得$\alpha_1>\beta_1>\alpha_2>\beta_2$.那么根据韦达定理总有:$p_1=-(\alpha_1+\beta_1),q_1=\alpha_1\beta_1$,后半部分同理.

我们尝试把这两个式子的一次项都去掉,当$p_1=p_2$的时候可以直接一起配方令$x=t-\frac{p}{2}$.当$p_1\ne p_2$的时候,尝试换元用$t$代替$x$,不妨设$x=\frac{u t+v}{1+t}$,那么$(x^2+p_1x+q_1)=\frac{P_2'(t)}{(1+t)^2}$.为了让其没有一次项应当解以下方程:
$$
\begin{cases}
2u v+p_1(u +v)+2q_1=0\\
2u v+p_2(u +v)+2q_2=0
\end{cases}
$$
此时可以解出来$\begin{cases}u+v=-\frac{2(q_1-q_2)}{p_1-p_2}\\u v=\frac{p_2q_1-p_1q_2}{p_1-p_2}\end{cases}$,为了解出$u ,v$是两个实数,应当检验$(u+v)^2-4uv\geq 0$是否成立.带入韦达定理发现左边等于$(\alpha_1-\alpha_2)(\alpha_1-\beta_2)(\beta_1-\alpha_2)(\beta_1-\beta_2)>0$.原因是如果是四个实根,则根据假设立刻得到.如果存在复根则配对一下复根也可以立刻得到.

总之,在上面的一系列操作后,我们要研究的形式统一成了$R(t,y=\sqrt{A(1+m_1t^2)(1+m_2t^2)})=\frac{a(t)+b(t)y}{c(t)+d(t)y}=B_1(t)+B_2(t)y$,其中$B_1(t),B_2(t)$都是$t$的有理函数.前者可以直接积分拆出去,那我们关心的就只剩下后面的$B_2(t)y=\frac{B_2(t)y}{y}$的部分.

我们之前已经提过可以将一个函数拆为偶函数和奇函数之和,事实上:
$$
R(t)=\frac{R(t)+R(-t)}{2}+\frac{R(t)-R(-t)}{2}=R_1(t^2)+tR_2(t^2)
$$
那么后者立刻可以换元积分,只剩下前者做不动.剩下的形式是:$\int \frac{R(t^2)}{\sqrt {A(1+m_1t^2)(1+m_2t^2)}}\text d t$,其中$A$可以先扔出去一些,使得$A=\pm 1$.而且由于被积函数是偶函数,所以还可以不妨设$t\geq 0$.由于根号下不能变成二次函数(不然就直接做完了),还不妨设$m_1\ne m_2,m_1\ne 0,m_2\ne 0$.下面开始讨论:

1. $A=1$,$m_1=-h_1^2,m_2=-h_2^2,h_1>h_2>0$.

此时令$t=\frac{z}{h_1}$带入化简变形再扔出去点常数得到原式变化为$\int \frac{R(z^2)}{\sqrt{(1-z^2)(1-k^2z^2)}}\text d z$,其中$k=\frac{h_2}{h_1}$.

2. $A=1$,$m_1=-h_1^2,m_2=h_2^2,h_1>0,h_2>0$.

此时令$t=\frac{\sqrt{1-z^2}}{h_1}$.扔出去一些常数仍然可以化简成(1)的形式,只是$k=\frac{h_2}{\sqrt{h_1^2+h_2^2}}$.

3. $A=1,m_1=h_1^2,m_2=h_2^2,h_1>h_2>0$.

令$t=\frac{z}{h_1\sqrt{1-z^2}}$.扔出去一些常数仍然是(1)的形式,只是$k=\frac{\sqrt{h_1^2-h_2^2}}{h_1}$.

4. $A=-1$,$m_1=-h_1^2,m_2=h_2^2,h_1>0,h_2>0$.

令$t=\frac{1}{h_1\sqrt{1-z^2}}$,还是(1)的形式,只是$k=\frac{h_1}{\sqrt{h_1^2+h_2^2}}$.

5. $A=-1$,$m_1=-h_1^2,m_2=-h_2^2,h_1>h_2>0$.

令$t=\frac{\sqrt{1-\frac{h_1^2-h^2_2}{h_1^2}z^2}}{h_2}$,还是(1),只是$k=\frac{\sqrt{h_1^2-h_2^2}}{h_1}$.

这样就全都转化为了$\int \frac{R(z^2)}{\sqrt{(1-z^2)(1-k^2z^2)}}\text d z$的形式了.然后把$R(z^2)$给拆开,那么这个式子就应该是$I_n=\int\frac{z^{2n}}{\sqrt{(1-z^2)(1-k^2z^2)}}\text d z$和$H_m=\int\frac{\text d z}{(z^2-a)^m\sqrt{(1-z^2)(1-k^2z^2)}}$的线性组合,注意这里系数有可能是复数.

此时注意到
$$
(z^{2n-3}\sqrt{(1-z^2)(1-k^2z^2)})'\\
=(2n-3)z^{2n-4}\sqrt{(1-z^2)(1-k^2z^2)}+z^{2n-3}\frac{2k^2z^3-(k^2+1)z}{\sqrt{(1-z^2)(1-k^2z^2)}}\\
=\frac{(2n-3)z^{2n-4}{(1-z^2)(1-k^2z^2)}+z^{2n-3}(2k^2z^3-(k^2+1)z)}{\sqrt{(1-z^2)(1-k^2z^2)}}\\
=\frac{(2n-1)k^2z^{2n}-(2n-2)(k^2+1)z^{2n-2}+(2n-3)z^{2n-4}}{\sqrt{(1-z^2)(1-k^2z^2)}}
$$
两边积分得到:
$$
(2n-1)k^2I_n-(2n-2)(k^2+1)I_{n-1}+(2n-3)I_{n-2}\\
=z^{2n-3}\sqrt{(1-z^2)(1-k^2z^2)}
$$
上述只要$n$是整数就行,甚至不需要$n$是正的.所以所有的$I_n$,包括负整数$n$,都可以被$I_0,I_1$表示.

接下来看$H_m$,当$a=0$的时候无非是$H_m=I_{-m}$,

接下来注意到:
$$
(\frac{z}{(z^2-a)^{m-1}}\sqrt{(1-z^2)(1-k^2z^2)})'\\
=\frac{(2-2m)z^2+z^2-a}{(z^2-a)^m}\sqrt{(1-z^2)(1-k^2z^2)}\\
+\frac{2k^2z^4-(k^2+1)z^2}{\sqrt{(1-z^2)(1-k^2z^2)}}\frac{1}{(z^2-a)^{m-1}}\\
$$
令$C=\sqrt{(1-z^2)(1-k^2z^2)}$,$W=z^2-a$,那么$C^2=k^2(W+a)^2-(k^2+1)(W+a)+1$.上式化简为:
$$
=\frac{(2-2m)(W+a)C^2}{W^mC}+\frac{C^2+2k^2(W+a)^2-(k^2+1)(W+a)}{W^{m-1}C}\\
=\frac{(2-2m)(W+a)(k^2(W+a)^2-(k^2+1)(W+a)+1)}{W^mC}\\+\frac{3k^2(W+a)^2-2(k^2+1)(W+a)+1}{W^{m-1}C}
$$
接下来是繁复的化简,然后两边积分得到:
$$
\frac{z}{(z^2-a)^{m-1}}\sqrt{(1-z^2)(1-k^2z^2)}\\
=(2m-2)(-a+(k^2+1)a^2-k^2a^3)H_m\\
-(2m-3)(1-2a(k^2+1)+3k^2a^2)H_{m-1}\\
+(2m-4)(k^2+1-3k^2a)H_{m-2}\\
-(2m-5)k^2H_{m-3}
$$
而注意到$H_0=I_0,H_{-1}=I_1-aI_0$.

综上,所有的椭圆积分都可以由$I_0,I_1,H_1$表示.也就是椭圆积分可以转化为以下三种积分:

1. $\int\frac{\text d z}{\sqrt{(1-z^2)(1-k^2z^2)}}$.
2. $\int \frac{z^2\text d z}{\sqrt{(1-z^2)(1-k^2z^2)}}$.
3. $\int \frac{\text d z}{(1+hz^2)\sqrt{(1-z^2)(1-k^2z^2)}}$,其中$h$可以是复数.

有意思的是如果令$z=\sin \varphi$,$\text d z=\sqrt{1-z^2}\text d \varphi$,这样形式就好看了很多.椭圆积分可以用以下的优美形式表示:

1. $E(k,\varphi)=\int\frac{\text d \varphi}{\sqrt{1-k^2\sin^2\varphi}}$.
2. $F(k,\varphi)=\int \sqrt{1-k^2\sin^2\varphi}\ \text d \varphi$.
3. $\Pi(h,k,\varphi)=\int \frac{\text d \varphi}{(1+h\sin^2\varphi)\sqrt{1-k^2\sin^2\varphi}}$,其中$h$可以是复数.

其中:
$$
\int \frac{z^2\text d z}{\sqrt{(1-z^2)(1-k^2z^2)}}=\int \frac{\sin^2\varphi\text d \varphi}{\sqrt{1-k^2\sin^2\varphi}}\\=\int(\frac{1}{k^2\sqrt{1-k^2\sin^2\varphi}}\text )-\frac{1-k^2\sin^2\varphi}{k^2\sqrt{1-k^2\sin^2\varphi}}\text )d \varphi\\
=\frac{E(k,\varphi)}{k^2}-\frac{F(k,\varphi)}{k^2}
$$

#### 黎曼积分

设$f(x)$是定义在$[a,b]$上的函数,对于区间$[a,b]$插入分点$x_i(i=0,1,\cdots,n)$,且有$a=x_0<x_1<\cdots<x_n=b$,我们称之为对区间$[a,b]$的一种**分割**,并记为$\Delta$.又记$\Delta x_i=x_i-x_{i-1},i\in[1,n]$,$\lambda(\Delta)=\max\{\Delta x_i\mid i\in[1,n]\}$,并定义黎曼和$S_\Delta=\sum_{i=1}^nf(\xi_i)\Delta x_i$.

若存在实数$J$,对$\forall \epsilon>0,\exists \delta>0$,对于任意分划,只要$\lambda(\Delta)<\delta$,都有$|S_\Delta -J|<\epsilon\\$,则称$f(x)$在$[a,b]$上**黎曼可积**,这个$J$为$f(x)$在$[a,b]$上的**定积分**,记作$\int_{a}^b f(x)\text d x\\$.容易见到定积分唯一,证明无非也是拿$\epsilon$去卡.

对于$[a,b]$上的连续函数,定积分实际上就是其与$f(x)$围成面积的代数和.

由此可以顺便定义**达布上下和**,即令$m_i=\inf_{[x_{i-1},x_i]}f(x),M_i=\sup_{[x_{i-1},x_i]}f(x)$,并定义**振幅**$w_i=M_i-m_i$,特别地定义$M=\sup(M_i),m=\inf(m_i)$.定义$\overline S_\Delta=\sum M_i\Delta x_i,\underline S_{\Delta}=\sum m_i\Delta x_i$.

并在此基础上定义上下积分,上积分$\overline{\int_a^b}f(x)\text d x=\inf \overline S_\Delta,\underline{\int_a^b}f(x)\text d x=\sup \underline S_\Delta$.上下积分显然都是存在的,我们会在后面证明黎曼可积等价于上下积分相等.

为了方便,我们约定:

1. $a=b$,$\int _a^b f(x)\text d x=0$.
2. $a>b$,$\int _a^b f(x)\text d x=-\int _b^a f(x)\text d x$.

下列性质应当是成立的:

1. (必要条件)如果一个函数黎曼可积,则它在该闭区间上有界.
2. 对于任意分划任取$\xi$,都有$\underline{S}_\Delta\leq S_\Delta(\xi)\leq \overline S_\Delta$.
3. $\forall \epsilon>0$,$\exists \xi_1,\xi_2$,使得$\underline S_\Delta-\epsilon\leq S_\Delta(\xi_1)$,$\overline S_\Delta+\epsilon\geq S_\Delta(\xi_2)$.

再定义所谓加细:如果$\Delta_1\subseteq \Delta_2$,那么称$\Delta_2$是$\Delta_1$的加细.此时应当有$\overline S_{\Delta_1}\geq \overline S_{\Delta_2},\underline S_{\Delta_1}\leq \underline S_{\Delta_2}$.另外容易注意到如果$|\Delta_2\setminus \Delta_1|=k$,那么$0\leq \overline S_{\Delta_1}-\overline S_{\Delta_2} \leq k\lambda(\Delta_1)(M-m)$,原因是考虑把这些新的断点一个一个加上去,每次加一个会新断开原本的断点,此时最多造成$(M-m)\Delta x\leq (M-m)\lambda(\Delta_1)$的差.注意到此时这些东西似乎全都可以被限制住.

这同样给出了原因:为什么我们会定义上下积分呢?因为随着分点的加多,也就是随着划分的加细,达布上下和会分别单调递减或递增,这样就应当存在一个极限,然而问题在于我们难以在划分上定义序,这样就少了某种极限的方式.然而,我们试图绕开来找到一种确定的能探到极限的方式.

另外的结论是对于任意两个分划$\Delta_1,\Delta_2$,总有$\underline S_{\Delta_1}\leq \overline S_{\Delta_2}$.原因是$\Delta=\Delta_1\cup \Delta_2$,立刻见到$\underline S_{\Delta_1}\leq \underline S_{\Delta}\leq \overline S_\Delta\leq \overline S_{\Delta_2}$.这意味着$\overline \int f\geq \underline \int f$.

接下来证明定积分中的**达布定理**,也就是$\forall \epsilon>0,\exists \delta>0$,$\forall \Delta,\lambda(\Delta)<\delta$的时候,$0\leq \overline S_\Delta-\overline {\int_a^b}f(x)\text d x\leq \epsilon$.换言之$\lim_{\lambda(\Delta)\to 0}\overline S_{\Delta}=\overline{\int_a^b}f(x)\text d x$,当然下积分也是同理的.

这个的证明考虑先用上积分的定义,应当存在一个$\Delta'$,满足$0\leq \overline S_{\Delta'}-\overline {\int_a^b}f(x)\text d x\leq \frac \epsilon 2$.不妨设$|\Delta'|=n_0$,接下来取$\delta<\frac{\epsilon}{2n_0(M-m)}$,那么接下来对于$\forall \Delta,\lambda(\Delta)<\delta$的时候,取$\Delta^*=\Delta'\cup \Delta$,根据上面我们证明的步骤,这里最多会多$n_0$个断点,自然得知$\overline S_{\Delta}-\overline S_{\Delta^*}\leq n_0\lambda(\Delta)(M-m)<\frac{\epsilon}{2}$.

这样的话就可以注意到:
$$
\overline S_\Delta-\overline{\int_a^b}f(x)\text d x\\=(\overline S_{\Delta'}-\overline{\int_a^b}f(x)\text d x)-(\overline S_{\Delta'}-\overline S_{\Delta})\\
<\frac{\epsilon}{2}-(\overline S_{\Delta^*}-\overline S_\Delta)<\epsilon
$$

这里可以证明原函数可积的充分必要条件是上下积分相等.

首先证明充分性:当上下积分相等的时候,设其为$J$,取足够小的$\epsilon$,由上知道$\exists \delta>0$,使得$J-\epsilon<\underline S_\Delta\leq S_\Delta(\xi)\leq\overline S_\Delta<J+\epsilon$,夹逼一下立刻知道其满足.

然后证明必要性:当原函数可积分的时候,只要分点足够密,应当有$J-\epsilon<\underline S_\Delta\leq \underline{\int}f(x)\text d x\leq\overline{\int}f(x)\text d x\leq\overline S_\Delta< J+\epsilon$.

而由于我们可以见到上和与下和应当是错开来的,而如果它们中间能接上自然意味着上下积分相等.因此推论为:如果$f$在闭区间上有界并且$\forall \epsilon,\exists \Delta$使得$0\leq \overline S_\Delta- \underline S_\Delta\leq \epsilon$,则$f$可积.我们最初拿到的那个定义也可以这么改:只需要存在一个分划而不需要使得任意分划都这么满足.方式就是用我们已经找到的这个去并我们想要限制的那个,然后把限制延后一点使得这些全被限制住.

由上述可以证明闭区间上的连续函数一定是闭区间上的可积函数.原因是闭区间上的连续函数一定是一致连续函数,那么对于一个$\epsilon$,可以找到一个$\delta$控制住区间长度,使得极差$<\epsilon$.此时直接找一个$\Delta$,注意到$\overline S-\underline S=\sum (M_i-m_i)\Delta x_i\leq \sum \epsilon \Delta x\leq (b-a)\epsilon$,只需取$\epsilon\to 0$就可以搞定.

还可以证明如果$f$在闭区间上有界并且只有有限个间断点,那么$f$可积.原因也很简单,只需要拿足够小的区间把这有限个间断点盖住,然后外面继续如上操作,间断点处尽可能缩小区间长度来取出其影响.

还可以证明单调函数一定可积,原因是$\overline S-\underline S\leq \delta(f(b)-f(a))\to 0$.

##### 可积性与连续性的关系(勒贝格定理)

我们还可以进一步探索黎曼可积和连续性的关系,例如,我们可以证明:$\forall f\in R[a,b],\forall \epsilon>0,\exists g\in C[a,b] \int_a^b |f(x)-g(x)|\text d x<\epsilon$.

如何证明呢?感觉上只需要用分段线性函数去逼近一下就可以了对吧.所以我们考虑对于划分$\Delta$,直接把所有的$(x_i,f(x_i))$连起来形成一个分段线性函数.然后观察此时的$|f-g|$,不妨分段考虑,对于$\int_{x_{i-1}}^{x_i}(f-g)(x)\text d x\leq (M_i-m_i)\Delta x_i$,而后者求和之后实际上就是$\overline S-\underline S$,当$f$可积的时候这个当然会趋近于$0$.

我们还可以证明$f\in R[a,b]$的连续点在$[a,b]$上稠密,只需要证明$\forall [\alpha,\beta]\in R[a,b],\alpha<\beta$,$[\alpha,\beta]$这个区间内存在连续点即可.考虑定义一点处的振幅$w_{f}(x_0)=\lim_{\delta\to +0}w_f(x_0-\delta,x_0+\delta)$,这个随着$\delta$缩小当然应当是单调不增的,因此一定有非负极限(但可能是无穷大).我们注意到$f$在$x_0$处连续的充要条件是$f$在$x_0$处的振幅为$0$.必要性和充分性都容易证明.

我们考虑用区间套来找到这个连续点,考虑对于$[\alpha_0,\beta_0]\in [a,b]$,取分划$\Delta_0$使得$\sum w_i\Delta x_i<\frac{1}{2}(\beta_0-\alpha_0)$,并且加细使得$\lambda(\Delta_0)<\frac{1}{2}(\beta_0-\alpha_0)$,注意这里的两个$\frac{1}{2}$其实用处不同,前者是为了让$w_i$足够小,后者是为了让区间长度足够小.此时考虑由于$\sum \Delta x_i=\beta_0-\alpha_0$,所以一定存在一个$w_i<\frac{1}{2}$,拿出这个区间作为$[\alpha_1,\beta_1]$,则区间长度至少折半,并且只需在上述对$[\alpha_n,\beta_n]$时取$\sum w_i\Delta x_i<\frac{1}{2^{n+1}}(\beta_0-\alpha_0)$就可以使得$w_i$也趋近于$0$,这样就能用区间套找到一个点使得它是连续点.

事实上可以证明一个函数黎曼可积的充分必要条件是其有界并且几乎处处连续.

为了方便不妨把$f$延拓到整个实轴,令$f(x)=\begin{cases}f(a)&x\leq a\\f(b)&x\geq b\\f(x)&otherwise\end{cases}$.

下面定义$E_r=\{x_0\mid w_f(x_0)\geq \frac{1}{r}\}$.我们考虑证明其是闭集,也就是证明其补集$E_r^c=\{x_0\in (a,b)\mid w_f(x_0)<\frac{1}{r}\}$是开集.如果$x_0\in E_r^c$,注意到此时$\exists \delta>0,w_f(x_0-\delta,x_0+\delta)<\frac{1}{r}$,那么这必然意味着$x_0$邻域中的点的振幅也会$<\frac{1}{r}$,这当然意味着其是开集.

令$D$是$f$的间断点集,容易见到$D=\bigcup_{n=1}^{\infty}E_n$,下面只需要证明每个$E_n$都是零测集即可.

先证明必要性:当$f\in R[a,b]$时,则$f$有界,现在对于固定的$m$,尝试证明$E_m$是零测的,由达布定理,$\forall \epsilon>0$,$\exists \Delta$,使得$\sum w_i\Delta x_i=\overline S_\Delta-\underline S_\Delta<\epsilon$.

观察前者,注意到:

$$
\epsilon>\sum w_i\Delta x_i=\overline S_\Delta-\underline S_\Delta\\
=\sum_{E_m\cap (x_{k-1},x_k)\ne \empty}w_k\Delta x_k+\sum_{E_m\cap (x_{k-1},x_k)=\empty}w_k\Delta x_k\\
\geq \frac{1}{m}\sum_{E_m\cap (x_{k-1},x_k)\ne \empty}\Delta x_k
$$

因此$m^*(E_m)\leq \sum_{E_m\cap (x_{k-1},x_k)\ne \empty}\Delta x_k<m\epsilon$,由于$m$一开始就固定,$\epsilon$可以尽量小,所以这意味着$E_m$是零测集.必要性得证.

再证明充分性:当每一个$E_m$都是零测集的时候,$\forall \epsilon>0$,取$r\in N_+,r>\frac{1}{\epsilon}$.此时见到$E_{r}^c$是开集,那它可以写成若干两两不交开区间的并,不妨记作$E_r^c=\bigcup_{k=1}^{\infty}T_k$.由于此时$E_m$是零测集,这意味着$\sum |T_k|=b-a$.接下来可以取足够大的$n$使得$\sum_1^n |T_k|>b-a-\frac{\epsilon}{2}$,接下来取$T_k$的闭子区间$J_k$,使得$\sum_1^n |J_k|>b-a-\epsilon$.这里为什么要把开区间再进一步缩成闭区间呢?原因是你这里如果直接用开区间构造分划可能会把端点处给包进去,因此我们不得不设其端点处的一部分值拿到闭区间,这样那些振幅较大的就被舍弃了.

接下来考虑由于每个点$x_0\in J_k$的$w_f(x_0)<\frac{1}{r}<\epsilon$,必定有其一个小邻域$(x_0-\delta,x_0+\delta)$满足整个邻域的振幅$<\epsilon$,用有限覆盖定理拿出一个开覆盖,然后缩掉这些开覆盖的区间就能拿到一个$J_k$的分划(或者干脆将这些开区间的端点和$J_k$的端点直接当作一个分划),把这些分划全都并起来就得到了一个划分$\Delta$.

此时观察这个划分,应当有:

$$
\overline S_\Delta-\underline S_\Delta\\
=\sum w_k\Delta x_k\\
=\sum_{[x_{k-1},x_k]\subseteq \bigcup J_i}w_k\Delta x_k+\sum_{[x_{k-1},x_k]\subsetneq \bigcup J_i}w_k\Delta x_k\\<\epsilon(b-a)+(M-m)\epsilon
$$

这样就可以使其任意小.

上述证明同样声明了:黎曼可积的充要条件是震荡区间超过某个值的区间长度之和随着细分的增加应当尽可能小.

上述结论立刻能见到比如黎曼函数是黎曼可积的,因为其间断点集就是有理点集,就是零测集.

##### 可积函数的简单性质

不妨设$f,g\in R[a,b]$,则:

1. $m(b-a)\leq \int_a^b f(x)\text d x\leq M(b-a)$.
2. $f\geq 0$则$\int_a^b f(x)\text d x\geq 0$.
3. $\int_{a}^b(f\pm g)\text d x=\int_a^bf\text d x\pm \int_a^bg\text d x$.
4. $\int_{a}^bCf\text d x=C\int_a^bf\text d x$.
5. (保序性)$f\geq g$则$\int_a^b f\text d x\geq \int_a^bg\text d x$.事实上只要$f\geq g$在一个稠密子集上成立即可.
6. 如果$c\in [a,b]$,那么此时$f\in R[a,c],f\in R[c,b]$,并且$\int_a^b f\text d x=\int_a^c f\text d x+\int_c^b f\text d x$.
7. 如果$g$是连续函数,那么$g(f(x))\in R[a,b]$.
8. $|f|\in R[a,b]$,并且$|\int_a^b f(x)\text d x|\leq \int_a^b|f(x)|\text d x$.
9. 如果除了一个有限的点集以外,$g=f$,则$g\in R[a,b]$并且$\int_a^bg\text d x=\int_a^b f\text d x$.
10. 如果$f\geq 0,\int_a^b f\text d x=0$,那么$f$在其连续点集合上恒等于$0$.

(1)(2)是显然的,(3)(4)(5)只需将积分看作黎曼和的极限,(6)的话考虑在$[a,b]$上间断点集都是零测的了,那么其子集$[a,c],[c,b]$上的间断点集当然也是零测集.

(7)的话考虑一个几乎处处连续的函数在外面套一层连续函数当然也是几乎处处连续的.

(8)的话考虑绝对值函数是连续函数,然后注意到$-|f(x)|\leq f(x)\leq |f(x)|$使用保序性就可以了.

(9)的证明是简单的.但是为什么是有限点集而不是零测集呢?因为如果是无限个点,那么它们不仅会影响自己的连续性,还会影响旁边点的连续性,例如迪利克雷函数.这里可以看出来黎曼函数是有其限制性的,推广到勒贝格积分后会消除这个问题.

(10)的话只需考虑反证法,对于一个连续点$x_0$,如果$f(x_0)>0$,那么周围应当有一个小邻域也大于$0$,或者干脆用一致连续性取$>\frac{f(x)}{2}$的一个小邻域,这样就反证完了.

##### 广义原函数

对于$f(x):I\to \mathbb R$,称$F(x)$为其**广义原函数**,若$\forall c,d\in I,c<d$满足$\frac{F(d)-F(c)}{d-c}\in[\inf_{[c,d]} f(x),\sup_{[c,d]} f(x)]$.容易见到原函数一定是广义原函数,原因是拉格朗日中值定理.

应当容易注意到以下命题:

1. 局部有界函数的广义原函数是局部李氏连续的,故几乎处处有导数.
2. $f(x)$在$x_0$处连续,$F(x)$是$f(x)$的广义原函数,则$F'(x_0)$存在并且恰好等于$f(x_0)$.

(1)是显然的.

(2)的证明的话,考虑$|\frac{F(x)-F(x_0)}{x-x_0}-f(x_0)|\leq w_f[x_0,x]$,由于$f(x_0)$连续,那么此时其振幅$w_f[x_0,x]$在$x\to x_0$时应当趋近于$0$,使用夹逼定理,于是证毕.

##### 牛顿-莱布尼茨公式(微积分基本定理)

先证明一个引理:如果$f(x)\in [a,b]$,$F(x)$是其广义原函数,那么$\underline{\int_a^b}f(x)\text d x\leq F(b)-F(a)\leq \overline{\int_a^b}f(x)\text d x $.

证明的话,只需证明对于任意分划$\Delta$都有$\underline{S}_\Delta (f)\leq F(b)-F(a)\leq \overline{S}_\Delta f(x)$.而注意到$F(b)-F(a)=\sum F(x_i)-F(x_{i-1})=\sum \frac{F(x_i)-F(x_{i-1})}{x_i-x_{i-1}}\Delta x_i$.而根据广义原函数的定义,$m_i\leq \frac{F(x_i)-F(x_{i-1})}{x_i-x_{i-1}}\leq M_i$,立刻证毕.

由此显然证明了牛顿-莱布尼茨公式(NL公式):设$f(x)\in R[a,b]$,并且能找到其一个广义原函数$F(x)$,则$F(b)-F(a)=\int_a^b f(x)\text d x$.

接下来声明一个定理:如果一个函数$f(x)\in R[a,b]$,它就存在广义原函数,而且不同的广义原函数最多相差一个常数.

先证明存在性,我们注意到变限积分$F(x)=\int_a^x f(t)\text d t$显然是$f(x)$的广义原函数,而且它是处处连续的,只需套用定义就可以证明.

再证唯一性,设$F_1(x),F_2(x)$都是$f(x)$的广义原函数.根据LN公式,有$F_1(x)-F_1(a)=\int_{a}^xf(t)\text d t=F_2(x)-F_2(a)$,立刻有$F_1(x)-F_2(x)=F_1(a)-F_2(a)=C$.

此时回忆到$f(x)$在$x_0$处连续可以推出$F(x)$可导,那么反过来是否成立呢?考虑以下经典反例:

$$
F(x)=\begin{cases}0&x=0\\x^2\sin \frac{1}{x}&otherwise\end{cases}\\
f(x)=F'(x)=\begin{cases}0&x=0\\2x\sin\frac{1}{x}-\cos\frac{1}{x}&otherwise\end{cases}
$$

但总之可以看到连续函数一定有原函数.

此时回忆到我们曾经定义过左右上下导数$(F_\pm)'$和$(F^\pm)'$.设$f\in R[a,b]$,$F$是$f$的广义原函数,不妨直接设$F(x)=\int_a^x f(t)\text d t$,那么我们声明$\int_a^b(F_\pm)'\text d x=\int_a^b(F^\pm)'\text d x=\int_a^b f(x)\text d x$.

四个证明类似,只考虑其中之一$F_+'$如何证明.取$m_0=\inf_{[c,d]}f$,下面令$G(x)=F(x)-F(c)-m_0(x-c)=\int_c^x(f(t)-m_0)\text d t$.当$x\in [c,d]$时,容易发现$G(x)$单增.既然如此,其任意左右上下导数当然都$\geq 0$,也就是$(G_+)'=(F_+)'-m_0\geq 0$.同理如果取$M_0=\sup_{[c,d]}f$的话还可以证明$(F_+)'\leq M_0$.

那么就有$w_{F_+'}[c,d]\leq w_{f}[c,d]$,我们知道$f$是黎曼可积的,也就是$0\leq \sum w_{F_+'}[x_{i-1},x_i]\Delta x_i\leq \sum w_f[x_{i-1},x_i]\Delta x_i<\epsilon$,此时就可以证明其可积性.

而注意到$f$几乎处处连续,$F$几乎处处可导,并且在$f$的连续点处$f(x)=F_+'$,这个连续点集当然是稠密集,因此它们的积分相等,这样就做完了.

###### Example1

求$\int_a^b\text{sgn}(x)\text d x$.

注意到$\text{sgn}(x)$的广义原函数是$|x|$,所以$\int_a^b\text{sgn}(x)\text d x=|b|-|a|$.

###### Example2(黎曼引理)

设$f\in R[a,b],g\in R[0,T]$并且$g$在$\R$上以$T$为周期.求证:
$$
\lim_{\lambda\to \infty}\int_a^bf(x)g(\lambda x)\text d x=\cfrac{\int_0^T g(x)\text d x}{T}\int_a^b f(x)\text d x
$$
首先注意到:

$$
\lim_{\lambda\to \infty}\int_a^bf(x)g(\lambda x)\text d x\\
=\lim_{\lambda\to \infty}\int_a^bf(x)(g(\lambda x)-\cfrac{\int_0^T g(x)\text d x}{T}+\cfrac{\int_0^T g(x)\text d x}{T})\text d x\\
$$

因此可以用$g(\lambda x)-\cfrac{\int_0^T g(x)\text d x}{T}$代替$g$,这样就能满足$\int_0^T g(x)\text d x=0$.

接下来只需证明$\lim_{\lambda\to \infty}\int_a^bf(x)g(\lambda x)\text d x=0$,不妨设$|f(x)|\leq M,|g(x)|\leq M$.

考虑对分划$\Delta$,满足$\sum w_i\Delta x_i\leq \epsilon$:

$$
|\int_a^b f(x)g(\lambda x)\text d x|\\
=|\sum \int_{x_{i-1}}^{x_i}(f(x)-f(x_i)+f(x_i))g(\lambda x)\text d x|\\
\leq |\sum \int_{x_{i-1}}^{x_i}f(x_i)g(\lambda x)\text d x|+|\sum \int_{x_{i-1}}^{x_i}(f(x)-f(x_i))g(\lambda x)\text d x|\\
\leq \sum |f(x_i)|\sdot|\int_{x_{i-1}}^{x_i}g(\lambda x)\text d x|+M\sum w_i\Delta x_i\\
\leq M\sum |\int_{x_{i-1}}^{x_i}g(\lambda x)\text d x|+M\sum w_i\Delta x_i\\
\leq M\sum |\frac{1}{\lambda}\int_{\lambda x_{i-1}}^{\lambda x_i}g(x)\text d x|+M\epsilon
$$

接下来设$G(x)=\int_0^x g(t)\text d t$,由于$g$的周期性,而且其在任何一个长度为$T$的区间上积分为$0$,所以$|G(x)|\leq W$,那么$\forall x,y,|G(x)-G(y)|\leq 2W$.

那么上式就$\leq 2MW\frac{n}{\lambda}+M\epsilon$,这里虽然有个无穷大的$n$,但由于$\lambda$可以尽可能大且在$n$之后决定取值,因此这个影响可以被消除.

这个命题的一个平凡推论是$g(x)=\sin (nx)$:

$$
\lim_{\lambda\to \infty}\int_a^b f(x)\sin(\lambda x){\rm d}x=\lim_{\lambda \to \infty}\int_a^b f(x)\cos(\lambda x){\rm d}x=0
$$

###### Example3

求$\lim_{n\to\infty}I_n$,其中$I_n=\frac{1}{n^{1+\alpha}}(\sum_{k=1}^nk^\alpha)=\frac{1}{n}\sum_{k=1}^n(\frac{k}{n})^\alpha$.

注意到$n\to \infty$的时候,上面就等于$\int_0^1 x^\alpha\text d x=\frac{1}{1+\alpha}$.

###### Example4(Dirichlet核)

求$\int_0^{\frac{\pi}{2}}\frac{\sin((2n+1)x)}{\sin x}\text d x$.

注意到根据积化和差有:

$$
\int_0^{\frac{\pi}{2}}\frac{\sin((2n+1)x)}{\sin x}\text d x\\
=\int_0^{\frac{\pi}{2}}(1+2\sum_k \cos(2kx))\text d x\\
=\frac{\pi}{2}+2\sum_{k}\frac{1}{2k}\sin(k\pi)\\
=\frac{\pi}{2}
$$

###### Example5

考虑$f\in \mathbb R$,$\forall a<b$,都有$f\in R[a,b]$,已知$f(x)=\int_0^xf(t)\text d t$,求证$f\equiv 0$.

首先注意到变限积分是李氏连续的,因此$f$当然就是局部李氏连续的,因此自然看出$f$在$\R$上都是连续的.既然如此,那$f$就在$\R$上处处可导并且$f'(x)\equiv f(x)$,同理还可以看出$f$实际上任意阶可导.理应见到$f$应当和$e^x$有关,凑$g(x)=\frac{f(x)}{e^x}$,则$g'(x)=\frac{f'-f}{e^x}\equiv 0$,这立刻得到$g(x)\equiv C$.

然而$g(0)=f(0)=0$,于是$g\equiv C\equiv 0$,所以$f\equiv 0$.

##### 换元法

不妨设$f\in R[a,b],F'=f$,$\varphi(t):[\alpha,\beta]\to [a,b]$,$\varphi(\alpha)=a,\varphi(\beta)=b$,并且$\varphi$可导,而且$f(\varphi(t))\varphi'(t)\in R[\alpha,\beta]$,则:

$$
\int_a^b f(x)\text d x=\int_\alpha^\beta f(\varphi(t))\varphi'(t)\text d t
$$

证明的话,考虑考虑NL公式,应当有:

$$
(F(\varphi(t)))' = f(\varphi(t))\varphi'(t)\\
\int_\alpha^\beta (F(\varphi(t)))' = \int_\alpha^\beta f(\varphi(t))\varphi'(t)\\
F(b)-F(a)=\int_\alpha^\beta f(\varphi(t))\varphi'(t)
$$

然而回忆到有原函数和是否黎曼可积之间是有区别的,适当调整条件可以得到另一个策略的换元法:不妨设$f\in R[a,b],\varphi(t):[\alpha,\beta]\to [a,b]$,并且$\varphi(t)$单增,$\varphi(\alpha)=a,\varphi(\beta)=b$,$\varphi$可导并且$\varphi'\in R[\alpha,\beta]$,则$f(\varphi(t))\varphi'(t)\in R[\alpha,\beta]$且上述积分仍然成立.

策略是对两边做黎曼和,利用$\varphi$的单调性和一致连续性,可以取$S_{\Delta}=\sum f(\eta_k)\Delta x_k$,利用中值定理可以写作$S_{\Delta}=\sum f(\varphi(\xi_k))\varphi'(\mu_k)\Delta t_k$.

而另一边的黎曼和$\sigma_{\Delta'}=\sum f(\varphi(\xi_k))\varphi'(\xi_k)\Delta t_k$.

只需证明二者之差趋近于$0$,令$M=\sup |f|,$做差得到:

$$
|\sum f(\varphi(\xi_k))(\varphi'(\xi_k)-\varphi'(\mu_k))\Delta t_k|\\
\leq M|\sum (\varphi'(\xi_k)-\varphi'(\mu_k))\Delta t_k|\\
\leq M|\sum w_k\Delta t_k|
$$

而$\varphi$可积,上述趋近于$0$.

这些定理都有点诡异,它们会在RS积分处再详细讨论.

###### Example1

求$\int_0^{\frac{\pi}{2}} \frac{\sin x}{\sin x +\cos x}\text d x$.

注意到:

$$
\int_0^{\frac{\pi}{2}} \frac{\sin x}{\sin x +\cos x}\text d x\\
=\int_0^{\frac{\pi}{4}} \left(\frac{\sin x}{\sin x +\cos x}+\frac{\sin (\frac{\pi}{2}-x)}{\sin (\frac{\pi}{2}-x) +\cos (\frac{\pi}{2}-x)}\right)\text d x\\
=\int_0^{\frac{\pi}{4}} \frac{\sin x+\cos x}{\sin x+\cos x}\text d x\\
=\frac{\pi}{4}
$$

其实就是找个点对折.

###### Example2

求$I=\int_0^\pi\frac{\theta\sin \theta}{a+b\cos^2\theta}\text d \theta$,其中$a,b>0$.

令$\theta=\pi-\varphi$,应当有:

$$
I=-\int_\pi^0 \frac{(\pi-\varphi)\sin \varphi}{a+b\cos^2\varphi}\text d \varphi\\
=\int_0^\pi \frac{(\pi-\varphi)\sin \varphi}{a+b\cos^2\varphi}\text d \varphi\\
=\pi\int_{0}^\pi\frac{\sin\varphi}{a+b\cos^2\varphi}\text d \varphi-I
$$

移项得到:

$$
I=\frac{\pi}{2}\int_0^\pi\frac{-\text d \cos \theta}{a+b\cos^2\theta}\\
=\frac{\pi}{\sqrt{ab}}\arctan\sqrt{\frac{b}{a}}
$$


###### Example3

如果$\forall a<b,f\in R[a,b]$,并且$f(x+y)=f(x)+f(y)$恒成立,求证$f(x)\equiv f(1)x$.

固定$x$,对$y$从$[0,1]$积分,得到:

$$
f(x)=\int_x^{x+1} f(t)\text d t-\int_0^1 f(y)\text d y
$$

而变限积分是局部李氏连续的,所以$f(x)$局部连续,所以$f(x)$整体连续,立刻得到$f(x)$可导,立刻得到其是$C^\infty$的.

那就可以两边对$x$求导,得到$\forall y,f'(x+y)=f'(x)$,因此$f'\equiv C$.

##### 分部积分

设$u,v\in R[a,b]$并且均可导,而且$u',v'\in R[a,b]$,则:

$$
(uv)'=u'v+uv'
$$

从这里能看出$(uv)'$也是黎曼可积的,那就可以用牛顿莱布尼茨公式两边积分得到:

$$
\int_a^buv'\text d x= uv\mid_a^b - \int_a^b u'v\text d x\\
=\int_a^bu\text d v= uv\mid_a^b - \int_a^b v\text d u
$$

这个版本当然是简单的,那能不能上点难度呢?

不妨考虑$f,g\in R[a,b]$,那它们的广义原函数可以取为$F(x)=\int_a^x f(t)\text d t+A,G(x)=\int_a^x g(t)\text d t +B$.

则$\int_a^bFg\text d x=FG\mid_a^b-\int_a^b Gf\text d x$.

先证明一个引理:

$$
\lim_{\lambda(\Delta)\to 0} \sum_k^n f(\xi_k)\int_{x_{i-1}}^{x_i}g(t)\text d t=\int_a^bf(x)g(x)\text d x
$$

证明无非是用黎曼和,设$M=\sup|f|$,直接考虑:

$$
|\sum_k^n f(\xi)\int_{x_{i-1}}^{x_i}g{\rm d}t-\sum_k^n\int_{x_{i-1}}^{x_i}fg\text d t|\\
\leq \sum|f(\xi_k)|\int_{x_{i-1}}^{x_i}|g(t)-g(\xi_k)|\text d t+|\sum f(\xi_k)g(\xi_k)\Delta t_k-\int_a^bf g \text d t|\\
\leq M\sum w_k(g)\text d t+\sum w_k(fg)\Delta t_i
$$

这个当然趋向于$0$.

使用abel求和法则,得到:

$$
(FG)\mid_a^b = \sum_k(F(x_k)G(x_k)-F(x_{k-1})G(x_{k-1}))\\
=\sum_k(F(x_k)G(x_k)-F(x_{k-1})G(x_k)+F(x_{k-1})G(x_k)-F(x_{k-1})G(x_{k-1}))\\
=\sum_{k}^n G(x_k)(F(x_k)-F(x_{k-1}))+\sum_{k} F(x_{k-1})(G(x_i)-G(x_{i-1}))\\
=\sum_k G(x_i)\int_{x_{i-1}}^{x_i}f(x)\text d x+\sum_{k}^n F(x_{i-1})\int_{x_{i-1}}^{x_i}g(x)\text d x
$$

使用引理就完事了.

###### Example1

求证:

$$
\int_a^x(\int_a^t f(u)\text d u)\text d t=\int_a^x(x-t)f(t)\text d t
$$

令$F(x)=\int_a^t f(t)\text d t$,$g(x)=1,G(x)=x-a$就可以直接套用分部积分公式得到:

$$
LHS=\int_a^x F(t)g(t)\text d t\\
=FG\mid_a^x-\int_a^x f(t)G(t)\text d t\\
=(x-a)\int_a^x f(t)\text d t-\int_a^x f(t)(t-a)\text d t\\
$$

这样就做完了.

或者设$F(a)=0$,将上式记作:

$$
\int_a^x F(t){\rm d}t=\int_a^x (x-t)F'(t){\rm d}t
$$

###### Example2 

求$I=\int_0^{\frac{\pi}{2}}\frac{x\sin x\cos x}{(a^2\cos^2x+b^2\sin^2x)^2}\text d x$,其中$a,b>0$.

有:

$$
\int_0^{\frac{\pi}{2}}\frac{x\sin x\cos x}{(a^2\cos^2x+b^2\sin^2x)^2}\text d x\\
=\frac{-1}{2(b^2-a^2)}\int_0^{\frac{\pi}{2}}x\text{d}(\frac{1}{a^2+(b^2-a^2)\sin^2 x})\\
=\frac{-1}{2(b^2-a^2)}(\frac{\pi}{2b^2}-\int_0^{\frac{\pi}{2}}\frac{\text d x}{a^2\cos^2 x+b^2\sin^2 x})\\
=\frac{1}{2(b^2-a^2)}(-\frac{\pi}{2b^2}+\int_0^{\frac{\pi}{2}}\frac{\text d \tan x}{a^2+b^2\tan^2 x})\\
=\frac{1}{2(b^2-a^2)}(-\frac{\pi}{2b^2}+\frac{\pi}{2ab})\\
=\frac{\pi}{4ab^2(a+b)}
$$

###### Example3(Wallis公式)

求$I_n=\int_0^{\frac{\pi}{2}} \sin^n x\text d x$.

换元后直接分部积分:

$$
I_n=-\int_0^{\frac{\pi}{2}}\sin^{n-1}x\text d \cos x\\
=\int_0^{\frac{\pi}{2}}\cos x\text d (\sin^{n-1}x)\\
=(n-1)\int_0^{\frac{\pi}{2}}\sin^{n-2}x(1-\sin^2 x)\text d x\\
=(n-1)I_{n-2}-(n-1)I_n\\
$$

于是$I_n=\frac{n-1}{n}I_{n-2}$.

注意到$I_0=\frac{\pi}{2},I_1=1$,所以$I_{2m}=\frac{(2m-1)!!}{(2m)!!}\frac{\pi}{2},I_{2m+1}=\frac{(2m)!!}{(2m+1)!!}$.

然而注意到$I_n$随着$n$增大而减小,于是应该有$I_{2m+1}\leq  I_{2m}\leq I_{2m-1}$,展开得到:

$$
a_n=\frac{1}{2n+1}(\frac{(2n)!!}{(2n-1)!!})^2\\
\leq \frac{\pi}{2}\leq\\
\frac{1}{2n}(\frac{(2n)!!}{(2n-1)!!})^2=b_n
$$

而$\lim a_n=\lim b_n$,因此$\lim a_n=\lim b_n= \frac{\pi}{2}$.

##### 泰勒公式的积分余项

回忆到泰勒公式的余项:

设$f(x)$在$a$处有$n$阶导数,则$f(x)=\sum_{k=0}^n\frac{f^{(k)}(a)}{k!}(x-a)^k+o((x-a)^n)$.

设$f(x)$在$(\alpha,\beta)$上有$n+1$阶导数,则$f(x)=\sum_{k=0}^n\frac{f^{(k)}(a)}{k!}(x-a)^k+\frac{f^{(n+1)}(\xi)}{(n+1)!}(x-a)^{n+1}$.

事实上还可以写积分余项,若$f^{(n+1)}\in R[\alpha,\beta]$,则$f(x)=\sum_{k=0}^n\frac{f^{(k)}(a)}{k!}(x-a)^k+\frac{1}{n!}\int_a^x(x-t)^nf^{(n+1)}(t)\text d t$.

首先能写出$f(x)=f(a)+\int_a^x f'(t)\text d t$,考虑不断展开后面那一项,用分部积分注意到:

$$
\int_a^xf^{(m+1)}(t)\frac{(x-t)^m}{m!}\text d t\\
=\int_a^x f^{(m+1)}(t)\text d (-\frac{(x-t)^{m+1}}{(m+1)!})\\
=f^{(m+1)}(a)\frac{(x-a)^{m+1}}{(m+1)!}+\frac{1}{(m+1)!}\int_a^x (x-t)^{m+1}f^{(m+2)}(t)\text d t
$$

###### Example1

函数$f(x)$在$[a,b]$上各阶导数存在且非负,求证:

$$
f(x)=\sum_{n=0}^\infty \frac{f^{(n)}(a)}{n!}(x-a)^n
$$

其实这个题应该放到幂级数那里,不过确实和积分余项关系更大.分析的话首先会发现要证两边相等一定要搞定余项对吧,但是拉格朗日余项要求对所有的点的任意阶导数都存在一个控制,但这是做不到的,原函数对导函数的控制只能是在一段连续区间上的控制(用NL公式),而做不到对所有单点的控制.因此使用积分余项,一个自然的想法是除了$b$端点以外的误差应该都比$b$处的误差要小,注意到:

$$
\frac{x-t}{x-a}\leq \frac{b-t}{b-a}
$$

所以:

$$
R_n(x)=\frac{1}{n!}\int_a^x f^{(n+1)}(t)(x-t)^n{\rm d}t\\
\leq \frac{(x-a)^n}{(b-a)^n}\frac{1}{n!}\int_a^x f^{(n+1)}(t)(b-t)^n{\rm d}t\\
\leq \frac{(x-a)^n}{(b-a)^n} R_n(b)
$$

接下来如果能控制$R_n(b)$就万事大吉,发现:

$$
R_n(b)=\frac{1}{n!}\int_a^b f^{(n+1)}(t)(b-t)^n{\rm d}t\\
=\frac{1}{n!}\int_a^b (b-t)^n{\rm d}f^{(n)}(t)\\
=-\frac{(b-a)^n}{n!}f^{(n)}(a)+\frac{1}{(n-1)!}\int_a^b (b-t)^{n-1}f^{(n)}(t)\\
\leq R_{n-1}(b)\\
\leq \cdots\leq f(b)-f(a)
$$

这就搞定了,当$x\in [a,b)$的时候,由于$\frac{(x-a)^n}{(b-a)^n}\to 0$而自然完事.而单调函数的端点是随便延拓的,写下来的话,如果端点处不满足则必然端点处不收敛,意味着:

$$
\lim_{x\to b-0}\sum_{n=0}^\infty \frac{f^{(n)}(a)}{n!}(x-a)^n=+\infty
$$

但这不可能,因为左边的恰好是原本收敛的那个函数$x\to b-0$处的表现.

##### 积分第一中值定理

设$f,g\in R[a,b]$并且$g$不变号.不妨设$f$的连续点集为$A$,称它的**本性上界**为$M=\inf_{x\in A}(f(x))$,同理定义**本性下界**$m$.则$\exists \mu\in [m,M]$使得:

$$
\int_a^b fg(x)\text d x=\mu\int_a^b g(x)\text d x
$$

来证明这个事,不妨设$g(x)\geq 0$.

当$\int_a^b g(x)\text d x=0$的时候,此时由于$g(x)\geq 0$,因此$g(x)$一定几乎处处为零(否则,存在一个$g$的连续点处不为$0$,在这个点邻域附近卡一下就能卡掉),那左右两边都等于$0$.

当$\int_a^b g(x)\text d x>0$的时候,由于$mg\leq gf\leq Mg$对于$f$的连续点集$A$成立,而这是个稠密集,立刻见到$\int_a^b mg\text d x\leq \int_a^b gf\text d x\leq \int_a^b Mg\text d x$.于是$\mu=\frac{\int_a^b f g \text d x}{\int_a^b g \text d x}\in [m,M]$.

作为更加常见的版本,当$f\in C[a,b]$的时候,$\exists \xi\in (a,b)$使得:

$$
\int_a^b fg(x)\text d x=f(\xi)\int_a^b g(x)\text d x
$$

该版本与之前的区别在于要断言$\xi\ne a,\xi \ne b$.使用反证法,在$\int_a^b g(x)\text d x> 0$的时候,不妨假设$\xi =b$,此时前面$(a,b)$中不能有和$f(b)$相同的,又因为其是连续函数,因此不妨设$(a,b)$中$f(x)$均比$f(b)$要小,用极限把端点用保号性卡出来就知道$[a,b]$中$f(x)\leq f(b)$.

由于此时$\int_a^b g(x)\text d x> 0$,于是一定存在一个$x_0,g(x_0)>0$,并且$x_0$是一个连续点.此时就存在其一个小邻域都$>\frac{1}{2}g(x_0)>0$,并且这个邻域上的$f(x)<f(b)$.在这个邻域上立刻可以发现:

$$
\int_a^b (M-f(x))g(x)\text d x>0
$$

这就有了矛盾.

###### Example1

$f\in R[0,1]$,$f(+0)$存在,则:

$$
\lim_{n\to +\infty}\int_0^1\frac{n f(x)}{1+n^2x^2}\text d x=\frac{\pi}{2}f(+0)
$$

不妨设$f(0)=f(+0)$,因为改变有限个点的取值是不会改变其积分的.

此时:

$$
\int_0^1\frac{n f(x)}{1+n^2x^2}\text d x\\
=\int_0^{\frac{1}{\sqrt n}}\frac{n f(x)}{1+n^2x^2}\text d x+\int_{\frac{1}{\sqrt n}}^{1}\frac{n f(x)}{1+n^2x^2}\text d x
$$

而考虑:

$$
\int_0^{\frac{1}{\sqrt n}}\frac{n f(x)}{1+n^2x^2}\text d x\\
=\mu\int_0^{\frac{1}{\sqrt n}}\frac{n}{1+n^2x^2}\text d x\\
=\mu \arctan\sqrt n\to f(0)\frac{\pi}{2}
$$

而后者:

$$
|\int_{\frac{1}{\sqrt n}}^{1}\frac{n f(x)}{1+n^2x^2}\text d x|\\
\leq \sup|f|(\arctan(nx)\mid_{\frac{1}{\sqrt n}}^1)\\
=\sup|f|\arctan(\frac{n-\sqrt n}{1+n\sqrt n})
\to 0
$$

这样就做完了.这里拆成两部分可能是难以想到的,事实上这么做的意义是使得$\mu \to f(0)$而放缩,其中这个$nx$的变化量很大,因此可以稍微匀一下变化.

###### Example2

设$f$二阶可导,并且一,二阶导可积,则$\exists \xi\in (a,b)$,使得$\int_a^bf(x)\text d x=(b-a)f(\frac{a+b}{2})+\frac{(b-a)^3}{24}f''(\xi)$.

考虑:

$$
\int_a^b f(x)\text d x\\
=\int_{a}^{\frac{a+b}{2}}f(x)\text d x+\int_{\frac{a+b}{2}}^{b}f(x)\text d x\\
=\int_0^{\frac{b-a}{2}}f(a+t)\text d t+\int_0^{\frac{b-a}{2}}f(b-t)\text d t\\
=\int_0^{\frac{b-a}{2}}f(a+t)+f(b-t)\text d t\\
=t(f(a+t)+f(b-t))\mid_0^{\frac{b-a}{2}}-\int_0^{\frac{b-a}{2}}t(f'(a+t)-f'(b-t))\text d t\\
=(b-a)f(\frac{a+b}{2})-\int_0^{\frac{b-a}{2}}t(f'(a+t)-f'(b-t))\text d t
$$

前半部分已经完事了,接下来看后半部分,继续分部积分:

$$
\int_0^{\frac{b-a}{2}}t(f'(a+t)-f'(b-t))\text d t\\
=\frac{1}{2}\int_0^{\frac{b-a}{2}}(f'(a+t)-f'(b-t))\text d(t^2)\\
=\left(\frac{t^2}{2}(f'(a+t)-f'(b-t))\right)\mid_{0}^{\frac{b-a}{2}}-\int_0^{\frac{b-a}{2}}\frac{t^2}{2}(f''(a+t)+f''(b-t))\text d t\\
=-\int_0^{\frac{b-a}{2}}\frac{t^2}{2}(f''(a+t)+f''(b-t))\text d t\\
=\mu\int_0^{\frac{b-a}{2}}\frac{t^2}{2}\text d t\\
=\mu \frac{(b-a)^3}{48}
$$

接下来看$\mu$,$\mu\in [\inf (f''(a+t)+f''(b-t)),\sup(f''(a+t)+f''(b-t)) ]$,由导函数介值性,意味着必然$\exists \xi_1\in [0,\frac{b-a}{2}]$使得$\mu= f''(a+\xi_1)+f''(b-\xi_1)$.再用一次$2f''$的介值性得到$\mu=2f''(\xi),\xi\in (a+\xi_1,b-\xi_1)\subseteq (a,b)$.

##### 积分第二中值定理

设$g\in R[a,b],f\geq 0$,并且$f$单调,则:

1. $f$单调递减时,$\exists \xi\in [a,b]$,$\int_a^b f g \text d x=f(a+0)\int_a^\xi g(x)\text d x$.
2. $f$单调递增时,$\exists \xi\in [a,b]$,$\int_a^b f g \text d x=f(b-0)\int_\xi^b g(x)\text d x$.
3. $\exists \xi\in [a,b]$,$\int_a^b f g \text d x=f(a+0)\int_a^\xi g\text d x+f(b-0)\int_\xi^b g\text d x$.
4. 在上述基础上,如果$f$不是几乎常值函数,那么上述的$\xi\in (a,b)$.

(1)(2)相似,下面只对(1)进行证明:

不妨设$f(a)=f(a+0),G=\int_a^x g(x)\text d x$是$g$的广义原函数,其满足$G(a)=0$,不妨设$M=\sup G,m=\inf G$,回忆到:

$$
\int_a^b f g\text d x=\lim_{\lambda(\Delta)\to 0} \sum_{i=1}^n f(x_{i-1})\int_{x_{i-1}}^{x_i}g\text d x\\
=\sum_{i=1}^nf(x_{i-1})(G(x_i)-G(x_{i-1}))\\
=f(x_{n-1})G(x_n)+\sum_{i=1}^{n-1}(f(x_{i-1})-f(x_{i}))G(x_i)\\
\leq f(a)M
$$

在最后一步放缩进行修改就可以改为$\geq f(a)m$,而我们已经得知$G$的连续性,因此用介值性就可以解决上述问题.

难点在于(4)如何证明.

当其不是几乎常值函数的时候,应该满足$f(a+0)>f(b-0)$.

当$\int_a^b f g\text d x=0$的时候,如果迫不得已必须取端点就应当有$\forall \xi\in (a,b)$,都有$\int_a^\xi g\text d x\ne 0$,由于其连续导出的介值性那就当然拿到了$G$的保号性.我们上面已经用Abel变换得到了:

$$
\int_a^b f g\text d x
=\sum_{i=1}^nf(x_{i-1})(G(x_i)-G(x_{i-1}))\\
=f(x_{n-1})G(x_n)+\sum_{i=1}^{n-1}(f(x_{i-1})-f(x_{i}))G(x_i)\\
$$

此时考虑取$c,d$使得$a<c<d<b$,并且满足$f(c)>f(d)$,由于$f$并不是几乎常值函数,这当然可以做到,上式每一项都是$\geq 0$的,那就可以把区间从$[a,b]$重新控制到$[c,d]$中.于是此时上式立刻:

$$
\int_a^b f g\text d x\\
\geq (f(c)-f(d))\min_{[c,d]}G(x)>0
$$

然而我们假设中$\int_a^b f g\text d x=0$,这样就导出了矛盾.

接下来考虑$\int_a^b f g\text d x\ne 0$的情况,如果其迫不得已必须取在端点的话当然要取到右端点此时应该有$\int_a^b f g\text d x=f(a)\int_a^b g\text d x$.

仍然看上面的Abel变换后的结果,同时按照上面同样的办法取出一个子区间$[c,d]$.回忆到为了保证前面都没有能和$G(b)$相同的,因此$G(b)$必然是前面的最大值.再回忆到$G(a)=0$,立刻能得到$G(b)>\sup_{[a,c]}G(x)$,设后者为$M$,应当有:

$$
\int_a^b f g\text d x\\
=f(x_{n-1})G(b)+\sum_{i=1}^{n-1}(f(x_{i-1})-f(x_{i}))G(x_i)\\
\leq f(x_{n-1})G(b)+M(f(a)-f(c))+G(b)(f(c)-f(b))\\
$$

这里这个$f(x_{n-1})$很烦,但是当$\lambda(\Delta)\to 0$的时候当然会使得$f(x_{n-1})\to f(b-0)$,上式变更为:

$$
\int_a^b f g\text d x\\
\leq f(b)G(b)+M(f(a)-f(c))+G(b)(f(c)-f(b))\\
=M(f(a)-f(c))+G(b)f(c)\\
<f(a)G(b)
$$

这就导出了矛盾.

最后来看(3),不妨设$f$单减,设$\varphi(x)=f(x)-f(b)\geq 0$且单减,用(1)的结论得到$\exists \xi\in [a,b]$使得$\int_a^b \varphi g \text d x=\varphi(a)\int_a^\xi g \text d x$.变形后就可以得到原本的式子.单增是同理的.总之,(3)是(1)(2)的一个更为优美的推论.

###### Example1

设$f\in C^1[a,b]$,并且满足$f'$单调递减,且$f'(b)\geq M>0$.求证:$|\int_a^b \cos f(x)\text d x|\leq \frac{2}{M}$.

注意到:

$$
\int_a^b \cos f(x)\text d x\\
=\int_a^b \frac{f'\cos f}{f'}\text d x\\
=\frac{1}{f'(b)}\int_\xi^bf'\cos f\text d x\\
=\frac{1}{f'(b)}(\sin f(b)-\sin f(\xi))\\
$$

这就完事了.

##### 有界变差函数

对于一个$f:[a,b]\to R$的函数,做分划$\Delta :a=x_0<\cdots<x_n=b$,定义其变差为$V_\Delta=\sum_k |f(x_i)-f(x_{i-1})|$,定义其全变差为$V_{[a,b]}(f)=\sup_\Delta V_\Delta$.当全变差为有限的时候,称$f$是有界变差函数.容易见到由介值定理,如果$f$可导而且$f'$可积,则$V_{[a,b]}(f)=\int_a^b |f'|{\rm d }x$.

应当容易见到,闭区间上的单调函数是有界变差的,且$V_{|f|}\leq V_f,V_{f+g}\leq V_f+V_g$.

接下来我们来证明:$f$是有界变差函数,当且仅当$f$是两个单调(不一定严格)函数的差.或者更严格一点,可以写成两个单调不减函数的差.另外,作为这个命题的平凡推论,我们知道有界变差函数几乎处处可导,并且有至多可数个间断点,且只有第一类间断点.

充分性显然,下面来看必要性.假设$f$是有界变差的,此时类似积分可以定义一个变上限函数$V_f(x)=V_f[a,x]$,其显然是单调不降的.

接下来定义$g=V_f(x)-f(x)$,我们来说明它一定也是单调不减的.

那考虑当$x_2>x_1$时,观察$g(x_2)-g(x_1)=V_f(x_2)-V_f(x_1)-(f(x_2)-f(x_1))$.

既然全变差的定义依赖于分划,我们当然可以加分点,自然有$V_f(x_2)-V_f(x_1)=V_{f,[x_1,x_2]}\geq |f(x_2)-f(x_1)|\geq f(x_2)-f(x_1)$.

自然得证.

可应当意识到连续函数不一定是有界变差函数,原因仍然考虑$f(x)=\begin{cases}0&x=0\\x\sin\frac{1}{x}&otherwise\end{cases}$.在闭区间$[0,1]$上取$x_k=\frac{1}{(k+\frac{1}{2})\pi}$,这样$\sin \frac{1}{x_i}$的取值正负交替,容易见到出现了调和级数.

然而,稍微加点条件,我们可以证明李氏连续是有界变差的.可以直接套用定义,也可以回忆到$f(x)=f(x)-Lx+Lx=Lx-(Lx-f(x))$.因此导函数有界就一定有界变差.并且此时,可以让其拆出两个不降的可导函数.

##### 定积分的应用

###### Example1(图形证明不等式)

回忆到我们证明积分形式的柯西不等式的时候,证明了当$\frac{1}{p}+\frac{1}{q}=1$时,$a^{\frac{1}{p}}b^{\frac{1}{q}}=e^{\frac{\ln a}{p}+\frac{\ln b}{q}}\leq \frac{a}{p}+\frac{b}{q}$.

下面可以用类似的策略理解这个式子,首先等价于证明$ab\leq \frac{a^p}{p}+\frac{b^q}{q}$.原因是考虑设$S_1=\frac{a^p}{p},S_2=\frac{b^q}{q}$,设$f(x)=x^{p-1}$,留意到$f(x)$的积分表示出了$S_1$,而$f^{-1}(x)$恰好表示出了$S_2$.

###### Example2(参数方程曲线面积)

回忆到换元法,取参数方程$\begin{cases}x=x(t)\\y=y(t)\end{cases}$,此时不妨假设$x(\alpha)=a,x(\beta)=b$,且$x(t)$单调递增且连续,$x'(t)$除有限点外都存在且黎曼可积.那当然应当有:

$$
\int_a^b f(x)\text d x=\int_\alpha^\beta y(t)x'(t)\text d t
$$

注意到如果反之条件使得$x(\alpha)=b,x(\beta)=a$,则需要补一个$-1$.

由此可见,对于不自交的任意参数方程表示的曲线,直接积分的正负号一抵消,仍然可以求出曲线下方的面积.如果自交,就是要把自交部分的面积减去.顺便而言,简单封闭曲线(例如圆或者椭圆)当然是上述我们所说的特殊情况.

不过对于封闭曲线,由于转一圈会回到同一个点处,使用分部积分得到:

$$
\int_\alpha^\beta y x' \text d t\\=\int_\alpha^\beta y\text d x\\=xy|_a^b-\int_\alpha^\beta xy'\text d t
\\=-\int_\alpha^\beta xy'\text d t
$$

兼顾一下对称性,可以写作$S=\frac{1}{2}|\int_\alpha^\beta(x'y-y'x)\text d t|$.由此立刻得到椭圆面积为$ab\pi$.

###### Example3(极坐标的面积)

设$r=r(\theta),\theta\in[\alpha,\beta],r(\theta)\in C(\alpha,\beta)$.

此时考虑对$\theta$做划分,取$\Delta:\alpha=\theta_0<\cdots<\theta_n=\beta$,并取$m_i=\inf_{[\theta_{i-1},\theta_i]}r,M_i=\sup_{[\theta_{i-1},\theta_i]}$,立刻得到$\frac{1}{2}m_i^2\Delta \theta_i\leq \Delta S_i\leq \frac{1}{2}\sum M_i^2\Delta \theta_i$,这样加密后得到$S=\frac{1}{2}\int_\alpha^\beta r^2(\theta)\text d \theta$.

###### Example4(参数方程曲线的弧长)

取参数方程$\begin{cases}x=x(t)\\y=y(t)\end{cases}$,并且假设$x,y$均连续.首先我们应当定义其"弧长"的含义.

可以使用折线逼近的策略定义弧长.具体而言,直接取分划$\Delta :\alpha=t_0<\cdots<t_n=\beta$.定义一个点$P_i=(x(t_i),y(t_i))$,这样我们当然就得到了若干个点,在这些点上就可以将相邻两个点求出折线,设折线长为$\overline{M_{i-1}M_i}$.

接下来,称曲线是可求长的,当且仅当取出的折线的上确界是有限的,并将此上确界定义为曲线长.

我们接下来证明,其可求长当且仅当$x(t)$和$y(t)$都是有界变差的.

先看必要性,任取分划$\Delta : \alpha=t_0<\cdots<t_n=\beta$.立刻得到$V_\Delta(x)=\sum |x_i-x_{i-1}|\leq \sum \overline{M_{i-1}M_i}$,当然是有界的,对于$y(t)$同理.

另外,斜边小于两条直角边的和(三角形不等式),于是充分性也显然.

然而真要实际应用需要稍加条件,即:$x(t),y(t)$可导且导函数黎曼可积(说明其导函数有界,为李氏连续函数,那自然是有界变差函数).则弧长为$\int_\alpha^\beta \sqrt{(x'(t))^2+(y'(t))^2}\text d t$.

证明比较容易,考虑勾股定理,$\overline{M_{i-1}M_i}=\sqrt{(x(t_i)-x(t_{i-1}))^2+(y(t_i)-y(t_{i-1}))^2}$.使用微分中值定理,得到其等于$\Delta t_i\sqrt{(x'(\xi_i))^2+(y'(\eta_i))^2}$.这里强行换成一个东西,于是就会得到$\Delta t_i\sqrt{(x'(\xi_i))^2+(y'(\xi_i))^2}+w_i$的形式,其中$w_i$恰为两项之差.然后换黎曼和,只需证明后者$w_i$随着加密而趋向于$0$.而这个根号差可以做分子有理化.具体而言:

$$
w_i=\Delta t_i\left(\sqrt{(x'(\xi_i))^2+(y'(\eta_i))^2}-\sqrt{(x'(\xi_i))^2+(y'(\xi_i))^2}\right)\\
=\Delta t_i\frac{y'(\eta_i)^2-y'(\xi_i)^2}{\sqrt{(x'(\xi_i))^2+(y'(\eta_i))^2}+\sqrt{(x'(\xi_i))^2+(y'(\xi_i))^2}}\\
=\Delta t_i\frac{(y'(\eta_i)+y'(\xi_i))}{\sqrt{(x'(\xi_i))^2+(y'(\eta_i))^2}+\sqrt{(x'(\xi_i))^2+(y'(\xi_i))^2}}(y'(\eta_i)-y'(\xi_i))\\
\leq \Delta t_i(y'(\eta_i)-y'(\xi_i))
$$

最后会得到$\leq \sum\omega_{y'}\Delta t_i$,立刻得到其趋向于$0$.顺便还要证明这个是上确界,只需要证明加点后答案增加即可,然后用黎曼积分那一部分的对划分取并的技巧即可.

由此应当能推出直角坐标系下弧长公式,即$l=\int_a^b\sqrt{1+(f'(x))^2}\text d x$.

###### Example5(极坐标下的弧长公式)

此时应当有$x'=r'\cos \theta-r\sin \theta,y'=r'\sin \theta+r\cos \theta$,注意到$(x')^2+(y')^2=r^2+(r')^2$,于是极坐标下的公式当然是$l=\int_\alpha^\beta\sqrt{r^2+(r')^2}\text d \theta$.

###### Example6(等周不等式)

求给定长度的闭合曲线能围出的最大面积.

设$\Gamma$是一个周长为$l$的简单可求长闭曲线,其面积为$A$,我们下面证明$A\leq \frac{l^2}{4\pi}$.

先考虑$\Gamma$比较光滑的情况,也就是其有导函数且导函数可积.参数方程给出$l=\int_\alpha^\beta\sqrt{(x')^2+(y')^2}\text d \theta$而$A=\int_\alpha^\beta xy'\text d \theta$.注意这里假设曲线是逆时针方向旋转的.

接下来,考虑在原基础上画个圆,这个圆的参数方程横坐标继承自上述$x(\theta)$而纵坐标由圆定义为$\bar y=\pm\sqrt {r^2-x^2}$,这里的$2r=\sup x-\inf x$.也就是横着的最长距离.为了使得这个圆转起来是合理的,实际上应该找到上述曲线的左右端点然后分段函数定义,我们这里略去此细节.

然后呢,此时观察到应当有$\pi r^2=-\int_\alpha^\beta\bar y x'\text d \theta$,发挥注意力得到:

$$
2\sqrt{A\pi r^2}\leq A+\pi r^2\\
=\int_\alpha^\beta (xy'+\bar y x') \text d \theta\\
\leq \int_\alpha^\beta \sqrt{x^2+(\bar y)^2}\sqrt{y'^2+x'^2}\text d t\\
=rl
$$

于是证毕.

那么不光滑怎么办呢?可以用分段光滑函数(比如折线)逼近,这样就行了.

###### Example7(求多面体体积)

体积的定义无非类似面积,用夹逼定理定义任意形状的体积.

考虑对于每个$x$,在$x$那里做截面,设得到的截面面积是$S(x)$,那么体积感觉上应当是$V=\int_a^b S(x)\text d x$.

然而这个结论的准确证明可能需要一些重积分知识.我们尝试越级打怪,加一个很神秘的条件:就是两个截面之间一定存在包含关系以及$S(x)$是关于$x$连续变化的.然后看看能不能把这个结论证出来.

考虑$x$的范围是$[a,b]$,接下来对其作分划$\Delta : a=x_0<\cdots<x_n=b$.接下来取$\eta_i,\xi_i$满足$S(\xi_i)=\max_{[x_{i-1},x_i]}S(x),S(\eta_i)=\min_{[x_{i-1},x_i]}S(x)$.此时回忆到包含关系,那么我们拿出来的$\xi_i$和$\eta_i$当然就是包含别人以及被包含的两个截面,以它们为底面积做柱体,留意到此时当然有:

$$
S(\eta_i)(x_{i}-x_{i-1})\leq V_{[x_{i-1},x_i]}\leq S(\xi_i)(x_i-x_{i-1})
$$

回忆到假设$S(x)$是连续变化的因此黎曼可积,于是这个东西两边其实就是达布上下和,立刻得到积分的结论.

旋转体当然满足以上条件,易见旋转体的体积为$\pi\int_a^b f^2(x){\rm d} x$,参数方程同理.

###### Example8(旋转体的侧面积公式)

曲面的面积难以定义,所以我们现在暂时成为物理系学生.下面只考虑旋转体.

对于旋转体来说,先用折线逼近原曲线,然后旋转后得到若干圆台,用圆台的侧面积之和去逼近旋转体的侧面积.

对于$[a,b]$,我们先做一个分划$\Delta: a=x_0<\cdots<x_n=b$,$y_i=f(x_i)$.圆台侧面积给出$\Delta S_i=\pi(y_{i-1}+y_i)|\overline{M_{i-1}M_i}|=\pi(y_{i-1}+y_i)\sqrt{1+(\cfrac{y_i-y_{i-1}}{x_i-x_{i-1}})^2}\Delta x_i$,利用微分中值定理,就有:

$$
S_i=\pi(y_{i-1}+y_i)\sqrt{1+(f'(\xi_i))^2}\Delta x_i\\
=2\pi f(\xi_i)\sqrt{1+(f'(\xi_i))^2}\Delta x_i+(y_i+y_{i-1}-2\pi f(\xi_i))\sqrt{1+(f'(\xi_i))^2}\Delta x_i
$$

加密后第二项可以用振幅控制住,所以答案就是$2\pi \int_a^bf(x)\sqrt{1+(f'(x))^2}{\rm d} x$.

对于参数方程当然也是平凡的,答案应当是$2\pi \int_a^b y\sqrt{1+(\frac{y'}{x'})^2}x'{\rm d} t=2\pi\int_a^by\sqrt{(x')^2+(y')^2}{\rm d} t$,注意这里是按照$x$轴翻转.应当看到由此一段球面上的环的面积就是$2\pi r\Delta x$.

###### Example9($\pi$的无理性证明)

假设$\pi=\frac{p}{q}\in \mathbb{Q}$并且$p\bot q,p,q\in \mathbb{N_+}$.

考虑设$f_n(x)=q^nx^n(p-qx)^n$,留意到$\forall 0\leq i\leq n-1$,有$f_n^{(i)}(0)=f_n^{(i)}(\pi)=0$.

接下来考察$\forall n\leq i\leq 2n$,此时留意到$f_n^{(i)}(0)$和$f_n^{(i)}(\pi)$都应当是整数并且是$n!$的倍数(是$0$也是$n!$的倍数).

考虑积分$I_n=\frac{1}{n!}\int_0^\pi f_n(x)\sin x{\rm d}x>0$.对其狂暴使用分部积分:

$$
I_n=\frac{1}{n!}\int_0^\pi f_n(x)\sin x{\rm d}x\\
=\frac{1}{n!}\int_0^\pi f_n(x){\rm d}(-\cos x)\\
$$

不断做分部积分,前面扔出来的项也就是$f_n^{(i)}(0)$或者$f_n^{(i)}(\pi)$然后再乘上$\sin,\cos$在$0,\pi$之类取值的问题.总之,这个东西算出来一定是$n!$的倍数,前面乘了个$\frac{1}{n!}$,他就一定是整数.又因为其大于$0$,他就一定$\geq 1$.

那么矛盾在哪呢?留意到$f_n=(qx(p-qx))^n\leq (\frac{p}{2})^{2n},\sin x\leq 1$,所以$I_n\leq \frac{1}{n!}(\frac{p}{2})^{2n}\pi\to 0$.

###### Example10(古鲁金第一定理)

躲开微元法,要证明一些东西可以扔进去的话(比如密度函数),应当可以通过"平均密度"的概念用之前那个引理(分部积分那里,步骤是先积分,乘系数,后求和),总之都是平凡的.

考虑一个旋转体,反正用上面那套东西,搞出来质心应当有$\bar y=\frac{\int_\alpha^\beta y\sqrt{x'^2+y'^2}{\rm d}t}{l}$.移项得到$2\pi\bar y l=2\pi\int_\alpha^\beta y\sqrt{x'^2+y'^2}{\rm d}t$.也就是:旋转线转一圈,质心周长乘以曲线的弧长等于侧面积.

###### Example11(古鲁金第二定理)

考虑两条曲线夹起来的部分,也就是设$[a,b]$上有$f(x)\geq g(x)\geq 0$,那这两条线就会围出一块二维出来.这一块的质心能不能求呢?当然也可以,只不过略有区别,$\bar x=\frac{\int_a^bx(f(x)-g(x){\rm d} x)}{S}$,而$\bar y=\frac{1}{2}\frac{\int_a^b(f^2(x)-g^2(x){\rm d} x)}{S}$.移项得到$2S\bar y=\int_a^b(f^2(x)-g^2(x){\rm d} x)$,也就是旋转面转一圈,质心周长乘以面积等于体积.

###### Example12(转动惯量)

定义为$J=\int mr^2$.

考虑以$x$轴为旋转轴,先考虑线的情况,无非是分成小段每一段分别处理,对于每一段用长度和密度的乘积来逼质量,关于$x$轴的转动惯量$J=\int_\alpha^\beta y^2(t)\rho(t)\sqrt{(x'(t))^2+(y'(t))^2}\text d t$.

###### Example13

设$F$是所有满足$f\in C^1[0,1]\mid f(0)=0,f(1)=1,f'\geq 0$,$f'$单调不降的函数$f$组成的集合,记$S_f$为$f(x)$绕$x$轴转一圈得到的侧面积,求$\sup_{f\in F}S_f$和$\inf_{f\in F}S_f$.

先猜下确界是$\pi$,上确界是$\sqrt 2\pi$.

先证明下界是对的,考虑:

$$
S_f=2\pi\int_0^1f\sqrt{1+(f')^2}{\rm d}x
\geq 2\pi\int_0^1f\sdot f'{\rm d}x\\
=\pi
$$

怎么逼近呢?这个当然是直接平的是最优秀的对吧,考虑$f_n(x)=x^n$,发现:

$$
\pi\leq S_{f_n}\leq 2\pi\int_0^1 f_n(1+f_n'){\rm d}x=\pi+2\pi\int_0^1f_n{\rm d}x=\pi+\frac{2\pi}{n+1}\to \pi
$$

上界怎么搞定呢?考虑肯定是$f(x)=x$是最好的,我们下面证明任意一段曲线,如果把曲线拉直就更优秀.

也就是考虑$(0,0)$和$(x_0,f(x_0))$的连线$y=\frac{f(x_0)}{x_0}x$,容易见到:

$$
\int_0^{x_0}y\sqrt{1+(y')^2}{\rm d}x\\
=\int_0^{x_0}\frac{f(x_0)}{x_0}x\sqrt{1+(\frac{f(x_0)}{x_0})^2}{\rm d}x\\
=\frac{1}{2}f(x_0)\sqrt{x_0^2+f(x_0)^2}
$$

只要我们证明:

$$
\int_0^x f(t)\sqrt{1+(f'(t))^2}{\rm d}t\leq \frac{1}{2}f(x)\sqrt{x^2+f^2(x)}
$$

即可.注意到它们在零点的取值相同,因此直接算导数之差,要证:

$$
\frac{1}{2}f'\sqrt{x^2+f^2}+\frac{x+f\sdot f'}{2\sqrt{x^2+f^2}}f-f\sqrt{1+(f')^2}\geq 0\\
2f\sqrt{1+(f')^2}\sqrt{x^2+f^2}\leq (x^2+2f^2)f'+xf
$$

敢算就敢赢,两边平方得到:

$$
3f^2x^2+4f^4\leq x^4(f')^2+ 2x^3 f\sdot f'+2xf^3f'
$$

注意到下凸函数满足$f'\geq \frac{f}{x}$,上述结论刚刚好得证,出这个题的lwg老师真是高人了.

##### 定积分近似计算

考虑对$\int_a^b f(x){\rm d}x$做数值近似,将$[a,b]$等分,取$x_k=a+\frac{k}{n}(b-a)$,其中$0\leq k\leq n$,而$\Delta x=\frac{b-a}{n}$,$x_{k-\frac{1}{2}}=\frac{x_k+x_{k-1}}{2}$.

那么用黎曼和逼积分,那么无非三种方式:要么算左端点的函数值,要么算右端点的函数值,要么算中间点的函数值.

问题在于计算误差,我们下面证明,如果$f$一阶可导,则:

1. $\int_a^b f(x){\rm d}x=\frac{b-a}{n}\sum_{k=1}^n y_k-\frac{(b-a)^2}{2n}f'(\xi)$
2. $\int_a^b f(x){\rm d}x=\frac{b-a}{n}\sum_{k=1}^n y_{k-1}+\frac{(b-a)^2}{2n}f'(\eta)$

两个是类似的,只证第一个,考虑:

$$
\int_a^b f(x){\rm d}x-\frac{b-a}{n}\sum_{k=1}^n y_k\\
=\sum_{k=1}^n\int_{x_{k-1}}^{x_k}(f(x)-f(x_k)){\rm d}x\\
=\sum_{k=1}^n\int_{x_{k-1}}^{x_k}(f(x)-f(x_k)){\rm d}(x-x_{k-1})\\
=-\sum_{k=1}^n\int_{x_{k-1}}^{x_k}(x-x_{k-1})f'(x){\rm d}x\\
=-\sum_{k=1}^nf'(\xi_k)\int_{x_{k-1}}^{x_k}(x-x_{k-1}){\rm d}x\\
=-\sum_{k=1}^nf'(\xi_k)\frac{(b-a)^2}{2n^2}\\
=-\frac{(b-a)^2}{2n}f'(\xi)
$$

最后一步是因为求均值也可以用介值定理.

那么在中点估计会怎么样呢?考虑给$f$更好的条件,让$f$二阶可导,则:

$$
\int_a^b f(x){\rm d}x=\frac{b-a}{n}\sum_{k=1}^n y_{k-\frac{1}{2}}-\frac{(b-a)^3}{24n^2}f''(\xi)
$$

在$x_{k-\frac{1}{2}}$处做泰勒展开,应当有:

$$
f(x)-f(x_{k-\frac{1}{2}})=(x-x_{k-\frac{1}{2}})f'(x_{k-\frac{1}{2}})+\frac{1}{2}(x-x_{k-\frac{1}{2}})^2f''(\xi_k)\\
$$

不妨设$m_k\leq f''(\xi_k)\leq M_k$,则:

$$
\int_{x_{k-1}}^{x_k}(f(x)-f(x_{k-\frac{1}{2}})){\rm d}x\leq \frac{M_k}{2}\int_{x_{k-1}}^{x_k}(x-x_{k-\frac{1}{2}})^2{\rm d}x\\
=M_k\frac{(b-a)^3}{24n^3}
$$

用介值定理就可以求和,原命题自然得证.

而上面的做法是矩形逼近,能不能用梯形逼近呢?用梯形逼近的话其实无非是$\frac{b-a}{n}(\frac{y_0+y_n}{2}\sum_{k=1}^{n-1} y_k)$,然而,对于$f\in C^2[a,b]$,我们声明:

$$
\int_a^b f(x){\rm d}x=\frac{b-a}{n}(\frac{y_0+y_n}{2}\sum_{k=1}^{n-1} y_k)-\frac{(b-a)^3}{12n^2}f''(\xi)
$$

证明当然还是分部积分,考虑:

$$
\int_{x_{k-1}}^{x_k}f(x){\rm d}x\\
=\int_{x_{k-1}}^{x_k}f(x){\rm d}(x-x_{k-1})\\
=f(x_k)\frac{b-a}{n}-\int_{x_{k-1}}^{x_k}(x-x_{k-1})f'(x){\rm d}x\\
=f(x_k)\frac{b-a}{n}-\frac{1}{2}\int_{x_{k-1}}^{x_k}f'(x){\rm d}(x-x_{k-1})^2\\
=f(x_k)\frac{b-a}{n}-\frac{1}{2}f'(x_k)(\frac{b-a}{n})^2+\frac{1}{2}\int_{x_{k-1}}^{x_k}f''(x)(x-x_{k-1})^2{\rm d}x\\
$$

同理,在上面换$x-x_k$而非$x-x_{k-1}$,就可以得到:

$$
\int_{x_{k-1}}^{x_k}f(x){\rm d}x\\
=f(x_{k-1})\frac{b-a}{n}+\frac{1}{2}f'(x_{k-1})(\frac{b-a}{n})^2+\frac{1}{2}\int_{x_{k-1}}^{x_k}f''(x)(x-x_{k})^2{\rm d}x\\
$$

此时用NL公式,应当能见到把上面两式子相加可以得到:

$$
\int_{x_{k-1}}^{x_k}f(x){\rm d}x\\
=\frac{f(x_{k-1})+f(x_k)}{2}\frac{b-a}{n}+\frac{1}{4}\int_{x_{k-1}}^{x_k}f''(x)((x-x_{k})^2+(x-x_{k-1})^2-(\frac{b-a}{n})^2){\rm d}x\\
=\frac{f(x_{k-1})+f(x_k)}{2}\frac{b-a}{n}+\frac{1}{2}\int_{x_{k-1}}^{x_k}f''(x)(x-x_k)(x-x_{k-1}){\rm d}x\\
=\frac{f(x_{k-1})+f(x_k)}{2}\frac{b-a}{n}+\frac{1}{2}f''(\xi_k)\int_{x_{k-1}}^{x_k}(x-x_k)(x-x_{k-1}){\rm d}x\\
=\frac{y_{k-1}+y_k}{2}\Delta x-\frac{1}{12}f''(\xi_k)\frac{(b-a)^3}{n^3}
$$

这样就完事了,接下来对$n$求个和就能得到之前的公式.

接下来是二次曲线逼近.考虑用$(x_{k-1},y_{k-1}),(x_{k-\frac{1}{2}},y_{k-\frac{1}{2}}),(x_k,y_k)$三个点确定一条二次曲线.不妨设这条二次曲线为$Q_k(x)=px^2+qx+r$.然而,此时观察到:

$$
\int_{x_{k-1}}^{x_k}Q_k(x){\rm d}x\\
=\frac{p}{3}(x_k^3-x_{k-1}^3)+\frac{q}{2}(x_k^2-x_{k-1}^2)+r(x_k-x_{k-1})\\
=\frac{x_k-x_{k-1}}{6}(Q_k(x_k)+Q_k(x_{k-1})+Q_k(x_{k-\frac{1}{2}}))\\
=\frac{b-a}{6n}(y_{k-1}+y_k+4y_{k-\frac{1}{2}})
$$

这也很合理,观察一下就可以知道这个恰好是在尝试把梯形逼近和中点逼近两种方式尝试约掉后面的误差项系数.

定理是,如果$f\in C^4[a,b]$,误差项为$-\frac{(b-a)^5}{2880n^4}f^{(4)}(\xi)$.这就是所谓辛普森积分法.

##### 广义积分

考虑函数$f:[a,+\infty)\to \mathbb{R}$,如果其$\forall b>a$,$f\in R[a,b]$,我们称其**内闭可积**.那我们就定义其广义积分为$\int_a^{+\infty}f {\rm d}x=\lim_{b\to +\infty}\int_a^b f{\rm d}x$,当后者极限存在时.同理当然可以定义区间$(-\infty,b]$上的广义积分.而如果$\int_0^{+\infty}f{\rm d}x$和$\int_{-\infty}^0f{\rm d}x$都收敛,则定义它们的和为$\int_{-\infty}^{+\infty}f{\rm d}x$.

称一个函数的**瑕点**为其局部无界的点,则考虑$f:[a,b)\to \mathbb{R}$,其中$b$是$f$的瑕点,且$f$内闭可积,则$\int_a^b f(x){\rm d}x=\lim_{A\to b-0}\int_a^Af(x){\rm d}x$,若后者极限存在.瑕积分和上述广义积分并无明显区别,下述也不再区分.

在进一步讨论前,我们应当再仔细研究一下广义积分的收敛性,也就是当什么时候其极限存在呢?我们有以下命题:

1. $f(x)\geq 0,x\in [a,+\infty)$时,$F(A)=\int_a^A f(x){\rm d}x$单增.那么此时$F(+\infty)$存在当且仅当$F(A)$有界.
2. 比较原理:$0\leq f\leq cg,c>0$,若$\int_a^{+\infty}g{\rm d}x$收敛,则$\int_a^{+\infty}f{\rm d}x$收敛.该结论可以移植到瑕积分上.
3. 比较原理的推论:$f,g\geq 0,0<c_1\leq \frac{f}{g}\leq c_2$,则$\int f{\rm d}x,\int g{\rm d}x$收敛性相同.进一步地,如果$f,g\geq 0$,在趋近于某个瑕点的时候,$\frac{f}{g}$趋近于某个非零常数,那它们在这一点的收敛性相同.

(1)是根据单调收敛准则,而(2)则是(1)的推论,只是给出了一个界.

定义函数$f$是**绝对收敛**,当$\int_a^{+\infty}|f|{\rm d}x<\infty$时.而考虑$f=|f|-(|f|-f)$,而$0\leq |f|-f\leq 2|f|$,所以绝对收敛可以推出收敛,但反之不可.因此,我们将收敛但不绝对收敛的函数称为**条件收敛**.该结论也可以移植到瑕积分上.

接下来,我们有**柯西收敛原理**:$\int_a^{+\infty}f{\rm d}x$收敛的充要条件是对于$\forall \epsilon>0,\exists M>a$使得$\forall A>B\geq M$,都有$|\int_{B}^Af{\rm d}x|<\epsilon$.

证明当然无非是函数部分柯西收敛准则的重复应用.

柯西收敛原理推到瑕积分上的话,考虑$F(x)=\int_a^x f(t){\rm d}t$,则瑕积分$\lim_{t\to b-0}F(t)$存在当且仅当$\forall \epsilon>0$,$\exists \delta>0$,当$x_1,x_2\in (b-\delta,b)$的时候,有$|F(x_1)-F(x_2)|<\epsilon$.

接下来还有**Dirichlet-Abel判别法**,设$f,g:[a,+\infty)\to \mathbb{R}$内闭可积,$f$单调,那有:

1. Dirichlet判别法:如果$f(+\infty)=0$,$G(A)=\int_a^A g{\rm d}x$有界,则$\int_a^{+\infty}fg{\rm d}x$收敛.
2. Abel判别法:如果$f$有界,$\int_a^{\infty} g{\rm d}x$收敛,则$\int_a^{+\infty}fg{\rm d}x$收敛.

用第二积分中值定理,就有$\int_A^B f g{\rm d}x=f(A)\int_A^\xi g{\rm d}x+f(B)\int_{\xi}^B g{\rm d}x$.用柯西收敛原理则上述两条均为显然.结论当然可以推广到瑕积分上.


###### Example1

设$f$在$(-\infty,+\infty)$上内闭可积,且$f(+\infty)=A,f(-\infty)=B$,对于$a>0$,求证$\int_{-\infty}^{+\infty}\left(f(x+a)-f(x)\right){\rm d}x$收敛并求其值.

注意到:

$$
\int_0^{C}\left(f(x+a)-f(x)\right){\rm d}x
=\int_{a}^{C+a}f(x){\rm d}x-\int_0^C f(x){\rm d}x\\
=\int_{C}^{C+a}f(x){\rm d}x-\int_0^a f(x){\rm d}x\\
\to aA-\int_0^af(x){\rm d}x
$$

另一侧同理为:

$$
\int_{-C}^{0}\left(f(x+a)-f(x)\right){\rm d}x
=\int_{-C+a}^{a}f(x){\rm d}x-\int_{-C}^0 f(x){\rm d}x\\
=-\int_{-C}^{-C+a}f(x){\rm d}x+\int_0^a f(x){\rm d}x\\
\to -aB+\int_0^af(x){\rm d}x
$$

于是答案是$a(A-B)$.

###### Example2

求证:

$$
\int_{1}^{+\infty}\frac{ {\rm d}x}{x^p}
=
\begin{cases}
\lt \infty & p\gt1\\ 
\infty & p\leq 1
\end{cases}
$$

显然.

###### Example3

求证:$\int_a^{+\infty}\frac{\sin^2 x}{x^2} {\rm d}x$收敛.

原因是由比较原理,所以$0\leq \frac{\sin^2 x}{x^2}\leq \frac{1}{x^2}$,立刻知其收敛性.

###### Example4

判断$\int_0^{+\infty}\frac{x{\rm d}x}{1+x^4\sin^2 x}$的收敛性.

考虑取$F(A)=\int_0^A\frac{x{\rm d}x}{1+x^4\sin^2 x}$,接下来考虑$F(n\pi)=\sum_{k=1}^nU_k$,其中$U_k=\int_{(k-1)\pi}^{k\pi}\frac{x{\rm d}x}{1+x^4\sin^2 x}$.

考虑:

$$
U_k=\int_{(k-1)\pi}^{k\pi}\frac{x{\rm d}x}{1+x^4\sin^2 x}\\
\geq \int_{(k-1)\pi}^{k\pi}\frac{x{\rm d}x}{1+(k\pi)^4\sin^2 x}\\
\geq 2(k-1)\pi\int_{0}^{\frac{\pi}{2}}\frac{x{\rm d}x}{1+(k\pi)^4(\sin x)^2}\\
\geq 2(k-1)\pi\int_{0}^{\frac{\pi}{2}}\frac{x{\rm d}x}{1+(k\pi)^4x^2}\\
=\frac{2(k-1)\pi}{k^2\pi^2}\arctan(k^2\pi^2x)|_0^{\frac{\pi}{2}}\\
\geq \frac{2(k-1)\pi}{k^2\pi^2}\arctan(\frac{\pi^3}{2})\\
=O(\frac{1}{k})
$$

这就证完了,必然发散.

###### Example5

$f:(-\infty,+\infty)\to \mathbb{R}$且内闭可积,$p>0$,并且$\int_{-\infty}^{+\infty}|f|^p{\rm d}x<\infty$,求证$\lim_{h\to 0}\int_{-\infty}^{+\infty}\left|f(x+h)-f(x)\right|^p{\rm d}x$收敛.

既然$\int_{-\infty}^{+\infty}|f|^p{\rm d}x<\infty$,那么一定存在一个足够大的$A$,使得$\int_{A}^{+\infty}|f|^p{\rm d}x$足够小,不妨取足够大的$A>0$使得$\int_{A-1}^{+\infty}|f|^p{\rm d}x<\epsilon_1$.

事实上考虑$|u+v|\leq |u|+|v|\leq 2\max(|u|,|v|)$,那么$|u+v|^p\leq 2^p(|u|^p+|v|^p)$,取$|h|<1$,则:

$$
\int_{A}^{+\infty}\left|f(x+h)-f(x)\right|^p{\rm d}x\\
\leq 2^p\int_{A}^{+\infty}|f(x+h)|^p{\rm d}x+2^p\int_{A}^{+\infty}|f(x)|^p{\rm d}x\\
=2^p\int_{A+h}^{+\infty}|f(x)|^p{\rm d}x+2^p\int_{A}^{+\infty}|f(x)|^p{\rm d}x\\
\leq 2^{p+1}\int_{A-1}^{+\infty}|f|^p{\rm d}x\\
\leq 2^{p+1}\epsilon_1
$$

同理,另一侧也可以放掉.现在的问题在于$(-A+1,A-1)$中间这一段.

考虑用一个线性函数逼近,做分划$\Delta: -A+1=x_0<\cdots<x_n=A-1$,取线性函数逼近,策略为$x\in[x_{k-1},x_k]$的时候$g(x)=\frac{f(x_k)-f(x_{k-1})}{x_k-x_{k-1}}(x-x_{k-1})+f(x_{k-1})$.接下来考虑对于每一段,设$M_k=\sup_{[x_{k-1},x_k]}f,m_k=\inf_{[x_{k-1},x_k]}f$,则每一段可以被控制为$M_k-m_k$.我们早在之前就提过黎曼可积的函数可被线性函数逼近,然而疑问是这种逼近能否在$p$次方意义下满足,于是取足够小的划分使得$\sum_{k=1}^n(M_k-m_k)\Delta x_k<\epsilon_2^2$.

接下来常规分治,取$C=\{1\leq k\leq n|M_k-m_k\geq \epsilon_2\}$而$B=\{1\leq k\leq n|M_k-m_k< \epsilon_2\}$.见到$\sum_{k\in C}\Delta x_k<\epsilon_2$.不妨设$M=\sup_{[-A+1,A-1]} f,m=\inf_{[-A+1,A-1]} f$立刻有:

$$
\int_{-A+1}^{A-1}\left|f-g\right|^p{\rm d}x\\
\leq \sum_{k}|M_k-m_k|^p\Delta x_k\\
=\sum_{k\in C}|M_k-m_k|^p\Delta x_k+\sum_{k\in B}|M_k-m_k|^p\Delta x_k\\
\leq |M-m|^p\epsilon_2+\epsilon_2^p(2A-2)
$$

这样这里就被限制住了,而考虑:

$$
\int_{-\infty}^{+\infty}\left|f(x+h)-f(x)\right|^p{\rm d}x\\
\leq \int_{-\infty}^{+\infty}3^p\left(\left|f(x+h)-g(x+h)\right|^p+\left|g(x+h)-g(x)\right|^p+\left|f(x)-g(x)\right|^p\right){\rm d}x\\
$$

前后两项都能被上述控制住,只剩中间的$|g(x+h)-g(x)|^p$是$O(h^p)$的,只需调整$h$就能控制住.

###### Example6

求证:$\int_1^{+\infty}\frac{\sin x}{x}{\rm d}x$是条件收敛的.

用DA判别法,取$f=\frac{1}{x}$当然趋近于$0$,而$|\int_1^A \sin x {\rm d}x|\leq 2$,于是收敛.

那接下来就要证明它并非绝对收敛.只需考虑:

$$
\int_1^{+\infty}\frac{|\sin x|}{x}{\rm d}x\\
\geq \int_1^{+\infty}\frac{\sin^2 x}{x}{\rm d}x\\
=\int_1^{+\infty}\frac{1-\cos^2 x}{x}{\rm d}x
$$

当然并不绝对收敛.

事实上还可以直接求出来这个积分的值.

接下来考虑$f(x)=\begin{cases}0&x=0\\\frac{1}{x}-\frac{1}{2\sin\frac{x}{2}}&x\in [-\pi,\pi]\setminus \{0\}\end{cases}$.只需泰勒展开就知道它是解析的.回忆到黎曼引理:

$$
\int_0^{\infty}\frac{\sin x}{x}{\rm d}x\\
=\lim_{n\to +\infty}\int_{0}^{(n+\frac{1}{2})\pi}\frac{\sin x}{x}{\rm d}x\\
=\lim_{n\to +\infty}\int_{0}^{\pi}\frac{\sin ((n+\frac{1}{2})t)}{t}{\rm d}t\\
=\lim_{n\to +\infty}\int_{0}^{\pi}\left(f(t)+\frac{1}{2\sin \frac{t}{2}}\right)\sin ((n+\frac{1}{2})t){\rm d}t\\
=\lim_{n\to +\infty}\int_{0}^{\pi}\frac{1}{2\sin \frac{t}{2}}\sin ((n+\frac{1}{2})t){\rm d}t\\
=\lim_{n\to +\infty}\int_{0}^{\pi}\left(\frac{1}{2}+\sum_{k=1}^n\cos(kt)\right){\rm d}t\\
=\frac{\pi}{2}
$$

###### Example7

给定$f\in C[0,+\infty]$,$\int_0^{+\infty}f^2{\rm d}x<+\infty,f(0)=0$,取$g(x)=\int_0^xf(t){\rm d}t$,求证:$\int_0^{+\infty}\frac{g^2(x)}{x^2}{\rm d}x\leq 4\int_0^{+\infty}f^2{\rm d}x$以证明前者存在.

当然是分部积分,用洛必达法则知道$\lim_{x\to 0}\frac{g}{x}\to 0$,考虑:

$$
\int_0^A \frac{g^2}{x^2}{\rm d}x=\int_0^A g^2{\rm d}(-\frac{1}{x})\\
=-\frac{g^2(x)}{x}|_0^A+\int_0^A\frac{2gf}{x}{\rm d}x\\
=-\frac{g^2(A)}{A}+2\int_0^A\frac{g}{x}f{\rm d}x\\
\leq 2(\int_0^A \frac{g^2}{x^2}{\rm d}x)^{\frac{1}{2}}(\int_0^A f^2{\rm d}x)^{\frac{1}{2}}
$$

整理一下就做完了.

###### Example8(欧拉积分)

求$\int_0^{+\infty}\frac{x^{p-1}}{1+x}{\rm d}x,0<p<1$的收敛性和值.

考虑$\frac{x^{p-1}}{1+x}$在$x\to 0$的时候趋近于$x^{p-1}$,而在$x\to \infty$的时候趋近于$x^{p-2}$,因此绝对收敛.

难点在于把这个东西求出来.接下来将采取一些不严谨的说法.

考虑$0\leq x<1$的时候,有:

$$
\frac{x^{p-1}}{1+x}=x^{p-1}\sum_{j=0}^\infty(-x)^j\\=\sum_{j=0}(-1)^jx^{p+j-1}\\
$$

暂时抛开多余的思量,两边做积分:

$$
\int_0^1 \frac{x^{p-1}}{1+x}{\rm d}x=\sum_{j=0}(-1)^j\int_0^1 x^{p+j-1}{\rm d}x\\
=\sum_{j=0}(-1)^j\frac{1}{p+j}
$$

另一边,其实只需换元,可以发现:

$$
\int_1^{+\infty}\frac{x^{p-1}}{1+x}{\rm d}x\\
=\int_0^1\frac{t^{1-p}}{1+t^{-1}}\frac{ {\rm d}t}{t^2}\\
=\int_0^1\frac{t^{-p}}{1+t}{\rm d}t\\
=\int_0^1\frac{t^{1-p-1}}{1+t}{\rm d}t\\
=\sum_{j=0}^{\infty}(-1)^j\frac{1}{1-p+j}\\
=\sum_{j=0}^{\infty}(-1)^{j+1}\frac{1}{p-(j+1)}\\
=\sum_{j=1}^{\infty}(-1)^{j}\frac{1}{p-j}\\
$$

结合起来,答案当然就是$\sum_{j=-\infty}^{+\infty}\frac{(-1)^j}{p+j}$.

我们暂且断言上面那个无穷级数等于$\frac{\pi}{\sin (p\pi)}$,原因是我们权且承认下述级数是正确的:

$$
\frac{1}{\sin t}=\sum_{j=-\infty}^{+\infty}\frac{(-1)^j}{p+j\pi}
$$

###### Example9(Frullani积分)

当$f\in C[0,+\infty)$,且$\int_0^{+\infty}\frac{f(x)}{x}{\rm d}x$收敛,求证$\int_0^{+\infty}\frac{f(ax)-f(bx)}{x}{\rm d}x=f(0)\ln\frac{b}{a}$.

考虑:

$$
\int_A^{B}\frac{f(ax)-f(bx)}{x}{\rm d}x\\
=\int_{aA}^{aB}\frac{f(x)}{x}{\rm d}x-\int_{bA}^{bB}\frac{f(x)}{x}{\rm d}x\\
=\int_{aA}^{bA}\frac{f(x)}{x}{\rm d}x-\int_{aB}^{bB}\frac{f(x)}{x}{\rm d}x\\
=\int_{a}^{b}\frac{f(Ax)}{x}{\rm d}x-\int_{aB}^{bB}\frac{f(x)}{x}{\rm d}x\\
$$

前者当$A\to 0$的时候当然转化为$\int_{a}^{b}\frac{f(0)}{x}{\rm d}x$,而后者用柯西判准知道趋近于$0$,立刻完事.

###### Example10

求$I=\int_0^{\frac{\pi}{2}}\ln(\sin x){\rm d}x$.

考虑$x\to 0$的时候,$\ln(\sin x)\sim \ln x$,而$\int \ln x=x\ln x-x$,因此是绝对收敛的.

那怎么求这个积分呢?考虑一些对称技巧,立刻有:

$$
I=\int_0^{\frac{\pi}{2}}\ln(\sin x){\rm d}x\\
=\int_{\frac{\pi}{2}}^{\pi}\ln(\sin x){\rm d}x\\
=\int_{0}^{\frac{\pi}{2}}\ln(\cos x){\rm d}x\\
$$

那么:

$$
I=\frac{1}{2}\int_0^\pi\ln(\sin x){\rm d}x\\
=\int_0^{\frac{\pi}{2}}\ln(\sin 2t){\rm d}t\\
=\int_0^{\frac{\pi}{2}}\left(\ln 2+\ln (\sin t)+\ln (\cos t)\right){\rm d}t\\
=\frac{\pi}{2}\ln 2+2I
$$

所以$I=-\frac{\pi}{2}\ln 2$.

###### Example11

讨论$\int_0^1\frac{\ln x}{x^{\alpha}},\alpha>0$的收敛性.

考虑$x\in (0,1]$的时候,$\frac{\ln x}{x^{\alpha}}\leq 0$,因此其收敛性当然等价于绝对收敛性.取$x\to 0$的部分,而考虑:

当$0<\alpha<1$,任取一个足够小的$\delta>0$使得$0<\alpha+\delta<1$,都可以取足够逼近的$x\to 0$使得$|\frac{\ln x}{x^{\alpha}}|<\frac{1}{x^{\alpha+\delta}}$.于是此时绝对收敛.反之,当$\alpha\geq 1$的时候,$|\frac{\ln x}{x^\alpha}|>\frac{1}{x}$,此时发散.

###### Example12

讨论$p,q>0$,$\int_2^{+\infty}\frac{ {\rm d}x}{x^p\ln^q x}$的收敛性.

当$p>1$的时候,显然$\frac{1}{x^p\ln ^q x}<\frac{1}{x^p}$,因此当然绝对收敛.

当$p<1$的时候,同Example11,只需放掉$\ln x$就行,因此是发散的.

当$p=1$的时候,估计出了点问题,此时做换元$t=\ln x$,则原式变为:

$$
\int_{\ln 2}^{+\infty}e^{-t}e^{t}\frac{1}{t^q}{\rm d}t\\
=\int_{\ln 2}^{+\infty}\frac{1}{t^q}{\rm d}t\\
$$

所以当$q>1$的时候收敛,当$q\leq 1$的时候发散.

###### Example13

讨论$I=\int_0^{\frac{\pi}{2}}\frac{\sin x}{x^p}{\rm d}x$的收敛性.

设$f=\frac{\sin x}{x}$,容易见到$f$在$(0,\frac{\pi}{2})$上是单调递减的,那就应当有:

$$
\frac{2}{\pi}\frac{1}{x^{p-1}}\leq \frac{\sin x}{x^p}=\frac{\sin x}{x}\frac{1}{x^{p-1}}\leq  \frac{1}{x^{p-1}}
$$

因此其收敛性等价于$\int_{0}^{\frac{\pi}{2}} \frac{1}{x^{p-1}}$的收敛性,当$p<2$时收敛.

###### Example14

设$f\in C^2[a,+\infty)$,若$\int_a^{+\infty} f^2(x){\rm d}x<+\infty,\int_a^{+\infty} (f'')^2(x){\rm d}x<+\infty$,求证$\int_a^{+\infty} (f')^2(x){\rm d}x<+\infty$.

此时考虑做分部积分,当然有:

$$
\int_a^x f(t)f''(t){\rm d}t=f(x)f'(x)-f(a)f'(a)-\int_a^x (f'(t))^2{\rm d}t\\

f(x)f'(x)=f(a)f'(a)+\int_a^x f(t)f''(t){\rm d}t+\int_a^x (f'(t))^2{\rm d}t\\
$$

此时回忆到柯西不等式:

$$
(\int_a^x f(t)f''(t){\rm d}t)\leq (\int_a^x f^2(t){\rm d}t)(\int_a^x (f''(t))^2{\rm d}t)
$$

因此这一项被控制住了,于是我们知道$ f(x)f'(x)$和$\int_a^x (f'(t))^2{\rm d}t$收敛性相同.

接下来反证原结论,假设后者并不收敛而是趋近$+\infty$.那$f(x)f'(x)\to +\infty$,对其做积分,则:
$$
\int_a^{+\infty}f(x)f'(x){\rm d}x=\int_a^{+\infty}f(x){\rm d}f(x)=f^2(x)-f^2(a)
$$

因此$f^2(x)\to +\infty$,这当然与$\int_a^{+\infty} f^2(x){\rm d}x<+\infty$相违背.

###### Example15

判断$\int_1^{+\infty}\tan\left(\cfrac{\sin x}{x}\right){\rm d}x$的收敛性.

考虑$\int_1^{+\infty} \frac{\sin x}{x}{\rm d}x$当然是收敛的,而对$\tan\frac{\sin x}{x}$做泰勒展开应该和$\frac{\sin x}{x}$同阶,于是:

$$
\int_1^{+\infty}\tan\left(\cfrac{\sin x}{x}\right){\rm d}x\\
=\int_1^{+\infty}\frac{\sin x}{x}{\rm d}x+\int_1^{+\infty}\left(\tan\left(\cfrac{\sin x}{x}\right)-\frac{\sin x}{x}\right){\rm d}x\\
$$

前半部分当然收敛,后半部分取绝对值放缩后是$O(\frac{1}{x^2})$的,当然也收敛.

###### Example16

讨论$\int_1^{+\infty}\frac{\sin x}{x^p}\arctan x{\rm d}x,p>0$的收敛性.

$\int_1^{+\infty}\frac{\sin x}{x^p}{\rm d}x$收敛,而$\arctan x$单调有界,立刻知道收敛.

###### Example17

讨论$\int_2^{+\infty}\frac{\cos \sqrt x}{x^p\ln x}{\rm d}x$的收敛性.

当然先做换元$t=\sqrt x$,那么原式变成:

$$
\int_{\sqrt 2}^{+\infty}\frac{2t\cos t{\rm d}t}{t^{2p}\ln t^2}\\
=\int_{\sqrt 2}^{+\infty}\frac{\cos t{\rm d}t}{t^{2p-1}\ln t}\\
$$

$2p-1>1,p>1$的时候当然绝对收敛,而$0\leq 2p-1\leq 1$的时候用DA判准知道收敛,$2p-1<0$的时候当然发散.

###### Example18(磨光核函数)

构造一个$C^{\infty}$的函数$g(x)$,满足当$x\leq 0$的时候,$g(x)=0$;而当$x\geq 1$的时候,$g(x)=1$.也就是你要造一个函数把两段直线焊接起来.

考虑$h(x)=\begin{cases}e^{-\frac{1}{x^2}} &x\ne 0\\0&x=0 \end{cases}$.这个$h(x)$容易检验是$C^{\infty}$的,只需看$0$点处的可导性即可.

取$H(x)=\int_{-\infty}^xh(t)h(1-t){\rm d}t$.容易见到这函数在$0$处和$1$处都无穷阶可导.稍微乘下常数就可以搞定.

###### Example19

$\int_0^{+\inf}f(x){\rm d}x$收敛,$xf(x)$单调,求证:$\lim_{x\to \inf}xf(x)\ln x=0$.

首先容易证明$xf(x)$单调递减趋于$0$.那么:

$$
\int_{x_0}^{x_0^2}f(x){\rm d}x\\
=\int_{x_0}^{x_0^2}f(x)\frac{x}{x}{\rm d}x\\
=\int_{x_0}^{x_0^2}xf(x){\rm d}\ln x\\
\geq x_0^2f(x_0^2)\ln x_0\\
=\frac{1}{2}x_0^2f(x_0^2)\ln x_0^2
$$

令$x_0\to \inf$用柯西准则证毕.

###### Example20

求证:当$\int_a^b |f|<\infty$的时候,黎曼引理仍然成立.

回忆到黎曼引理要求$f\in R[a,b]$,但这里的确可以推广,原因是瑕积分肯定只有有限个瑕点(有无限个瑕点能立刻证明发散),只需要用足够小的区间把这有限个瑕点盖住,用柯西准则就可以证明这部分很小,而外面的部分当然是正常的黎曼引理.

#### RS积分

设$\alpha(x):[a,b]\to \mathbb R$是一个单调不减的函数,对于有界函数$f(x):[a,b]\to \mathbb{R}$,考虑一个分划$P:a=x_0<\cdots<x_n=b$,定义$\Delta \alpha_k=\alpha(x_k)-\alpha(x_{k-1})\geq 0$,并定义$M_k=\sup_{[x_{k-1},x_k]}f(x),m_k=\inf_{[x_{k-1},x_k]}f(x)$.在此基础上定义**上和**$U(P,f,\alpha)=\sum_{k=1}^n M_k\Delta \alpha_k$,同理定义**下和**$L(P,f,\alpha)=\sum_{k=1}^n m_k\Delta \alpha_k$.定义**上积分**$\overline{\int_{a}^b}f(x){\rm d}\alpha=\inf_P {U(P,f,\alpha)}$,同理定义**下积分**$\underline{\int_{a}^b}f(x){\rm d}\alpha=\inf_P {L(P,f,\alpha)}$.那么如果有以下$\overline{\int_{a}^b}f(x){\rm d}\alpha=\underline{\int_{a}^b}f(x){\rm d}\alpha$,称$f$在$[a,b]$上关于$\alpha$**RS可积**,记作$f\in {R}_\alpha[a,b]$,而将此值称作$f$关于$\alpha$的RS积分,记作$\int_a^bf(x){\rm d}\alpha$.

类比黎曼积分那套理论,应当有以下显而易见的性质:

1. 分划$P^*$如果是$P$的加密,那么$L(P,f,\alpha)\leq L(P^*,f,\alpha),U(P,f,\alpha)\geq U(P^*,f,\alpha)$.
2. $L(P_1,f,\alpha)\leq U(P_2,f,\alpha)$.
3. $\underline{\int_{a}^b}f(x){\rm d}\alpha\leq \overline{\int_{a}^b}f(x){\rm d}\alpha$.
4. $f\in {R}_\alpha$,其充要条件是$\forall \epsilon>0$,存在分划$P$使得$0\leq U(P,f,\alpha)-L(P,f,\alpha)<\epsilon$.

接下来考虑证明一些性质:

1. $f_1,f_2\in {R}_\alpha[a,b]$,则$f_1+f_2\in {R}_\alpha[a,b]$,并且$\int_a^b (f_1+f_2){\rm d}\alpha=\int_a^b f_1{\rm d}\alpha+\int_a^b f_2{\rm d}\alpha$.
2. $f\in {R}_\alpha[a,b]$,则$cf\in {R}_\alpha[a,b]$,并且$\int_a^b (cf){\rm d}\alpha=c\int_a^b f{\rm d}\alpha$.
3. $f_1,f_2\in {R}_\alpha[a,b],f_1\leq f_2$,则$\int_a^b f_1{\rm d}\alpha\leq \int_a^b f_2{\rm d}\alpha$.
4. $f\in {R}_\alpha[a,b],c\in [a,b]$,那么$f\in {R}_\alpha[a,c],f\in {R}_\alpha[c,b]$,且$\int_a^b f{\rm d}\alpha=\int_a^c f{\rm d}\alpha+\int_c^b f{\rm d}\alpha$.
5. $f\in {R}_\alpha[a,b],|f|\leq M$,则$|\int_a^b f{\rm d}\alpha|\leq M\left(\alpha(b)-\alpha(a)\right)$.
6. $f\in {R}_{\alpha_1}[a,b]$并且$f\in {R}_{\alpha_2}[a,b]$,则$f\in {R}_{\alpha_1+\alpha_2}[a,b]$,并且$\int_a^b f{\rm d}(\alpha_1+\alpha_2)=\int_a^b f{\rm d}\alpha_1+\int_a^b f{\rm d}\alpha_2$.
7. $f\in {R}_\alpha[a,b],c>0$.则$f\in {R}_{c\alpha}[a,b]$,并且$\int_a^b f{\rm d}(c\alpha)=c\int_a^b f{\rm d}\alpha$.
8. $f\in {R}_\alpha[a,b],m\leq f\leq M,g \in C[m,M]$.则$g(f(x))\in {R}_\alpha[a,b]$.
9. $f\in {R}_\alpha[a,b]$,则$|f|\in {R}_\alpha[a,b]$,且$|\int_a^b f{\rm d} \alpha|\leq \int_a^b |f|{\rm d} \alpha$.
10. $f_1,f_2\in {R}_\alpha[a,b]$,则$f_1f_2\in {R}_\alpha[a,b]$.
11. 如果极限$\lim_{\lambda(P)\to 0}(\sum f(t_k)\Delta \alpha_k),t_k\in [x_{k-1},x_k]$存在,则$f\in {R}_\alpha[a,b]$并且其RS积分就是上述极限.然而,逆命题未必成立.

对于(1),考虑对于分划$P_1,P_2$而言:

$$
L(P_1,f_1,\alpha)+L(P_2,f_2,\alpha)\leq L(P_1\cup P_2,f_1,\alpha)+L(P_1\cup P_2,f_2,\alpha)\\
\leq L(P_1\cup P_2,f_1+f_2,\alpha)\\
\leq U(P_1\cup P_2,f_1+f_2,\alpha)\\
\leq U(P_1\cup P_2,f_1,\alpha)+U(P_1\cup P_2,f_2,\alpha)
\leq U(P_1,f_1,\alpha)+U(P_2,f_2,\alpha)
$$

对此式子两边同取上下确界可以证明$\forall \epsilon>0,U(P_1\cup P_2,f_1+f_2,\alpha)-L(P_1\cup P_2,f_1+f_2,\alpha)<\epsilon$,即可得证(1).

对于(2),显然$U(P,cf,\alpha)=cU(P,f,\alpha)$,下和同理,于是立即得证.

对于(3),由(1),显然$f_2\geq f_1$时,$\int_a^b(f_2-f_1){\rm d}\alpha\geq 0$,于是$\int_a^b f_2{\rm d}\alpha\geq \int_a^b f_1{\rm d}\alpha$.

对于(4),考虑既然$f\in R_\alpha[a,b]$,那么$\forall \epsilon>0$,都应该$\exists P$使得$U_{[a,b]}(P,f,\alpha)-L_{[a,b]}(P,f,\alpha)<\epsilon$,而令$P'=P\cup\{c\}$,也就是将$c$强行作为一个分点,上式仍然成立,而可以将$P'$拆分为两个部分,不妨记作$P_1,P_2$,应该有:

$$
U_{[a,b]}(P',f,\alpha)-L_{[a,b]}(P',f,\alpha)\\
=U_{[a,c]}(P_1,f,\alpha)-L_{[a,c]}(P_1,f,\alpha)+U_{[c,b]}(P_2,f,\alpha)-L_{[c,b]}(P_2,f,\alpha)<\epsilon
$$

此二项均为正,因此分别$<\epsilon$,那也就说明了$f\in R_\alpha[a,c]$且$f\in R_\alpha[c,b]$,并且仍然是钦定$c$为分点就可以见到$\int_a^b f{\rm d}\alpha=\int_a^c f{\rm d}\alpha+\int_c^b f{\rm d}\alpha$.

对于(5),由(3)可以得知:

$$
-\int_a^b M{\rm d}\alpha\leq \int_a^b f{\rm d}\alpha\leq \int_a^b M{\rm d}\alpha
$$

而由定义立刻见到$\int_a^b M{\rm d}\alpha=M(\alpha(b)-\alpha(a))$.

对于(6),考虑$\Delta(\alpha_1+\alpha_2)_k=\Delta \alpha_1+\Delta \alpha_2$,因此$U(P,f,\alpha_1+\alpha_2)=U(P,f,\alpha_1)+U(P,f,\alpha_2)$.因此证明与(4)无异.

对于(7),只需考虑$\Delta(c\alpha)=c\Delta\alpha$,于是$U(P,f,c\alpha)=cU(P,f,\alpha)$,于是证明与(2)无异.

考虑(8)的证明,由于$g$是闭区间上的连续函数,立刻知道其一致连续.也就是$\forall \epsilon>0,\exists 0<\delta<\epsilon$,使得$|s-t|<\delta$时一定有$|g(s)-g(t)|<\epsilon$.接下来考虑由于$f\in R_\alpha[a,b]$,应当可以取一组划分$P: a=x_0<\cdots<x_n=b$,使得$U(P,f,\alpha)-L(P,f,\alpha)<\delta^2$.考虑将划分分为两类:$A=\{1\leq k\leq n\mid M_k-m_k<\delta\}$和$B=\{1\leq k\leq n\mid M_k-m_k\geq \delta\}$.这个思路就会和根号分治很像.那接下来考虑定义$M_k^*=\sup_{[x_{k-1},x_k]}g(f(x)),m_k^*=\inf_{[x_{k-1},x_k]}g(f(x))$.

那这就会使得当$k\in A$的时候,由于$g$的一致连续性,立刻可以知道$M_k^*-m_k^*\leq \epsilon$.而当$k\in B$的时候,考虑:

$$
\delta\sum _{k\in B}\Delta \alpha_k\leq \sum_{k\in B}(M_k-m_k)\Delta \alpha_k\\
=U(P,f,\alpha)-L(P,f,\alpha)<\delta^2
$$

于是$\sum _{k\in B}\Delta \alpha_k<\delta$.如此一来:

$$
U(P,g\circ f,\alpha)-L(P,g\circ f,\alpha)\\
=\sum_{k\in A}(M_k^*-m_k^*)\Delta \alpha_k+\sum_{k\in B}(M_k^*-m_k^*)\Delta \alpha_k\\
<\epsilon(\alpha(b)-\alpha(a))+2(\sup |g|)\delta\\
<\epsilon\left(\alpha(b)-\alpha(a)+2\sup |g|\right)
$$

这样就证毕了.

考虑(9)的证明,首先由(8)知道$|f|\in {R}_\alpha[a,b]$.而我们可以说存在$c=\pm 1$,使得$|\int_a^b f{\rm d} \alpha|=c(\int_a^b f{\rm d} \alpha)$.而$cf(x)\leq |f(x)|$,由(3)的保序性得证.

考虑(10)的证明,嵌套函数$g=x^2$并利用(9),则我们可以知道如果$f\in {R}_\alpha[a,b]$,那么$f^2\in {R}_\alpha[a,b]$.此时观察到$4f_1f_2=(f_1+f_2)^2-(f_1-f_2)^2$,立即得证.

(11)的证明比较显然.(11)直接的逆命题的话有个反例是考虑$f(x)=\begin{cases}1&x\geq 0\\0&x<0\end{cases},\alpha(x)=\begin{cases}1&x> 0\\0&x\leq 0\end{cases}$.此时会发现$x=0$这个点取不取在分划里是重要的,只要不取在分划里,怎么加密也没有用.

接下来当然应当研究一下连续性对RS可积的意义.我们声明:

1. 如果$f\in C[a,b]$,则$f\in {R}_\alpha[a,b]$且对$\forall \epsilon>0,\exists \delta>0$,并且只要$\lambda(P)<\delta$,那么$|\sum f(t_k)\Delta \alpha_k-\int_a^b f{\rm d}\alpha|<\epsilon$,其中$t_k\in [x_{k-1},x_k]$.
2. (1)中的条件如果改成$f\in R_\alpha[a,b],\alpha\in C[a,b]$,结论仍然成立.
3. 如果$f$单调,但是$\alpha$连续,则我们也能证明$f_\alpha\in {R}_\alpha[a,b]$.
4. 作为(3)的推论,如果$f$是有界变差函数,$\alpha$连续,也能证明$f_\alpha\in {R}_\alpha[a,b]$.
5. 如果$[a,b]$上的有界函数$f$只有有限个间断点,并且$f$和$\alpha$的间断点集交集为空,则$f\in R_\alpha[a,b]$.同时,(1)中的性质仍然成立.

(1)的证明,考虑$\forall \epsilon>0$,取$\gamma>0$,使得$\left(\alpha(b)-\alpha(a)\right)\gamma<\epsilon$.此时,$f$是闭区间上的连续函数,那它就一定是一致连续的函数.那么$\exists \delta>0$,当$|x-y|<\delta$的时候,有$|f(x)-f(y)|<\gamma$.此时只需考虑$U(P,f,\alpha)-L(P,f,\alpha)<\epsilon$就做完了.

(2)的证明,考虑既然$f\in R_{\alpha}[a,b]$,因此一定存在一组分划$P^*$使得$U(P^*,f,\alpha)<\int_a^b f{\rm d}\alpha+\epsilon$.

那么接下来对于任意一组分划$P:a=x_0<\cdots<x_m=b$,将其分为两组:一组是$A=\{k\mid \exists i,x_i^*\in (x_{k-1},x_k)\}$,$B=\{1,2,\cdots,m\}\setminus A$.也即$B$中的每个区间都被包含在$P^*$的分划中.既然如此,在$B$上的和的部分当然就要$\leq U(P^*,f,\alpha)<\int_a^b f{\rm d}\alpha+\epsilon$.只要我们能证明$A$能被控制住即可.

而由于$\alpha$连续,其在$[a,b]$上绝对连续,可以取足够小的分划使得$\Delta \alpha$足够小.既然如此,$\sum_{k\in A}f(t_k)\Delta \alpha_k\leq nM\max\{\Delta\alpha_k\}$,其中$M$是$f$在$[a,b]$上的上界,这个立刻可以被$\max\{\Delta\alpha_k\}$控制住.

(3)的证明,不妨设$f$单调不降,此时考虑取足够大的$n$使得$\frac{\alpha(b)-\alpha(a)}{n}(f(b)-f(a))<\epsilon$.那么由于$\alpha$连续,所以其有介值性,可以通过选取划分使得$\Delta \alpha_k\equiv \frac{\alpha(b)-\alpha(a)}{n}(f(b)-f(a))$,那还是考虑$U(P,f,\alpha)-L(P,f,\alpha)<\epsilon$即可.

(5)的证明相当平凡,只需用闭区间上的连续函数是一致连续处理间断点即可,在此略过.

于上述基础上进一步拓展,可以拓展到$\alpha$是有界变差的情况.原因是有界变差函数可以表示为两个不降函数的差,不妨设$\alpha(x)=\beta(x)-\gamma(x)$,则我们定义$\int_a^b f {\rm d}\alpha=\int_a^b f {\rm d}\beta-\int_a^b f {\rm d}\gamma$.

这个还要证明是良定的,原因是如果一个有界变差函数能表示成两种情形,有$\beta_1(x)-\gamma_1(x)=\beta_2(x)-\gamma_2(x)$,那么$\beta_1(x)+\gamma_2(x)=\beta_2(x)+\gamma_1(x)$,这两边都是不降函数,那根据$\alpha$部分可以加减的性质就完事了.

既然如此,对拓展的版本进一步寻找性质,不妨设$V(x)=V_a^x\alpha$.

1. $f\in C[a,b]$且$\alpha$是有界变差,或$f,\alpha$都是有界变差且$f$连续时,$|\int_a^b f {\rm d}\alpha|\leq \int_a^b f {\rm d}V$
2. 作为(1)的推论,上述条件时,$|\int_a^b f{\rm d}\alpha|\leq \sup|f|\times V_a^b\alpha$
3. (分部积分):当$\alpha,f$都是有界变差,并且它们有一个是连续的时候,$\int_a^b f {\rm d}\alpha=f\alpha\mid_a^b-\int_a^b \alpha {\rm d}f$.应当见到其实该定理只需要$\alpha,f$有一个是连续就行.
4. (第一积分中值定理):$f\in C[a,b]$且$\alpha$不降,则$\int_a^b f {\rm d}\alpha=f(\xi)\left(\alpha(b)-\alpha(a)\right)$.
5. (第二积分中值定理):$f$在$[a,b]$上单调,$\alpha$是连续有界变差.则$\int_a^b f {\rm d}\alpha=f(a)\left(\alpha(\xi)-\alpha(a)\right)+f(b)\left(\alpha(b)-\alpha(\xi)\right)$.
6. (变量替换公式):如果$f,g\in C[a,b]$并且$g$严格增,取$h=g^{-1}$,$c=g(a),d=g(b)$,那么$\int_a^b f(x){\rm d}x=\int_c^d f(h(y)){\rm d}h(y)$.
7. (和黎曼可积的关系):$f\in R[a,b]$,$\alpha$可导并且$\alpha'\in R[a,b]$,则$f\in R_\alpha[a,b]$,且$\int_a^b f {\rm d}\alpha=\int_a^b f \alpha'{\rm d}x$

考虑(1)的证明,由于此时已经可以使用求和来逼近,那不妨考虑:

$$
|S(P,f,\alpha)|=|\sum f(t_k)\Delta \alpha_k|\\
\leq |\sum f(t_k)|\sdot|\Delta \alpha_k|\\
\leq |\sum f(t_k)|\Delta V_k
$$

于是证毕.(2)自是(1)的推论.

(3)的话,当然是Abel求和,考虑:

$$
S(P,f,\alpha)=\sum_{k=1}^n f(t_k)(\alpha(x_k)-\alpha(x_{k-1}))\\
=f(b)\alpha(b)-f(a)\alpha(a)-\sum_{k=1}^{n+1}\alpha(x_{k-1})(f(t_k)-f(t_{k-1}))\\
=f\alpha|_a^b-S(Q,\alpha,f)
$$

容易见到$|P|\to 0$会导致$|Q|\to 0$,便是显然.

考虑(4),由于$f$是连续的,直接取$f$的上下界并介值定理一下即是显然.

考虑(5),用分部积分公式,立刻有:

$$
\int_a^b f{\rm d}\alpha=f(b)\alpha(b)-f(a)\alpha(a)-\int_a^b\alpha{\rm d}f\\
=f(b)\alpha(b)-f(a)\alpha(a)-\alpha(\xi)(f(b)-f(a))
$$

整理一下即可.

考虑(6),由于连续性,因此对$x$做的划分同样通过一个$h$成为了新的一组划分,而且由于闭区间上的连续函数是一致连续的,这个划分当然也趋近于$0$,立即见到成立.

对于(7),首先用介值定理得到$\Delta \alpha_k=\alpha'(\xi_k)\Delta x_k$,那当然有:

$$
S(P,f,\alpha)=\sum f(t_k)\Delta \alpha_k\\
=\sum f(t_k)\alpha'(t_k)\Delta x_k+\sum f(t_k)(\alpha'(\xi_k)-\alpha'(t_k))\Delta x_k
$$

而后面那一项绝对值不超过$|\sup f|\sum \omega_{\alpha'}$,当然趋近于$0$.

### 级数

对于数列$\{a_n\}$,定义$S_n=\sum_{k=1}^n a_k$.如果$\lim_{n\to\infty}S_n$存在,则称级数$\sum_{k=1}^{\infty}a_n$**收敛**,记作$S=\sum_{k}^{\infty}a_n$,否则如若上述极限不存在,称其**发散**.



容易见到以下性质显然成立,然而逆命题并不总是成立:

1. $\sum^{+\infty}a_n=A,\sum^{+\infty}b_n=B$,则$\sum^{+\infty}(a_n+b_n)=A+B$.
2. $\sum^{+\infty}a_n=A$,则$\sum^{+\infty}c a_n=cA$.
3. 结合律:$\sum^{+\infty}a_n=A$,若$\{n_k\}$是递增正整数序列且$n_0=1$,设$C_k=\sum_{n=n_{k-1}}^{n_k-1}a_n$,则$\sum_k^{+\infty}C_k=A$.

(1)(2)显然,(3)当然是因为原数列收敛立刻能推出其子列收敛.

如果$\sum |a_n|<\infty$,则称此级数**绝对收敛**.考虑$a_n=|a_n|-(|a_n|-a_n)$,前后两者都是正项数列而且$|a_n|-a_n\leq 2|a_n|$,所以绝对收敛当然能推出收敛,而反之不可.将收敛但不绝对收敛的级数称作**条件收敛**.绝对收敛的两个级数之和当然也绝对收敛,乘一个系数后也绝对收敛.

我们有**柯西收敛原理**:$\sum a_n$收敛的充要条件是$\forall \epsilon>0$,$\exists N>0,\forall m>n\geq N$,$|\sum_{k=n}^m a_k|<\epsilon$.用柯西准则立刻见到如若$\lim _{n\to \infty}S_n$收敛则必有$\lim_{n\to \infty}a_n=0$.但反命题当然不成立.这是相当重要的,正是因为$\lim_{n\to \infty}a_n=0$,所以我们才总能使用泰勒展开等手段进行估计和放缩.

再还有Abel-Dirichlet判别法,设$a_n$单调,$B_n=\sum_{k=1}^n b_k$,$B_n$有界,$|B_n|\leq M$:

1. 引理:$|\sum_{k=1}^n a_kb_k|\leq M(|a_1|+2|a_n|)\leq 2M(|a_1|+|a_n|)$.
2. Dirichlet判别法:若$a_n\to 0$,$B_n$有界,则$\sum a_kb_k$收敛.
3. Abel判别法:若$B_n$收敛,$a_n$有界,则$\sum a_kb_k$收敛.

(1)的话当然只需使用Abel变换,有:

$$
\sum_{k=1}^n a_kb_k=\sum_{k=1}^na_k(B_k-B_{k-1})\\
=a_nB_n+\sum_{k=1}^{n-1}(a_k-a_{k+1})B_k
$$

则:

$$
|\sum_{k=1}^n a_kb_k|\\
\leq |a_nB_n|+\sum_{k=1}^{n-1}|a_k-a_{k+1}|\sdot |B_k|\\
\leq M(|a_1-a_n|+|a_n|)\\
\leq M(|a_1|+2|a_n|)
$$

(2)(3)的话用柯西判准,考虑任意一段求和$|B_{n+p}-B_{n-1}|\leq 2M$,于是:

$$
|\sum_{k=n}^{n+p}a_kb_k|\leq 2M(|a_n|+2|a_{n+p}|)
$$

则属显然.

DA判别法的一个推论是,对于$\sum b_k$和$\sum c_k$来说,如果$\frac{b_k}{c_k}$单调有界且不趋于$0$,那么它们敛散性相同.

###### Example1

求$\sum \frac{\sin(nx)}{n}$的收敛性.

首先当$x=2k\pi$的时候当然收敛,否则使用DA判别法,考虑证明$\sum \sin(nx)$有界.

用积化和差,考虑:

$$
|\sum_{k=1}^n\sin (kx)|=\left|\frac{\sum 2\sin\frac{x}{2}\sin(kx)}{2\sin(\frac{x}{2})}\right|\\
=\left|\frac{\sum \cos(k-\frac{1}{2})x-\cos(k+\frac{1}{2})x}{2\sin(\frac{x}{2})}\right|\\
\leq \frac{1}{|\sin\frac{x}{2}|}
$$

这就完事.

能不能进一步把上面的这个级数求出来呢?考虑欧拉公式,令$z=e^{ix}$,知道$e^{inx}=\cos (nx)+i\sin(nx)$,所以$\sum \frac{z^n}{n}=-\ln(1-z)$的虚部就是所求.

而:

$$
-\ln(1-z)=-\ln(1-\cos x-i\sin x)\\
=-\ln\left(2(\sin\frac{x}{2})(\sin \frac{x}{2}-i\cos \frac{x}{2})\right)\\
=-\ln\left(2(\sin\frac{x}{2})e^{\frac{x-\pi}{2}i}\right)\\
=-\ln(2\sin\frac{x}{2})+\frac{\pi - x}{2}i
$$

这就顺便解决了$\sum \frac{\cos(nx)}{n}$的情况.

###### Example2

判断$\sum \frac{(-1)^n}{n^p+(-1)^n},p>0$的收敛性.

观察到:

$$
\frac{(-1)^n}{n^p+(-1)^n}\\
=\frac{(-1)^n}{n^p}\frac{1}{1+\frac{(-1)^n}{n^p}}\\
=\frac{(-1)^n}{n^p}\frac{1-\frac{(-1)^n}{n^p}}{1-\frac{1}{n^{2p}}}\\
=\frac{(-1)^n}{n^p-\frac{1}{n^p}}-\frac{1}{n^{2p}-1}
$$

前半部分当然收敛,于是$p\leq \frac{1}{2}$的时候发散,$\frac{1}{2}<p\leq 1$的时候条件收敛.

###### Example3

求$\sum \frac{\sin(\frac{n\pi}{4})}{n^p+\sin(\frac{n\pi}{4})}$的敛散性.

考虑:

$$
\frac{\sin(\frac{n\pi}{4})}{n^p+\sin(\frac{n\pi}{4})}\\
=\frac{\sin(\frac{n\pi}{4})}{n^p} \frac{1}{1+\frac{\sin(\frac{n\pi}{4})}{n^p}}\\
=\frac{\sin(\frac{n\pi}{4})}{n^p}\left(1-\frac{\sin(\frac{n\pi}{4})}{n^p}+o(\frac{1}{n^p})\right)\\
=\frac{\sin(\frac{n\pi}{4})}{n^p}-\frac{\sin^2(\frac{n\pi}{4})}{n^{2p}}+o(\frac{1}{n^{2p}})
$$

见到应该是$p\leq \frac{1}{2}$发散,$\frac{1}{2}<p\leq 1$的时候条件收敛,$p>1$的时候绝对收敛.

#### 正项级数

1. 当$a_n\geq 0$的时候,$S_n$有界$\Leftrightarrow $$S_n$有极限.
2. 当$0\leq a_n\leq b_n$,若$\sum^{+\infty}b_n<\infty$,则$\sum^{+\infty}a_n<\infty$.
3. 若$a_n,b_n>0$,则若$\varlimsup_{n\to \infty}\frac{a_n}{b_n}<\infty$,则$\sum^{+\infty}b_n<\infty\Rightarrow \sum^{+\infty}a_n<\infty$.
4. 若$a_n,b_n>0$,则若$\varliminf_{n\to \infty}\frac{a_n}{b_n}>0$,则$\sum^{+\infty}b_n=\infty\Rightarrow \sum^{+\infty}a_n=\infty$.
5. 若$a_n$单调递减趋近于$0$.则$\sum^{+\infty}_na_n<\infty\Leftrightarrow \sum_{k}^{+\infty}2^ka_{2^k}<\infty$.
6. $\sum_{k=1}^{+\infty}\frac{1}{n^p}$,当$p\leq 1$的时候发散,而当$p>1$的时候收敛.
7. $\sum_{k=2}^{+\infty}\frac{1}{n\ln^p n}$,当$p\leq 1$的时候发散,而当$p>1$的时候收敛.
8. 设单调递减连续正函数$f(x)$满足$a_n=f(n)$,则$\sum_{k=1}^{+\infty}a_k<\infty\Leftrightarrow \int_1^{+\infty}f(x){\rm d}x<\infty$.
9. 比值判别法:对于$a_n>0$,如果$\varlimsup_{n\to \infty}\frac{a_{n+1}}{a_n}<1$,则$\sum a_n<\infty$.
10. 比值判别法:对于$a_n>0$,如果$\varliminf_{n\to \infty}\frac{a_{n+1}}{a_n}>1$,则$\sum a_n=\infty$.
11. 根值判别法:对于$a_n>0$,设$l=\varlimsup_{n\to\infty}(a_n)^{\frac{1}{n}}$,如果$l<1$,则$\sum a_n<\infty$;若$l>1$,则$\sum a_n=\infty$.
12. 对于$a_n>0$,如果$\varliminf_{n\to \infty}n\ln \frac{a_n}{a_{n+1}}>1$,则$\sum a_n<\infty$.
13. 对于$a_n>0$,如果$\varlimsup_{n\to \infty}n\ln \frac{a_n}{a_{n+1}}<1$,则$\sum a_n=\infty$.
14. 拉贝判别法:对于$a_n>0$,若$\varliminf_{n\to \infty}n(\frac{a_n}{a_{n+1}}-1)>1$,则$\sum a_n<\infty$.
15. 拉贝判别法:对于$a_n>0$,若$\varlimsup_{n\to \infty}n(\frac{a_n}{a_{n+1}}-1)<1$,则$\sum a_n=\infty$.



(1)当然是单调收敛准则.(2)只是(1)的推论.

(3)(4)只需套用定义即可.

(5)的话,不妨设$S_n=\sum_{k=1}^n a_k$而$T_n=\sum_{k=1}^n2^ka_{2^k}$考虑当$2^k\leq n$的时候,有:

$$
S_n\geq \sum_{j=1}^{2^k}a_j\\
=a_1+\sum_{j=0}^{k-1}\sum_{i=2^j+1}^{2^{j+1}}a_i\\
\geq \frac{1}{2}T_k
$$

当$n<2^{k+1}$的时候,则有:

$$
S_n\leq \sum_{j=1}^{2^k}a_j\\
=\sum_{j=0}^{k-1}\sum_{i=2^j}^{2^{j+1}-1}a_i\\
\leq T_k
$$

(6)的话,首先$p\leq 1$的时候$\frac{1}{n^p}\geq \frac{1}{n}$于是显然,而当$p>1$的时候,考虑用(5),观察$\sum 2^k\frac{1}{2^{pk}}=\sum 2^{(1-p)k}$,当然收敛.

(7)的话亦然考虑(5),等价于$\sum 2^k\frac{1}{2^kk^p\ln^p 2}=\sum \frac{1}{k^p\ln^p 2}$,等价于(6)了.这个结论还可以推广到$\sum\frac{1}{n\ln n(\ln\ln n)^p}$,总之都是类似的做法,结论也类似.

(8)的证明只需考虑:

$$
a_{n+1}=f(n+1)\leq \int_n^{n+1}f(t){\rm d}t\leq f(n)=a_n
$$

立刻证毕.

而且这个估计还可以更精细,事实上我们可以证明下列极限总是存在:

$$
\lim_{n\to \infty}(\sum_{k=m}^n f(k)-\int_m^n f(x){\rm d}x)=\alpha
$$

而且还满足$0\leq \alpha\leq f(m)$.

考虑固定$m$,设$g(n)=\sum_{k=m}^n f(k)-\int_m^n f(x){\rm d}x$,用单调收敛准则,我们先证明它单调递减且有下界,注意到:

$$
g(n)-g(n+1)=-f(n+1)+\int_{n}^{n+1}f(x){\rm d}x\geq 0
$$

这就证明了其单调递减,而又有:

$$
g(n)=\sum_{k=m}^{n-1}\left(f(k)-\int_{k}^{k+1}f(x){\rm d}x\right)+f(n)\geq f(n)\geq 0
$$

因此有下界.同时注意到$g(m)=f(m)$,因此总有$0\leq \alpha\leq f(m)$成立.

不仅如此,如果$f(n)$还满足$\lim_{x\to \infty}f(x)=0$,我们还可以把这个极限收敛的速度求出来.我们有:

$$
|\sum_{k=m}^nf(k)-\int_m^n f(x){\rm d}x-\alpha|\leq f(n)
$$

干脆令$l>n$,考虑LHS应该是:

$$
\sum_{k=m}^nf(k)-\int_m^n f(x){\rm d}x-\alpha\\
=\sum_{k=m}^nf(k)-\int_m^n f(x){\rm d}x-\left(\lim_{l\to \infty}(\sum_{k=m}^l f(k)-\int_m^l f(x){\rm d}x)\right)\\
=\lim_{l\to \infty}\left(\int_n^lf(x){\rm d}x-\sum_{k=n+1}^l f(k)\right)\\
=\lim_{l\to \infty}\left(\sum_{k=n+1}^l(\int_{k-1}^kf(k){\rm d}x-f(k))\right)\\
\leq \lim_{l\to \infty}\left(\sum_{k=n+1}^l(f(k-1)-f(k))\right)\\
\leq f(n)
$$

而上式换个方向放缩就可以知道$\geq 0$,这就搞定.

(9)(10)类似(3)(4),用等比数列控制住即可.

(11)的话,如若$l=\varlimsup_{n\to\infty}(a_n)^{\frac{1}{n}}$.

当$l<1$的时候,取$\epsilon>0$使得$l+\epsilon<1$,则$\exists N_0>0$,$\forall n\geq N_0$,$(a_n)^{\frac{1}{n}}<l+\epsilon$,意味着$a_n<(l+\epsilon)^n$,立刻见到其收敛.

当$l>1$的时候,取$\epsilon>0$使得$l-\epsilon>1$,则存在原数列的一个无穷子列$\{n_k\}$,使得$a_{n_k}>(l-\epsilon)^{n_k}\to \infty$,当然发散.

而注意到根据Stolz定理:

$$
\varliminf \frac{a_{n+1}}{a_n}\leq \varliminf \sqrt[n]{a_n}\leq \varlimsup \sqrt[n]{a_n}\leq \varlimsup \frac{a_{n+1}}{a_n}
$$

这就意味着根值判别法理论上严格强于比值判别法.

(12)(13)的证明,考虑当$l=\varliminf_{n\to \infty}n\ln \frac{a_n}{a_{n+1}}>1$的时候,取$\epsilon>0,p=l-\epsilon>1$,那么就有$\exists N_0>0,\forall n\geq N_0$,都有:

$$
n\ln\frac{a_n}{a_{n+1}}>p\\
\frac{a_n}{a_{n+1}}>e^{\frac{p}{n}}>\left((1+\frac{1}{n})^n\right)^{\frac{p}{n}}\\
\frac{a_{n+1}}{a_n}<\cfrac{\frac{1}{(n+1)^p}}{\frac{1}{n^p}}
$$

取$b_n=\frac{1}{n^p}$,由上见到$a_n<\frac{a_{N_0}}{b_{N_0}}b_n$,这就控制住了.(13)同理.控制$e$的时候改用不等式$e<(1+\frac{1}{n-1})^n$即可.

(14)(15)仍然类似,设$b_n=n(\frac{a_n}{a_{n+1}}-1)$.

当$l=\varliminf_{n\to \infty}b_n>1$的时候,取$\epsilon>0$,$l-\epsilon>1$,则$\exists N_0>0,\forall n\geq N_0$,$b_n>l-\epsilon=l'$.而:

$$
\frac{a_n}{a_{n+1}}=1+\frac{b_n}{n}\\
\ln \frac{a_n}{a_{n+1}}=\ln(1+\frac{b_n}{n})>\ln(1+\frac{l'}{n})\\
\varliminf n\ln\frac{a_n}{a_{n+1}}\geq \varliminf \ln(1+\frac{l'}{n})^{n}=l'>1
$$

这就做完了.(15)同理.

###### Example1

斐波那契数列$f_0=0,f_1=1$,求$\sum \frac{1}{f_k}$的收敛性.

考虑$f_{n-1}\leq 2f_{n-2}$,所以$f_n=f_{n-1}+f_{n-2}\geq \frac{3}{2}f_{n-1}\geq (\frac{3}{2})^{n-1}$.这就做完了.

###### Example2

假设$0<a_n$单调递增且有界,求证:$\sum(1-\frac{a_k}{a_{k+1}})<\infty$.

考虑:

$$
\sum(1-\frac{a_k}{a_{k+1}})\\
=\sum {a_k}(\frac{1}{a_k}-\frac{1}{a_{k+1}})\\
\leq \sup{a}\sum(\frac{1}{a_k}-\frac{1}{a_{k+1}})\\
\leq \frac{\sup{a}}{a_1}
$$

###### Example3

求$\sum \frac{1}{n^p-n^q},p>q>0$的收敛性.

直接考虑$\lim_{n\to \infty}\cfrac{\frac{1}{n^p-n^q}}{\frac{1}{n^p}}=1$,所以$p\leq 1$的时候发散,$p>1$的时候收敛.

###### Example4

求$\sum \sin \frac{1}{n^2}$的收敛性.

直接考虑$\lim_{n\to \infty}\cfrac{\sin \frac{1}{n^2}}{\frac{1}{n^2}}=1$,所以收敛.

###### Example5

$a_n=(1-\sqrt[3]{\frac{n-1}{n+1}})^p,p>0$,讨论$\sum a_n$的敛散性.

考虑:

$$
\sqrt[3]{\frac{n-1}{n+1}}=(1-\frac{1}{n})^{\frac{1}{3}}(1+\frac{1}{n})^{-\frac{1}{3}}\\
=(1-\frac{1}{3n}+O(\frac{1}{n^2}))(1-\frac{1}{3n}+O(\frac{1}{n^2}))\\
=1-\frac{2}{3n}+O(\frac{1}{n^2})
$$

所以$a_n$和$\frac{1}{n^p}$同敛散.

###### Example6

求$S_n=\sum_{k=1}^n\frac{1}{\sqrt k}-2\sqrt n$的敛散性.

考虑:

$$
a_{n+1}=S_{n+1}-S_n\\
=\frac{1}{\sqrt{n+1}}-2\sqrt{n+1}+2\sqrt{n}\\
=\frac{1}{\sqrt{n+1}}-\frac{2}{\sqrt{n+1}+\sqrt{n}}\\
=\frac{\sqrt{n}-\sqrt{n+1}}{\sqrt{n+1}(\sqrt{n+1}+\sqrt{n})}\\
=\frac{-1}{\sqrt{n+1}(\sqrt{n+1}+\sqrt{n})^2}
=O(-n^{-\frac{3}{2}})
$$

所以收敛.

###### Example7

求$a_n=\frac{1}{\sqrt n}-\sqrt{\ln(1+\frac{1}{n})}$的级数和收敛性.

注意到:

$$
a_n=\frac{1}{\sqrt n}-\sqrt{\ln(1+\frac{1}{n})}\\
=\frac{1}{\sqrt n}-\sqrt{\frac{1}{n}-\frac{1}{2n^2}+o(\frac{1}{n^2})}\\
=\frac{1}{\sqrt n}-\frac{1}{\sqrt n}\left(1-\frac{1}{2n}+o(\frac{1}{n})\right)^{\frac{1}{2}}\\
=\frac{1}{\sqrt n}-\frac{1}{\sqrt n}\left(1-\frac{1}{4n}+o(\frac{1}{n})\right)\\
=O(n^{-\frac{3}{2}})
$$

所以收敛.

###### Example8

设$\zeta(a)=\sum_{k=1}^{\infty} \frac{1}{k^a}$,求$\lim_{\sigma\to 0+0}\sigma\zeta(1+\sigma)$和$\lim_{\sigma\to 0+0}\left(\zeta(1+\sigma)-\frac{1}{\sigma}\right)$.

取$f(x)=\frac{1}{x^{1+\sigma}}$,设$F(x)=\int_1^xf(t){\rm d}t$考虑:

$$
\sum_{k=n+1}^{\infty}a_k\leq F(+\infty)-F(n)\leq \sum_{k=n}^{\infty}a_k\\
F(+\infty)-F(n+1)\leq \sum_{k=n+1}^{\infty}a_k\leq F(+\infty)-F(n)\\
\frac{1}{\sigma}\frac{1}{(n+1)^\sigma}\leq \sum_{k=n+1}^{\infty}a_k\leq \frac{1}{\sigma}\frac{1}{n^\sigma}\\
\frac{1}{(n+1)^\sigma}\leq \sigma\sum_{k=n+1}^{\infty}a_k\leq \frac{1}{n^\sigma}
$$

取$n=0$和$n=1$得到$1\leq \sigma\zeta(1+\sigma)\leq \sigma+1$,于是$\lim_{\sigma\to 0+0}\sigma\zeta(1+\sigma)=1$.

而同理,见到:

$$
\zeta(1+\sigma)=1+\frac{1}{2^{1+\sigma}}+\cdots+\frac{1}{n^{1+\sigma}}+\sum_{k=n+1}^{\infty}\frac{1}{k^{1+\sigma}}
$$

而:

$$
1+\frac{1}{2^{1+\sigma}}+\cdots+\frac{1}{n^{1+\sigma}}+\frac{1}{\sigma}\frac{1}{(n+1)^\sigma}\\\leq\zeta(1+\sigma)\leq \\1+\frac{1}{2^{1+\sigma}}+\cdots+\frac{1}{n^{1+\sigma}}+\frac{1}{\sigma}\frac{1}{n^\sigma}\\
$$

用上述不等式:
$$
1+\frac{1}{2^{1+\sigma}}+\cdots+\frac{1}{n^{1+\sigma}}+\frac{1}{\sigma}\left(\frac{1}{(n+1)^\sigma}-1\right)\\\leq\zeta(1+\sigma)-\frac{1}{\sigma}\leq \\1+\frac{1}{2^{1+\sigma}}+\cdots+\frac{1}{n^{1+\sigma}}+\frac{1}{\sigma}\left(\frac{1}{n^\sigma}-1\right)\\
$$


两边对$\sigma\to 0+0$取极限,见到:
$$
1+\frac{1}{2}+\cdots+\frac{1}{n}-\ln(n+1)\\\leq \varliminf_{\sigma\to 0+0}(\zeta(1+\sigma)-\frac{1}{\sigma})\leq \varlimsup_{\sigma\to 0+0}(\zeta(1+\sigma)-\frac{1}{\sigma})\leq \\1+\frac{1}{2}+\cdots+\frac{1}{n}-\ln(n)
$$

两边对$n$取极限得到$\lim_{\sigma\to 0+0}\zeta(1+\sigma)-\frac{1}{\sigma}=\gamma$.

###### Example9

$a_n>0$,证明:若$\varlimsup_{n\to\infty}a_n^{\frac{1}{\ln n}}<\frac{1}{e}$则收敛.若$\varliminf_{n\to\infty}a_n^{\frac{1}{\ln n}}>\frac{1}{e}$则发散.

只证前者,则存在$p>1$使得$\varlimsup_{n\to\infty}a_n^{\frac{1}{\ln n}}<\frac{1}{e^p}$,两边取$\ln$就做完了.

###### Example10

$a_n>0$,证明:若$\varliminf_{n\to\infty}\cfrac{\ln \frac{1}{a_n}}{\ln n}>1$则收敛.若$\varlimsup_{n\to\infty}\cfrac{\ln \frac{1}{a_n}}{\ln n}<1$则发散.

只证前者,考虑前者等价于$\varlimsup_{n\to\infty}\ln a_n^{\frac{1}{\ln n}}<-1$,转化为Example9.

###### Example11

数列$\{a_n\}$恒正,数列$b_n=\frac{a_n}{a_{n+1}}-1-\frac{1}{n}$的部分和有界,求证$\sum a_n$发散.

考虑:

$$
\frac{a_n}{a_{n+1}}= 1+\frac{1}{n}+b_n\leq e^{\frac{1}{n}+b_n}\\
\frac{a_1}{a_{n+1}}\leq e^{\ln n+\gamma+M}=cn\\
a_n\geq \frac{ca_1}{n}
$$

这就搞定.

#### 交错级数

考虑$\sum (-1)^{n-1} a_n$,其中$a_n>0$.

Leibniz定理:如果$a_n$单调下降趋近于$0$,则交错级数收敛.

考虑$S_{2n}=(a_1-a_2)+(a_3-a_4)+\cdots$而$S_{2n+1}=a_1-(a_2-a_3)-(a_4-a_5)-\cdots$,所以见到$S_{2n}$单调上升,$S_{2n+1}$单调下降.并且显然$S_{2n}<S_{2n+1}$.而且$S_{2n+1}-S_{2n}=a_{2n+1}\to 0$,所以二者均收敛且极限相同极限相同.

当然,这也只是DA判别法的一个特例.

Leibniz定理还可以如此理解:由于后面的项正负抵消,我们发现:

$$
|\sum_{k=n}^\infty (-1)^ka_k|\leq a_n\to 0
$$

也就是后面的交错都可以被首项控制住.

###### Example1

考虑$\sum (-1)^{n}\frac{\ln^2 n}{n}$的收敛性.

取$f(x)=\frac{\ln^2 x}{x}$,则$f'=\frac{2\ln x-\ln^2 x}{x^2}$,显然当$x$足够大的时候$f'(x)<0$且$f(+\infty)=0$,所以原式收敛.

###### Example2

设$a_n>0$,求证:若$\varliminf n(\frac{a_n}{a_{n+1}}-1)>0$,则$\sum (-1)^na_n$收敛.

考虑$n$足够大的时候$\exists \epsilon >0$,使得$n(\frac{a_n}{a_{n+1}}-1)>\epsilon$,也就是$\frac{a_{n+1}}{a_n}<\frac{n}{n+\epsilon}<1$,这就能证明$a_n$单调递减.下面只需要证明$\lim a_n=0$即可.

而考虑取定$N>0$,对于$n\geq N$,有$\frac{a_n}{a_N}=\prod_{k=N}^{n-1}\frac{a_{k+1}}{a_{k}}$,$a_N$是定死的,下面只需要证明后面那个乘积趋近于$0$.

注意到:

$$
\prod_{k=N}^{n-1}\frac{a_{k+1}}{a_{k}}\\
<\prod_{k=N}^{n-1}\frac{n}{n+\epsilon}\\
=\prod_{k=N}^{n-1}\frac{1}{1+\frac{\epsilon}{n}}\\
$$

然而注意到:

$$
\prod_{k=N}^{n-1}(1+\frac{\epsilon}{n})\\
\geq 1+\frac{\epsilon}{N}+\cdots+\frac{\epsilon}{n-1}\to \infty
$$

这就完事.

###### Example3

取$f(x)=\sum_{n=1}^\infty \frac{(-1)^{n-1}}{n^x},x\in (0,\infty)$,求$\sup f$和$\inf f$.

首先显然能搞出$f(x)\in C^\infty(0,+\infty)$,原因是能证明其和其若干阶导数都内闭一致收敛(用DA判法).

首先交错级数的绝对值要$\leq$首项,所以先猜$\sup f=1$.考虑:

$$
f(x)=1-\sum_{n=1}^\infty\left(\frac{1}{(2n)^x}-\frac{1}{(2n+1)^x}\right)
$$

容易见到只要让$x\to \infty$那就逐项趋近于$0$.

还有一个问题是$\inf f$.考虑:

$$
\sum_{n=1}^\infty \frac{1}{(2n)^x}-\frac{1}{(2n+1)^x}\\
=\sum_{n=1}^\infty \int_{2n}^{2n+1}\frac{x{\rm d}t}{t^{x+1} }\\
$$

这里注意到我们积分的那几段都是$[2n,2n+1]$,我们可以把奇数段补上,因为$x>0$,所以补的那一段更大,自然有:

$$
\sum_{n=1}^\infty \int_{2n}^{2n+1}\frac{x{\rm d}t}{t^{x+1} }\\
\leq \frac{1}{2}\int_1^{+\infty}\frac{ {\rm d}t}{t^{x+1}}=\frac{1}{2}
$$

而$f(x)=1-\sum_{n=1}^\infty\left(\frac{1}{(2n)^x}-\frac{1}{(2n+1)^x}\right)$,所以$f(x)\geq \frac{1}{2}$.

接下来来证明其能取到,考虑:

$$
\sum_{n=1}^\infty \frac{1}{(2n)^x}-\frac{1}{(2n+1)^x}\\
=\sum_{n=1}^\infty \frac{1}{(2n)^x}\left(1-\frac{1}{(1+\frac{1}{2n})^x}\right)\\
=\sum_{n=1}^\infty \frac{1}{(2n)^x}\left(\frac{x}{2n}+O(\frac{x}{n^2})\right)\\
=\frac{x}{2^{x+1} }\zeta(x+1)+xO(1)\\
\to \frac{1}{2}
$$

原因是$\lim_{x\to 0+0}x\zeta(x+1)\to 1$,这个结论我们之前搞定过.

#### 积分判别法

取$\omega_n=\sup_{[n,n+1]} f-\inf_{[n,n+1]} f\geq 0$.我们声称若$\sum \omega_n<\infty$,则$\int_1^{\infty}f(x){\rm d}x$与$\sum f(n)$收敛性相同.

考虑:

$$
\sum_{k=1}^n f_k-\int_1^{n+1}f(x){\rm d}x\\
=\sum_{k=1}^n\int_k^{k+1}(f(k)-f(x)){\rm d}x\\
$$

考虑证明右式绝对收敛,有:

$$
\sum_{k=1}^n\int_k^{k+1}|f(k)-f(x)|{\rm d}x\\
\leq \sum_{k=1}^n\int_k^{k+1}\omega_k{\rm d}x\\
=\sum_{k=1}^{n+1}\omega_k
$$

这就证毕.

问题又来到如何搞定$\sum \omega_n<\infty$的条件,其实只要有界变差就行了对吧,我们断言:如果$f$可导,$f'$可积且$\int_1^{\infty}|f'|{\rm d}x<\infty$则$\sum \omega_n<\infty$.

对于$\omega_n$,不妨$[n,n+1]$的上界在$x_1$处取到,下界在$x_2$处取到,于是:

$$
\omega_n=f(x_1)-f(x_2)\\
=|\int_{x_1}^{x_2}f'{\rm d}x|\\
\leq |\int_{x_1}^{x_2}|f'|{\rm d}x|\\
\leq \int_{n}^{n+1}|f'|{\rm d}x\\
$$

于是$\sum \omega\leq \int_1^{+\infty} |f'|{\rm d}x$.

###### Example1

求$\sum \frac{\sin (n^\alpha x)}{n^\beta}$的收敛性,其中$\beta>\alpha>0,\alpha+\beta>1$.

用积分判别法,考虑取$f(t)=\frac{\sin (t^\alpha x)}{t^\beta}$,考虑:

$$
f'(t)=\frac{x\alpha t^{\alpha-1+\beta}\cos(t^\alpha x)-\beta t^{\beta-1}\sin(t^\alpha x)}{t^{2\beta}}
$$

逐项观察,都是$O(\frac{1}{t^{\beta-\alpha+1}})$级别,这样$\int |f'|{\rm d}t$就收敛了.

那就只需要看积分的收敛性,取$s=t^\alpha$考察:

$$
\int_1^{\infty}f(t){\rm d}t=\int_1^{\infty}\frac{\sin (t^\alpha x)}{t^\beta}{\rm d}t\\
=\int_1^{\infty}\frac{\sin (s x)\frac{1}{\alpha}s^{\frac{1}{\alpha}-1}}{s^\frac{\beta}{\alpha}}{\rm d}s\\
=\int_1^{\infty}\frac{\sin (s x)}{\alpha s^\frac{\beta+\alpha-1}{\alpha}}{\rm d}s\\
$$

后者用DA判法知道收敛.

#### 级数的交换顺序

定义$\mathbb{N}_+$的一个重排$f:\N_+\to \N_+$,当且仅当$f(1),f(2),\cdots$这个数列中,每个正整数都恰好出现一次.

我们声称正项级数满足:$\sum a_n=\sum a_{f(n)}$.

如何证明呢?

当$\sum a_n<\infty$的时候,考虑设$A_n=\sum_{k=1}^n a_k,B_n=\sum_{k=1}^n a_{f(k)}$.考虑取$N_n=\max_{1\leq k\leq n} f(k)$,那就有:

$$
B_n=\sum_{k=1}^n a_{f(k)}\leq A_{N_n}\leq \sum_{1}^{\infty} a_n
$$

于是$B_n$当然是收敛的,并且$\sum a_{f(n)}\leq \sum a_n$,而如果$a_{f(n)}$是$a_n$的重排,那么反之,$a_n$是$a_{f(n)}$的重排,于是$\sum a_{f(n)}\geq  \sum a_n$,这就证明了二者相同.

既如此,如果$\sum a_n$发散的时候,就可以反证法证明$\sum a_{f(n)}$不可能收敛.这样就完成了证明.

下面我们证明,如果$\sum a_n$绝对收敛,换言之$\sum |a_n|<\infty$,那么仍有$\sum a_n=\sum a_{f(n)}$.

考虑取$a_n^+=\frac{|a_n|+a_n}{2},a_n^-=\frac{|a_n|-a_n}{2}$.那考虑$0\leq a_n^+\leq |a_n|,0\leq a_n^-\leq |a_n|$,所以它们当然都绝对收敛,那当然有:

$$
\sum a_n=\sum(a_n^+-a_n^-)\\
=(\sum a_n^+) - (\sum a_n^-)
$$

而后者当然可以随意重排.

那么对于条件收敛呢?我们有**黎曼重排定理**:如果$\sum a_n$条件收敛,则$\forall B\leq A$,其中$A,B\in \mathbb{R}\cup\{\pm \infty\}$,则存在重排$f$,令$S_n=\sum_{k=1}^n a_{f(k)}$,有:

$$
\varliminf S_n=B,\varlimsup S_n=A
$$

为证明此,考虑设$I_+=\{n\in \mathbb{N}_+\mid a_n\geq 0\}=\{n_1<n_2\cdots\}$,$I_-$类似设出.

容易见到首先$\lim a_n\to 0$,并且$\sum_{n\in I_+}a_n=+\infty,\sum_{n\in I_-}a_n=-\infty$.

考虑先从$I_+$里取数,然后不断加加加加加直到超过$A$,由于正的部分相加是正无穷,这当然能做到,然后就继续从$I_-$里取数开始回退,退退退退到$B$之下,再重复上述过程.由于$a_n\to 0$,这玩意当然会满足条件.

那么如果$A$是正无穷呢?那没关系,我就每次加数的时候变化$A$,第一次个循环令$A=1$,第二次令$A=2$,以此类推.由此见上述命题的合理性.而且具体的论证过程只需要对这上面写抽象语言就行.

可是,难道一般的级数就真的不能交换顺序了嘛?事实上使用柯西准则,我们可以证明如果重排是局部的,或者说$\exists M>0,\forall n,|n-f(n)|\leq M<\infty$,换言之交换的距离有上界,那交换后敛散性不变.

这个怎么证明呢?首先如果原级数收敛的话,用柯西准则,因为通项趋于$0$,所以往外扩张适当有界长度(实际上就是$M$)应当无影响.如果原级数发散的话,如果通项还是不趋于$0$那当然还是完蛋,否则的话还可以用上面的证明.这就证毕了.

###### Example

$a_n>0$,当$\sum \frac{1}{a_n}<\infty$的时候,求证$\sum \frac{n}{\sum_{k=1}^n a_k}<\infty$.

首先考虑$\{a_n\}$单增的情况,如果不单增的话,可以直接对其进行排序.

考虑$\frac{2n}{\sum_{k=1}^{2n} a_k}<\frac{2n}{na_n}=\frac{2}{a_n}$,同理$\frac{2n-1}{\sum_{k=1}^{2n} a_k}<\frac{2n-1}{na_n}<\frac{2}{a_n}$,于是证毕.

#### 级数的乘法

考虑$(\sum a_j)(\sum b_k)=\sum a_jb_k$,首要的问题在于如何指定后者的求和顺序.

柯西策略是,考虑取$c_n=\sum_{j+k=n+1}a_jb_k$,然后取原式顺序为$\sum c_n$.此好处是可以求解生成函数乘积,$(\sum a_jx^j)(\sum b_kx^k)$.

矩形策略是,考虑取$c_n=\sum_{\max(j,k)=n}a_jb_k$,然后取原式顺序为$\sum c_n$.此好处是它的求和天然是$(\sum_{j=1}^n a_j)(\sum_{k=1}^n b_k)$的极限.所以此方法天然有如果$\sum a_j=A,\sum b_k=B$,则$\sum c=AB$.

接下来我们证明,如果$\sum a$和$\sum b$都绝对收敛,那它们的乘积任意排列都是相等的.换言之只要双重指标集合$\{(j_i,k_i)\}=\{(j,k)\}$那就都收敛于同一值.

证明策略类似,取$N_n=\max_{1\leq i\leq n}(j_i,k_i)$,那么当然有:

$$
\sum_{i=1}^n|a_{j_i}b_{k_i}|\leq (\sum_{j=1}^{N_n}|a_{j}|)(\sum_{k=1}^{N_n}|b_{k}|)
$$

那当然就绝对收敛,再类似之前的可以证明其与顺序无关,这个时候用矩形策略就可以知道最终就收敛于$(\sum a)(\sum b)$.

如果是条件收敛,矩形和当然没问题,仍然收敛,难点当然在于柯西和何时收敛?我们声称:如果$\sum a_n$收敛,$\sum |b_n|<\infty$,也就是一个收敛一个绝对收敛,我们就能推出柯西和是收敛的而且就等于矩形和.

考虑设矩形法的$u_k=\sum_{\max(i,j)=k}a_ib_j$,柯西法的$c_k=\sum_{i+j=k+1}a_ib_j$,考虑求:

$$
|\sum_{k=1}^n u_k-c_k|\\
=|\sum_{i_j>n+1,i\leq n,j\leq n}a_ib_j|\\
\leq \sum_{i=2}^n|b_i||\sum_{j=n-i+2}^n a_j|\\
=\sum_{i=2}^N|b_i||\sum_{j=n-i+2}^n a_j|+\sum_{i=N+1}^n|b_i||\sum_{j=n-i+2}^n a_j|
$$

接下来用柯西收敛原理,先取$N$,只要$N$足够大,由于$|\sum_{j=n-i+2}^n a_j|$当然是有界的,因为$\sum a$收敛,只要$N$足够大,不管$n$如何,$\sum_{i=N+1}^n|b_i|$用柯西收敛原理就足够小.这样后半项就解决了.

前半项同理,在$N$取定的时候$\sum_{i=2}^N|b_i|$就取定有界了.而只要在此基础上使得$n$尽可能大,$|\sum_{j=n-i+2}^n a_j|$用柯西收敛原理就控制住了,于是整体就很小.那就证明了此时柯西和恰好就是矩形和.

一般的柯西和没有这么好的结论,但其实证明可以类似上面,具体地,我们取$A_n=\sum_{k=1}^na_k,B_n=\sum_{k=1}^nb_k$并且$\lim A,\lim B$都存在,取柯西和$c_n=\sum_{k=1}^n a_kb_{n-k+1}$,我们断言$C_n=\sum_{k=1}^n c_k$收敛当且仅当极限$\lim_{n\to \infty}\sum_{k=1}^na_k\sum_{j=n-k+1}^nb_j$存在.

原因是:

$$
\sum_{k=1}^na_k\sum_{j=n-k+1}^nb_j\\
=\sum_{k=1}^na_k(B_n-B_{n-k})\\
=B_nA_n-(\sum_{k=1}^{n-1}a_kB_{n-k})\\
=B_nA_n-C_{n-1}
$$

这就证毕,而且当然见到只有上述极限为$0$的时候,柯西和才会有$\lim C=(\lim A)(\lim B)$.

#### 无穷乘积

对于数列$\{a_n\}$,设$A_n=\prod_{k=1}^na_k$,若$\lim_{n\to\infty}A_n$存在且非零,则称$\prod_{k=1}^{+\infty}a_k$收敛.为什么这里要求非零呢?因为我们希望类似无穷和,这里去掉前有限项后,敛散性不变.这就要求我们最好判掉零的情况.如若其极限为$0$,则仍称其发散,但有的时候发散到零也会有一些性质,到时我们会单独讨论.

既然如此,不妨设$a_n\ne 0$.有以下性质:

1. 若$\prod a_n$收敛,则$\lim a_n=1$.既然如此,$a_n$从某一项开始就满足$a_n>0$.之后不妨干脆设$a_n>0$.
2. $A_n$有非$0$极限当且仅当$\ln A_n$有极限.
3. 柯西收敛准则:$A_n$有非零极限的必要条件是$\forall \epsilon>0,\exists N,\forall N\leq n\leq m$,$|1-\prod_{k=n}^m a_k|<\epsilon$.特别地,如果$\forall a_n\ne 0$,则上式为充要条件.

(1)(2)显然.

(3)的话也比较显然.

总之,既然$\lim a_n\to 1$,不妨干脆设其为$1+a_n$,其中$\lim a_n=0$,从而原本要研究的$\prod (1+a_n)$可以转为研究$\sum \ln(1+a_n)$.

1. 当$a_n$定号的时候,$\sum \ln(1+a_n)$和$\sum a_n$的敛散性相同.
2. 当$a_n$不定号的时候,若$\sum a_n^2<\infty$,则$\sum \ln(1+a_n)$和$\sum a_n$的敛散性相同.同理如若$\sum a_n^2$发散,则上述两个级数至少一个发散.
3. $\sum |\ln(1+a_n)|$和$\sum |a_n|$的敛散性相同.

(1)是因为$\lim \frac{\ln(1+a_n)}{a_n}=1$.

(2)是因为$a_n-\ln(1+a_n)$可以被$a_n^2$控制住,原因是$a_n\to 0$,换言之存在$c_1,c_2$使得$c_1 x^2<a_n-\ln(1+a_n)<c_2 x^2$,只需要对$0$附近做个简单放缩.既然它们的差收敛,那它们当然敛散性相同.

(3)还是因为$\lim \frac{|\ln(1+x)|}{|x|}=1$.

###### Example1

当$|x|<1$的时候,求$\prod_{k=1}(1+x^{2^{k-1}})$的极限.

容易见到$A_n=\prod_{k=1}^n(1+x^{2^{k-1}})=\frac{1-x^{2^n}}{1-x}$,这样自然就收敛了.

###### Example2

求$\prod_{k=1}\cos\frac{\varphi}{2^k}$.

当$\varphi\ne 0$的时候,考虑$A_n=\prod_{k=1}^n\cos\frac{\varphi}{2^k}=\frac{\sin \varphi}{2^n\sin\frac{\varphi}{2^n}}$.虽然这里当$\varphi=2^k\pi$的时候会有若干项分母为$0$,但由于我们可以干脆去掉这前几项,而只看后几项分母均不为$0$,当然收敛于$\frac{\sin \varphi}{\varphi}$.

而$\varphi=0$的时候每一项都是$1$,乘起来仍然收敛.

###### Example3

考虑$\Gamma(x)=\frac{1}{x}\prod_{n=1}^{\infty}\frac{(1+\frac{1}{n})^x}{1+\frac{x}{n}}$敛散性,其中$x\ne 0,-1,-2,\cdots$.

直接泰勒展开,见到:

$$
\frac{(1+\frac{1}{n})^x}{1+\frac{x}{n}}\\
=(1+\frac{x}{n}+\frac{x(x-1)}{2n^2}+o(\frac{1}{n^2}))(1-\frac{x}{n}+\frac{x^2}{n^2}+o(\frac{1}{n^2}))\\
=1+\frac{x(x-1)}{2n^2}+o(\frac{1}{n^2})
$$

这就绝对收敛了.

而我们观察$A_n$,有:

$$
A_n=\frac{1}{x}\prod_{k=1}^{n}\frac{(\frac{k+1}{k})^x}{1+\frac{x}{k}}\\
=\frac{(n+1)^x}{x(1+\frac{x}{1})\cdots(1+\frac{x}{n})}\\
=(\frac{n+1}{n})^x\frac{n!n^x}{x^{\overline{n+1}}}\\
$$

于是见到:

$$
\frac{\Gamma(x+1)}{\Gamma(x)}=\lim_{n\to \infty}\frac{(n+1)x}{x+1+n}=x
$$

所以$\Gamma(x+1)=x\Gamma(x)$,而且$\Gamma(1)=1$,所以$\Gamma(n)=n!$,这实际上是阶乘函数的一个推广.

而此时有$\Gamma(x+1)=x \Gamma(x)=\prod_{n=1}^\infty\frac{(1+\frac{1}{n})^x}{1+\frac{x}{n}}$.

而观察这个级数:

$$
\prod_{n=1}^\infty\frac{e^{\frac{1}{n}} }{1+\frac{1}{n}}\\
=\frac{e^{1+\frac{1}{2}+\cdots +\frac{1}{n}}}{n+1}\\
=\frac{e^{\ln n+\gamma+c_n}}{n+1}
\to e^\gamma
$$

其中$\gamma$是欧拉常数,误差项$c_n=H_n-\ln n-\gamma \to 0$.

从而观察到:

$$
e^{\gamma x}\Gamma(x+1)\\
=\prod_{n=1}^\infty \frac{e^{\frac{x}{n}}}{1+\frac{x}{n}}
$$

从而$\frac{1}{\Gamma(x+1)}=e^{\gamma x}\prod_{n=1}^\infty (1+\frac{x}{n})e^{-\frac{x}{n}}$.

在此基础上,如若设$b_n=\frac{n! n^x}{x^{\overline{n+1}}}$.容易见到$\lim b_n=\lim A_n=\Gamma(x)$,而观察到:

$$
\frac{b_{n+1}}{b_n}=\frac{(1+\frac{1}{n})^{x+1}}{1+\frac{x+1}{n}}\\
=1+\frac{x(x+1)}{2n^2}+o(\frac{1}{n^2})\\
\to 1
$$

而且如若$x(x+1)>0$,并且取$n$足够大,则上式大于$1$,反之上式小于$1$.因此当$n$足够大的时候$b_n$单调有界(原因是其极限存在).

因此还可以得到结论:$\sum \frac{n! a_n}{x^{\overline{n+1}} }$和$\sum \frac{a_n}{n^x}$敛散性相同,其中$x\ne 0,-1,-2,\cdots$.原因只是因为这俩的比值恰好是$b_n$单调有界,根据DA判别法知道二者同敛散.

###### Example4

求$\sum_{n=1}^\infty \frac{(nx)^n}{n!}$的收敛性.

考虑:

$$
\frac{a_{n+1}}{a_n}=(\frac{n+1}{n})^n x\to ex
$$

根据比值判法,当$|x|<\frac{1}{e}$的时候绝对收敛,而当$|x|>\frac{1}{e}$的时候发散,问题只在于$|x|=\frac{1}{e}$的时候如何.

当$x=\frac{1}{e}$的时候,使用拉贝判法:

$$
n(\frac{a_n}{a_{n+1}}-1)=n((1-\frac{1}{n+1})^n e-1)\\
=n(e^{n\ln(1-\frac{1}{n+1})+1}-1)\\
=n(e^{n(-\frac{1}{n+1}-\frac{1}{2(n+1)^2}+o(\frac{1}{n^2}))+1}-1)\\
=n(e^{\frac{1}{n+1}-\frac{n}{2(n+1)^2}+o(\frac{1}{n})}-1)\\
=n(\frac{1}{n+1}-\frac{n}{2(n+1)^2}+o(\frac{1}{n}))\\
\to \frac{1}{2}<1
$$

所以发散.

当$x=-\frac{1}{e}$的时候,使用DA判法:应当见到$a_n=(-1)^n|a_n|$,此时$|a_n|$单调递减且趋于$0$,用DA判法知道其收敛.

所以条件收敛.

###### Example5

求证$\sum a_n(x^2-1)(x^2-2^2)\cdots(x^2-n^2)$对$\forall x\notin \mathbb{Z}$,其都有相同的敛散性.就是对于一个确定的$\{a_n\}$,对所有的$x$,要么都收敛,要么都发散.

考虑设$f_n(x)=(x^2-1)(x^2-2^2)\cdots(x^2-n^2)$,观察到:

$$
\frac{f_n(x)}{f_n(x_0)}=\frac{(x^2-1)(x^2-2^2)\cdots(x^2-n^2)}{(x_0^2-1)(x_0^2-2^2)\cdots(x_0^2-n^2)}
$$

这当然是个无穷乘积,观察到$\frac{x^2-n^2}{x_0^2-n^2}=1+\frac{x^2-x_0^2}{x_0^2-n^2}=1+O(\frac{1}{n^2})$,所以$\lim_{n\to \infty} \frac{f_n(x)}{f_n(x_0)}$存在且非零.

而且,当$n$足够大的时候,或者更进一步说$|n|>\max(|x|,|x_0|)$的时候$\frac{f_n(x)}{f_n(x_0)}$当然就定号了,而且$\frac{x^2-n^2}{x_0^2-n^2}=1+\frac{x^2-x_0^2}{x_0^2-n^2}$,其与$1$比较只取决于$x^2-x_0^2$,这就意味着$\frac{f_n(x)}{f_n(x_0)}$在$n$足够大的时候是单调有界非零的,于是DA判法知道同敛散.

###### Example6

求证$\sin x=x\prod_{k=1}^{\infty}(1-\frac{x^2}{k^2\pi^2})$.

先看$\sin x=0$的根,假设其根为$u+iv$,考虑:

$$
\sin x=\frac{e^{ix}-e^{-ix}}{2i}=0\\
e^{2ix}=1\\
e^{2iu-2v}=1\\
|e^{2iu-2v}|=1\\
e^{-2v}=1\\
v=0
$$

所以其只有实根,用多项式理论知道如若$\sin x$的根只有$2k\pi$,应该有:

$$
\sin x=cx\prod_{k=1}^{\infty}(1-\frac{x^2}{k^2\pi^2})
$$

容易见到这个无穷乘积的确收敛,而$x\to 0$的时候$\frac{\sin x}{x}=1$,所以$c=1$.

那么考虑$\sin x$的泰勒展开,有:

$$
1-\frac{x^2}{6}+\cdots=\prod_{k=1}(1-\frac{x^2}{k^2\pi^2})\\
=1-(\frac{1}{\pi^2}\sum_k\frac{1}{k^2})x^2+\cdots
$$

这就能知道$\sum_{k=1}^\infty \frac{1}{k^2}=\frac{\pi^2}{6}$.

然而上述的多项式理论还是太过粗糙,下面提供一种更加严谨的证明:

我们有欧拉公式:

$$
(\cos x + i\sin x)^{2n+1}=\cos(2n+1)x+i\sin(2n+1)x\\
$$

然而左侧可以使用二项式定理展开,有:

$$
(\cos x + i\sin x)^{2n+1}=\sum \binom{2n+1}{k}\cos^k x(i\sin x)^{2n+1-k}
$$

容易见到为了贡献虚部,$k$必须是偶数,那此时$\cos^k x=(1-\sin^2 x)^{\frac{k}{2}}$,总之,我们发现应该存在一个$n$次多项式$P_n$,使得:

$$
\sin(2n+1)x=(\sin x)P_n(\sin^2 x)\\
$$

这个多项式的根应该怎么取呢?考虑当$\varphi_k=\frac{k\pi}{2n+1},1\leq k\leq n$的时候,左侧为$0$,而右侧$\sin \varphi_k$不该为$0$,更进一步地,$\sin ^2 \varphi_k$两两不同,这就知道它们确实是$P_n$的$n$个根.同时见到$P(0)=\lim_{x\to 0}P_n(\sin ^2 x)=2n+1$,从而我们已经能唯一确定这个有限项多项式$P_n(x)=(2n+1)\prod_{k=1}^n(1-\frac{x}{\sin^2 \varphi_k})$.

接下来,将原本上式中的$x$换成$\frac{x}{2n+1}$,我们就能得到:

$$
\sin x=(2n+1)\sin(\frac{x}{2n+1})\prod_{k=1}^n(1-\frac{\sin^2\frac{x}{2n+1}}{\sin^2 \varphi_k})
$$

这里已经十分接近我们想要的答案了,但是这个项数和$n$一起增大,策略当然是大小步极限,考虑取:

$$
U_m=(2n+1)\sin\frac{x}{2n+1}\prod_{k=1}^m(1-\frac{\sin^2\frac{x}{2n+1}}{\sin^2 \varphi_k})\\
V_m=\prod_{k=m+1}^n(1-\frac{\sin^2\frac{x}{2n+1}}{\sin^2 \varphi_k})\\
$$

取定$m$,当然总能取足够大的$n$使得$U_m\to x\prod_{k=1}^m(1-\frac{x^2}{k^2\pi^2})$.

而用柯西准则,当$m$足够大的时候,如果能证明$V_m\to 1$就完事了.我们考虑:

$$
1\geq V_m\geq \prod_{k=m+1}^\infty(1-\frac{\sin^2\frac{x}{2n+1}}{\sin^2 \varphi_k})\\
$$

而回忆到当$\sin^2 x\leq x^2$恒成立,在$x\in [0,\frac{\pi}{2}]$的时候,$\sin x\geq \frac{2}{\pi}x$也成立,所以:

$$
V_m\geq \prod_{k=m+1}^\infty(1-\frac{\sin^2\frac{x}{2n+1}}{\sin^2 \varphi_k})\\
\geq \prod_{k=m+1}^\infty(1-\frac{x^2}{4k^2})\\
$$

此时回忆到$\prod_{k=1}^\infty(1-\frac{x^2}{4k^2})$是收敛的,所以由柯西准则,当$m$足够大的时候,$\prod_{k=m+1}^\infty(1-\frac{x^2}{4k^2})$趋近于$1$,这就证毕了.

### 函数项数列

也就是每一项都是一个函数的数列,假设定义域在$I\subseteq \mathbb{R}$上,写作$\{f_n(x)\}$.此时对于一个固定的$x_0\in I$,则$\{f_n(x_0)\}$就是一个普通的数列.当然不一定$I$中的每一个$x_0$都收敛,我们将**收敛点**的集合称为函数项级数的**收敛域**,其它的点称为**发散点**,发散点组成的集合称为**发散域**.

对于收敛域,直接记$f(x)=\lim_{n\to \infty}f_n(x)$,我们将这里的$f(x)$就是这个函数列的**极限函数**.既然有了极限就可以定义级数,也就是可以定义$S_n=\sum_{k=1}^nf_k(x)$,然后取其极限拿到$\sum_{k=1}^\infty f_n(x)=S(x)$.

我们要解决的问题大概有以下三个:

1. 如果$f_n$均是连续的,那么要求什么条件,可以满足$f$连续.也就是两层极限什么时候可以交换顺序.
2. 如果$f_n$均是黎曼可积的,那么要求什么条件,可以满足$f$黎曼可积并且$\lim \int f_n=\int f$,也就是极限和积分什么时候可以交换顺序.
3. 如果$f_n$均可导,那么什么时候$f$也可导,并且$\lim f_n'=f'$,也就是极限和导数什么时候可以交换顺序.

如果不加条件,上述三条全都是不满足的,下述Example会给出若干例子.

###### Example1

取$f_n(x)=x^n,x\in [0,1]$,求其极限函数.

显然$f(x)=\begin{cases}1&x=1\\0&\text{otherwise}\end{cases}$,由此可以见到,连续函数的极限函数不一定连续.

###### Example2

取$f_n(x)=\begin{cases}1&(n!)x\in \mathbb{Z}\\0&\text{otherwise}\end{cases}$,求其极限函数.

显然是迪利克雷函数.

则$f_n(x)$只有有限个间断点,容易见到其黎曼可积,然而迪利克雷函数并不黎曼可积.可积函数的极限函数不一定可积.

###### Example3

取$f_n(x)=nx(1-x^2)^n,x\in [0,1]$,求其极限函数.

容易见到$\lim f_n(x)=0$,也就是这个函数处处收敛到$0$.

然而,对其求积分:

$$
\int_0^1 f_n(x){\rm d}x=\int_0^1 nx(1-x^2)^n{\rm d}x\\
=-\frac{n}{2}\int_0^1(1-x^2)^n{\rm d}(1-x^2)\\
=-\frac{n}{2(n+1)}(1-x^2)^{n+1}|_0^1\\
=\frac{n}{2(n+1)}\to \frac{1}{2}
$$

由此见到,极限函数的积分不一定等于积分的极限.极限和积分不一定可交换.

###### Example4

考虑$f_n(x)=\begin{cases}2n^2x\\x\in[0,\frac{1}{2n}]\\2n^2(x-\frac{1}{n})&x\in(\frac{1}{2n},\frac{1}{n}]\\0&\text{otherwise}\end{cases}$,也就是这个函数在$\frac{1}{2n}$处高度为$n$,底长为$\frac{1}{n}$的一个小三角形.求其极限函数.

对于每一个点,这个函数当然逐点收敛于$0$.然而也可以见到,这个函数的积分对于每一个$n$来说都是$\frac{1}{2}$.

再次说明了极限函数的积分不一定等于积分的极限.极限和积分不一定可交换.

###### Example5

考虑$f_n(x)=\frac{\sin(nx)}{\sqrt n}$,求其极限函数.

当然逐点收敛于$0$,然而观察到$f_n'(x)=\sqrt n \cos nx$,$f'(x)=0$.因此,光滑函数列的导数的极限又不一定等于极限的导数.

#### 一致收敛

我们称一列函数$\{f_n(x)\}$是**一致收敛**到$f(x)$,当且仅当$\forall \epsilon>0$,$\exists N=N(\epsilon)$,使得当$n\geq N$的时候,$|f(x)-f_n(x)|<\epsilon,\forall x\in I$.注意这里的$N$不取决于$x$的选取,这就是和极限不同的地方,一致收敛要求整个函数都在逐步地贴向最终的函数.此时,记$f_n(x)\rightrightarrows f(x)$.还可以定义**内闭一致收敛**的概念,也就是对于一个$I$的任意的子紧集(在$\mathbb{R}$上表现为有界闭集)都一致收敛.

一致收敛的等价定义当然是,取$M_n=\sup_{x\in I}|f_n(x)-f(x)|$,那么一致收敛等价于$\lim M_n=0$.

一致收敛强大的地方在于,对于收敛来说,是取定了$x$再挪动$n$,因此$n$可以控制$x$.然而,一致收敛要求先取定$n$再挪动$x$.这种天然的交换能力赋予了一致收敛的强大.另外还容易见到,当$I$的点集是有限的时候,一定一致收敛.

我们还可以定义**一致有界**的概念,当存在一个上界$M$,使得$\forall n,|f_n|\leq M$恒成立.显然,如果一个函数列一致有界,则极限函数肯定有界.

另外,我们可以见到,如果$f_n\rightrightarrows f$,那么如果$f$有界,则$f_n$一致有界.反之,如果$f_n$有界,则$f_n$一致有界.

考虑前者,是因为:

$$
|f_n|\leq |f_n-f|+|f|
$$

这俩当然是有界的.

对于后者,考虑反证,假设$f$无界(因为如果有界则前者即可推出),那么:

$$
|f|\leq |f-f_n|+|f_n|
$$

由于$|f-f_n|$必然需要有界,因此前者无界,后者无论对于哪个$n$来说,必然也得无界.

我们有以下性质:

1. $f_n\rightrightarrows f,g_n\rightrightarrows g,x\in I $,则$c_1f_n+c_2g_n\rightrightarrows c_1f+c_2g,x\in I$.
2. $f_n\rightrightarrows f,g_n\rightrightarrows g,x\in I $,而且$f,g$有界,则$f_ng_n\rightrightarrows fg,x\in I$.
3. 如果$x\in I$的时候$f_n\rightrightarrows f$,$x\in J$的时候也$f_n\rightrightarrows f$,则当$x\in I\cup J$的时候也有$f_n\rightrightarrows f$.

对于(1),考虑令$M_{n}=\sup |f_n-f|,K_n=\sup |g_n-g|$,则:

$$
|(c_1f_n+c_2g_n)-(c_1f+c_2g)|\\
\leq |c_1|M_n+|c_2|K_n\to 0
$$

立刻证毕.

对于(2),经典的做法:

$$
|f_ng_n-fg|\\
=|f_n(g_n-g)+g(f_n-f)|\\
\leq |f_n|\sdot |g_n-g|+|g|\sdot |f_n-f|
$$

而$|f_n|$一致有界,$|g_n|$一致有界得到$|g|$有界,所以上式就被控制住了.

对于(3),只需要把两部分的$\sup$分开算就可以见到仍能被控制住.

这里也有**柯西准则**:对于一列函数$\{f_n\}$,其一致收敛的充要条件是,$\forall \epsilon>0,\exists N,\forall n,m\geq N,|f_n(x)-f_m(x)|<\epsilon$.

必要性显然,只证充分性,对于此时固定一个确定的$x$,那么$\{f_n(x)\}$就是一个柯西列且存在极限.逐点取极限就可以拿到$f(x)=\lim_{n\to \infty} f_n(x)$.而$|f_n(x)-f_m(x)|<\epsilon$中只需要让$m\to \infty$就拿到了$|f_n(x)-f(x)|\leq \epsilon$,这恰好是一致收敛的定义.

###### Example1

求证:$f_n(x)=\frac{x}{1+n^2x^2}$一致收敛.

当然处处收敛于$0$了,考虑用均值不等式,有$|f_n(x)|\leq \frac{|x|}{2n|x|}=\frac{1}{2n}$,这就搞定.

###### Example2

求证:$f_n(x)=x^n,x\in [0,1)$并不一致收敛.

当然处处收敛到$0$,但是$\sup f_n(x)=1$,所以并不一致收敛.

###### Example3 

取$f_n(x)=\frac{1}{n},g_n(x)=x,x\in (0,+\infty)$,容易见到$f_n\rightrightarrows 0,g_n\rightrightarrows x$.而且$f_n(x)$当然一致有界.然而,$f_ng_n\not \rightrightarrows 0$.

###### Example4

当$g\in C[0,1],g(1)=0$,求证:$f_n(x)=g(x)x^n\rightrightarrows 0$.

由于$g$在闭区间上连续,则其必然在闭区间上一致连续.因此$\forall \epsilon>0$,取$\delta\in (0,1)$,使得$x\in [\delta,1]$的时候,$|g(x)|<\epsilon$,而取$M=\sup_{[0,1]} |g|$,那么$|f_n(x)|\leq M\delta^n+\epsilon$,注意这里的$\delta$不取决于$n$的选取这就搞定.

###### Example5

设$f_n(x)=n\left(\sqrt{x+\frac{1}{n}}-\sqrt{x}\right),x\in(0,+\infty)$,判断其收敛性.

容易发现$f_n(x)=\frac{1}{\sqrt{x+\frac{1}{n}}+\sqrt x}\to \frac{1}{2\sqrt x}$,因此其收敛.问题在于判断其是否一致收敛.

观察到:

$$
|f_n(x)-\frac{1}{2\sqrt x}|\\
=\frac{\sqrt{x+\frac{1}{n}}-\sqrt x}{2\sqrt{x}\left(\sqrt{x+\frac{1}{n}}+\sqrt x\right)}\\
=\frac{1}{2n\sqrt x\left(\sqrt{x+\frac{1}{n}}+\sqrt x\right)^2}
$$

一个策略是直接对此式子求导,但也可以采取另一种分析策略,考虑:

$$
\sqrt{x+\frac{1}{n}}=\sqrt{x}\left(1+\frac{1}{nx}\right)^{\frac{1}{2}}\\
=\sqrt x\left(1+\frac{1}{2nx}+O(\frac{1}{n^2})\right)
$$

所以当$x$足够小的时候可能会出问题,直接取$x=\frac{1}{n}$,误差为:

$$
\frac{1}{2\sqrt n\left(\frac{1+\sqrt 2}{\sqrt n}\right)^2}=O(\sqrt n)
$$

这就证明了并非一致收敛.

###### Example6

$[0,1]$上的$f_n(x)$满足$\exists L>0,\forall x\ne y,|f_n(x)-f_n(y)|\leq L|x-y|$.并且$\forall x\in [0,1],f_n(x)\to f(x)$.求证:$f_n(x)\rightrightarrows f(x)$.也就是:一致李氏连续的收敛函数列一定一致收敛.

李氏连续意味着我们可以用控制自变量的方式控制因变量.回忆到定义在有限集上的收敛函数列一定是一致收敛的.因此我们选择在数轴上打足够密的点来控制.

具体地,$\forall \epsilon>0$,我们将$[0,1]$拆为长度不超过$\frac{\epsilon}{3L}$的小区间.假设分点是$x_1,\cdots,x_K$.这当然对于一个$\epsilon$来说是有限个点,我们可以让它们同时趋近于一致收敛,用柯西准则,也就是拿到一个$N$,使得$\forall n,m\geq N$,$|f_n(x_i)-f_m(x_i)|<\frac{\epsilon}{3}$.

现在$\forall x\in [0,1],\exists x_i,|x-x_i|<\frac{\epsilon}{3L}$.于是:

$$
|f_n(x)-f_m(x)|\\\leq |f_n(x)-f_n(x_i)|+|f_n(x_i)-f_m(x_i)|+|f_m(x_i)-f_m(x)|\\
<\epsilon
$$

这就搞定.



#### 函数项级数

函数项级数上也有柯西准则,也就是函数项级数的一致收敛性等价于$\forall \epsilon,\exists N,\forall n,m\geq N,|\sum_{k=n}^m f_k(x)|<\epsilon,\forall x\in I$.

还可以在这里定义**绝对收敛**:也就是$\sum |f_n(x)|$收敛.当然仍有绝对收敛推收敛.如果$\sum |f_n(x)|$还一致收敛,则称原级数**绝对一致收敛**.注意,这里的绝对一致收敛是定义在$|f_n|$的基础上.也就是说,一个函数既绝对收敛,又一致收敛,但它可能并不绝对一致收敛.然而反之是成立的,如果一个函数绝对一致收敛,可以推出它一致收敛,原因是$\sum |f_n|\geq |\sum f_n|$,用柯西准则就可以得知.

另外,用柯西准则可以见到如果$\sum f_n\rightrightarrows f$,那么$f_n(x)\rightrightarrows 0$.

###### Example1

考虑函数项级数$\sum (-x)^n(1-x),x\in [0,1]$.

直接考虑其余项,$R_n(x)=\sum_{k=n+1}^\infty (-x)^k(1-x)=\frac{(-x)^{n+1}(1-x)}{1+x}$.此时考虑:

$$
|R_n|\leq x^{n+1}(1-x)\\=\frac{1}{n+1}(n+1)(1-x)x^{n+1}\\\leq \frac{1}{n+1}(\frac{n+1}{n+2})^{n+2}\\\leq \frac{1}{n+1}\to 0
$$

所以该级数一致收敛.

那么是否绝对收敛呢?考虑:

$$
\sum_{k=1}^n |(-x)^n(1-x)|\\
=\sum_{k=1}^n (x^k-x^{k+1})\\
=x-x^{n+1}
$$

所以当然也绝对收敛到$\begin{cases}x&x\in [0,1)\\0&x=1\end{cases}$.

可是,它并不是绝对一致收敛的.考虑$\sup_{[0,1)}|S(x)-S_n(x)|=\sup_{[0,1)}|x^{n+1}|=1$.

###### Example2

求$\sum \left(\cos\frac{\pi x}{n}\right)^{n^3}$的收敛域.

不妨设$u_n(x)=\left(\cos\frac{\pi x}{n}\right)^{n^3}$,考虑:

$$
\sqrt[n]{u_n(x)}=\left(\cos\frac{\pi x}{n}\right)^{n^2}\\
=\left(1-\frac{\pi^2x^2}{2n^2}+O(\frac{1}{n^4})\right)^{n^2}\\
=\exp(n^2\ln\left(1-\frac{\pi^2x^2}{2n^2}+O(\frac{1}{n^4})\right))\\
=\exp(n^2\left(-\frac{\pi^2x^2}{2n^2}+O(\frac{1}{n^4})\right))\\
=\exp(-\frac{\pi^2 x^2}{2}+O(\frac{1}{n^2}))\\
\to e^{-\frac{\pi^2x^2}{2}}
$$

所以最后模拟成了一个等比数列,那收敛域就是$\mathbb{R}\setminus\{0\}$.

##### M判别法

若$|f_n(x)|\leq a_n,x\in I$,如果$\sum a_n<\infty$,则$\sum f_n(x)$绝对一致收敛.

证明的话,考虑用一般级数可以拿到$\forall \epsilon>0,\exists N,\forall n,m\geq N,\sum_{k=n}^m |a_n|<\epsilon$,但又有$\forall x\in I,\sum_{k=n}^m |f_n(x)|\leq \sum_{k=n}^m a_k$,这就知道其绝对一致收敛.

##### DA判别法

假设$\{a_n(x)\},\{b_n(x)\}$是两个函数列.对于任意固定的$x\in I$,$\{a_n(x)\}$是一个单调数列.注意这里其实不要求大家都递增或递减,只要对于每一个点都是单调的.而取$B_n(x)=\sum_{k=1}^n b_k(x)$.则当:

1. Dirichlet判别法:当$a_n(x)\rightrightarrows 0$,$B_n(x)$一致有界时,则$\sum a_nb_n$一致收敛.
2. Able判别法:当$\{a_n(x)\}$一致有界,$B_n(x)$一致收敛时,则$\sum a_nb_n$一致收敛.

证明和级数部分完全一样,只需要对于任意的$x$都这么搞即可:

$$
|\sum_{k=n+1}^{n+p}a_k(x)b_k(x)|\\
=|\sum_{k=n+1}^{n+p}a_k(x)(B_{k}(x)-B_{k-1}(x))|\\
=|a_{n+p}B_{n+p}(x)+\sum_{k=n+1}^{n+p-1}(a_k(x)-a_{k+1}(x))B_k(x)|\\
\leq M(a_{n+1}(x)+2a_{n+p}(x))
$$

其中$M=\sup_{1\leq i\leq p}\{|\sum_{k=n+1}^{n+i}b_k(x)|\}$.

###### Example1

当$b_n$单调下降趋近于$0$的时候,求证:$\sum_{k=1}^\infty b_n\sin(nx),x\in \mathbb{R}$一致收敛的充要条件是$b_n=o(\frac{1}{n})$.

先证必要性,设$S_{n,p}=\sum_{k=n}^p b_k\sin(kx)$.由柯西原理,$\forall \epsilon>0,\exists N,\forall n,p\geq N,|S_{n,p}|<\epsilon$.此时取$p=2n-1,x=\frac{\pi}{4n}$,此时当$n\leq k\leq p$的时候,$\sin (k\pi)>\sin \frac{\pi}{4}$,立刻有:

$$
\epsilon>|S_{n,p}(x)|\\
>nb_{2n}\sin\frac{\pi}{4}=\frac{\sqrt 2}{2}n b_{2n}
$$

这就搞定了$b_{2n}=o(\frac{1}{n})$,又因为$b_n$是单调下降的,那当然搞定了.

再证充分性,只要证明在$[0,\pi]$上一致收敛,由于$\sin(nx)$是奇函数,那么在$[-\pi,0]$上当然也一致收敛,那就搞定了.

设$\mu_n=\max_{m\geq n}{mb_m}$,注意到$\mu_n$单调下降趋近于$0$.下面开始讨论:

当$x\in [0,\frac{\pi}{p}]$的时候,此时$S_{n,p}$中的每一项都同号.自然有:

$$
\sum_{k=n}^p b_k \sin(kx)\leq \sum_{k=n}^p b_k kx\\
\leq \frac{\pi}{p}\sum_{k=n}^p b_k k\\
\leq \frac{\pi}{p}(p-n)\mu_n\\
\leq \pi\mu_n
$$

这就完事.

当$x\in [\frac{\pi}{n},\pi]$的时候,和差化积公式给出:

$$
|\sum_{k=1}^n \sin(kx)|\\
=\left|\frac{\cos(k-\frac{1}{2})x-\cos(k+\frac{1}{2})}{2\sin \frac{x}{2}}\right|\\
\leq \frac{1}{|\sin\frac{x}{2}|}\\
<n
$$

原因是当$\frac{x}{2}\leq \frac{\pi}{2}$的时候,$\sin \frac{x}{2}\geq \frac{2}{\pi}\frac{x}{2}\geq n$.

回忆到Abel变换给出$|S_{n,p}|\leq n(2b_n+b_p)<3nb_n\to 0$.这就搞定.

当$x\in (\frac{\pi}{p},\frac{\pi}{n})$的时候,取$m=\lfloor\frac{\pi}{x}\rfloor\leq \frac{\pi}{x}$.那我们就有:$\frac{\pi}{m+1}\leq x\leq \frac{\pi}{m}$.

当$k\in [n,m]$的时候,$kx\leq \pi$,那$\sin(kx)\leq kx$就成立,那么和上面一样:

$$
|S_{n,m}|\leq \pi \mu_n
$$

当$k\in[m+1,p]$的时候,此时$\frac{1}{|\sin \frac{x}{2}|}\geq \frac{1}{m+1}$,因此:

$$
|S_{m+1,p}|\leq (m+1)(2b_{m+1}+b_p)<3\mu_{m+1}
$$

则:

$$
|S_{n,p}|\leq |S_{n,m}|+|S_{m+1,p}|<\pi\mu_n+3\mu_{m+1}
$$

这就搞定.

#### 连续性

我们还可以定义**等度连续**的概念:我们说$f_n$等度连续,当$\forall \epsilon>0,\exists \delta>0$,使得$\forall x,y$,若$|x-y|<\delta$,则$\forall n,|f_n(x)-f_n(y)|<\epsilon$.其实就是某种一致(函数列)的一致(定义域上)连续.

我们可以证明:如果$f_n(x)\in C[a,b]$,而且其等度连续,则如果$\lim_{n\to \infty}f_n(x)=f(x),x\in [a,b]$,则$f_n\rightrightarrows f,x\in [a,b]$.

证明与前述Example6证明一致李氏连续类似,事实上Example6是这个的一个推论.我们取定$\delta$,使得$|x-y|<\delta$时一定有$\forall n,|f_n(x)-f_n(y)|<\frac{\epsilon}{3}$,此时再令$n\to \infty$,就有$|f(x)-f(y)|\leq \frac{\epsilon}{3}$,再取一个分划$\Delta :a=x_0<\cdots<x_m=b$并要求$\lambda (\Delta)<\delta$.

自然有:

$$
|f_n(x)-f(x)|\leq |f_n(x)-f_n(x_i)|+|f_n(x_i)-f(x_i)|+|f(x_i)-f(x)|\\
\leq \epsilon
$$

原因仍是有限点集上的收敛一定是一致收敛.

下面要探索的就是一致连续是否保持了原函数项级数的作用.

先看一个引理:当$x_0\in [a,b]$的时候,如若当$x\in [a,b]\setminus\{x_0\}$,$f_n(x)\rightrightarrows f(x)$,已知$\lim_{x\to x_0}f_n(x)=\alpha_n$存在.则$\lim_{n\to \infty}\alpha_n$存在而且$\lim_{x\to x_0}f(x)=\lim_{n\to \infty}\alpha_n$.换言之,$\lim_{x\to x_0}\lim_{n\to \infty}f_n(x)=\lim_{n\to \infty}\lim_{x\to x_0}f_n(x)$.

证明的话,首先要证明$\lim_{n\to\infty}\alpha_n$存在,考虑$\forall \epsilon>0,\exists N,\forall n,m\geq N$,有$|f_n(x)-f_m(x)|<\frac{\epsilon}{3}$,取极限知道$|\alpha_n-\alpha_m|\leq \frac{\epsilon}{3}$,这说明$\{\alpha_n\}$是柯西列,当然收敛.不妨设$\alpha=\lim_{n\to \infty}\alpha_n$.

此时有:

$$
|f(x)-\alpha|\leq |f(x)-f_N(x)|+|f_N(x)-\alpha_N|+|\alpha_N-\alpha|
$$

两边一取极限使得$x\to x_0$,立刻见到$|f(x)-\alpha|<\epsilon$.这已经能看出一致连续的强大了.正是因为一致连续可以先固定$N$,我们才可以在这里对$x\to x_0$取极限.所以这里有一个天然的可交换性.

这个引理的一个推论是:如果$f_n\rightrightarrows f,x\in [a,b]$,则对于$x_0\in [a,b]$,如果$f_n(x)$在$x_0$处连续,则$f$也在$x_0$处连续.原因是既然连续,则$\alpha_n=\lim_{x\to x_0}f_n(x)=f_n(x_0)$,所以$\lim_{x\to x_0}f(x)=\lim_{n\to \infty}f_n(x_0)=f(x_0)$.这就说明在$x_0$处连续.更有推论:如果$f_n\in C[a,b],f_n\rightrightarrows f$,则$f\in C[a,b]$.从而还可以见到,如果一串连续函数收敛到一个不连续函数,那这肯定也不是一致收敛的.

那么反过来,如果连续函数列收敛到连续函数,能不能推出一致收敛呢?我们事实上有**迪尼定理**:如果$f_n(x)\in C[a,b]$,当$n<m$的时候,$\forall x\in [a,b],f_n(x)\leq f_m(x)$,此时如若$f_n\rightarrow f$,而且$f\in C[a,b]$,则$f_n\rightrightarrows f$.也就是只要有一定的单调性,我们就可以说一致连续是连续函数列收敛到连续函数的充要条件.

取$R_n(x)=f(x)-f_n(x)\geq 0$,注意到$\lim_{n\to \infty}R(x)=0$而且$R_n(x)$连续且逐点随$n$单调递减,则必有$R_n\to 0$.如果我们能证明$R_n(x)\rightrightarrows 0$,那就万事大吉.不妨设$M_n=\max_{x\in [a,b]}R_n(x)=R_n(x_n)$,那么只要证明$M_n\to 0$就可以搞定一致收敛.然而$M_n$本身是单调下降的.反证,如果$\lim_{n\to \infty}M_n=c> 0$,此时观察$x_n\in [a,b]$,既然如此,$\{x_n\}$必定有收敛子列,任取一个收敛子列$\{x_{n_k}\}$并假设其收敛到$x_0$,接下来看:

$$
\lim_{m\to \infty}R_m(x_0)\\
=\lim_{m\to \infty}\lim_{x_{n_k}\to x_0}R_m(x_{n_k})\\
\geq \lim_{m\to \infty}\lim_{n_k>m,x_{n_k}\to x_0}R_{n_k}(x_{n_k})\\
=\lim_{x_{n_k}\to x_0}R_{n_k}(x_{n_k})>0
$$

可这就出事了.于是矛盾,反证成立.

迪尼定理的一个推论是如果$f_n(x)\geq 0$,那么此时$S_n(x)=\sum_k^n f(x)$就是逐点单调的了.因此我们可以看到此时$\sum f_n$一致收敛性也就等价于其连续性.

###### Example1

考虑$f_n\geq 0$,而且$\forall n,f_n$单增的时候,如若$\sum f_n(x)\leq M,\forall x\in [a,b)$.求证此时$\lim_{x\to b-0}\sum_{n=1}^\infty f_n(x)=\sum_{n=1}^\infty f_n(b-0)$.

设$S_n=\sum_k^n f_k(x)$,取定$m$,此时$\forall x\in [a,b)$,$S(x)\geq S_m(x)$.令$x\to b-0$,自然有$\sum_{n=1}^m f_n(b-0)\leq \lim_{x\to b-0}S(x)$.直接令$S(b-0)=\lim_{x\to b-0}S(x)$,这个极限存在,原因是$S(x)$是单调的.从而取极限得到$\sum_{n=1}^\infty f_n(b-0)\leq S(b-0)$.

接下来另一个方向,考虑:

$$
\sum_{n=1}^m f_n(x)\leq \sum_{n=1}^m f_n(b-0)
\leq \sum_{n=1}^\infty f_n(b-0)\\
S(x)\leq \sum_{n=1}^\infty f_n(b-0)\\
S(b-0)\leq \sum_{n=1}^\infty f_n(b-0)
$$

这就搞定.

###### Example2

$f_n$在$[a,b]$上可微而且$f_n'\in R[a,b]$,若$\forall x\in [a,b],\forall n,|\sum_{k=1}^n f_k'(x)|\leq M$,$\sum f_n(x)$在$[a,b]$上收敛.求证其在$[a,b]$上一致收敛.

容易观察到$\sum f_n(x)$一致李氏连续,用之前等度连续的结论就可以做掉.

#### 可积性

如果$f_n\in R[a,b],f_n\rightrightarrows f$,则我们声明$f\in R[a,b]$且$\int_a^b f{\rm d}x=\lim_{n\to \infty}\int_a^b f_n{\rm d}x$.

考虑证明,首先$f_n$是有界的,又因为一致收敛,所以能推出$f$也是有界的.

一个想法是使用勒贝格定理.直接取$K_n$是$f_n$的间断点集,立刻得到$K_n$是零测集.接下来令$K=\bigcup K_n$,那$K$还是零测集(可数个零测集的并),此时对于$\forall x\in [a,b]\setminus K$,$\forall n,f_n$都在$x$处连续,那$f$肯定也在$x$处连续,所以$f$的间断点集就一定被包含于零测集$K$,所以黎曼可积.

接下来的问题是是否积分相等,取$M_n=\sup_{[a,b]}|f-f_n|$,注意到:

$$
|\int_a^b f{\rm d}x-\int_a^b f_n{\rm d}x|\\
\leq \int_a^b|f-f_n|{\rm d}x\\
\leq \int_a^bM_n{\rm d}x\\
=M_n(b-a)\to 0
$$

这就搞定.然而注意到这里需要依赖于$b-a$的有界性.

另一个策略是直接用分划达布和证明$f\in R[a,b]$.不妨设$|f_n|\leq M,|f|\leq M$.则$\forall \epsilon>0,\exists n$,$|f_n(x)-f(x)|<\epsilon$.由于$f_n(x)\in R[a,b]$,可以取定分划$\Delta:a=x_0<\cdots<x_p=b$,使得$\sum(M_i-m_i)\Delta x_i<\epsilon$.

于此,我们取$\bar M_i=\sup_{[x_{i-1},x_i]} f(x),\bar m_i$同理.观察到$\forall x\in [x_{i-1},x_i],f(x)\leq f_n(x)+\epsilon\leq M_i+\epsilon$,所以一定有$\bar M_i\leq M_i+\epsilon$.对于$\bar m_i$同理.此时:

$$
\sum (\bar M_i-\bar m_i)\Delta x_i\\
\leq \sum(M_i-m_i)\Delta x_i+2\epsilon(b-a)\\
\leq \epsilon(1+2(b-a))
$$

这就搞定.

事实上这个结论可以更强,可以放弃一致收敛的条件.我们有**控制收敛定理**:假设$f_n\in R[a,b]$,而且它们一致有界,$\forall n,\forall x\in [a,b],|f_n|\leq M$.并且$f(x)=\lim_{n\to \infty}f_n(x)\in R[a,b]$,则$\int_a^b f(x){\rm d}x=\lim_{n\to \infty}\int_a^b f_n(x){\rm d}x$.

为证明此,先引入一些相关的定义:

定义**阶梯函数**为分段常值函数,假设$f\in R[a,b]$,则我们可以用阶梯函数逼近$f$,具体而言,假设$g$是阶梯函数,并且$g\leq f$,那么我们可以将$g$类似达布下和而分划够细,以使得$\int_a^b f{\rm d}x=\sup_{g\leq f,\text{g is a step function}}\{\int_a^b g(x){\rm d}x\}$.

定义**初等集**:有限个不交区间的并,可以见到其测度$m(E)$就是所有不交区间的长度之和.

定义**闭初等集**:有限个不交闭区间的并.

定义**有界闭初等集列**:$A_n$是闭初等集.容易见到,如果$A_n\ne \empty$,而且$A_{n+1}\subseteq A_n$,则$\bigcap_{n=1}^\infty A_n\ne \empty$,证明的话只需类似区间套定理,每个$A_n$内取一个点,这个点列是有界的,那它就一定有收敛子列,收敛到的那个点一定被含在每一个$A_n$中.这就搞定.

不妨设$A_n$是有界非空集列,满足$A_{n+1}\subseteq A_n,A_n\ne \empty,\bigcap_{n=1}^\infty A_n=\empty$.此时定义$\alpha_n=\sup\{m(E)\mid E\subseteq A_n,\text{E is a closed elementary set}\}$.我们下面证明$\lim_{n\to \infty}\alpha_n=0$.

首先显然$\alpha_n$单调递减.如若其不以$0$为极限,则一定有正下界$\alpha_n>\delta>0$.我们可以取$A_n$的闭初等子集$E_n\subseteq A_n$满足$m(E_n)>\alpha_n-\frac{\delta}{2^n}$.

接下来定义$H_n=\bigcap_{k=1}^n E_k$,这当然是初等集,此时见到$H_{n+1}\subseteq H_n$.接下来对$A_n\setminus H_n$的任意初等子集$E$,显然:

$$
E=E\setminus H_n\\
=E\setminus(\bigcap_{k=1}^n E_k)\\
=\bigcup_{k=1}^n (E\setminus E_k)
$$

而$E\subseteq A_n\subseteq A_k$,所以$E_k$和$E\setminus E_k$都是$A_k$的初等子集.所以:

$$
m(E\setminus E_k)+m(E_k)\leq \alpha_k\\
m(E\setminus E_k)\leq \frac{\delta }{2^k}
$$

所以$m(E)\leq \delta$.如果$H_n=\empty$,那么$m(E)=\alpha_n$,这就不符.所以$H_n\ne \empty$,所以$\bigcap_{n=1}^\infty H_n\ne \empty$,所以$\bigcap A_n\ne \empty$.矛盾,这就证明了原本的结论.

接下来考虑$f(x)-f_n(x)$,当没有一致连续的条件时,可能出现问题的地方在于有的地方两者不一定靠的足够近,想办法让这种地方所占的长度尽可能小就行.

对于$\forall \epsilon>0,A_n=\{x\in [a,b]\mid \exists j\geq n,|f_j(x)-f(x)|\geq \epsilon\}$.容易见到$A_{n+1}\subseteq A_n$,由于$f_n\to f$,所以$\bigcap_{n=1}^\infty A_n=\empty$.引理告诉我们$\alpha_n\to 0$.既然如此,存在$N$,使得$\forall n\geq N$,$A_n$的任意初等子集$E$都有$m(E)<\epsilon$.取$F=[a,b]\setminus E$.这其实更像是取$[a,b]$的一个分划,因此如果我们在上面取阶梯函数的话,完全可以让阶梯函数所需的那个分划和这个分划并起来,从而完全由上述结果取到阶梯函数,此外我们的$A_n\setminus E$的部分由于落在$F$里而需要被$\epsilon$控制.

此时取阶梯函数$S(x)$满足$0\leq S(x)\leq |f(x)-f_n(x)|$.这个时候看$S(x)$的积分:

$$
\int_a^b S(x){\rm d}x=\int_E S(x){\rm d}x+\int_F S(x){\rm d}x\\
\leq 2M\epsilon+\epsilon(b-a)
$$

这个对任意$S(x)$都成立,由于我们之前说的阶梯函数的上界可以逼近原函数,所以这就意味着:

$$
|\int_a^bf_n(x)-f(x){\rm d}x|\\
\leq \int_a^b|f_n(x)-f(x)|{\rm d}x\\
\leq 2M\epsilon+\epsilon(b-a)
$$

这就搞定.不过细心的读者也许会注意到一些问题,首先是这里用内测度逼近疑似有点神秘,然而如果用勒贝格测度的定义直接跑其实一眼就能看出来上面的结论.

还有另一个结论:假设$f_n\in R[a,b],|f_n|\leq M$,已知$\lim_{n\to \infty} f_n(x)=f(x)$存在(注意这里并不要求$f(x)\in R[a,b]$),我们就可以证明$\lim_{n\to \infty}\int_a^b f_n(x){\rm d}x$存在.

考虑$|\int_a^b f_n{\rm d}x|\leq M(b-a)$有界,所以其可以取上下极限操作.令$\overline F=\varlimsup_{n\to \infty}\int_a^b f_n{\rm d}x,\underline F=\varliminf_{n\to \infty}\int_a^b f_n{\rm d}x$.既然如此,就一定存在两列$\{n_k\}$和$\{m_k\}$,使得$\overline F=\lim_{k\to \infty}\int_a^b f_{n_k}{\rm d}x,\underline F=\lim_{k\to \infty}\int_a^b f_{m_k}{\rm d}x$.然而:

$$
\overline F-\underline F=\lim_{k\to \infty}\int_a^b(f_{n_k}-f_{m_k}){\rm d}x\\
=\int_a^b\lim_{k\to \infty}(f_{n_k}-f_{m_k}){\rm d}x
=0
$$

这你就可以见到,我们其实可以把这个极限干脆定义成$f$的积分.这实际上就是勒贝格积分.

###### Example1

求证:$\int_0^1 \frac{1}{x^x}{\rm d}x=\sum_{n=1}^\infty\frac{1}{n^n}$.

展开:

$$
\frac{1}{x^x}=e^{-x\ln x}\\
=\sum_{n\geq 0}\frac{(-x\ln x)^n}{n!}
$$

容易证明这个函数项级数是一致收敛的,那我们就可以逐项积分,而且积分内可以做多少次分部积分都无所谓(反正是有限逼近无限).

那么:

$$
\int_0^1 \frac{1}{x^x}{\rm d}x\\
=\int_0^1\sum_{n\geq 0}\frac{(-x\ln x)^n}{n!}{\rm d}x\\
=\sum_{n\geq 0}\int_0^1\frac{(-x\ln x)^n}{n!}{\rm d}x
$$

令$x=e^{-t}$,换元:

$$
\sum_{n\geq 0}\int_0^1\frac{(-x\ln x)^n}{n!}{\rm d}x\\
=-\sum_{n\geq 0}\frac{1}{n!}\int_0^{+\infty}e^{-(n+1)t}t^n{\rm d}t
$$

而考虑:

$$
\int_0^{+\infty}e^{-(n+1)t}t^n{\rm d}t\\
=\frac{-1}{n+1}\int_0^{+\infty}t^n{\rm d}e^{-(n+1)t}\\
=\frac{-1}{n+1}\left(e^{-(n+1)t}t^{n}|_0^{+\infty}-n\int_0^{+\infty}e^{-(n+1)t}t^{n-1}\right)\\
=\frac{n}{n+1}\int_0^{+\infty}e^{-(n+1)t}t^{n-1}\\
=\cdots\\
=\frac{n!}{(n+1)^{n}}\int_0^{+\infty}e^{-(n+1)t}{\rm d}t\\
=-\frac{n!}{(n+1)^{n+1}}
$$

和上面结合一下就证毕.

###### Example2

设函数列$\{f_n(x)\}$任意内闭可积,并且$\{f_n(x)\}$在$\mathbb{R}$上一致收敛到$f(x)$,再设存在函数$g(x)$满足:

$$
|f_n(x)|\leq g(x)
$$

并且$\int_{-\infty}^{+\infty}g(x){\rm d}x<+\infty$.求证下列积分收敛而且:

$$
\int_{-\infty}^{+\infty}f(x){\rm d}x=\lim_{n\to \infty}\int_{-\infty}^{+\infty}f_n(x){\rm d}x
$$

首先要证明$\int_{-\infty}^{+\infty}f(x){\rm d}x$收敛.只需观察到$|f_n|\leq g$两遍取极限就可以拿到$|f|\leq g$,所以绝对收敛.

接下来考虑如何证明相等,这里的问题在于虽然$f(x)\rightrightarrows f$,但是由于区间长度无限长,因此这个控制会被放大.于此我们的策略是考虑将区间分成三段:$[-\infty,-A],[-A,A],[A,+\infty]$,对于中间的部分用一致收敛控制住,对于前后的部分用柯西准则控制住.

具体而言,$|f(x)-f_n(x)|\leq 2g(x)$取足够大的$A$使得$\int_{A}^{+\infty}g(x){\rm d}x\leq \frac{\epsilon}{6}$以及$\int_{-\infty}^{-A}g(x){\rm d}x\leq \frac{\epsilon}{6}$,接下来考虑取足够大的$n$使得在$[-A,A]$上,$|f_n(x)-f(x)|<\frac{\epsilon}{6A}$:

$$
|\int_{-\infty}^{+\infty}f_n(x)-f(x){\rm d}x|\\
\leq |\int_{-A}^{+A}f_n(x)-f(x){\rm d}x|+|\int_{-\infty}^{-A}f_n(x)-f(x){\rm d}x|+|\int_{+A}^{+\infty}f_n(x)-f(x){\rm d}x|\\
\leq 2A\frac{\epsilon}{6A}+\frac{\epsilon}{6}\times 4\\
=\epsilon
$$

这就搞定了.

#### 可微性

先看一个平凡的命题:假设$f_n\in C^1[a,b],x_0\in [a,b]$,已知$\{f_n(x_0)\}$收敛,而且$f_n'\rightrightarrows g$.则$f_n\rightrightarrows f$,其中$f\in C^1[a,b]$,并且恰好满足$f'=g$.

考虑$g(x)=\lim_{n\to \infty}f_n'(x),x\in [a,b]$当然能得到$g(x)$是连续函数.

接下来考虑NL公式给出:

$$
f_n(x)=f_n(x_0)+\int_{x_0}^x f'_n(t){\rm d}t\\
\lim_{n\to \infty}f_n(x)=\lim_{n\to \infty}f_n(x_0)+\lim_{n\to \infty}\int_{x_0}^x f'_n(t){\rm d}t\\
\lim_{n\to \infty}f_n(x)=\lim_{n\to \infty}f_n(x_0)+\int_{x_0}^x\lim_{n\to \infty} f'_n(t){\rm d}t\\
f(x)=\lim_{n\to \infty}f_n(x_0)+\int_{x_0}^xg(t){\rm d}t\\
$$

这就见到$f_n\to f,f\in C^1[a,b]$而且$f'=g$.现在的问题是能否推出一致收敛$f_n\rightrightarrows f$.有:

$$
|f_n(x)-f(x)|\\
=|f_n(x_0)+\int_{x_0}^xf_n'(t){\rm d}t-f(x_0)-\int_{x_0}^xg(t)|\\
\leq |f_n(x_0)-f(x_0)|+|\int_{x_0}^xf_n'(t){\rm d}t-\int_{x_0}^xg(t)|\\
\leq|f_n(x_0)-f(x_0)|+(b-a)\sup|f_n'-g|
$$

这就完事.

上述结果能不能加强呢?事实上有一个更一般的结果:假设对于一串连续函数$f_n(x)\in C[a,b]$,已知$\exists x_0\in [a,b]$,$\lim_{n\to \infty}f_n(x_0)$存在.若对于$\forall \epsilon>0$,$\exists N>0,\forall n,m\geq N$,函数$g(x)=f_n(x)-f_m(x)$的李氏常数$<\epsilon$.则我们可以推出$\{f_n(x)\}$一致收敛,设$f_n(x)\rightrightarrows f(x)$,若对于某个$x\in [a,b]$,如果$f_n'(x)$恒存在,而且$\lim_{n\to \infty} f_n'(x)$存在,则$f'(x)$存在而且$f'(x)=\lim_{n\to \infty} f_n'(x)$.

这个结论的正确性比较直观,就是用李氏连续的性质限制整个函数的波动.这个结论当然要严格强于原本的结论,因为闭区间上的可导函数自然是李氏连续函数,如若$f_n'(x)$一致连续,当然能推出上述的李氏连续性质,立刻得到原本结论.

下面考虑证明:$\forall \epsilon>0,\exists N,\forall n,m\geq N$,都有$|f_n(x_0)-f_m(x_0)|<\frac{\epsilon}{2}$,并且$f_n(x)-f_m(x)$的李氏常数$<\frac{\epsilon}{2(b-a)}$.注意到此时:

$$
|f_n(x)-f_m(x)|\\
\leq |f_n(x)-f_m(x)-(f_n(x_0)-f_m(x_0))|+|f_n(x_0)-f_m(x_0)|\\
< \frac{\epsilon}{2(b-a)}|x-x_0|+\frac{\epsilon}{2}\leq \epsilon
$$

这就证明了$f_n$一致收敛,不妨设$f_n\rightrightarrows f$.

接下来对于某个$x\in [a,b]$,定义$\varphi_n(t)=\frac{f_n(t)-f_n(x)}{t-x},t\in [a,b]\setminus \{x\}$,并补定义$\varphi(t)=\frac{f(t)-f(x)}{t-x},t\in [a,b]\setminus \{x\}$.显然$\varphi_n(t)\in C[a,b]\setminus \{x\}$

首先注意到:

$$
|\varphi_n(t)-\varphi_m(t)|\\
=\frac{1}{|t-x|}|f_n(t)-f_m(t)-(f_n(x)-f_m(x))|\\
\leq \frac{1}{|t-x|}\frac{\epsilon}{2(b-a)}|t-x|\\
=\frac{\epsilon}{2(b-a)}
$$

于是$\varphi_n(t)$一致收敛,而且容易见到$\varphi_n\rightrightarrows \varphi$.从而:

$$
f'(x)=\lim_{t\to x}\varphi(t)\\
=\lim_{t\to x}\lim_{n\to \infty}\varphi_n(t)\\
=\lim_{n\to \infty}\lim_{t\to x}\varphi_n(t)\\
=\lim_{n\to \infty}f_n'(x)
$$

这就搞定了.

容易发现上述的推论是可导函数列的收敛性质(并不需要$C^1$).而且我们还可以推出一个推论:那就是导函数一致收敛到的函数也是导函数.

###### Example1

假设$\{q_n\}$是$[0,1]$中的全体有理数排成的数列.定义$f(x)=\sum_{n=1}^\infty \frac{|x-q_n|}{3^n},x\in [0,1]$.

首先用大M判别法,注意到$\frac{|x-q_n|}{3^n}\leq \frac{1}{3^n}$,而每一项都是连续的,所以总的$f(x)$是连续函数,而且这个定义也是收敛的.而且对于$x_0\in [0,1]\setminus \mathbb{Q}$,$f_n'(x_0)$当然都是存在的,而且见到$f_n'(x_0)=\sum_{k=1}^n\frac{\text{sgn}(x_0-q_k)}{3^k}$,并且这个导数还足够小.因此立刻见到当$n,m$足够大的时候,立刻见到$f_n-f_m$的李氏常数可以尽可能小.

于是套用上面的结论可以知道$f(x)$在$[0,1]\setminus \mathbb{Q}$处可导.那么其在$\mathbb{Q}\cap [0,1]$上可导么?如若$x\in \mathbb{Q}$,不妨设$x=q_n$,则考虑$f(x)=\frac{|x-q_n|}{3^n}+\sum_{k\ne n}\frac{|x-q_k|}{3^k}$,前半部分在$q_n$处不可导可后半部分在$q_n$处可导,于是总和当然不可导.

###### Example2

设$f(x)=\sum_{n=1}^\infty\frac{\sin (nx)}{n^3}$,求其导函数.

观察到$\sum_{n=1}^\infty|(\frac{\sin (nx)}{n^3})'|\leq \sum \frac{1}{n^2}<\infty$,所以其导函数也绝对一致收敛.立刻能推出$f\in C^1(-\infty,+\infty)$,而且导函数就是逐项求导的$f'(x)=\sum_{n=1}^\infty\frac{\cos(nx)}{n^2}$.

#### 部分应用

##### Borel引理

对任何一个实数列$\{a_n\}$,都存在一个函数$f(x)\in C^{\infty}(\mathbb{R})$,满足$f^{(n)}(0)=a_n$.

考虑造一个$g(x)=\begin{cases}1&0\leq x\leq \frac{1}{2}\\0&x\geq 1\end{cases}$,其中中间少了一段用磨光核连起来.考虑造一个单调递增无界正数列$\{\lambda_n\}$,并取:

$$
f(x)=\sum_{n=0}^\infty\frac{a_nx^n}{n!}g(\lambda_n|x|)
$$

这个的好处是什么呢?我们发现当$\lambda_n|x|\geq 1$,$|x|\geq \frac{1}{\lambda_n}$的时候$g(\lambda_n|x|)=0$,又因为$\lambda_n$单调递增趋近于无穷,所以除了$0$以外的所有点上都是有限和,因此全都收敛而且全都可导,所以$f\in C^{\infty}(\mathbb{R}\setminus \{0\})$.

考虑$f_n(x)=\frac{a_nx^n}{n!}g(\lambda_n|x|)$,这个当然是$C^{\infty}$的.并且当然有$f_n^{(n)}(0)=a_n$.现在如果我们能搞定这个导数级数的收敛性就搞定了$0$处的可导性了对吧.取$M_k=\max_{0\leq l\leq k}|g^{(l)}|$,当$k\leq n$的时候考虑:

$$
|f_n^{(k)}(x)|\leq \frac{|a_n|}{n!}\sum_{m=0}^k\left|\binom{k}{m}n^{\underline{k-m}}x^{n-k+m}g^{(m)}(\lambda_n|x|)\lambda_n^m\right|\\
\leq \frac{|a_n|}{n!}\sum_{m=0}^k\left|\binom{k}{m}n^{\underline{k-m}}x^{n-k+m}M_k\lambda_n^m\right|\\
$$

注意这里的$x$一定很小,$|x|<\frac{1}{\lambda_n}$的时候上式才有意义,开始放缩:

$$
|f_n^{(k)}(x)|\leq \frac{|a_n|}{n!}\sum_{m=0}^k\left|\frac{k!}{m!(k-m)!}n^{\underline{k-m}}M_k\frac{1}{\lambda_n^{n-k}}\right|\\
\leq \frac{|a_n|}{n!}\sum_{m=0}^k\left|\frac{k!}{m!(k-m)!}n^{\underline{k-m}}M_k\frac{1}{\lambda_n^{n-k}}\right|\\
=\frac{|a_n|}{n!}\sum_{m=0}^k\left|\frac{k!}{m!(k-m)!}\frac{n!}{(n-k+m)!}M_k\frac{1}{\lambda_n^{n-k}}\right|\\
\leq |a_n|M_k\frac{1}{\lambda_n^{n-k}}k!
$$

显然只需取足够小的$\lambda_n$就用大M判别法控制住了.

##### 维尔斯特拉斯函数

考虑造一个处处连续处处不可导的函数,首先它应该在任何一个区间上都没有单调性(因为单调函数几乎处处可导).

首先造一个连续的$2$周期函数$h(x)=\begin{cases}x&0\leq x\leq 1\\2-x&1\leq x\leq 2\\h(x-2)&otherwise\end{cases}$.或者我们干脆定义$h(x)=\sin(\frac{\pi}{2}4^nx)$,这样这个函数更光滑一点,但对论证几乎没有区别.

考虑造$f(x)=\sum_{n=0}^{\infty}(\frac{3}{4})^n h(4^nx)$,注意到:

$$
f(x)\leq \sum(\frac{3}{4})^n<\infty
$$

根据大M判别法,所以这个函数的确绝对一致收敛.那这个函数就确实继承了连续性.

首先有引理:如果$f'(x_0)$存在,而且$\alpha_n\leq x_0\leq \beta_n,0<\beta_n-\alpha_n\to 0$,则$\lim_{n\to \infty}\frac{f(\beta_n)-f(\alpha_n)}{\beta_n-\alpha_n}\to f'(x_0)$.取$\lambda_n=\frac{\beta_n-x_0}{\beta_n-\alpha_n}$,容易见到$0\leq \lambda_n\leq 1$,见到:

$$
\frac{f(\beta_n)-f(\alpha_n)}{\beta_n-\alpha_n}-f'(x_0)\\
=\lambda_n(\frac{f(\beta_n)-f(x_0)}{\beta_n-x_0}-f'(x_0))+(1-\lambda_n)(\frac{\alpha_n-f(x_0)}{\alpha_n-x_0}-f'(x_0))
$$

容易见到趋近于$0$.不过要简单判断一下$\alpha_n=x_0$的情况.

既然如此,$\forall x\in \mathbb{R}$,考虑定义$\alpha_m=\frac{\lfloor4^m x\rfloor}{4^m},\beta_m=\alpha_m+\frac{1}{4^m}$,容易见到$\alpha_m\leq x< \beta_m$.并且当$n\leq m$的时候，$(4^n\alpha_m,4^n\beta_m)$上不存在整数.

见到:

$$
4^n(\beta_m-\alpha_m)=\begin{cases}\in 4\mathbb{N}&n>m\\1&n=m\\<1&otherwise\end{cases}
$$

于是:

$$
|h(4^n \beta_m)-h(4^n\alpha_m)|=\begin{cases}0&n>m\\ |4^n\beta_m-4^n\alpha_m|=4^{n-m}&n\leq m\end{cases}
$$

把$n<m$的地方往负了放,自然有:

$$
|f(\beta_m)-f(\alpha_m)|=\sum_{n=0}^m(\frac{3}{4})^n(h(4^n\beta_m)-h(4^n\alpha_m))\\
\geq (\frac{3}{4})^m-\sum_{n=0}^{m-1}(\frac{3}{4})^n4^{n-m}\\
=(\frac{3}{4})^m-\frac{1}{4^m}\frac{3^m-1}{2}\\
=\frac{1}{4^m}\frac{3^m+1}{2}
$$

所以:

$$
\frac{|f(\beta_m)-f(\alpha_m)|}{\beta_m-\alpha_m}\geq \frac{3^m+1}{2}
$$

然而如若这点导数存在,那么这里的导数就是$\lim_{m\to \infty}\frac{3^m+1}{2}=\infty$,这就矛盾了.因此这个函数处处不可导.

##### 连续曲线充满平面

存在充满$[0,1]\times [0,1]$的连续曲线.连续曲线在这里定义为$(x(t),y(t)),t\in [0,1]$满足$x(t),y(t)\in C[0,1]$,任意$(a,b)\in [0,1]\times [0,1]$,都存在$t$使得$a=x(t),b=y(t)$.

搞一个连续函数$g(t)$满足:

$$
g(t)=\begin{cases}
0&t\in [0,\frac{1}{3}]\cup [\frac{5}{3},2]\\
3t-1&t\in [\frac{1}{3},\frac{2}{3}]\\
1&t\in [\frac{2}{3},\frac{4}{3}]\\
3t+5&t\in [\frac{4}{3},\frac{5}{3}]\\
\end{cases}
$$

并且满足$g(t+2)=g(t)$,也就是一个周期梯形函数.

接下来定义:

$$
x(t)=\sum_{n=1}^\infty\frac{1}{2^n}g(3^{2n-2}t)\\
y(t)=\sum_{n=1}^\infty\frac{1}{2^n}g(3^{2n-1}t)
$$

容易发现$0\leq x(t),y(t)\leq \sum_{n=1}^\infty \frac{1}{2^n}=1$,所以的确绝对一致收敛,继承了连续性而且确实在$[0,1]\times [0,1]$中.

接下来$\forall (a,b)\in [0,1]\times [0,1]$,我们尝试找到一个$t$使得$a=x(t),b=y(t)$.考虑取二者的二进制表示$a=\sum_{n=1}^\infty\frac{a_n}{2^n},b=\sum_{n=1}^\infty\frac{b_n}{2^n}$,其中$a_n,b_n\in \{0,1\}$.取一个新的数列$c_{2n-1}=a_n,c_{2n}=b_n$.取$0\leq c=2\sum_{n=1}^\infty \frac{c_n}{3^n}\leq 1$,下面我们证明$x(c)=a,x(c)=b$.

考虑$3^kc=2\sum_{n=1}^k\frac{c_n}{3^{n-k}}+2\sum_{n=k+1}^\infty\frac{c_n}{3^{n-k}}$.其中前半部分是个偶整数,因此在带入函数$g$后会消失,观察后半部分,容易见到:如果$c_{k+1}=0$,则后半部分$\leq \frac{1}{3}$;如果$c_{k+1}=1$,则后半部分$\geq \frac{2}{3}$,因此$g(3^kc)=c_{k+1}$.带入原式定义就看到$x(c)=a,y(c)=b$.

甚至说,这里的$c$其实就在康托尔三分集中,所以事实上这个曲线上的零测集就足以覆盖整个平面.

#### 幂级数

我们把形如$f(x)=\sum_{n=0}^{\infty}a_n(x-x_0)^n$的函数项级数称作**幂级数**,由于可以做自变量平移,不妨假设$x_0=0$而$f(x)=\sum_{n=0}^{\infty}a_nx^n$.

一个最先遇到的问题当然是幂级数的收敛范围,首先$x=0$肯定收敛.其次我们可以发现如果$f(r)$收敛,那么$\forall x\in \mathbb{R},|x|<|r|$都有$f(x)$收敛,原因是:

$$
\sum |a_nx^n|=\sum |a_nr^n|\sdot|\frac{x}{r}|^n
$$

由于$\sum a_nr^n$收敛,所以其通项$\lim_{n\to \infty}a_nr^n=0$,那至少是有界的,而$|\frac{x}{r}|^n$又是个无穷递降等比数列,那就肯定收敛.既然如此,取$R=\sup\{|x|,x\in \mathbb{R}|f(x)\text{ is converge}\}$.由上面的结论我们知道$\forall x\in (-R,R)$都有$f(x)$收敛,另外$\forall x,|x|>R$都有$f(x)$发散.既然如此,我们将$(-R,R)$称作幂级数的**收敛区间**,容易见到收敛域和收敛区间最多只可能差两个点,换言之设收敛域为$E$,则自然有$(-R,R)\subseteq E\subseteq [-R,R]$.

取$\rho=\varlimsup_{n\to +\infty}\sqrt[n]{|a_n|}$,则$R=\rho^{-1}$(这里要做一些约定:不妨设$\inf^{-1}=0$和$0^{-1}=\inf$).原因是$\varlimsup_{n\to +\infty}\sqrt[n]{|a_nx^n|}=|x|\rho$,用根值判别法就显然了.

这里还有一个更弱的比值判别法的版本:如若$a_n\ne 0$,$\rho=\lim_{n\to \infty}\left|\frac{a_{n+1}}{a_n}\right|$存在,则$R=\rho^{-1}$.原因是Stolz定理给出:

$$
\varliminf \frac{|a_{n+1}|}{|a_n|}\leq \varliminf \sqrt[n]{|a_n|}\leq \varlimsup \sqrt[n]{|a_n|}\leq \varlimsup \frac{|a_{n+1}|}{|a_n|}
$$

下面我们证明,在收敛区间的内部$(-R,R)$内,$\sum a_nx^n$是内闭绝对一致收敛的.而如果其在端点处也收敛,例如在$x=R$时收敛的话,则其在$[0,R]$上一致收敛.

前者的话,只需用大M判别法即可,任取$[-r,r]\subseteq (-R,R)$然后看$|a_nx^n|\leq |a_nr^n|$.

而对于后者的话,考虑$\sum a_nx^n=\sum a_nR^n(\frac{x}{R})^n$,而$(\frac{x}{R})^n$是单调一致有界,而$\sum a_nR^n$收敛,用DA判法则搞定.

上述结论的一个显然推论是幂级数是收敛域上的连续函数(因为在收敛域上都是一致连续的).

###### Example1

假设$R=1$,既然有一致连续性,若$\sum a_n$收敛,则极限存在有$\lim_{x\to 1-0}f(x)=f(1)=\sum a_n$.然而这个结论反过来不成立,例如$f(x)=\sum (-x)^n=\frac{1}{1+x}$,容易见到极限为$\frac{1}{2}$可是$f(1)$并不收敛.

上述结论能不能加点条件使得反之成立呢?我们下面证明若$a_n=o(\frac{1}{n})$,若$\lim_{x\to 1-0}f(x)=S$存在,则$\sum a_n=S$.

考虑取$\sigma_n=\frac{1}{n}\sum_{k=0}^nk|a_k|$,由Stolz定理知道$\lim \sigma_n=\lim n|a_n|=0$.取$S_n=\sum_{k=1}^na_k$,我们要证明的就是当$n\to \inf$的时候$|S-S_n|$尽可能小.当$x\in (-1,1)$的时候,注意到:

$$
S_n-S=f(x)-S-\sum_{k=n+1}^\infty a_kx^k+\sum_{k=0}^na_k(1-x^k)\\
=f(x)-S-\sum_{k=n+1}^\infty a_kx^k+\sum_{k=0}^na_k(1-x)(1+x+\cdots+x^{k-1})\\
\leq f(x)-S-\sum_{k=n+1}^\infty a_kx^k+(1-x)\sum_{k=0}^nka_k\\
$$

$\forall \epsilon>0$,取足够大的$n$使得$\sigma_n<\frac{\epsilon}{3}$而且使得$|na_n|<\frac{\epsilon}{3}$总成立,那么:

$$
|S_n-S|\leq |f(x)-S|+(1-x)n\sigma_n+\frac{\epsilon}{3n}\frac{x^{n+1} }{1-x}
$$

取$x=1-\frac{1}{n}$,即可得证.

如果加点单调性也可以.设$a_n>0$,如果$f(x)=\sum_n a_nx^n$满足$f(1-)$存在,则$\sum a_n=f(1-)$.

这个结论与之前的一致收敛那里的单调性质完全相似.首先由$\sum_n a_n\leq f(1-)$得到这个级数收敛,然后又可以放到$\sum a_nx^n\leq \sum_n a_n$,让左边趋近一下得到$f(1-)\leq \sum_n a_n$.

##### 柯西和

设$\sum a_n=A,\sum b_n=B$,设$c_n=\sum_{k=0}^n a_{n-k}b_k$,我们下面证明若$\sum c_n$收敛,则$\sum c_n=AB$.

考虑设$f(x)=\sum a_nx^n,g(x)=\sum b_nx^n$,则其收敛半径肯定$\geq 1$,既然如此在任何内闭区间$[-r,r]\subseteq (-1,1)$,$f,g$都在$[-r,r]$上绝对一致收敛,既然如此我们就可以把这个乘积随便打乱顺序,得到$f(x)g(x)=\sum c_nx^n,\forall x\in [-r,r]$.而我们又有$\sum c_n$收敛,既然如此$\sum c_n=h(1)=\lim_{x\to 1-0}h(x)=(\lim_{x\to 1-0}f(x))(\lim_{x\to 1-0}g(x))=AB$,则搞定.

其中$\sum c_n$收敛这个条件是必须的,如果去掉后结论不成立,有如下反例:

设$a_n=b_n$考虑对$(1+x)^{-\frac{1}{2} }$做泰勒展开,得到$f(x)=g(x)=\sum a_n x^n=\frac{1}{\sqrt{1+x} }$此时$f(1),g(1)$都收敛,原因是$a_n=(-1)^n\frac{(2n-1)!!}{2^nn!}$,用DA判法知道其收敛.然而$f(x)g(x)=\frac{1}{1+x}$,$c_n=(-1)^n$,因此$\sum c_n$不收敛,这就完蛋了.

##### 逐项微积分

既然幂级数在收敛域上一致收敛,那当然可以逐项积分.而且注意到$\varlimsup \sqrt[n+1]{\frac{|a_n|}{n+1} }=\varlimsup \sqrt[n]{|a_n| }$,原因是$\lim \sqrt[n]{n}=1$,因此收敛半径不变,然而积分后,端点处有可能被包含入收敛域,原因是DA判法,原本的$a_nx^n$之和如果有界,配了一个多的下降$\frac{1}{n}$就会收敛,例如$\sum \frac{x^n}{n}$.

同理,对于逐项求导也可以发现收敛半径不变.所以幂级数可以在收敛域上逐项积分任意多次,也可以在收敛区间内逐项求导任意多次.

###### Example1

求$\sum_{n=0}^\infty\frac{(-1)^n}{3n+1}$.

考虑:

$$
f(x)=\sum_{n=0}^\infty\frac{(-1)^n}{3n+1}x^{3n+1}\\
=\sum_{n=0}^\infty\int_0^x(-1)^nt^{3n}{\rm d}t\\
$$

我们想让这里的求和和积分交换顺序对吧!那首先得规定$x\in (-1,1)$而取不到$1$,有:

$$
f(x)=\sum_{n=0}^\infty\int_0^x(-1)^nt^{3n}{\rm d}t\\
=\int_0^x\sum_{n=0}^\infty (-t)^{3n}{\rm d}t\\
=\int_0^x\frac{ {\rm d}t}{1+t^3}
$$

后面那个可以积出来,而且$f(x)$在$(-1,1)$上取值就是如此.而发现$f(x)$在$1$处收敛,因此拿这个积出来的结果极限逼近一下就行.

##### 泰勒级数

对于一个$f\in C^\infty$,考虑泰勒展开有:

$$
f(x)=\sum_{k=0}^n\frac{f^{(k)}(x_0)}{k!}(x-x_0)^k+r_n(x)
$$

其中依照拉格朗日余项,$r_n(x)=\frac{f^{(n+1)}(\xi)}{(n+1)!}(x-x_0)^{n+1}$,若存在$\delta>0$,当$x\in (x_0-\delta,x_0+\delta)$的时候,$\lim_{n\to \infty}r_n=0$,则称该函数在$x_0$处解析,也就是可以用泰勒级数逼近它.

我们有一个命题:如果$f(x)$在$x_0$处解析,则其在$x_0$附近的一个小邻域上解析(证明可以参考Example15).因此解析的概念以拓宽到区间上,如果其在一个开区间$I$上解析,则称其$\in C^\omega(I)$.

而且我们还可以见到,如果$f^{(n)}(x)$一致有界(或者至少与$n$无关),则由于$(x-x_0)^n$会被$\frac{1}{(n+1)!}$控制住,所以其在$\mathbb{R}$上都是解析的.所以$\sin x,\cos x$都是解析的,另外$e^x,\ln x$的导数大小与$n$无关,所以也解析.

当$f,g$在$x_0$处解析的时候,应当有$f\pm g,fg,\frac{f}{g}$都是解析的.而且如果$f$在$g(x_0)$处解析,则$f(g(x))$也在$x_0$处解析.这个太难算了,我们承认这个事实好了,反正就是硬算.

###### Example1(广义二项式定理)

求$f(x)=(1+x)^\alpha$在$0$处的解析性.

也就是要证明$f(x)=(1+x)^\alpha=\sum_{n=0}^\infty\frac{\alpha^{\underline n} }{n!}x^n$.

用比值法,观察到:

$$
|\frac{\binom{\alpha}{n}}{\binom{\alpha}{n+1}}|\\
=|\frac{n+1}{\alpha-n}|\to 1
$$

所以右端收敛半径为$1$.既然如此先搞定右端那个幂级数,设$g(x)=\sum_{n=0}^\infty\binom{\alpha}{n}x^n,x\in(-1,1)$.观察到:

$$
g'(x)=\sum_{n=1}^\infty n\binom{\alpha}{n}x^{n-1}\\
(1+x)g'(x)=\sum_{n=1}^\infty n\binom{\alpha}{n}x^{n-1}+\sum_{n=1}^\infty n\binom{\alpha}{n}x^{n}\\
=\alpha+\sum_{n=1}^\infty\left(n\binom{\alpha}{n}+(n+1)\binom{\alpha}{n+1}\right)x^n\\
=\alpha+\alpha\sum_{n=1}^\infty\binom{\alpha}{n}x^n\\
=\alpha g
$$

此时见到:

$$
\left(\frac{g}{(1+x)^\alpha}\right)'\\
=-\alpha(1+x)^{-\alpha-1}g+(1+x)^{-\alpha}g'\\
=(1+x)^{-\alpha-1}(-\alpha g+(1+x)g')\equiv 0
$$

所以$\frac{g}{(1+x)^\alpha}$是一常数,演算$x=0$的部分就可以知道当$x\in (-1,1)$的时候它俩相等.

最后再补一下端点,

当$\alpha>0,x=\pm 1$的时候,下面直接证明它绝对收敛,也就是证明$\sum |\binom{\alpha}{n}|<\inf$.,考虑拉贝判法:

$$
n\left(\frac{|\binom{\alpha}{n}|}{|\binom{\alpha}{n+1}|}-1\right)\\
=n(\frac{n+1}{n-\alpha}-1)\to 1+\alpha>0
$$

所以此时收敛域是$[-1,1]$.

当$-1<\alpha<0$的时候,估一下发现:

$$
|\binom{\alpha}{n}|=|(1-\frac{\alpha+1}{1})\cdots(1-\frac{\alpha+1}{n})|\\
=e^{\sum_k \ln |1-\frac{1+\alpha}{k}|}\\
\approx e^{-\sum_k\frac{1+\alpha}{k}}\\
\approx \frac{1}{n^{1+\alpha}}
$$

而且此时$\binom{\alpha}{n}$本身是个交错级数,所以在$-1$处不收敛而在$1$处收敛.同理当$\alpha\leq -1$的时候,$|\binom{\alpha}{n}|$并不趋向于$0$所以两端都不收敛.

###### Example2(反三角函数)

考虑由Example1,当$|x|<1$的时候:

$$
(1-x^2)^{-\frac{1}{2}}=1+\sum_{n=1}^\infty\frac{(2n-1)!!}{(2n)!!}x^{2n}
$$

两遍逐项积分,有:

$$
\arcsin x=x+\sum_{n=1}^\infty\frac{(2n-1)!!}{(2n)!!}\frac{x^{2n+1}}{2n+1}
$$

继续用拉贝判别法,注意到:

$$
n\left(\frac{a_n}{a_{n+1}}-1\right)\to \frac{3}{2}
$$

所以收敛.而且还可以发现这一下积分使得端点处也搞定了,所以上式对$[-1,1]$都成立.

###### Example3(平方倒数和)

还可再整点活,令$x=\sin t$带入上式,得到:

$$
t=\sin t+\sum_{n=1}^\infty\frac{(2n-1)!!}{(2n)!!}\frac{\sin^{2n+1}t}{2n+1}\\
$$

两遍在$[0,\frac{\pi}{2}]$上积分,得到:

$$
\frac{\pi^2}{8}=1+\sum_{n=1}^\infty\frac{(2n-1)!!}{(2n)!!(2n+1)}\frac{(2n)!!}{(2n+1)!!}\\
=1+\sum_{n=1}^\infty\frac{1}{(2n+1)^2}\\
=\sum_{n=1}^\infty \frac{1}{(2n-1)^2}
$$

所以奇数的平方和是$\frac{\pi^2}{8}$,那么:

$$
\sum_{n=1}^\infty\frac{1}{n^2}=\frac{\pi^2}{8}+\frac{1}{4}\sum_{n=1}^\infty \frac{1}{n^2}\\
\sum_{n=1}^\infty\frac{1}{n^2}=\frac{\pi^2}{6}
$$

###### Example4

当$c>0$的时候,求证存在多项式列$\{P_n(x)\}\rightrightarrows |x|,x\in [-c,c]$.

考虑:

$$
|x|=(x^2)^{\frac{1}{2}}\\
=(c^2-(c^2-x^2))^{\frac{1}{2} }\\
=c(1-\left(1-\frac{x^2}{c^2}\right))^{\frac{1}{2}}
$$

由于$1-\frac{x^2}{c^2}\in [0,1],\frac{1}{2}>0$,只需要对此用广义二项式定理即可.

###### Example5

求$\int_0^\pi \frac{\ln(1+a \cos x)}{\cos x}{\rm d}x$.

对着上面那个$\ln$做泰勒级数,自然有:

$$
\int_0^\pi \frac{\ln(1+a\cos x)}{\cos x}{\rm d}x\\
=\int_0^\pi a+\sum_{n=1}^\infty(-1)^n\frac{a^{n+1}\cos^n x}{n+1}\\
$$

而回忆到$\int_0^\pi \cos^m x{\rm d}x$在$m$是奇数的时候当然为$0$,偶数的时候转化为$\sin x$,答案是$\frac{(m-1)!!}{m!!}\pi$.

所以原式变成:

$$
\int_0^\pi a+\sum_{n=1}^\infty(-1)^n\frac{a^{n+1}\cos^n x}{n+1}\\
=\pi(a+\sum_{m=1}^\infty\frac{(2m-1)!!}{(2m)!!}\frac{a^{2m+1}}{2m+1})\\
=\pi\arcsin a
$$

###### Example6

求$S=\sum_{n=0}^\infty\frac{(-1)^n(n+1)^3}{n!}$.

见到:

$$
\sum_{n=0}^\infty\frac{(-1)^n(n+1)^3}{n!}e^{(n+1)x}\\
=\sum_{n=0}^\infty\frac{(-1)^n}{n!}(e^{(n+1)x})'''\\
$$

取$f(x)=\sum_{n=0}^\infty\frac{(-1)^n}{n!}e^{(n+1)x}$,则:

$$
f(x)=\sum_{n=0}^\infty\frac{(-1)^n}{n!}e^{(n+1)x}\\
=e^x\sum_{n=0}^\infty\frac{(-e^x)^n}{n!}\\
=e^{x-e^x}
$$

则:

$$
f'=(1-e^x)f\\
f''=(1-3e^x+e^{2x})f\\
f'''=(-3e^x+2e^{2x})f+(1-e^x)(1-3e^x+e^{2x})f\\
f'''(0)=-f(0)=-\frac{1}{e}
$$

这就搞定.

###### Example7

求$\sum_{n=1}^\infty\frac{\cos (nx)}{n}$.

取$t\in(-1,1]$,设:

$$
f(t)=\sum_{n=1}^\infty\frac{\cos (nx)}{n}t^n\\
f'=\sum_{n=1}^\infty \cos(nx)t^{n-1}\\
=\Re \sum_{n=1}^\infty t^{n-1}e^{inx}\\
=\Re\frac{e^{ix}}{1-te^{ix}}\\
=\Re\frac{\cos x+i\sin x}{1-t\cos x-it\sin x}\\
=\Re\frac{(\cos x+i\sin x)(1-t\cos x+it\sin x)}{(1-t\cos x)^2+t^2\sin^2 x}\\
=\frac{\cos x(1-t\cos x)-t\sin^2 x}{(1-t\cos x)^2+t^2\sin^2 x}\\
=\frac{\cos x-t}{1-2t\cos x+t^2}\\
$$

此时应当有:

$$
f(t)=f(0)+\int_0^t f'(t){\rm d}t\\
=-\frac{1}{2}\ln(1-2t\cos x+t^2)
$$

而$f(1)=-\ln 2-\ln|\sin \frac{x}{2}|$,这就求出了原式.

###### Example8

考虑$\zeta(x)=\sum_{n=1}^\infty \frac{1}{n^x}$,有定理说:

$$
\zeta(x)=\frac{1}{x-1}+\sum_{n=0}^\infty\frac{(-1)^n\gamma_n}{n!}(x-1)^n
$$

其中后半部分是整个复平面上的解析函数,并有:

$$
\gamma_n=\lim_{m\to \infty}\left(\sum_{k=1}^m \frac{(\ln k)^n}{k}-\frac{(\ln m)^{n+1}}{n+1}\right)
$$

###### Example9

求$S(x)=1+\sum_{n=1}^\infty \frac{(2n-1)!!}{(2n)!!}x^n$,其中$|x|<1$.

容易见到其收敛半径是$1$,考虑:

$$
\begin{aligned}
S'(x)&=\frac{1}{2}+\sum_{n=1}^\infty\frac{(2n+1)!!}{(2n+2)!!}(n+1)x^n\\
2S'(x)&=1+\sum_{n=1}^\infty\frac{(2n+1)!!}{(2n)!!}x^{n}\\
2xS'(x)&=\sum_{n=1}^\infty\frac{(2n-1)!!}{(2n-2)!!}x^n\\2xS'(x)&=\sum_{n=1}^\infty\frac{(2n-1)!!}{(2n)!!}2nx^n\\
2S'(x)-2xS'(x)&=S(x)\\
\frac{ {\rm d}x}{2(1-x)}&=\frac{ {\rm d} S(x)}{S(x)}\\
-\frac{1}{2}\ln(1-x)&=\ln S(x)\\
S(x)&=\frac{1}{\sqrt{1-x}}
\end{aligned}
$$

###### Example10

当$|x|<1$,求$S(x)=\sum_{n=0}^\infty \frac{(-1)^nx^n}{2n+1}$.

容易见到收敛半径为$1$,令$x=t^2$,有:

$$
\begin{aligned}
S(t^2)=\sum_{n=0}^\infty(-1)^n\frac{1}{t}\int_0^tu^{2n}{\rm d}u\\
=\frac{1}{t}\int_0^t\sum_{n=0}^\infty(-1)^nu^{2n}{\rm d}u\\
=\frac{1}{t}\int_0^t \frac{ {\rm d}u}{1+u^2}\\
=\frac{\arctan t}{t}\\
\end{aligned}
$$

所以$S(x)=\frac{\arctan \sqrt x}{\sqrt x}$.

###### Example11

求$\sum_{n=2}^\infty\frac{(-1)^n}{n^2+n-2}$.

注意到:

$$
\begin{aligned}
S(x)=\sum_{n=2}^\infty\frac{(-1)^nx^{n-1}}{n^2+n-2}\\
=\sum_{n=2}^\infty\frac{(-1)^nx^{n-1}}{3}\left(\frac{1}{n-1}-\frac{1}{n+2}\right)\\
=\frac{1}{3}\sum_{n=2}^\infty \frac{(-1)^n x^{n-1}}{n-1}-\frac{1}{3x^3}\sum_{n=2}^\infty \frac{(-1)^n x^{n+2}}{n+2}\\
=\frac{1}{3}\ln(1+x)-\frac{1}{3x^3}\left(-\ln(1+x)+x-\frac{x^2}{2}+\frac{x^3}{3}\right)
\end{aligned}
$$

而注意到带入$x=1$的时候$S(x)$收敛,这就搞定了,带入$x=1$就行.

###### Example12

求$\sum_{n=1}^\infty \frac{n}{(n+1)!}$.

设$S(x)=\sum_{n=0}^\infty \frac{(n+1)x^n}{(n+2)!}$,显然收敛,直接逐项积分.

$$
\int_0^x S(t){\rm d}t=\sum_{n=0}^\infty\frac{x^{n+1}}{(n+2)!}\\
=\sum_{n=1}^\infty\frac{x^{n}}{(n+1)!}\\
x(\int_0^x S(t){\rm d}t+1)=e^x-1\\
S(x)=\left(\frac{e^x-1}{x}\right)'=\frac{xe^x-e^x+1}{x^2}
$$

这就搞定了.

###### Example13

求$\int_0^\infty \frac{x{\rm d}x}{e^{2\pi x}-1}$.

注意到:

$$
\begin{aligned}
\int_0^\infty \frac{x{\rm d}x}{e^{2\pi x}-1}\\
=\int_0^\infty \frac{xe^{-2\pi x} {\rm d}x}{1-e^{-2\pi x}}\\
=\int_0^{\infty}xe^{-2\pi x}\sum_{n=0}^\infty e^{-2\pi n x} {\rm d}x\\
=\sum_{n=0}^\infty \int_0^{+\infty}xe^{-2\pi(n+1)x} {\rm d}x\\
=\sum_{n=0}^\infty \frac{1}{-2\pi(n+1)}\int_0^{+\infty}x {\rm d}e^{-2\pi(n+1)x}\\
=\sum_{n=0}^\infty \frac{1}{2\pi(n+1)}\int_0^{+\infty}e^{-2\pi(n+1)x}{\rm d}x\\
=\sum_{n=0}^\infty \frac{1}{4\pi^2 (n+1)^2}\\
=\frac{1}{24}
\end{aligned}
$$

其中中间的交换积分号和求和号是源于所有的部分和都可以小于等于$g(x)$,并且$\int_0^{\infty}g$收敛.

###### Example14

求$\int_0^1 \frac{\ln (1+x)}{x} {\rm d}x$.

直接展开对吧,注意到:

$$
\int_0^1 \frac{\ln (1+x)}{x} {\rm d}x\\
=\int_0^1 \sum_{n=1}^\infty \frac{(-1)^{n-1}x^{n-1}}{n}\\
=\sum_{n=1}^\infty \int_0^1 \frac{(-1)^{n-1}x^{n-1}}{n}{\rm d}x\\
=\sum_{n=1}^\infty \frac{(-1)^{n-1}}{n^2}\\
=\sum_{n=1}^\infty \frac{1}{(2n-1)^2}-\sum_{n=1}^\infty \frac{1}{(2n)^2}\\
=\sum_{n=1}^\infty \frac{1}{n^2}-2\sum_{n=1}^\infty \frac{1}{(2n)^2}\\
=\frac{1}{2}\sum_{n=1}^\infty \frac{1}{n^2}\\
=\frac{\pi^2}{12}
$$

###### Example15

设$I$是开区间,$f\in C^\infty (I)$,求证$f\in C^\omega(I)$当且仅当$\forall x_0\in I,\exists M,B,r>0$,使得$\forall |x-x_0|<r,|f^{(n)}(x)|\leq MB^nn!$.就是说对于区间内任何一个点,在它的小邻域处,任意阶导数都可以被和$n$有关的一个速率控制住.

充分性比较简单,考虑取$S(x)=\sum \frac{f^{(n)}(x_0)}{n!}(x-x_0)^n$对其取$|\sqrt[n]{\frac{f^{(n)}(x_0)}{n!}}|\leq \sqrt[n]{MB^n}=\sqrt[n]{M}B<+\infty$,所以总之收敛半径不为$0$.那就总能取一个足够小的$r<\frac{1}{B}$.此时用泰勒展开的拉格朗日余项:

$$
\begin{aligned}
\left|f(x)-\sum_{k=0}^n \frac{f^{(k)}(x_0)}{k!}(x-x_0)^k\right|&=\left|\frac{f^{(n+1)}(\xi)}{(n+1)!}(x-x_0)^{n+1}\right|\\
&\leq |B^{n+1}(x-x_0)^{n+1}M|\\
&\leq |(Br)^{n+1}M|\to 0
\end{aligned}
$$

这就搞定.

接下来是必要性.考虑$\varlimsup_{n\to \infty}\sqrt[n]{|\frac{f^{(n)}(x_0)|}{n!}}=\frac{1}{R}>0$,那就可以取$0<r<M<R$,存在一个$N$,使得$\forall n\geq N$都有$|f^{(n)}(x_0)|\leq n!(\frac{1}{M})^n$.

接下来想要把这个性质挪到$x_0$周围小邻域上对吧.于此我们考虑先逼近后逐项求导:

$$
\begin{aligned}
|f^{(n)}(x)|&=|\sum_{k=0}^\infty\frac{f^{(n+k)}(x_0)}{k!}(x-x_0)^k|\\
&\leq \sum_{k=0}^\infty\frac{|f^{(n+k)}(x_0)|}{k!}|x-x_0|^k\\
&\leq \sum_{k=0}^\infty\frac{(n+k)!}{k!M^{n+k}}r^k\\
&=\frac{n!}{M^n}\sum_{k=0}^\infty\frac{(n+k)^{\underline k}}{k!}(\frac{r}{M})^k\\
&=\frac{n!}{M^n}\sum_{k=0}^\infty\frac{(-n-1)^{\underline k}}{k!}(\frac{-r}{M})^k\\
&=\frac{n!}{M^n}(1-\frac{r}{M})^{-n-1}\\
&=Mn!(M-r)^{-(n+1)}
\end{aligned}
$$

这就搞定了.

###### Example16

若非常数$f$在$(a,b)$的每一点都解析,则其零点集在$(a,b)$内无聚点.

考虑如果一个点$c\in (a,b)$是$f$的聚点的话,那么$c$一定是$f$的任意阶导数的聚点.由于任意阶导数都是连续的,因此$c$处的任意阶导数都是$0$.又因为$f$解析,所以$c$周围的一个小邻域必然全部为$0$.既如此取最长的小邻域为$(\alpha,\beta)$,下面我们证明$(\alpha,\beta)=(a,b)$,不妨假设$a\ne \alpha$,由于连续性得知$f$在$[\alpha,\beta)$上取值为$0$,然而这意味着$\alpha$必然也是零点集的聚点,因此它还可以继续往左延申一个小邻域,这就与$a$是最靠左的矛盾了.

不过这个做法不能推广到复平面上,原因是平面上的介值定理难搞一点啊.

提供另一种证明方法,考虑$f$在$(a,b)$上的零点的聚点$x_0$,如果$x_0$的任意阶导数都是$0$的话,那还是按照上面一样往外面扩就行了对吧.反之,设$f(x_0)=f^{(1)}(x_0)=\cdots=f^{(N-1)}(x_0)=0,f^{(N)}(x_0)\ne 0$,此时考虑$f(x)=(x-x_0)^{N+1}\sum_{n=0}^\infty \frac{f^{(N+n+1)}}{(N+n+1)!}(x-x_0)^n$,不妨设$g(x)=\sum_{n=0}^\infty \frac{f^{(N+n+1)}}{(N+n+1)!}(x-x_0)^n$,容易发现$g(x_0)\ne 0$,既然如此,$g$肯定在$x_0$的一个小邻域(要与解析的那个邻域取交)上非$0$,设为$(x_0-\delta,x_0+\delta)$上非零,可这样$f(x)$在$(x_0-\delta,x_0)\cup(x_0,x_0+\delta)$上必然非零,这与$x_0$是零点聚点矛盾了.

##### 多项式逼近

如果对于函数$f(x):[a,b]\to \mathbb {R}$,存在一个多项式列可以一致收敛到$f(x)$,则称其可以被多项式**一致逼近**.容易见到一个必要条件是$f(x)\in C[a,b]$.

事实上我们有**维尔斯特拉斯定理**:有界闭区间上的连续函数可由多项式一致连续逼近.首先不妨假设$[a,b]=[0,1]$.下面给出两种证明:

其一是构造性证明,我们有**Bernstein多项式**:

$$
B_n(f)=\sum_{p=0}^n\binom{n}{p}f(\frac{p}{n})x^p(1-x)^{n-p}
$$

这个多项式有很多性质,首先随便放缩一下就知道这个多项式在$[0,1]$上有界,更具体地有$\min f\leq B_n\leq \max f$,原因是:

$$
B_n(f)\leq \sum_p \binom{n}{p}f(\frac{p}{n})x^p(1-x)^{n-p}\\
\leq (\max f)\sum_p \binom{n}{p}x^p(1-x)^{n-p}\\
=\max f
$$

另一边同理.

其次有$\deg(B_n)\leq n$.以及这个甚至是$f$的线性映射,或言$B_n(\alpha f+\beta g)=\alpha B_n(f)+\beta B_n(g)$.

取$r_p(x)=\binom{n}{p}x^p(1-x)^{n-p}$,注意到:

1. $\sum_p r_p(x)=1$.
2. $\sum_p pr_p(x)=nx$.
3. $\sum_p p(p-1)r_p(x)=n(n-1)x^2$.
4. $\sum_p p^2 r_p(x)=n(n-1)x^2+nx$.
5. $\sum_{p=0}^n(p-nx)^2 r_p(x)\leq \frac{n}{4}$.

前四条类似,以(3)为例,证明的话考虑对$(x+y)^n=\sum_p\binom{n}{p}x^py^{n-p}$两边对$x$求两次导数:

$$
n(n-1)x^2(x+y)^{n-2}=\sum_p^n\binom{n}{p}p(p-1)x^py^{n-p}
$$

带入$y=1-x$即可.

(5)的话也只需要计算,观察到:

$$
\sum_{p=0}^n(p-nx)^2 r_p(x)\\
=\sum_p p^2r_p-2nx \sum_p pr_p+n^2x^2\sum r_p\\
=n(n-1)x^2+nx-2n^2x^2+n^2x^2\\
=nx(1-x)\leq \frac{n}{4}
$$

我们见到:

$$
|B_n(f)-f|=\left|\sum_{p=0}^n\binom{n}{p}f(\frac{p}{n})x^p(1-x)^{n-p}-\sum_{p=0}^n \binom{n}{p}f(x)x^p(1-x)^{n-p}\right|\\
\leq \sum_{p=0}^n\binom{n}{p}|f(\frac{p}{n})-f(x)|x^p(1-x)^{n-p}\\
$$

既然$f(x)$闭区间上连续则有一致连续,$\forall \epsilon>0$,取$\delta>0$,使得只要$|x-x'|<\delta,|f(x)-f(x')|<\frac{\epsilon}{2}$,那有:

$$
|B_n(f)-f|\leq \sum_{p=0}^n\binom{n}{p}|f(\frac{p}{n})-f(x)|x^p(1-x)^{n-p}\\
=\sum_{|\frac{p}{n}-x|<\delta}^n\binom{n}{p}|f(\frac{p}{n})-f(x)|x^p(1-x)^{n-p}+\sum_{|\frac{p}{n}-x|\geq \delta}^n\binom{n}{p}|f(\frac{p}{n})-f(x)|x^p(1-x)^{n-p}\\
\leq \frac{\epsilon}{2}+\frac{2\max |f|}{n^2\delta^2}\sum_{p=0}^n(p-nx)^2r_p(x)\\
\leq \frac{\epsilon}{2}+\frac{\max |f|}{2n\delta^2}
$$

倒数第二步是由于当$|\frac{p}{n}-x|\geq \delta$,所以$\frac{(p-nx)^2}{n^2\delta^2}\geq 1$.

最后一步是因为当$\epsilon$确定的时候,$\delta$就已经确定了,然而$n$可以足够大,这就控制住了.

再给另一个存在性证明.回忆到$[a,b]$上的连续函数组成的集合$C[a,b]$是一个完备线性空间.称$A\subseteq C[a,b]$是一个**代数**当且仅当$\forall f,g\in A$,$\forall c\in \mathbb{R}$,$f+g,fg,cf\in A$.容易见到如果$P$是一个有限项多项式,$f\in A$,则$P(f)\in A$.

在此基础上定义$\bar A$为$A$的**闭包**,定义为:

$$
\bar A=\{f\in C[a,b]|\exists \{f_n\}\in A,f_n\rightrightarrows f\}
$$

容易见到$A\subseteq \bar A$.而且容易见到$\bar A$肯定也是一个代数.而见到$\overline{(\bar A)}=\bar A$,原因是柯西列导出的已经是闭的了,无法再往外拓展.

回忆到绝对值函数可以由多项式函数逼近,所以如果$f\in \bar A$,则$|f|\in \bar A$.更进一步,如果$f,g\in \bar A$,由于$\max (f,g)=\frac{f+g}{2}+\frac{|f-g|}{2}$,所以$\max (f,g)\in \bar A$,同理$\min(f,g)\in \bar A$.

我们有一个定理说:设$A\subseteq C[a,b]$是一个代数,并且$f(x)=c\in A$.并且$\forall x,y\in [a,b],x\ne y,\exists f\in A,f(x)\ne f(y)$则$\bar A=C[a,b]$.容易见到这个定理强于维尔斯特拉斯定理.这个定理叫**Stone-Weierstrass定理**.

下面我们来证明这个结论.

先搞一个引理:$\forall x_1,x_2\in [a,b],x_1\ne x_2,\forall \alpha,\beta \in \mathbb{R}$,则$\exists h\in A,h(x_1)=\alpha,h(x_2)=\beta$.

证明的话,考虑由已知$\exists \varphi \in A,\varphi(x_1)\ne \varphi(x_2)$.此时把这个函数给挪一下,取$h(x)=\alpha+(\beta-\alpha)\frac{\varphi(x)-\varphi(x_1)}{\varphi(x_2)-\varphi(x_1)}$.(其实这个应该是双传递性,但是我目前没看出如何用双传递性来解释证明过程.)

接下来考虑证明原定理,来证明$\forall f\in C[a,b],\forall n,\exists g_n\in \bar A,|f(x)-g_n(x)|<\frac{1}{n}$,由于我们刚才的引理,$\forall x,y\in [a,b],\exists h_{x,y}(z)\in A,h_{x,y}(x)=f(x),h_{x,y}(y)=f(y)$.这个函数由$x,y$所确定.

固定$x$不动,对$\forall y\in [a,b]$,$\exists \delta=\delta(y)$,由于一致连续性,当$z\in I_y=(y-\delta,y+\delta)$的时候,$h_{x,y}(z)<f(z)+\frac{1}{n}$.而这里的$I_y$构成了一个开覆盖(边界上可能需要挪一下,但反正能做到),用有限覆盖定理拿到有限个区间$I_{y_1,},\cdots,I_{y_m}$覆盖了$[a,b]$.此时取$h_x(z)=\min\{h_{x,y_1},\cdots, h_{x,y_m}\}$,见到$h_x(z)<f(z)+\frac{1}{n},\forall z\in [a,b]$.而且$h_x(x)=f(x)$.

既然$h_x(x)=f(x)$,考虑$\exists \delta,I_x=(x-\delta,x+\delta)$使得$h_x(z)>f(z)-\frac{1}{n},\forall z\in I_x$.对此再做有限覆盖得到$h_{x_1},\cdots,h_{x_l}$,取$g_n=\max\{h_{x_1},\cdots,h_{x_l}\}$.显然:

$$
f(z)-\frac{1}{n}<g_n(z)<f(z)+\frac{1}{n}
$$

搞定了.

那么,这个多项式能不能再来点限制呢?比如说我不许在多项式里出现$x^2$项可以吗?也可以.考虑$f\in C[a,b]$,则$f(x^{\frac{1}{3}})\in C[a^{\frac{1}{3}},b^{\frac{1}{3}}]$,所以存在一个多项式$P(x)\rightrightarrows f(x^{\frac{1}{3}})$,意味着$P(x^3)\rightrightarrows f(x),x\in [a,b]$.而$P(x^3)$当然没有$x^2$项.

也就是说这些多项式很多都是浪费的.那么有没有一个非常厉害的结论能告诉我什么样的多项式可以呢?考虑在$[0,1]$上,${\rm Span}\{1,x^{\lambda_1},\cdots \}$张成的多项式,其中$0<\lambda_1<\lambda_2<\cdots$,其中$\lambda_n\to \infty$.注意这里不要求$\lambda_i$是整数,如果其闭包能张成$C[0,1]$,当且仅当$\sum \frac{1}{\lambda_i}=+\infty$.这被称为**Muntz定理**,(其一个更强的推广是去掉$\lambda_n\to \infty$的结论,取而代之的是需要满足$\sum \frac{\lambda_i}{1+\lambda_i^2}=+\infty$)

这个定理的必要性我们目前难以证明,先证明其充分性.

考虑$\forall m \in \mathbb{N}_+\setminus \{\lambda_i\}$,都可以定义一个函数列,其中$Q_0(x)=x^m,Q_n(x)=(\lambda_n-m)x^{\lambda_n}\int_x^1 Q_{n-1}(t)t^{-1-\lambda_n}{\rm d}t$.我们先证明$Q_n$一定长成$x^m-\sum_{i=1}^n a_{n,i}x^{\lambda_i}$的形式.

数学归纳证明此结论,设$Q_n=x^m-\sum_{i=1}^n a_{n,i}x^{\lambda_i}$,来看$Q_{n+1}$:

$$
Q_{n+1}(x)=(\lambda_{n+1}-m)x^{\lambda_{n+1}}\int_x^1 Q_{n}(t)t^{-1-\lambda_{n+1}}{\rm d}t\\
=(\lambda_{n+1}-m)x^{\lambda_{n+1}}\int_x^1 (t^m-\sum_{i=1}^n a_{n,i}t^{\lambda_i})t^{-1-\lambda_{n+1}}{\rm d}t\\
$$

后面那部分直接积出来就是了,然后前面有一个$\lambda_{n+1}$,这样就搞定了.

下面定义一个函数的模长$\Vert Q\Vert=\max_{[0,1]}|Q(x)|$,此时:

$$
\begin{aligned}
|Q_n(x)|&\leq |\lambda_n-m|x^{\lambda_n}\Vert Q_{n-1}\Vert\int_x^1 t^{-1-\lambda_n}{\rm d}t\\
&=|\lambda_n-m|x^{\lambda_n}\sdot \Vert Q_{n-1}\Vert\frac{x^{-\lambda_n}-1}{\lambda_n}\\
&=|1-\frac{m}{\lambda_n}|\sdot \Vert Q_{n-1}\Vert (1-x^{\lambda_n})\\
&\leq |1-\frac{m}{\lambda_n}|\sdot \Vert Q_{n-1}\Vert\\
&\leq \prod_{i=1}^n |1-\frac{m}{\lambda_i}|
\end{aligned}
$$

然而回忆到$\sum \frac{1}{\lambda_i}$发散.所以后面那个连乘积必然发散,但是容易见到$m$取死后,其从$\lambda_i>m$的时候就是单调递减的,那又发散必然知道其趋近于$0$.所以$\lim \Vert Q_n \Vert=0$.那又回忆到$Q_n=x^m-\sum_{i=1}^n a_{n,i}x^{\lambda_i}$,这已经说明$x^m$可以被$\{x^{\lambda_i}\}$一致逼近,原命题充分性搞定.

现在来看固定在$\lambda_i\in \mathbb{N}_+$的情形,能否给出一个充要条件呢?我们下面证明对于互异的$\{\lambda_i\}\in \mathbb{N}_+$,$C[a,b]=\overline{\mathrm {span} }\{x^{\lambda_i},1\}$的充要条件是:

1. 如果$0\notin (a,b)$,当$\sum \frac{1}{\lambda_i}=+\infty$.
2. 如果$a<0<b$,要求$\sum_{\lambda_i\in {\rm even}} \frac{1}{\lambda_i}=+\infty,\sum_{\lambda_i\in {\rm odd}} \frac{1}{\lambda_i}=+\infty$.

先证这个命题的充分性,对于$0\notin (a,b)$,不妨设$0\leq a<b=1$,容易发现此时的确满足$\lim \lambda_n=+\infty$而且$\sum_i \frac{1}{\lambda_i}=+\infty$,这就搞定了.

对于$a<0<b$,我们只要证明在$C[-1,1]$上的函数都可以被一致逼近即可对吧.考虑$g_1(x)=\frac{f(x)+f(-x)}{2}$是其偶函数部分,自然可以被偶次的部分逼近(原因是只需要逼近$[0,1]$上就可以延拓).反之亦然同理,$g_2(x)=\frac{f(x)-f(-x)}{2}$也可以被奇数次的部分逼近并且延拓,这也就搞定.

回忆到如果一个函数如果在一个无穷区间上可以被有限次多项式一致逼近,用柯西准则做那个函数列就可以证明这个函数一定是一个多项式.下面我们来推广这个结论.

设$V$是一个内积空间,其中$g,f_1,\cdots,f_n\in V$是其中的$n$个向量,设$E={\rm Span}\{f_1,\cdots,f_n\}$,回忆到$g$到$E$的距离定义为:

$$
d=\min_{p\in E}\Vert g-p\Vert=\sqrt{\cfrac{G(g,f_1,\cdots,f_n)}{G(f_1,\cdots,f_n)}}
$$

其中$G(f_1,\cdots,f_n)=\det|(f_i|f_j)|$.

而考虑取两列数列$\{\alpha_i\},\{\beta_j\}$,回忆到李文威考试题给出过:

$$
\det(\frac{1}{\alpha_i+\beta_j})=\cfrac{\prod_{1\leq i<j\leq n}(\alpha_j-\alpha_i)(\beta_j-\beta_i)}{\prod_{1\leq i,j\leq n}(\alpha_i+\beta_j)}
$$

又回忆到$C[0,1]$也是一个内积空间,可以定义$(f|g)=\int_0^1 fg{\rm d}x$.容易见到$(x^a|x^b)=\frac{1}{a+b+1}$.现在我们就可以用上面的行列式硬算了.考虑取$E={\rm span}\{x^{\lambda_1},\cdots,x^{\lambda_n}\}$,取$d=(x^{\lambda},E)$,容易见到:

$$
d(x^\lambda,E)=\frac{1}{\sqrt{2\lambda+1}}\prod_{i=1}^n \left|\frac{\lambda-\lambda_i}{\lambda+\lambda_i+1}\right|
$$

下面我们引进一个引理:

对于数列$\{\lambda_i\}>0$且单增,如果$c=\inf_{i\geq 1}\{\lambda_{i+1}-\lambda_i\}>0$,如果$\sum_{i=1}^\infty \frac{1}{\lambda_i}<\infty$,则$\lim_{m\to \infty}\frac{1}{\lambda_m}\sum_{i=1,i\ne m}^\infty \ln\left|\frac{\lambda_i+\lambda_m}{\lambda_i-\lambda_m}\right|=0$.

这个定理的详细证明较为麻烦,而且与主线无关,将在下述Example3处讲解.

考虑取$P(x)\in {\rm Span}_{i\ne m}\{x^{\lambda_i}\}$,由于子空间只有有限项非零,所以$\exists n$使得$P(x)\in E={\rm Span}_{i\ne m,i\leq n}\{x^{\lambda_i}\}$,此时观察到:

$$
d(x^{\lambda_m},P(x))\\
\geq d(x^\lambda_m,E)\\
= \frac{1}{\sqrt{2\lambda_m+1}}\prod_{i=1,i\ne m}^n\left|\frac{\lambda_m-\lambda_i}{\lambda_m+\lambda_i+1}\right|\\
\geq \frac{1}{\sqrt{2\lambda_m+1}}\prod_{i=1,i\ne m}^{\infty}\left|\frac{\lambda_m-\lambda_i}{\lambda_m+\lambda_i+1}\right|\\
$$

由上述引理,我们知道最后那一项取$\ln$后是一个$o(\lambda_m)$的东西,不妨将后面那一项记作$e^{-l_m\lambda_m}$,其中$l_m>0,\lim_{m\to \infty}l_m=0$.

为了简化说明引进$\lambda_0=0$,

再来一个引理:对$\forall \epsilon>0$,$\exists C_\epsilon>0$,满足$\forall P(x)\in {\rm Span}_{i\geq 0}\{x^{\lambda_i}\},\lambda_0=0$,不妨设$P(x)=\sum_{i=0}^n a_i x^{\lambda_i}$,我们有$|a_m|\leq C_\epsilon (1+\epsilon)^{\lambda_m}\Vert P\Vert$.

注意到:

$$
\frac{P(x)}{a_m}=x^{\lambda_i}+\sum_{i\leq n,i\ne m}^n\frac{a_i}{a_m}x^{\lambda_m}\\
\frac{\Vert P(x)\Vert}{|a_m|}\geq d(x^{\lambda_m},E)
\geq e^{-l_m\lambda_m}\\
|a_m|\leq e^{l_m\lambda_m}\Vert P(x)\Vert
$$

而当$m$充分大的时候,当然有$e^{l_m}\leq 1+\epsilon$,这就证毕了.

万事俱备,下面我们来证明必要性:办法是使用反证法,假设$f\in C[0,1]$,当$\lambda_0=0,\sum_{i=1}^\infty \frac{1}{\lambda_i}<\infty$时,并且$P_n(x)=\sum_{i=0}^n a_{i,n}x^{\lambda_i}\rightrightarrows f(x)$.则当然也有$\Vert P_n\Vert\to \Vert f\Vert$,那$\Vert P_n\Vert$就有界了,既然如此,$a_{i,n}\leq C_\epsilon(1+\epsilon)^{\lambda_i}$,也就是说所有的$a_{i,n}$都是有界的.

我们想要求出一个子列$\{n_k\}$,使得$\forall i,\lim_{k\to \infty}a_{i,n_k}$收敛于某个数$a_i$,方法是对角线法则,对于第一行其肯定存在收敛子列,取其第一个数为$n_1$,对于第二行,在第一行对应下来的元素中肯定也有收敛子列,取其第一个元素作为$n_2$,依次类推.这样我们就得到了一个新的函数列$P_{n_k}\to g(x)=\sum_{i=0}^\infty a_i x^{\lambda_i}$.此时考虑$\varlimsup \sqrt[\lambda_i]{|a_i|}\leq 1+\epsilon$,因此$g(x)$收敛半径$\geq 1$.然而,$P_{n_k}\to f(x)$,因此$f(x)=g(x)$.然而$g(x)$的收敛半径$\geq 1$,因此一般的函数当然不能被一致逼近.

另一个问题是如果$a<0<b$该怎么办,接下来假设$\sum_{\lambda_i\in {\rm even}}\frac{1}{\lambda_i}<\infty$.放缩一下边界,我们下面来证明此时不能逼近$C[-1,1]$上的偶函数.

假设$f\in C[-1,1]$并且是偶函数,对于一列函数列,可以拆成偶函数$g_n$和奇函数$h_n$部分,前者只包含偶数次后者只包含奇数次.此时有$g_n+h_n\rightrightarrows f$,由于是偶函数,换号得到$g_n-h_n\rightrightarrows f$,这就必然得到$g_n\rightrightarrows f$,但$g_n$并不能逼近所有的,这就矛盾了.

###### Example1

若$f(x)$在无穷区间上可以用多项式一致逼近,求证$f$为多项式.

考虑多项式列$\{P_n\}$上有柯西准则,所以当$N$足够大的时候,$\forall n,m\geq N$,$\sup|P_n-P_m|$足够小,然而由于无穷区间,如果$\deg (P_n-P_m)>0$就完蛋了,所以从某一项开始,这个多项式列的两两之间一定之差一个常数.不妨干脆设$P_n=P+c_n$的形式,既然$P_n$收敛,那么$c_n$肯定收敛,设其收敛于$c$,则$f(x)=P+c$.

###### Example2

若$f\in C[a,b]$,$\{p_n\}\rightrightarrows f$,并且$\{p_n\}$为次数一致有界的多项式列,求证$f$为多项式.

设次数上界为$n$,一个想法是说我这些多项式是有限多个的,你直接拿切比雪夫多项式作为基,既然现在都是有限和的情况了,那在线性空间上收敛当然是在每一个基对应的系数都收敛,这就意味着$f$肯定是多项式.

###### Example3(Lemma)

对于数列$\{\lambda_i\}>0$且单增,如果$c=\inf_{i\geq 1}\{\lambda_{i+1}-\lambda_i\}>0$,如果$\sum_{i=1}^\infty \frac{1}{\lambda_i}<\infty$,则$\lim_{m\to \infty}\frac{1}{\lambda_m}\sum_{i=1,i\ne m}^\infty \ln\left|\frac{\lambda_i+\lambda_m}{\lambda_i-\lambda_m}\right|=0$.

考虑证明,由于每一项都是正的,因此我们分别证明后面的无穷和部分在$[1,m-1]$和$[m+1,+\infty)$上均为$0$.

思路呼之欲出,将$\lambda_i$按照与$\lambda_m$的比例分为两段:任意取定常数$\epsilon>0$,下面开始分析.

设当$i\geq N_1$的时候有$\lambda_i\geq (1-\epsilon )\lambda_m$成立,考虑:
$$
\frac{1}{\lambda_m}\sum_{i=1}^{N_1-1}\ln(1+\frac{2\lambda_i}{\lambda_m-\lambda_i})\\
\leq \frac{1}{\lambda_m}\sum_{i=1}^{N_1-1} \frac{2\lambda_i}{\lambda_m-\lambda_i}\\
=\sum_{i=1}^{N_1-1} \frac{2\lambda_i}{(\lambda_m-\lambda_i)\lambda_m}\\
=2\sum_{i=1}^{N_1-1} (\frac{1}{\lambda_m-\lambda_i}-\frac{1}{\lambda_m})\\
\leq 2\sum_{i=1}^{N_1-1} \frac{1}{\epsilon\lambda_m}-\frac{1}{\lambda_m}\\
=\frac{N_1-1}{\lambda_m}(\frac{2}{\epsilon}-1)\\
\leq \frac{m}{\lambda_m}(\frac{2}{\epsilon}-1)
$$
然而注意到$\{\frac{1}{\lambda_m}\}$单调递减,用柯西准则得到:
$$
\sum_{k=m+1}^{3m}\frac{1}{\lambda_k}\geq \frac{m}{\lambda_{2m}}
$$
所以得到$\lim_{m\to +\infty}\frac{m}{\lambda_m}\to 0$.

然后考虑设$i> N_2$时有$(1+\epsilon)\lambda_m\leq \lambda_i$,设$(1+\epsilon)=\frac{1}{1-\epsilon_2}$,则$\lambda_m\leq (1-\epsilon_2)\lambda_i$留神到:
$$
\frac{1}{\lambda_m}\sum_{i=N_2+1}^{+\infty} \ln(1+\frac{2\lambda_m}{\lambda_i-\lambda_m})\\
\leq \frac{1}{\lambda_m}\sum_{i=N_2+1}^{+\infty} \frac{2\lambda_m}{\lambda_i-\lambda_m}\\
=\sum_{i=N_2+1}^{+\infty} \frac{2}{\lambda_i-\lambda_m}\\
\leq \sum_{i=N_2+1}^{+\infty} \frac{4}{\epsilon_2\lambda_i}\\
\leq \sum_{i=m}^{+\infty} \frac{4}{\epsilon_2\lambda_i}\\
$$
由柯西准则就可以知道其趋近于$0$.

由上面的分析见到如果预先取定$\epsilon$,再移动$m$,开头结尾两端都可以尽可能小对吧.最后我们要做的就是中间很靠近$\lambda_m$的两段了,考虑:
$$
\frac{1}{\lambda_m}\sum_{i=N_1}^{m-1} \ln(\frac{2\lambda_m}{\lambda_m-\lambda_i}-1)\\
\leq \frac{1}{\lambda_m}\sum_{i=N_1}^{m-1} \ln(\frac{2\lambda_m}{c(m-i)}-1)\\
$$
不妨设$M=m-N_1$,上式变为:
$$
\frac{1}{\lambda_m}\sum_{i=N_1}^{m-1} \ln(\frac{2\lambda_m}{c(m-i)}-1)\\
=\frac{1}{\lambda_m}\sum_{i=1}^{N} \ln(\frac{2\lambda_m}{ic}-1)\\
\leq \frac{1}{\lambda_m}\sum_{i=1}^{N} \ln(\frac{2\lambda_m}{ic})\\
$$
而
$$
\frac{1}{\lambda_m}\sum_{i=m+1}^{N_2} \ln(1+\frac{2\lambda_m}{\lambda_i-\lambda_m})\\
\leq \frac{1}{\lambda_m}\sum_{i=m+1}^{N_2} \ln(1+\frac{2\lambda_m}{c(i-m)})\\
\leq \frac{1}{\lambda_m}\sum_{i=m+1}^{N_2} \ln(\frac{2\lambda_m+cm}{c(i-m)})
$$
然而$\frac{m}{\lambda_m}\to 0$,于是后面的主要项仍是$\ln(\frac{\lambda_m}{ic})$形状,而考虑$\max(N_2-m,m-N_1)\leq \frac{\epsilon \lambda_m}{c}$,接下来只要证明:
$$
\frac{1}{\lambda_m}\sum_{i=1}^{\frac{\epsilon\lambda_m}{c}} \ln(\frac{\lambda_m}{ic})\to 0
$$
就做完了对吧,发现:
$$
\frac{1}{\lambda_m}\sum_{i=1}^{\frac{\epsilon\lambda_m}{c}} \ln(\frac{\lambda_m}{ic})=\frac{1}{\lambda_m}\sum_{i=1}^{\frac{\epsilon\lambda_m}{c}} \ln(\lambda_m)-\frac{1}{\lambda_m}\sum_{i=1}^{\frac{\epsilon\lambda_m}{c}} \ln(ic)\\
=\frac{\epsilon}{c}\ln(\frac{\lambda_m}{c})-\frac{1}{\lambda_m}\sum_{i=2}^{\frac{\epsilon\lambda_m}{c}} \ln (i)\\
=\frac{\epsilon}{c}\ln(\frac{\lambda_m}{c})-\frac{1}{\lambda_m}\sum_{i=2}^{\lfloor\frac{\epsilon\lambda_m}{c}\rfloor} \int_{i-1}^{i}\ln(i){\rm d t}\\
\leq \frac{\epsilon}{c}\ln(\frac{\lambda_m}{c})-\frac{1}{\lambda_m}\int_{1}^{\frac{\epsilon\lambda_m}{c}-1}\ln t{\rm d t}\\
=\frac{\epsilon}{c}\ln(\frac{\lambda_m}{c})-\frac{1}{\lambda_m}\int_{1}^{\frac{\epsilon\lambda_m}{c}}\ln t{\rm d t}+\frac{1}{\lambda_m}\int_{\frac{\epsilon\lambda_m}{c}-1}^{\frac{\epsilon\lambda_m}{c}}\ln t{\rm d}t\\
\leq \frac{\epsilon}{c}\ln(\frac{\lambda_m}{c})-\frac{1}{\lambda_m}\int_{1}^{\frac{\epsilon\lambda_m}{c}}\ln t{\rm d t}+\frac{\ln(\frac{\epsilon\lambda_m}{c})}{\lambda_m}\\
=\frac{\epsilon}{c}\ln(\frac{\lambda_m}{c})+\frac{\ln(\frac{\epsilon\lambda_m}{c})}{\lambda_m}-\frac{1}{\lambda_m}(\frac{\epsilon\lambda_m}{c}\ln(\frac{\epsilon\lambda_m}{c})-1)\\
=\frac{\epsilon}{c}\ln \epsilon+\frac{1+\ln(\frac{\epsilon\lambda_m}{c})}{\lambda_m}
$$
由于$\epsilon$在$m$之前决定,因此$\lambda_m\to \infty$的时候可以让后者趋近于$0$,此时再让$\epsilon\to 0$可以让前者为$0$,这就搞定了.

###### Example4(黎曼引理)

求证:$f\in C[a,b]$,$\lim_{n\to \infty}\int_a^b f(x)\sin(nx){\rm d}x=0$.

当$f\in C^1[a,b]$的时候,由于:

$$
\int_a^b f(x)\sin(nx){\rm d}x\\
=-\frac{1}{n}\int_a^b f{\rm d}\cos (nx)\\
=-\frac{1}{n}f\cos(nx)|_a^b+\frac{1}{n}\int_a^b \cos(nx)f'{\rm d}x
$$

两边取绝对值然后让$n\to \infty$就搞定.

既然如此,考虑当$f\in C[a,b]$的时候,用多项式逼近$f$,有$ |f-P|<\epsilon$,当然:

$$
\varlimsup_{n\to \infty}|\int_a^b f\sin (nx){\rm d}x|\\
\leq \varlimsup_{n\to \infty}|\int_a^b |f-P|\sin (nx){\rm d}x|\\
\leq \epsilon(b-a)
$$

###### Example5

当$f\in C[a,b]$,如果$\forall n\in \mathbb{N}$,$\int_a^b f x^n{\rm d}x=0$,求证$f\equiv 0$.

也就是任意多项式$P$都有$\int_a^b fP{\rm d}x=0$.取$P$使得$|P-f|<\epsilon$,于是:

$$
|\int_a^b f^2{\rm d}x|=|\int_a^bfP{\rm d}x+\int_a^b f(f-P){\rm d}x|\\
=|\int_a^b f(f-P){\rm d}x|\\
\leq \epsilon\int_a^b |f|{\rm d}x
$$

所以$\int_a^b f^2{\rm d}x=0$所以$f\equiv 0$.

#### 傅里叶级数

定义**三角系**为形如$\{1,\cos(nx),\sin(nx)\}$的集合,它们张成的线性空间中,也就是有限的线性组合$\sum_{k=0}^n a_k\cos(kx)+b_k\sin(kx)$称为$n$阶**三角多项式**.当$n\to \infty$的时候则称其为**三角级数**.容易见到$f(x)\equiv f(x+2\pi)$.

下面我们来证明三角系两两正交,只需检查:

1. 当$m\ne n$的时候,$\int_{-\pi}^{\pi}\cos(mx)\cos(nx){\rm d}x$.
2. 当$m\ne n$的时候,$\int_{-\pi}^{\pi}\sin(mx)\sin(nx){\rm d}x$.
3. $\forall n,m\in \mathbb{Z}$,$\int_{-\pi}^{\pi}\sin(nx)\cos(mx){\rm d}x=0$.
4. $\int_{-\pi}^{\pi}\cos^2(nx){\rm d x}=\int_{-\pi}^{\pi}\sin^2(nx){\rm d x}=\pi$.

不妨设$\frac{a_0}{2}+\sum_{1}^\infty a_k\cos(kx)+b_k\sin(kx)\rightrightarrows f(x)$,假如$f(x)$的确一致收敛到某个函数,我们实际上可以用$f(x)$去反推系数,两边乘一些东西再积分应该见到:

1. $a_0=\frac{1}{\pi}\int_{-\pi}^{\pi}f(x){\rm d}x$.
2. $a_k=\frac{1}{\pi}\int_{-\pi}^{\pi}f(x)\cos(kx){\rm d}x$.
3. $b_k=\frac{1}{\pi}\int_{-\pi}^{\pi}f(x)\sin(kx){\rm d}x$.

应该是显然见到的.

当$f$在$[-\pi,\pi]$上绝对可积,也就是$\int_{-\pi}^\pi|f|{\rm d}x<\infty$,那我们就可以按照上述方法拿到一个三角级数的系数,这个三角级数被称为$f(x)$的**傅里叶级数**或者**傅里叶展开**.注意我们这里其实还并没有说这个三角级数的确收敛到$f(x)$,而只能说我们可以如此由$f(x)$构造出一个傅里叶级数.我们有时也将这样的函数记作$f(x)\in F[-\pi,\pi]$,不妨干脆将$f(x)$依照$f(x)\equiv f(x+2\pi)$的条件延拓$f(x)$定义(或者干脆要求$f(x)$是以$2\pi$为周期的函数),应当见到以下结论:

1. $\lim_{n\to \infty}a_n=\lim_{n\to \infty}b_n=0$.
2. $a_n(\alpha f+\beta g)=\alpha a_n(f)+\beta a_n(g)$.
3. $b_n(\alpha f+\beta g)=\alpha b_n(f)+\beta b_n(g)$.
4. 如果$f(x)$是奇函数,则$a_n(f)\equiv 0$.
5. 如果$f(x)$是偶函数,则$b_n(f)\equiv 0$.
6. 记$f_h(x)=f(x+h)$,则$a_n(f_h)=a_n(f)\cos(nh)+b_n(f)\sin(nh)$.
7. 记$f_h(x)=f(x+h)$,则$b_n(f_h)=b_n(f)\cos(nh)-a_n(f)\sin(nh)$.
8. 如果$f(x)\equiv f(x+\pi)$,则$a_{2n-1}(f)=b_{2n-1}(f)$.
9. 对$\alpha\in (0,1]$,若$|f(x)-f(y)|\leq L|x-y|^\alpha,\forall x,y\in [-\pi,\pi]$,则$a_n=b_n=O(\frac{1}{n^\alpha})$.
10. 若$f$可导,$f'$绝对可积,则$a_n=b_n=o(\frac{1}{n})$.
11. 若$f$可导,$f'$绝对可积,则$a_n(f)=-\frac{b_n(f')}{n}$.
12. 若$f$可导,$f'$绝对可积,则$b_n(f)=\frac{a_n(f')}{n}$.
13. 若$f$可以导$m$次,而且$f^{(m)}$绝对可积,则$a_n=b_n=o(\frac{1}{n^m})$.
14. 若$f\in C(\mathbb{R}),f(x+2\pi)\equiv f(x)$,则$\forall \epsilon>0$,则存在一列三角多项式一致逼近$f$.
15. 若$f\in C(\mathbb{R}),f(x+2\pi)\equiv f(x)$,若$\forall n\geq 0,a_n\equiv b_n\equiv 0$,则$f\equiv 0$.

(1)是由于黎曼引理,我们前面已经用若干种方法反复证过了.

(2)(3)是积分线性的显然结果,(4)(5)只需带入定义即可发现.

(6)(7)的话当然是因为和差角公式.(8)的话根据(6)和(7)由于$a_n(f)=a_n(f_\pi)=a_n(f)(-1)^n$,这就搞定.

(9)的话,考虑:

$$
\begin{aligned}
a_n&=\frac{1}{\pi}\int_{-\pi}^{\pi}f(x)\cos(nx){\rm d}x\\
&=\frac{1}{\pi}\int_{-\pi-\frac{\pi}{n}}^{\pi-\frac{\pi}{n}}f(x+\frac{\pi}{n})\cos(nt+\pi){\rm d}x\\
&=-\frac{1}{\pi}\int_{-\pi}^{\pi}f(x+\frac{\pi}{n})\cos(nt){\rm d}x\\
a_n&=\frac{1}{2\pi}\int_{-\pi}^{\pi}(f(x)-f(x+\frac{\pi}{n}))\cos(nx){\rm d}x\\
|a_n|&\leq \frac{1}{2\pi}\int_{-\pi}^{\pi}L(\frac{\pi}{n})^\alpha\\
&=O(1)\frac{1}{n^\alpha}
\end{aligned}
$$

这就搞定.

至于(10)的话,$a_n=b_n=O(\frac{1}{n})$当然只是(9)的推论.那怎么证明是$o(\frac{1}{n})$的呢?考虑:

$$
\begin{aligned}
a_n&=\frac{1}{\pi}\int_{-\pi}^{\pi}f\cos(nx){\rm d}x\\
&=\frac{1}{n\pi}\int_{-\pi}^{\pi}f{\rm d}\sin(nx)\\
&=-\frac{1}{n\pi}\int_{-\pi}^{\pi}\sin(nx)f'{\rm d}x\\
&=o(\frac{1}{n})
\end{aligned}
$$

上述过程也自然拿到了(11)和(12).

(13)的话也只需要由(11)和(12)拿到:

$$
|a_n(f)|+|b_n(f)|\leq \frac{|a_n(f^{(m)})|+|b_n(f^{(m)})|}{n^m}
$$

其实还有一个推论是如果$f$是$2\pi$周期的解析函数,则$|a_n(f)|,|b_n(f)|\leq Me^{-nc}$.而且甚至说这个条件是充要的,如果趋向于零的速度确实为此,则$f$也的确是解析函数.可惜的是这个结论需要用到复分析的结果,不得不在此略过.

(14)的话,乍一看像是Stone-Weierstrass定理的另一则应用,直接观察Stone-Weierstrass定理的证明发现的确可以照搬到三角多项式上(因为难点只在$f(-\pi)=f(\pi)$,但原证明中分离的性质只用于贴合函数上的两个点),然而如果想要直接封装原定理拿过来的话的确难以处理$f(x)\equiv f(x+2\pi)$的性质(其实可以从复平面绕过去).下面提供另一则证明:

先看当$f(x)$是偶函数的时候,观察$x\in [0,\pi]$,取$y=\cos x$,见到$f(\arccos y)\in C[-1,1]$.此时存在一个多项式$P(y)=P(\cos x)$一致逼近$f(x)$,而$P(\cos x)$当然是三角多项式.

考虑取$g(x)=f(x)+f(-x)$,则如上知道$\forall \epsilon>0$,存在三角多项式$T_1(x)$使得$\forall x\in\mathbb{R},|g(x)-T_1(x)|<\frac{\epsilon}{2}$.

再取$\varphi(x)=(f(x)-f(-x))\sin x$当然也是偶函数,存在三角多项式$T_2(x)$使得$\forall x\in\mathbb{R},|\varphi(x)-T_2(x)|<\frac{\epsilon}{2}$.

既然如此,考虑取$T_3(x)=T_1\sin^2 x+T_2\sin x$,由上见到$|2f(x)\sin^2 x-T_3(x)|<\epsilon$,也就是说对于任何$f(x)$都能找到如此一个$T_3(x)$满足条件.

既然如此,考虑对$f(x-\frac{\pi}{2})$做上述操作,也就是存在三角多项式$T_4(x)$满足:

$$
\begin{aligned}
|2f(x-\frac{\pi}{2})\sin^2 x-T_4(x)|<\epsilon\\
|2f(x)\cos^2 x-T_4(x+\frac{\pi}{2})|<\epsilon
\end{aligned}
$$

既然如此,得到:$|2f(x)-T_3(x)-T_4(x+\frac{\pi}{2})|<2\epsilon$,这就搞定了.



对于(15),考虑找一列三角多项式一致逼近$f(x)$,由于$f(x)$的傅里叶级数总是$0$,所以对于三角多项式$T_n$来说一定有$\int_{-\pi}^{\pi}f(x)T_n(x){\rm d}x\equiv 0$,既然如此考虑:

$$
\begin{aligned}
\int_{-\pi}^{\pi}f^2{\rm d}x&=\int_{-\pi}^\pi f(f-T_n){\rm d}x\\
&\leq \int_{-\pi}^\pi |f|\sdot|f-T_n|{\rm d}x\\
&\leq \epsilon \int_{-\pi}^\pi |f|{\rm d}x
&\to 0
\end{aligned}
$$

因此$f\equiv 0$.这意味着连续函数的傅里叶展开是唯一的.

(15)的一则类似结果是,你可以从中看出$\{1,\sin (nx),\cos(nx)\}$三角系是缺一不可的,而不像多项式那样可以去掉很多.例如如果我们在其中去掉$\sin x$的话,那么$\sin x$本身就无法逼近,原因是反证法,如果能逼近,考虑一列三角多项式$\{T_n\}$并且其中不含$\sin x$,然而,由于三角系中两两正交,因此$\forall T_n,$都有$\int_{-\pi}^{\pi}T_n \sin x{\rm d}x\equiv 0$,因此从(15)的证明过程就可以见到必然有$\sin x\equiv 0$,这当然就出事了.

###### Example1

设$x\in [-\pi,\pi]$,考虑一列$\{b_n\}$单调下降趋近于$0$,那么根据DA判法当然有$S(x)=\sum_{n=1}^\infty b_n \sin(nx)$是逐点收敛的,假设收敛于$f(x)$.如果$f(x)$的确是$[-\pi,\pi]$上的绝对可积函数.求证:此时上述$S(x)$的确是$f(x)$的傅里叶级数.

容易见到$f(x)$是奇函数,其傅里叶级数中的$a_n\equiv 0$.此时不妨只看$x\in[0,\pi]$上的结果.

难点当然在于$S(x)\to f$并不一定是一致逼近,但应该观察到的是:

$$
|\sin(mx)\sum_{k=1}^n \sin(kx)|\leq \left|\frac{\sin(mx)}{\sin \frac{x}{2}}\right|\leq \left|\frac{mx}{\frac{x}{\pi}}\right|=|m\pi|
$$

所以这个一致有界,前面的$|b_n|$单调趋近于$0$,DA判法告诉我们$\sin(mx)S(x)$一致收敛,那逐项积分就证完了$\int_{-\pi}^{\pi}g(x)\sin(mx){\rm d}x=\pi b_m$.

###### Example2

求$f(x)=\frac{1-r^2}{1-2r\cos x+r^2},|r|<1$的傅里叶级数.

考虑在复数上做此操作,留神到:

$$
\frac{1}{1-z}=\frac{1-\bar z}{(1-z)(1-\bar z)}\\
=\frac{1-\bar z}{1-2\Re z+|z|^2}\\
=\frac{1-r\cos x+ir\sin x}{1-2r\cos x+r^2}
$$

所以$2\Re(\frac{1}{1-z})-1=f(x)$,然而:

$$
\frac{1}{1-z}=\sum_{n\geq 0}z^n=\sum_{n\geq 0}r^ne^{inx}\\
\Re(\frac{1}{1-z})=1+\sum_{n\geq 1}r^n\cos(nx)\\
$$

所以$f(x)=1+\sum_{n\geq 1}2r^n\cos(nx)$.

##### 与原函数的收敛关系

做完以上操作后,我们来考虑这个傅里叶级数到底是否逼近于$f(x)$本身,设$S_n(x)=\frac{a_0}{2}+\sum_{k=1}^n(a_k\cos(kx)+b_k\sin(kx))$.此时考虑:

$$
\begin{aligned}
S_n(x_0)&=\frac{a_0}{2}+\sum_{k=1}^n \int_{-\pi}^\pi f(x)(\cos kx\sdot \cos kx_0+\sin kx\sdot \sin kx_0){\rm d}x\\
&=\frac{1}{2\pi}\int_{-\pi}^\pi f(x){\rm d}x+\sum_{k=1}^n\int_{-\pi}^\pi f(x)\cos k(x-x_0){\rm d}x\\
&=\frac{1}{\pi}\int_{-\pi}^\pi f(x)\left(\frac{1}{2}+\sum_{k=1}^n \cos k(x-x_0)\right){\rm d}x\\
&=\frac{1}{\pi}\int_{-\pi}^\pi f(x)\frac{\sin(n+\frac{1}{2})(x-x_0)}{2\sin\frac{x-x_0}{2} } {\rm d}x\\
\end{aligned}
$$

不妨令迪利克雷核$D_n(x)=\frac{\sin(n+\frac{1}{2})x}{2\sin\frac{x}{2}},\varphi(x_0,x)=\frac{f(x_0+x)+f(x_0-x)}{2}$,应当见到$\frac{1}{\pi}\int_{-\pi}^\pi D_n(x){\rm d}x=1$,则:

$$
\begin{aligned}
S_n(x_0)&=\frac{1}{\pi}\int_{-\pi}^\pi f(x)D_{n}(x-x_0) {\rm d}x\\
&=\frac{1}{\pi}\int_{-\pi}^\pi f(t+x_0)D_{n}(t) {\rm d}t\\
&=\frac{1}{\pi}\int_0^\pi (f(x_0+t)+f(x_0-t))D_n(t){\rm d}t\\
&=\frac{2}{\pi}\int_0^\pi \varphi(x_0,t)D_n(t){\rm d}t
\end{aligned}
$$

问题在于最后如何判断这个积分的收敛性,事实上可以找到若干个逼近的充分条件:

1. 分段可导时候的情况

下面我们来证明:当$f$是Holder连续的时候,$\lim_{n\to \infty}S_n(x)\to f(x)$.

我们有如下**黎曼局部化引理**:对$\forall 0<\delta<\pi$,都有$\lim_{n\to \infty}\left(S_n(x_0)-\frac{2}{\pi}\int_0^\delta \varphi(x_0,t)\frac{\sin(n+\frac{1}{2})t}{t} {\rm d}t\right)=0$.

如何证明此呢?考虑:

$$
\begin{aligned}
\frac{\pi}{2}S_n(x_0)&=\int_0^\delta \varphi(x_0,t)D_n(t){\rm d}t+\int_\delta^\pi \varphi(x_0,t)D_n(t){\rm d}t\\
&=\int_0^\delta \varphi(x_0,t)\frac{\sin(n+\frac{1}{2})t}{t}{\rm d}t+\int_0^\delta \varphi(x_0,t)\frac{t-2\sin\frac{t}{2}}{2t\sin\frac{t}{2}}\sin(n+\frac{1}{2})t{\rm d}t\\&+\int_\delta^\pi \frac{\varphi(x_0,t)}{2\sin\frac{t}{2}}\sin(n+\frac{1}{2})t{\rm d}t\\
\end{aligned}
$$

留神到$\frac{t-2\sin\frac{t}{2}}{2t\sin\frac{t}{2}}$极限存在,因此其的确有界,对后面两项用黎曼引理就知道趋近于$0$.

接下来我们还有一个引理:当$\varphi\in R[0,\delta]$的时候,如果$\varphi(0+)$存在,并且$\int_0^\delta \frac{|\varphi(t)-\varphi(0+)|}{t}<\infty$,则$\lim_{\lambda\to \infty}\int_0^\delta\varphi(t)\frac{\sin\lambda t}{t} {\rm d}t=\frac{\pi}{2}\varphi(0+)$.

于此证明,根据黎曼引理:

$$
\begin{aligned}
\lim_{\lambda\to \infty}\int_0^\delta\frac{\varphi(t)-\varphi(0+)}{t}\sin\lambda t{\rm d}t&=0\\
\lim_{\lambda\to \infty}\int_0^\delta\frac{\varphi(t)}{t}\sin\lambda t{\rm d}t&=\lim_{\lambda\to \infty}\int_0^\delta\frac{\varphi(0+)}{t}\sin\lambda t{\rm d}t\\
&=\varphi(0+)\lim_{\lambda\to \infty}\int_0^{\lambda \delta}\frac{\sin t}{t}{\rm d}t\\
&=\frac{\pi}{2}\varphi(0+)
\end{aligned}
$$

既然如此,当$f$是霍尔德连续,也就是$\exists \alpha\in (0,1],\exists C>0$,使得$|f(x_0+t)-f(x_0)|\leq C|t|^\alpha$,此时见到$|\varphi(x_0,t)-\varphi(x_0,0+)|=O(|t|^\alpha)$,用上述引理知道此时$S_n(x_0)\to f(x_0)$.

上述结论还可以得到一个更神秘的结论,我们上述引理只用到了在$x_0$附近的结果,那如果$f$在$x_0$附近不连续呢?

假设$f(x_0\pm 0)$均存在,我们可以定义**广义导数**,定义右广义导数为$D_+ f(x_0)=\lim_{h\to 0+}\frac{f(x_0+h)-f(x_0+0)}{h}$,同理定义左广义导数.

当$D_\pm f(x_0)$存在的时候,考虑此时$\frac{|\varphi(t)-\varphi(0+)|}{t}$没有瑕点,那上述结论当然也就满足.

如果我们定义分段可导为除了若干个断点处的左右广义导数不相同,其余所有点的左右广义导数都存在.那上述结论也就满足.

2. 分段单调时候的情况

我们有**Jordan引理**:设$\varphi$在$[0,\delta]$上单调,则$\lim_{\lambda\to \infty}\int_0^\delta \frac{\varphi(x)}{x}\sin(\lambda x){\rm d}x=\frac{\pi}{2}\varphi(0+)$.

对Jordan引理的证明,不妨设$\varphi$单调递增,对$\forall \epsilon>0,\exists 0<\delta'<\delta$,使得$\forall t\in (0,\delta'),0\leq \varphi(t)-\varphi(0+)< \epsilon$.

不妨设$|\int_0^t\frac{\sin x}{x}|\leq M,\forall t>0$,考虑根据积分第二中值定理:

$$
|\int_0^{\delta'}(\varphi(t)-\varphi(0+))\frac{\sin (\lambda t)}{t} {\rm d}t|\\
=|(\varphi(\delta')-\varphi(0+))\int_\xi^{\delta'}\frac{\sin (\lambda t)}{t} {\rm d}t|\\
\leq 2M\epsilon
$$

而在$[\delta',\delta]$上可以直接用黎曼引理这就搞定了.

如果$\varphi$是有限个单调函数的和,上述结论当然也成立了.所以该结论对有界变差函数是成立的.

因此,当$f\in [-\pi,\pi]$上逐段单调的时候,$\varphi$逐段是两个单调函数的差,那此时当然也有$\lim_{n\to \infty}S_n(x)=\frac{f(x+0)+f(x-0)}{2}$.

###### Example1

设$f(x)=\begin{cases}-1&x\in (-\pi,0)\\0&x=0\lor x=-\pi\lor x=\pi\\1&x\in (0,\pi)\end{cases}$.

见到这的确是一个奇函数,直接求它的傅里叶级数,见到是:

$$
\frac{4}{\pi}\sum_{n=0}^\infty \frac{\sin(2n+1)x}{2n+1}=\frac{f(x+0)+f(x-0)}{2}=f(x)
$$

我们竟然得到了一个一个神秘级数,很牛.

而且这个三角级数在$(0,\pi)$上处处收敛到$1$,这个性质在幂级数的时候是无法想象的,我们之前证明过了开区间上的非常值幂级数的零点集一定没有聚点.

###### Example2

设$f(x)=\begin{cases}\pi+x&x\in [-\pi,0)\\ \pi-x & x\in [0,\pi)\end{cases}$,也就是一个锯齿状的小函数,当然分段单调了.由于是一个偶函数,积分得到$a_n=\frac{2}{\pi}\int_0^\pi (\pi-x)\cos(nx){\rm d}x=\frac{2}{n^2\pi}(1-(-1)^n)$.

这当然是个连续函数,那此时有:

$$
f(x)=\frac{\pi}{2}+\frac{4}{\pi}(\sum_{k=0}^\infty\frac{\cos(2k+1)x}{(2k+1)^2})
$$

两边带入$x=0$就知道$\sum_{k=0}^\infty\frac{1}{(2k+1)^2}=\frac{\pi^2}{8}$.

还没完,用大M判别法容易检验这个式子两边是绝对一致收敛的,那我们两边逐项积分得到:

$$
\int_0^x f(t){\rm d}t=\frac{\pi}{2}x+\frac{4}{\pi}\sum_{k=0}^\infty \int_0^x \frac{\cos(2k+1)t}{(2k+1)^2} {\rm d}t\\
=\frac{\pi}{2}x+\frac{4}{\pi}\sum_{k=0}^\infty \frac{\sin(2k+1)t}{(2k+1)^3}\\
$$

把左边的$f$积出来,设其为$g(x)=\begin{cases}\pi x+\frac{x^2}{2}&x\in [-\pi,0)\\ \pi x-\frac{x^2}{2} & x\in [0,\pi)\end{cases}$.那它的傅里叶展开就等于右边的东西(因为一致收敛).带入$x=\frac{\pi}{2}$,可以得到$\frac{\pi^3}{32}=\sum_{k=0}^\infty\frac{(-1)^k}{(2k+1)^3}$.

那接下来还可以两边再积分对吧!过程我也懒得写了,总之能推出$\sum_{k=0}^\infty \frac{1}{(2k+1)^4}=\frac{\pi^4}{96}$,可以得到$\sum_{k=1}^\infty \frac{1}{k^4}=\frac{\pi^4}{90}$.

所以总之,我们可以用上面的策略得到所有的$\zeta(2n)$.

###### Example3

考虑对$\cos (ax)$在$[-\pi,\pi]$截取出来并延拓,然后对其作傅里叶展开,得到:

$$
\cos(ax)=\frac{\sin(a\pi)}{a\pi}+\sum_{n=1}^\infty \frac{(-1)^n 2a}{a^2-n^2}\frac{\sin(a\pi)}{\pi}\cos(nx)\\
\frac{1}{\sin(a\pi)}=\frac{1}{a\pi}+\sum_{n=1}^\infty \frac{(-1)^n 2a\pi}{(a\pi)^2-(n\pi)^2}
$$

令$z=a\pi$,得到:

$$
\frac{1}{\sin z}=\frac{1}{z}+\sum_{n=1}^\infty(-1)^n (\frac{1}{z-n\pi}+\frac{1}{z+n\pi})
$$

###### Example4

假设$\int_{-\infty}^{+\infty}|f|{\rm d}x<\infty$,且存在$x_0,\delta$,$f(x_0\pm 0)$存在,使得以下二者其一成立:

1. $f$在$(x_0-\delta,x_0+\delta)$上是有界变差的.
2. 积分$\int_0^\delta \frac{|f(x_0\pm t)-f(x_0\pm 0)|}{t}<\infty$.

则:

$$
\lim_{\alpha\to \infty}\frac{1}{\pi}\int_{-\infty}^{+\infty} f(x_0+t)\frac{\sin(\alpha t)}{t}{\rm d}t =\frac{1}{2}(f(x_0+0)+f(x_0-0))
$$

只需要证明:

$$
\lim_{\alpha\to \infty}\int_{0}^\infty (f(x_0+t)-f(x_0+0))\frac{\sin(\alpha t)}{t}{\rm d}t=0
$$

首先$[0,+\infty)$上,对于$[0,\delta]$只需要正常做就行了.可是$\frac{f(x_0+t)-f(x_0+0)}{t}$在$(\delta,+\infty)$上未必可积,这就有点难绷.开动脑筋,想起来既然这里有一个$\mathbb{R}$上的绝对可积,那当然得用一用我们的柯西准则,考虑将后面拆成$(\delta,M)$和$[M,+\infty)$.如果我们能把后面的$[M,+\infty)$搞定,那中间那一段用黎曼引理就搞定了.

后面怎么做呢,简单放缩:

$$
|\int_{M}^\infty (f(x_0+t)-f(x_0+0))\frac{\sin(\alpha t)}{t}{\rm d}t|\\
\leq|\int_{M}^\infty f(x_0+t)\frac{\sin(\alpha t)}{t}{\rm d}t|+|\int_{M}^\infty f(x_0+0)\frac{\sin(\alpha t)}{t}{\rm d}t|\\
\leq \int_{M}^\infty |f(x_0+t)|{\rm d}t+|f(x_0+0)\int_{M}^\infty \frac{\sin(\alpha t)}{t}{\rm d}t|\\
\to 0
$$

这就搞定了.

##### 一致收敛到原函数的情形

那此时该函数肯定连续,然而连续函数未必总能一致逼近,Example1给出了一个反例.

那么应该加如何条件呢?先证明一个加强版的黎曼引理:

设$g(x)$是$[A,B]$上的绝对可积函数,取$S_p=\sup_{a,b\in [A,B]}|\int_a^b g(t)\sin pt{\rm d}t|+\sup_{a,b\in [A,B]}|\int_a^b g(t)\cos pt{\rm d}t|$,我们要证明$\lim_{p\to \infty}S_p=0$.这个还并不显然,因为$a,b$是可以在$p$之后决定的.实际上这里就是为了拿到一个一致控制性,我们想让我们的黎曼引理是能一致控制住整个区间的,这样我们才能得到一致收敛的条件.

首先考虑$\forall \epsilon>0$,当$g(x)\in R[a,b]$的时候,可以先用连续函数逼近,再将连续函数变为多项式逼近.而当$g(x)$有瑕点的时候,可以将瑕点逼掉.所以总能找到一个多项式$f(x)$满足$\int_A^B|g(x)-f(x)|{\rm d}x<\frac{\epsilon}{2}$.然后考虑:

$$
\begin{aligned}
\int_a^b f\sin (pt){\rm d} t&=\frac{1}{p}f\cos(pt)|_a^b+\frac{1}{p}\int_a^b \cos(pt)f'{\rm d}t\\
&\to 0
\end{aligned}
$$

由此还可以再推出一个引理,$\lim_{p\to \infty}\sup_{A\leq x_0\pm a,x_0\pm b\leq B}|\int_a^b g(x_0\pm t)\cos(pt){\rm d}t|=0$.原因是换元后用和差角公式拆开就行.

进一步推广,考虑$\gamma(t)$是$[a,b]$上的有界变差函数,我们证明:$\lim_{p\to \infty}\sup_{A\leq x_0\pm a,x_0\pm b\leq B}|\int_a^b g(x_0\pm t)\gamma(t)\cos(pt){\rm d}t|=0$.

不妨设$\gamma(t)$单调递增,然后用积分第二中值定理就可以发现:

$$
\int_a^b g(x_0\pm t)\gamma(t)\sin(pt){\rm d}t\\
=\gamma(a)\int_a^\xi g(x_0\pm t)\sin (pt){\rm d}t+\gamma(b)\int_\xi^b g(x_0\pm t)\sin (pt){\rm d}t\\
\to 0
$$

接下来设$\varphi(t)=f(x+t)+f(x-t)-2f(x)$,假设$f$是$[-\pi,\pi]$上的绝对可积函数,而且$f$限制在子区间$[a,b]$上是连续的.我们有**迪尼定理**:若$\forall \epsilon>0,\exists \delta>0,\sup_{x\in [a,b]}\int_0^\delta \frac{|\varphi(t)|}{t} {\rm d}t<\epsilon$,则其傅里叶级数$S_n(x)$在区间$[a,b]$上一致逼近$f$.

考虑:

$$
S_n(x)-f(x)=\frac{1}{\pi}\int_0^\pi \varphi(t)\frac{\sin(n+\frac{1}{2})t}{2\sin \frac{t}{2}}{\rm d}t\\
=\frac{1}{\pi}(\int_0^\delta+\int_\delta^\pi)\varphi(t)\frac{\sin(n+\frac{1}{2})t}{2\sin \frac{t}{2}}{\rm d}t\\
$$

考虑前者:

$$
\int_0^\delta \leq \frac{1}{\pi}\int_0^\delta \frac{|\varphi(t)|}{t}\frac{\frac{t}{2}}{\sin \frac{t}{2}}{\rm d}t\\
\leq \frac{1}{2}\int_0^\delta \frac{|\varphi(t)|}{t}{\rm d}t\\
<\frac{\epsilon}{2}
$$

至于后者,只需上述引理就搞定.

于是最后我们可以得到,如果$f$是Holder连续的时候,那对于任意的$[a,b]$,都一定有$S_n\rightrightarrows f(x)$.

而如果当$f$是有界变差的时候,此时考虑$f$几乎处处可导,而在其可导的地方当然由上面的结果就一致收敛了,所以可以知道这个函数列几乎处处一致收敛到$f$.

然而容易检验如果一个连续函数列几乎处处一致收敛到一个连续函数,那当然它们就是一致收敛的关系.考虑反证,$\forall \epsilon>0$,设可导点集为$A$,则$\exists n,\sup_{x\in A}(f(x)-f_n(x))<\epsilon$.

假设不一致收敛,此时$\exists x_0$使得$f(x_0)-f_n(x_0)>4\epsilon$.然而由于$n$已经定死,所以现在$f$和$f_n$都是一致连续的,往旁边区间绕一下就行了.不管这个区间多小,由于$[-\pi,\pi]\setminus A$是零测的,所以总有与$A$的交.

从上面也可以看到,只要一个连续函数几乎处处可导,那它的傅里叶级数就可以一致逼近它.

###### Example1

设:

$$
Q_{m,n}&=\sum_{k=1}^n \frac{\sin(m+n-k)x}{k}-\sum_{k=1}^n \frac{\sin(m+n+k)x}{k}\\
&=-2\cos(m+n)x \sum_{v=1}\frac{\sin(vx)}{v}
$$

容易检查这个函数是有界的,只需:

$$
\sum_{v=1}^n \frac{\sin vx}{v}&=\int_0^x \sum_{v=1}^n \cos vt {\rm d}t\\
&=\int_0^x (-\frac{1}{2}+\frac{\sin (n+\frac{1}{2})t}{2\sin \frac{t}{2}})\\
&=-\frac{x}{2}+\int_0^x (\frac{1}{2\sin \frac{t}{2}}-\frac{1}{t})+\int_0^{(n+\frac{1}{2})x}\frac{\sin t}{t}{\rm d}t
$$

这样用黎曼引理立刻见到其有界,存在常数$C$使得$|\sum_{v=1}^n \frac{\sin vx}{v}|\leq C$,那$|Q_{n,m}|\leq 2C$也有界.

定义$m_k=n_k=2^{k^3}$,取函数$\varphi(x)=\sum_{k=1}^\infty \frac{1}{k^2}Q_{m_k,n_k}(x)$,既然$Q_{m_k,n_k}$有界,根据大M判别法,这个东西当然一致收敛而且$2\pi$周期连续.

既然如此,考察$\varphi(x)$是奇函数,它的傅里叶展开的确就是上述形式(原因是连续函数有唯一傅里叶展开).

接下来考虑其波动考虑由于我们的$m_k=n_k=2^{k^3}$,所以其实这里很稀疏的,留神到:

$$
\begin{aligned}
S_{2n_k-1}(x)-S_{n_k-1}(x)&=\frac{1}{k^2}\left(\sum_{l=1}^{n_k}\frac{\sin (2n_k-l)x}{l}\right)
\end{aligned}
$$

接下来带入$x=\frac{\pi}{4n_k}$,见到:

$$
\begin{aligned}
(S_{2n_k-1}-S_{n_k-1})(\frac{\pi}{4n_k})&> \frac{\sqrt 2}{2}\frac{1}{k^2}(\sum_{l=1}^{n_k}\frac{1}{l})\\
&>\frac{\sqrt 2}{2}\frac{1}{k^2}\ln n_k\\
&>\frac{\sqrt 2\ln 2}{2}k\\
&\to \infty
\end{aligned}
$$

因此见到不一致收敛,很厉害!

##### Cesaro收敛

接下来我们引入**Cesaro收敛**的概念,回忆到Stolz定理给出若取定$\sigma_n=\frac{a_1+\cdots+a_n}{n}$,回忆到如果$a_n\to a$,则$\sigma_n\to a$,然而反之未必成立.因此,$\sigma_n$如果收敛的话,这是一个比$a_n$收敛弱的条件.

在傅里叶级数这里也引入这个概念,我们不再研究$S_n(x)$的收敛性,而考虑$\sigma_n(x)=\frac{S_0(x)+\cdots+S_n(x)}{n+1}$的收敛性.考虑:

$$
\sigma_n(x)=\frac{S_0(x)+\cdots+S_n(x)}{n+1}\\
=\frac{1}{(n+1)\pi}\int_{-\pi}^\pi  f(x+t)\frac{\sum_{k=0}^n \sin(k+\frac{1}{2})t}{2\sin \frac{t}{2}} {\rm d}t\\
=\frac{1}{(n+1)\pi} \int_{-\pi}^\pi f(x+t)\frac{ 1-\cos(n+1)t}{2\sin^2 \frac{t}{2}} {\rm d}t\\
=\frac{1}{(n+1)\pi} \int_{-\pi}^\pi f(x+t)\frac{ \sin^2(\frac{n+1}{2}t)}{2\sin^2 \frac{t}{2}} {\rm d}t\\
$$

设费叶核$\phi_n(t)=\frac{\sin^2\frac{n+1}{2}t}{2(n+1)\sin^2 \frac{t}{2}}$.容易见到$\phi_n(t)=\phi_n(-t)$,$\phi(0)=\frac{n+1}{2}$,而且$\frac{1}{\pi}\int_{-\pi}^\pi \phi_n(t){\rm d}t=1$,原因是$\phi_n(t)$原本是个三角多项式,因此这里的积分只和其常数项有关,而常数项为$\frac{1}{2}$.

现在我们说,当$f(x)\equiv f(x+2\pi)$,$f$在$[-\pi,\pi]$上绝对可积,并且$f(x_0\pm 0)$极限存在,则$\lim_{n\to \infty}\sigma_n(x_n)=\frac{f(x_0+0)+f(x_0-0)}{2}$.进一步,如果$f\in C(\mathbb{R})$,则$\sigma_n(x)\rightrightarrows f(x)$.

考虑设$I_n=\sigma_n(x)-\frac{f(x+0)+f(x-0)}{2}$,有:

$$
I_n=\frac{1}{\pi}\int_{-\pi}^\pi\phi_n(t)(f(x+t)+f(x-t)-f(x+0)-f(x-0)){\rm d}t
$$

此时考虑$f(x\pm 0)$存在意味着$\forall \epsilon>0,\exists \delta>0$,使得当$0<t\leq \delta$的时候,总有$|f(x\pm t)-f(x\pm 0)|<\frac{\epsilon}{2\pi}$,于是我们上面的$I_n$就可以拆成$\int_0^\delta+\int_\delta^\pi$两部分.

对于前者:

$$
\frac{1}{\pi}|\int_0^\delta|\leq \frac{\epsilon}{2\pi}\int_0^\pi \phi_n(t){\rm d}t<\frac{\epsilon}{2}
$$

对于后者只要能控制$\max_{[\delta,\pi]}\phi_n(t)$就行,而这是显然的,因为当$t\geq \delta$的时候,分母就有下界了,让$n\to \infty$的时候当然趋近于$0$.所以$\lim_{n\to \infty}I_n=0$.

接下来的疑问是当$f$连续时为何一致收敛,这当然是因为$f$是一致连续的,那我上述唯一涉及$x$的地方,也就是当$f(x\pm t)\to f(x)$的部分就可以被控制住,这就搞定了.

Cesaro收敛的另一个好处在于,当$f(x+0)$和$f(x-0)$存在的时候,由于如果逐点收敛一定就能导出Cesaro收敛,因此,如果傅里叶级数在某一点处收敛,则它必然也在这一点处Cesaro收敛且收敛于同一个$\frac{f(x+0)+f(x-0)}{2}$,换言之如果收敛则一定收敛于此.

##### 平均收敛

考虑如果一列函数$f_n$,如果$\int |f_n-f|\to 0$则我们称这列函数**平均收敛**到$f$.一个平均收敛的函数列可以处处点不收敛,一个逐点收敛的函数也可能并不平均收敛(因为积分不一定收敛).

考虑定义**平方可积函数**$f$为几乎处处连续而且只有有限个瑕点的$f$并且满足$\int_{-\pi}^\pi f^2{\rm d}x<\infty$.容易见到闭区间上的平方可积要强于绝对可积,原因正是柯西不等式:

$$
\int_{-\pi}^\pi |f|{\rm d}x\leq \sqrt{(\int_{-\pi}^\pi f^2{\rm d}x)(\int_{-\pi}^\pi 1{\rm d}x)}
$$

从而看到如果$\int (f_n-f)^2{\rm d}x\to 0$则当然有$\int |f_n-f|{\rm d}x\to 0$,因此我们往往计算前者.

假设$f$是平方绝对可积的,考虑定义$\mathcal{F}_n$为$n$阶三角多项式组成的集合,设$S_n$是$f$的傅里叶展开的前$n$项,我们有:

1. $\int_{-\pi}^\pi (f-S_n)^2{\rm d}x=\min_{T_n\in \mathcal{F}_n}\int_{-\pi}^\pi (f-T_n)^2{\rm d}x$.
2. **Bessel不等式**:$\frac{a_0^2}{2}+\sum_{n=1}^\infty(a_n^2+b_n^2)\leq \frac{1}{\pi}\int_{-\pi}^\pi f^2{\rm d}x$.
3. **Parseval等式**:$\frac{a_0^2}{2}+\sum_{n=1}^\infty(a_n^2+b_n^2)= \frac{1}{\pi}\int_{-\pi}^\pi f^2{\rm d}x$.
4. **广义Parseval等式**:设$f,g$都平方可积,则$\frac{1}{\pi}\int_{-\pi}^\pi fg{\rm d}x=\frac{a_0(f)a_0(g)}{2}+\sum_{n=1}^\infty(a_n(f)a_n(g)+b_n(f)b_n(g))$.


先看(1)和(2),假设$T_n=\frac{A_0}{2}+\sum_{k=1}^n A_k\cos(kx)+B_k\sin(kx)$,考虑:

$$
\int_{-\pi}^\pi (f-T_n)^2{\rm d}x\\
=\int_{-\pi}^\pi f^2{\rm d}x+\pi\left(\frac{A_0^2}{2}+\sum_{k=1}^n (A_k^2+B_k^2)\right)-2\pi(A_0\frac{a_0}{2}+\sum_{k=1}^n a_kA_k+b_kB_k)\\
=\int_{-\pi}^\pi f^2{\rm d}x+\pi\left(\frac{(A_0-a_0)^2}{2}+\sum_{k=1}^n (a_k-A_k)^2+(b_k-B_k)^2\right)-\pi\left(\frac{a_0^2}{2}+\sum_{k=1}^n a_k^2+b_k^2\right)\\
\geq \int_{-\pi}^\pi f^2{\rm d}x-\pi\left(\frac{a_0^2}{2}+\sum_{k=1}^n a_k^2+b_k^2\right)\\
$$

等号成立当且仅当$A_k=a_k,B_k=b_k$,这样上述两者就都证毕.

再看(3),由(1)和(2),只需要证明对于平方可积函数总存在一列三角多项式$T_n$使得$\lim_{n\to \infty}\int_{-\pi}^\pi (f-T_n)^2{\rm d }x=0$即可.

当$f$是黎曼可积的时候这里较为平凡,只需要先用连续函数$g$逼近$f$,再用三角多项式逼近$g$即可.最终因为有$(T-f)^2\leq 2(T-g)^2+2(f-g)^2$就行.顺便一提这个不等式还可以证明平方可积的函数的和差也一定是平方可积.

接下来要把瑕点给处理掉,由于平方可积,所以瑕点那里的积分会足够小,直接把那一段抹平成$0$就行.抹平了后的函数可以被逼近,而抹平带来的影响又很小,这就搞定了.

再看(4),对$(f+g)$和$(f-g)$分别做Parseval等式,然后用它们一作差就能证明此结论.

关于Bessel不等式和Parseval等式,一个更好的理解是它体现了正交基上的操作,Bessel不等式说对于一组正交基我们总有$\Vert u\Vert^2\leq \sum_k(u|e_k)^2$成立.

###### Example1

求证:函数$f,g$在$[-\pi,\pi]$上黎曼可积,则它们的傅里叶级数相同的充分必要条件是$\int_{-\pi}^\pi |f-g|{\rm d}x=0$.

先证明充分性,比较显然:

$$
|\int_{-\pi}^\pi(f-g)\sin(nx){\rm d}x|\\
\leq \int_{-\pi}^\pi |f-g|{\rm d}x
$$

再证必要性,也很显然,考虑如果傅里叶级数相同,则Parseval等式必有$\int f^2{\rm d}x=\int g^2{\rm d}x=\int fg{\rm d}x$,这说明$\int(f-g)^2{\rm d}x=0$,立刻推出$\int |f-g|{\rm d}x=0$.

###### Example2

求证:当$b_n$单调递减趋近于$0$的时候,如果$\sum \frac{b_n}{n}<\infty$,则$f(x)=\sum b_n\sin(nx)$在$[-\pi,\pi]$上绝对可积.

不妨设$x\in [-\pi,\pi]$,首先考虑:

$$
|\sum_{k=n+1}^{n+m}\sin(kx)|\\
=|\frac{\cos(n+m+\frac{1}{2})-\cos(n+\frac{1}{2})}{2\sin\frac{x}{2}}|\\
\leq \frac{1}{|\sin \frac{x}{2}|}
$$

此时考虑设$S_m=\sum_{k=n+1}^{m} \sin(kx)$,取$x\in [\frac{\pi}{n+1},\frac{\pi}{n}]$,用Abel变换立刻有:

$$
|\sum_{k=n+1}^{n+m}b_k\sin(kx)|\\
=|\sum_{k=n+1}^{n+m}b_k(S_{k}-S_{k-1})|\\
=|\sum_{k=n+1}^{n+m-1}(b_{k}-b_{k+1})S_{k}-b_{n+1}S_{n}+b_{n+m}S_{n+m}|\\
=|\sum_{k=n+1}^{n+m-1}(b_{k}-b_{k+1})S_{k}+b_{n+m}S_{n+m}|\\
\leq \frac{b_n}{|\sin \frac{x}{2}|}\\
\leq \frac{\pi}{x}b_{n+1}\\
\leq (n+1)b_n
$$

这里已经和$m$没有关系了,既然如此考虑取$B_n=\sum_{k=1}^n b_k$,自然有:

$$
\int_0^\pi |f|{\rm d x}=\sum_{n=1}^\infty \int_{\frac{\pi}{n+1}}^{\frac{\pi}{n}}|f|{\rm d}x\\
\leq \sum_{n=1}^\infty \int_{\frac{\pi}{n+1}}^{\frac{\pi}{n}}(B_n+(n+1)b_n){\rm d}x\\
=\sum_{n=1}^\infty \frac{\pi}{n(n+1)}(B_n+(n+1)b_n)\\
$$

此时只需检验$\sum \frac{B_n}{n(n+1)}$收敛就行了对吧.这个比较平凡:

$$
\sum_{n=1}^\infty \frac{B_n}{n(n+1)}=\sum_{n=1}^\infty \frac{1}{n(n+1)}\sum_{k=1}b_k\\
=\sum_{k=1}^\infty b_k\sum_{n=k}^\infty \frac{1}{n(n+1)}\\
=\sum_{k=1}^\infty \frac{b_k}{k}\\<\infty
$$

这就搞定了.

##### 逐项积分

设$f$绝对可积,其傅里叶级数为$\frac{a_0}{2}+\sum a_n\cos(nx)+b_n\sin(nx)$,考虑取$F(x)=\int_0^x (f(t)-\frac{a_0}{2}){\rm d}t$,容易检查$F(x)$是有界变差的,原因是$\sum|F(x_i)-F(x_{i-1})|=\sum|\int_{x_{i-1}}^{x_i}f(t){\rm d}t|\leq \int |f|$.

既然如此,它就会被它的傅里叶级数一致逼近,不妨记作$F(x)=\frac{A_0}{2}+\sum_{n=1}^\infty A_n\cos(nx)+B_n\sin(nx)$.

接下来考虑把这些系数都求出来,先令$x=0$求出$\frac{A_0}{2}+\sum_{n=1}^\infty A_n=0$,当$n\geq 1$的时候,留神到:

$$
A_n=\frac{1}{\pi}\int_{-\pi}^\pi F(x)\cos(nx){\rm d}x\\
=\frac{1}{\pi}F(x)\frac{\sin(nx)}{n}|_{-\pi}^\pi-\frac{1}{n\pi}\int_{-\pi}^\pi f(x)\sin(nx){\rm d}x\\
=-\frac{b_n}{n}
$$

注意这里用的分部积分是广义分部积分,我们曾经搞过.

同理$B_n=\frac{a_n}{n}$,补一个$x$的傅里叶级数进去,于是我们见到:

$$
\int_0^x f(t){\rm d}t=\frac{a_0}{2}x+\sum_{n=1}^\infty a_n\int_0^x \cos(nt){\rm d}t+b_n\int_0^x \sin(nt){\rm d}t
$$

也就是一个函数和傅里叶级数逐项积分后总相等.

##### 逐项微分

设$f$可导而且是$2\pi$周期的,并且$\int_{-\pi}^\pi |f'|{\rm d}x<\infty$,那由逐项积分那里的结论,自然有$a_0(f')=0,a_n(f')=nb_n(f),b_n(f')=-na_n(f)$,也就是当:$f=\frac{a_0}{2}+\sum a_n\cos(nx)+b_n\sin (nx)$,则$f'\sim \sum -na_n\sin(nx)+nb_n\cos(nx)$.

##### 收敛速度的估计

最后来分析一下它的收敛能力,不妨假设$f$是$2\pi$周期逐段可导函数,并且$\int_{-\pi}^{\pi}(f')^2{\rm d}x<\infty$,则$\max|f(x)-S_n(x)|=o(\frac{1}{\sqrt{n}})$,原因是由Parseval等式,有:

$$
\frac{1}{\pi}\int_{-\pi}^\pi (f')^2{\rm d}x=\frac{a_0(f')^2}{2}+\sum_{n=1}^\infty(a_n(f')^2+b_n(f')^2)
$$

也就是后面这个级数收敛对吧,那柯西准则给出:

$$
\sum_{k=n+1}^\infty (a_k(f')^2+b_k(f')^2)=o(1)
$$

现在来看误差项:

$$
|f(x)-S_n(x)|\leq |\sum_{k=n+1}^\infty a_k(f)\cos(kx)+b_k(f)\sin(kx)|\\
\leq \sum_{k=n+1}^\infty |a_k(f)|+|b_k(f)|\\
= \sum_{k=n+1}^\infty \frac{|a_k(f')|+|b_k(f')|}{k}\\
\leq \sqrt{\sum_{k=n+1}^\infty |a_k(f')^2|+|b_k(f')^2|}\sqrt{\sum_{k=n+1}^\infty \frac{1}{k^2}}\\
=o(1)\frac{1}{\sqrt n}
$$

##### 复数形式

考虑:

$$
f(x)\sim \frac{a_0}{2}+\sum a_n\cos(nx)+b_n\sin(nx)\\
=\frac{a_0}{2}+\sum a_n\frac{e^{inx}+e^{-inx}}{2}+\sum b_n\frac{e^{inx}-e^{-inx}}{2i}\\
=\sum_{n=-\infty}^\infty c_n e^{inx}
$$

此时有$c_n=\frac{1}{2\pi}\int_{-\pi}^\pi f(x)e^{-inx}{\rm d}x$.

##### 三角级数的唯一性问题

接下来来解决点看上去很平凡的问题,如果:

$$
\frac{a_0}{2}+\sum_{n=1}^\infty a_n\cos(nx)+b_n\sin(nx)=\frac{\tilde{a_0}}{2}+\sum_{n=1}^\infty \tilde{a_n}\cos(nx)+\tilde{b_n}\sin(nx)
$$

而且左右两端都收敛到某个函数.能否导出$a_n=\tilde{a_n},b_n=\tilde{b_n}$.注意这里并不能两遍乘个$\cos(nx)$然后积分,因为未必可积.

我们下面可以证明,如果上面这种相等除了至多可数个点以外都成立,那就是同一个三角级数.当然只需要证明当:

$$
\frac{a_0}{2}+\sum_{n=1}^\infty a_n\cos(nx)+b_n\sin(nx)=0
$$

至多除了一个可数集合$E=\{x_1,x_2,\cdots\}$以外成立的时候有$a_n=b_n=0$即可.

先来证明一个引理:此时$\lim_{n\to \infty}a_n=\lim_{n\to \infty}b_n=0$.这还并不显然,因为这个三角级数并不一定是傅里叶级数.

首先级数收敛,通项趋近于$0$,首先设$\rho_n=\sqrt{a_n^2+b_n^2}$,则$a_n\cos(nx)+b_n\sin(nx)=\rho_n\sin(nx+\varphi_n)$.现在只需要证明$\lim_{n\to \infty}\rho_n=0$.

反证,如果没有上述结论,则总存在一个无穷子列$\{\rho_{n_k}\}_k$大于某一个正常数$c$,设其指标集合为$F=\{n_1<n_2<\cdots\}$.

现在考虑$[0,2\pi]$上,找一个区间$I_1=[l_1,r_1]$使得其不含有$x_1$,由于$F$通项肯定趋近无穷大,总能找到一个$m_1\in F$,使得$|[m_1l_1,m_1r_1]|=m_1|I_1|>2\pi$.既然它的区间长度足够大,那其中总有一个子区间$J_1\subseteq I_1$,使得$\sin(m_1x+\varphi_{m_1})>\frac{1}{2}$在$J_1$上成立.那我当然可以再找一个子区间$I_2\subseteq J_1$使得$I_2$中不含有$x_2$,重复以上操作,这就是一个闭区间套.最后套出来了一个点$w\notin E$,此时$\sin(m_k w+\varphi_{m_k})>\frac{1}{2}$,自然有$\varphi_{m_k}\sin(m_k w+\varphi_{m_k})>\frac{c}{2}$,这立刻导出矛盾.

再来一个引理:假设$F(x)\in C(a,b)$,可数集合$E\subseteq (a,b)$.定义其**二阶差分**为$\Delta_h^2F(x)=F(x+2h)+F(x-2h)-2F(x)$.已知其**广义二阶导数**$\delta^2 F=\varlimsup_{h\to +0}\frac{\Delta_h^2 F(x)}{4h^2}\geq 0,\forall x\notin E$,而$\varlimsup_{h\to +0}\frac{\Delta_h^2 F(x)}{h}\geq 0,\forall x\in E$.则结论是$F(x)$是一个下凸函数.

这个结论的自然之处在于如果$F''(x)$存在,那$F(x+2h)=F(x)+F'(x)2h+\frac{F''(x)}{2}(2h)^2+o(h^2)$并且$F(x-2h)=F(x)-F'(x)2h+\frac{F''(x)}{2}(2h)^2+o(h^2)$,所以上面那个$\delta^2 F=F''(x)$,那如果它非负当然是下凸函数.

那如果$F''(x)$不存在呢?反证,假设$F$不凸,则一定存在一个子区间$(\alpha,\beta)\subseteq (a,b)$,使得在这个区间上,$F(x)$函数在$(\alpha,\beta)$这条弦之上(因为连续性,至少有三个点是违背三弦定理的,作一条弦后,函数有一些部分在上面有一些部分在下面,取某个恒在上面的子区间即可).

总之,我们找到了一个子区间$(\alpha,\beta)$使得:

$$
F(x)-\left(\frac{F(\beta)-F(\alpha)}{\beta-\alpha}(x-\alpha)+F(\alpha)\right)>0
$$

接下来要推矛盾,分成两种情况:

第一种情况是$\delta^2 F>0$的情形,考虑辅助函数$G_\mu(x)=F(x)-(\mu(x-a)+F(\alpha))$,其中$\mu=\frac{F(\beta)-F(\alpha)}{\beta-\alpha}+\epsilon$,也就是比原本那条弦的斜率稍微大了一点点,那只要$\epsilon$落在某个足够小的区间内,$G_\mu(x)$仍然有大于零的部分.直接取$G_\mu(x_0(\mu))=\max_{[\alpha,\beta]}G_\mu$,此时观察到$G_\mu$和$F$只差一个一次函数,它们作二阶差分后就相等了,然而,由于$G_\mu(x_0(\mu))$取的是最大值,当$h$取足够小的时候,它的二阶差分一定小于等于$0$,因此能导出$x_0(\mu)\in E$,并且同理能导出$\varlimsup_{h\to +0}\frac{\Delta_h^2 F(x_0(\mu))}{h}=0$.从而导出:

$$
\varlimsup_{h\to +0}\frac{G_\mu(x_0(\mu)-2h)-G_\mu(x_0(\mu))}{h}=\varlimsup_{h\to +0}\frac{G_\mu(x_0(\mu)+2h)-G_\mu(x_0(\mu))}{h}=0
$$

欸,你怎么变一阶差分了,那我的一次函数不就有用了么,必定有:

$$
\varlimsup_{h\to +0}\frac{F(x_0(\mu)-2h)-F(x_0(\mu))}{h}=2\mu
$$

所以$\mu\to x_0(\mu)$为单射,这与$x_0(\mu)\in E$矛盾.

那么等于$0$的情况呼之欲出了对吧,此时任意的$n$都可以做到$F(x)+\frac{x^2}{n}$是凸的,随便搞个反证法就可以知道$F(x)$必定是凸的.

最后来设$F(x)=\frac{a_0}{4}x^2-\sum_{n=1}^\infty\frac{a_n\cos(nx)+b_n\sin(nx)}{n^2}$,由于$a_n\to 0,b_n\to 0$,所以这玩意用大M判别法知道绝对一致收敛,那就是一个连续函数.

此时计算:

$$
\begin{aligned}
\Delta_h^2F(x)&=\frac{a_0}{4}((x+2h)^2+(x-2h)^2-2x^2)\\
&-\sum_{n=1}^\infty \frac{1}{n^2}(a_n(\cos(n(x+2h))+\cos(n(x-2h))-2\cos(nx))\\
&+b_n(\sin(n(x+2h))+\sin(n(x-2h))-2\sin(nx)))\\
\end{aligned}
$$

而此时对后面的部分用和差化积,留神到:

$$
\cos(n(x+2h))+\cos(n(x-2h))-2\cos(nx)\\
=2\cos(nx)\cos(2nh)-2\cos(nx)\\
=2\cos(nx)(\cos(2nh)-1)\\
=-4\cos(nx)\sin^2(nh)
$$

于是:

$$
\frac{\Delta_h^2F(x)}{4h^2}=\frac{a_0}{2}+\sum_{n=1}^\infty (a_n\cos(nx)+b_n\sin(nx))(\frac{\sin(nh)}{nh})^2
$$

乍一看已经要完事了对吧!很可惜并没有,因为这个$n\to \infty$了,我们还需要证明下面这个引理:当$\sum_{n=1}^\infty a_n=s$收敛的时候,那么$\lim_{h\to 0+}\sum_{n=1}^\infty a_n(\frac{\sin(nh)}{nh})^2=s$.

最难过的是这里的$a_n$不定号,我们可能第一反应是前部分可以让$n\to \infty$而后半部分可以直接抛掉,可在不定号的时候这一切都难以实现了.对于不定号项的求和,我们也许唯一的武器只有Abel变换.设$a_0=0,b_n(h)=(\frac{\sin(nh)}{nh})^2,b_0(h)=1,S_n=\sum_{k=1}^n a_k$.立刻得到:

$$
\sum_{n=0}^\infty a_nb_n(h)=\sum_{n=1}^\infty(S_n-S_{n-1})b_n(h)\\
=\sum_{n=0}^\infty S_n(b_n(h)-b_{n+1}(h))
$$

既然$S_n$收敛,不妨设$|S_n|<M$,下面来估计:

$$
\begin{aligned}
|b_{n+1}(h)-b_{n}(h)|&=|\int_{nh}^{(n+1)h}(\frac{\sin^2 t}{t^2})'{\rm d}t|\\
&\leq \int_{nh}^{(n+1)h}|\frac{2\sin t(\cos t-\frac{\sin t}{t})}{t^2}|{\rm d}t\\
\sum|b_{n+1}(h)-b_{n}(h)|&\leq \int_{0}^{+\infty}|\frac{2\sin t(\cos t-\frac{\sin t}{t})}{t^2}|{\rm d}t\\
\end{aligned}
$$

盯着这个积分,发现其在$0$附近极限存在(泰勒展开就能看到),而在无穷处其实就是$O(\frac{1}{t^2})$,所以这个积分存在,当然有$\sum|b_{n+1}(h)-b_{n}(h)|\leq +\infty$,其实上面的过程就是在证明这个函数是有界变差的对吧!准备工作业已就绪,不妨设$\varphi_n=b_n(h)-b_{n+1}(h)$,请看:

$$
|\sum_{n=0}^\infty a_nb_n(h)-s|\\
=|\sum_{n=0}^\infty (s_n\varphi_n(h)-s\varphi_n(h))|\\
\leq \sum_{n=0}^N |S_n-s|\sdot|\varphi_n(h)|+\sum_{n=N+1}^\infty |S_n-s|\sdot|\varphi_n(h)|
$$

只要取定足够大的$N$就可以使得$|S_n-s|\to 0$,而在此时只要让$h\to 0$就可以让前半部分定死的$\varphi_n(h)\to 0$,于是就搞定了.

从而我们可以看到上述引理的确成立了,进一步地两个上极限都等于$0$,所以$F(x)$的确是下凸函数,然而正是因为等于$0$,所以$-F(x)$也是下凸函数,所以$F(x)$既是上凸,又是下凸,那它必然是一个一次函数.那$a_0=0$就是必然的了.既然$a_0=0$,$F(x)$就是一个$2\pi$周期函数,所以它必然是常值函数.然而一个三角级数一致收敛到一个常值函数,这个三角级数本身必然是常值函数,所以$a_n=b_n=0$,这就搞定了.

##### 傅里叶级数的最佳逼近性

下面我们想证明:傅里叶级数是最佳逼近,如果一个三角级数处处收敛到了一个黎曼可积函数,则这个三角级数就是这个函数的傅里叶级数.(事实上这个结论在绝对可积时也是对的,甚至可以有有限个点不收敛到它)

不妨设$f\in R[-\pi,\pi]$,有界$m\leq f\leq M$.三角级数$\frac{a_0}{2}+\sum a_n\cos(nx)+b_n\sin(nx)$逐点逼近于它本身.我们上面引入的也就是$\delta ^2 F(x_0)=f(x_0)$.

来引入一个引理:如果$\forall x_0\in (a,b),\delta^2 F=\lim_{h\to +0}\frac{\Delta_h^2 F}{4h^2}\in [m,M]$,则不加极限的情况,$\forall x_0\in (a+h,b-h),h>0,\frac{\Delta_h^2 F}{4h^2}\in [m,M]$.

我想这应该是平凡的,方向呼之欲出:使用介值定理.不过这里是二次函数,那我们就取定$F(x_0),F(x_0\pm 2h)$三个点,过这三个点做一个二次函数.事实上这个二次函数就是$\varphi(x)=F(x_0)+(x-x_0)\frac{F(x_0+2h)-F(x_0-2h)}{4h}+\frac{(x-x_0)^2}{2}\frac{\Delta_h^2 F(x_0)}{4h^2}$.取$g(x)=F(x)-\varphi(x)$,则$g(x)$会在$x_0,x_0\pm 2h$取值为$0$,并且其在$(x_0-2h,x_0+2h)$上肯定有一个最大值$x_1$和一个最小值$x_2$,在前者的广义二阶导数$\leq 0$而后者$\geq 0$,得到:

$$
\delta^2 F(x_1)-\frac{\Delta_h^2 F(x_0)}{4h^2}\leq 0 \leq \delta^2 F(x_2)-\frac{\Delta_h^2 F(x_0)}{4h^2}\\
m\leq \frac{\Delta_h^2 F(x_0)}{4h^2}\leq M
$$

现在我们就知道如何做了,回忆到:

$$
\frac{\Delta_h^2F(x)}{4h^2}=\frac{a_0}{2}+\sum_{n=1}^\infty (a_n\cos(nx)+b_n\sin(nx))(\frac{\sin(nh)}{nh})^2
$$

当我不取$h\to 0$的时候这个两边可是一致收敛的.那右边肯定是前边的傅里叶级数,立刻有:

$$
a_n(\frac{\sin(nh)}{nh})^2=\frac{1}{\pi}\int_{-\pi}^\pi \frac{\Delta_h^2 F(x)}{4h^2}\cos (nx){\rm d}x\\
b_n(\frac{\sin(nh)}{nh})^2=\frac{1}{\pi}\int_{-\pi}^\pi \frac{\Delta_h^2 F(x)}{4h^2}\sin (nx){\rm d}x\\
$$

那我们自然有:

$$
a_n=\lim_{h\to +0}a_n(\frac{\sin(nh)}{nh})^2\\
=\lim_{h\to +0}\frac{1}{\pi}\int_{-\pi}^\pi \frac{\Delta_h^2 F(x)}{4h^2}\cos (nx){\rm d}x\\
$$

然而此时$\frac{\Delta_h^2 F(x)}{4h^2}\in [m,M]$,后面积分那个东西有界.欸,取$h=\frac{1}{n}\to 0$可以拿到一个一致有界的函数列,这不是我们控制收敛定理么,交换顺序!

$$
a_n=\lim_{h\to +0}\frac{1}{\pi}\int_{-\pi}^\pi \frac{\Delta_h^2 F(x)}{4h^2}\cos (nx){\rm d}x\\
=\frac{1}{\pi}\int_{-\pi}^\pi \lim_{h\to +0}\frac{\Delta_h^2 F(x)}{4h^2}\cos (nx){\rm d}x\\
=\frac{1}{\pi}\int_{-\pi}^\pi f(x)\cos (nx){\rm d}x\\
$$

这就证毕了.

##### 傅里叶级数不逐点收敛到自身的连续周期函数

定义$C(T)$为$2\pi$周期连续函数组成的线性空间.此时定义$\Vert f-g\Vert=\sup|f-g|$,其实也就是Banach空间对吧.有了范数定义距离,有了距离就可以有开闭集的概念.Baire纲定理告诉我们此时可数个开稠集的交集仍然是稠密集(不过未必是开集).

现在考虑$f(x)$的傅里叶级数,取$S_n(f,x)=\frac{a_0}{2}+\sum_{k=1}^n a_k\cos(kx)+b_k\sin(kx)$,迪利克雷核的结论告诉我们$S_n(f,x)=\frac{1}{\pi}\int_{-\pi}^\pi f(t)D_n(x-t){\rm d}t$,其中$D_n(w)=\frac{\sin(n+\frac{1}{2})w}{2\sin \frac{w}{2}}$.

接下来定义$\varphi_f(x)=\sup_{n\geq 1}|S_n(f,x)|$,取$E_m(x)=\{f\in C(T)|\varphi_f(x)>m\}$.下面我们来证明:$E_m(x)$是$C(T)$的开稠子集.

是开集应该是自然的结论,因为当$n,x$定死的时候,$S_n(f,x)$从定义看是连续地依赖于$f$.如果$f\in E_m(x)$,则$\exists n,|S_n(f,x)|>m$,那它周围当然有一个小邻域,使得其中的$g$都满足$|S_n(g,x)|>m$了,所以这肯定是开集.

接下来得证明它是稠集,定义$g_n(t)=\begin{cases}1&D_n(t)\geq 1\\-1&D_n(t)\leq -1\\D_n(t)&otherwise\end{cases}$.此时发现:

$$
\frac{1}{\pi}\int_{-\pi}^\pi g_n(t)D_n(t){\rm d}t\\
=O(1)+\frac{1}{\pi}\int_{-\pi}^\pi |D_n(t)|{\rm d}t\\
$$

原因是$|D_n(t)|\geq 1$的部分二者相等,而不同的部分在有限区间上积分不会太大.

然而:

$$
\frac{1}{\pi}\int_{-\pi}^\pi |D_n(t)|{\rm d}t\\
=\frac{2}{\pi}\int_0^\pi |D_n(t)|{\rm d}t\\
=\frac{1}{\pi}\int_0^\pi |\frac{\sin(n+\frac{1}{2})t}{2\sin \frac{t}{2}}|{\rm d}t\\
>\frac{1}{\pi}\int_0^\pi |\frac{\sin(n+\frac{1}{2})t}{t}|{\rm d}t\\
>\frac{1}{\pi}\int_0^{(n+\frac{1}{2})\pi} \frac{|\sin t|}{t}{\rm d}t\\
>\frac{1}{\pi}\sum_{k=2}^n \int_{(k-1)\pi}^{k\pi}\frac{|\sin t|}{t}{\rm d}t\\
=\sum_{k=2}^n O(1)\frac{1}{k}\\
=O(\ln n)
$$

接下来考虑取$h_n=\frac{g_n}{\sqrt{ln n}}\to 0$,则$S_n(h_n,x)=O(\sqrt{\ln n})$.此时设$U_{\epsilon}(f)=\{g\in C(T)|\Vert f-g\Vert<\epsilon\}$,回忆到三角多项式可以一致逼近任何连续周期函数,所以存在三角多项式(有限项)$T\in U_{\frac{\epsilon}{2}}(f)$.

取$r_n(x)=T+h_n$,由于$h_n\to 0$,所以当$n$足够大的时候$r_n(x)=g+h_n\in U_\epsilon(f)$.此时$S_n(r_n,x)=S_n(T,x)+S_n(h_n,x)$,但是$T$是有限项三角多项式,所以$S_n(r_n,x)=O(1)+O(\sqrt{\ln n})\to \infty$.这就搞定了,那此时$r_n\in E_m(x)$而且在$f$的邻域里.所以这个集合是稠密的.

接下来取$E_x=\bigcap_{m=1}^\infty E_m(x)$,容易发现$E_x$中的所有函数在$x$这一点的傅里叶级数竟然都无界,我可以干脆取$E=\bigcap_{x\in \mathbb{Q}} E_x$,则这里面存在一个函数,这个函数的傅里叶级数在任意有理点处发散.

其实还可以更牛,取$F_m(f)=\{x\in \mathbb{R}|\varphi_f(x)>m\}$,这显然是个开集.取$F_f=\bigcap_{n=1}^\infty F_n$,这就是可数个开集的交.而且从上面的论证可以发现$\mathbb{Q}\subseteq F_f$,可是这里的$\mathbb{Q}$是稠密的,所以$F_f$必然也是稠密的,那它就是可数个开稠集的交,有结论说可数个开稠集的交一定是不可数集合.

然而,还有结论说一个周期连续函数的傅里叶级数其实几乎处处收敛于本身.

###### Example1

设$f(x)$是$2\pi$周期连续函数,设$M_n=\max_{x\in [-\pi,\pi]}|S_n(x)|$,求证:$\lim_{n\to \infty}\frac{M_n}{\ln n}=0$.

不妨设$M_f=\max f$,考虑$|S_n(x)|\leq |S_n(x)-f|+M_f$,而后者是个死数,除以$\ln n$后当然趋近于$0$.

现在只需要看前者,然而:

$$
S_n(x)-f=\frac{1}{\pi}\int_{0}^\pi(f(x+t)+f(x-t)-2f(x))D_n(t){\rm d}t\\
=\frac{1}{\pi}\int_{0}^\delta(f(x+t)+f(x-t)-2f(x))D_n(t){\rm d}t
$$

而回忆到$\int_0^\pi |D_n(t)|=O(\ln n)$,由一致连续性前半部分显然可以决定一个$\delta$是前者$<\epsilon$,这就搞定了.


这个来测试format，例如:

$$
a \land b \lor c \land d{ \lor c \lor d } \land ( b \lor c )
$$

还有空集符号$\emptyset$也要被转化.

别忘了$\mathbb{ R } , \mathbb{ Q } , \mathbb{ Z } , \mathbb{ C }$.

哦还有$\cdot$要被转化为$\cdot$.以及$\exists$也要被转换。别忘了$\langle \_ , \_ \rangle$.

好的，接下来是$\mathrm{ Hom } ( A , B )$这个要被处理掉.然后:

$$
\int_0^1 \frac{ \mathrm{ d } x }{ x }
$$

我做的很棒！下面一步是处理其它word。例如：

$$
\begin{gathered}
\mathrm{ Hom } ( A , B ) \\
\mathrm{ Span } ( A , B ) \\
\mathrm{ Stab } ( X ) \\
\mathrm{ univ } , \mathrm{ unit } , \mathrm{ sym } , \mathrm{ odd } , \mathrm{ even } \\
\mathrm{ coker } , \mathrm{ char } , \mathrm{ Trace } , \mathrm{ Min } \\
\mathrm{ Char } \\
\mathrm{ Mul } , \mathrm{ Bil } , \mathrm{ Frac } \\
\begin{cases}
a & b \\
c & \text{ otherwise }
\end{cases}
\end{gathered}
$$

另外不要误伤已经搞定的$\mathrm{ Hom } ( A , B )$以及$\text{ otherwise }$这些东西.

---

最后一步了家人们，现在让我们处理environment:

$$
\begin{aligned}
a & = b \\
c & = d \\
b & \equiv c \\
& < d \\
& \leq w \\
& > sss \geq w \\

\end{aligned}
$$

做法是如果发现行内换行符但是外部没有environment就补一个environment（默认为aligned），同时为其中的指令补$\&$符号。

最后还有一种environment的情况如下:

$$
\begin{aligned}
& sfs \\
= & bsddad \\
= & csda
\end{aligned}
$$

哦最后一个问题是这个咋办:

$$
\overline{ span }
$$


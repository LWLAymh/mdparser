这个来测试format，例如:
$$
a\and b\or c\and d{\or c\or d}\and(b\or c)
$$
还有空集符号$\empty$也要被转化.

别忘了$\R,\Q,\Z,\C$.

哦还有$\sdot$要被转化为$\cdot$.以及$\exist$也要被转换。别忘了$\lang\_,\_\rang$.

好的，接下来是${\rm Hom}(A,B)$这个要被处理掉.然后:
$$
\int_0^1\frac{ {\rm d}x}{x}
$$
我做的很棒！下面一步是处理其它word。例如：
$$
Hom(A,B)\\
Span(A,B)\\
Stab(X)\\
univ,unit,sym,odd,even\\
coker,char,Trace,Min\\
Char\\
Mul,Bil,Frac\\
\begin{cases}a&b\\c&otherwise\end{cases}
$$
另外不要误伤已经搞定的$\mathrm{Hom}(A,B)$以及$\text{otherwise}$这些东西.

---

最后一步了家人们，现在让我们处理environment:
$$
a=b\\
c=d\\
b\equiv c\\
<d\\
\leq w\\
>sss\geq w\\
$$
做法是如果发现行内换行符但是外部没有environment就补一个environment（默认为aligned），同时为其中的指令补$\&$符号。

最后还有一种environment的情况如下:
$$
sfs\\
=bsddad\\
=csda
$$
哦最后一个问题是这个咋办:
$$
\overline{span}
$$

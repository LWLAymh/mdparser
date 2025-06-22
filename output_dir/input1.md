$$
\begin{gathered}
\\
\frac{ 1 }{ 2 } \\
\sqrt[3]{ 5 } \\
\sqrt[\lambda]{ a } \\
\text{ \} } \\
\text{ ab }
\end{gathered}
$$

最后,在某些模块中加空格会影响排版,例如:

$$
\begin{gathered}
\text{ ab } \\
\text{ a b } \\
\text{ a b }
\end{gathered}
$$

这个真有点难写的，首先我要把这些全拿到就很困难，给parser上点强度吧：

$$
\sqrt[\text{ a bc d } + \textbf{ cd ef } + \text{ c } dw]{ \frac{ \textbf{ a + b } \textrm{ b ad } \text{ a } }{ b } + \text{ c } + \textbf{ d } + \text{ w } sawd }
$$

我草这个中括号怎么这么坏啊$[ 0 , 1 )$.

这个中括号识别的我头疼:

$$
\begin{gathered}
\sqrt{ a } \\
\sqrt[b]{ a } \\
\sqrt[\lambda + \eta]{ s + w + a } \\
\sqrt[a_{ [ }]{ bcd }
\end{gathered}
$$

然后还有画图有一个中括号：

$$
\xymatrix{ a \ar[r] & b \\
c \ar @{->>}[r] & d }
$$


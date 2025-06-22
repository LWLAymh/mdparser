$$
\\
\frac12\\
\sqrt[3]{5}\\
\sqrt[\lambda]{a}\\
\text{\}}\\
\text{ab}
$$

最后,在某些模块中加空格会影响排版,例如:
$$
\text{ab}\\
\text{a b}\\
\text{a  b}
$$
这个真有点难写的，首先我要把这些全拿到就很困难，给parser上点强度吧：
$$
\sqrt[\text{a bc d}+\textbf{cd ef}+\text cdw]{\frac{\textbf{a + b }\textrm{b  ad  }\text{a}}{b}+\text{c}+\textbf{d}+\text wsawd}
$$
我草这个中括号怎么这么坏啊$[0,1)$.

这个中括号识别的我头疼:
$$
\sqrt{a}\\
\sqrt[b]{a}\\
\sqrt[\lambda+\eta]{s+w+a}\\
\sqrt[a_{[}]{bcd}
$$
然后还有画图有一个中括号：
$$
\xymatrix{
a\ar[r]&b\\
c\ar@{->>}[r]&d
}
$$

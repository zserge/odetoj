# odetoj

Rewrite of Arthur Whitney's one-page J interpreter in Rust.

> One summer weekend in 1989, Arthur Whitney visited Ken Iverson at Kiln Farm and produced—on one page and in one afternoon—an interpreter fragment on the AT&T 3B1 computer. I studied this interpreter for about a week for its organization and programming style; and on Sunday, August 27, 1989, at about four o'clock in the afternoon, wrote the first line of code that became the implementation described in this document.

Arthur's one-page interpreter fragment is as follows (I only changed `I` type from `long` to `long long` to fit 64-bit systems):

```c
typedef char C;typedef long long I;
typedef struct a{I t,r,d[3],p[2];}*A;
#define P printf
#define R return
#define V1(f) A f(w)A w;
#define V2(f) A f(a,w)A a,w;
#define DO(n,x) {I i=0,_n=(n);for(;i<_n;++i){x;}}
I *ma(n){R(I*)malloc(n*4);}mv(d,s,n)I *d,*s;{DO(n,d[i]=s[i]);}
tr(r,d)I *d;{I z=1;DO(r,z=z*d[i]);R z;}
A ga(t,r,d)I *d;{A z=(A)ma(5+tr(r,d));z->t=t,z->r=r,mv(z->d,d,r);
 R z;}
V1(iota){I n=*w->p;A z=ga(0,1,&n);DO(n,z->p[i]=i);R z;}
V2(plus){I r=w->r,*d=w->d,n=tr(r,d);A z=ga(0,r,d);
 DO(n,z->p[i]=a->p[i]+w->p[i]);R z;}
V2(from){I r=w->r-1,*d=w->d+1,n=tr(r,d);
 A z=ga(w->t,r,d);mv(z->p,w->p+(n**a->p),n);R z;}
V1(box){A z=ga(1,0,0);*z->p=(I)w;R z;}
V2(cat){I an=tr(a->r,a->d),wn=tr(w->r,w->d),n=an+wn;
 A z=ga(w->t,1,&n);mv(z->p,a->p,an);mv(z->p+an,w->p,wn);R z;}
V2(find){}
V2(rsh){I r=a->r?*a->d:1,n=tr(r,a->p),wn=tr(w->r,w->d);
 A z=ga(w->t,r,a->p);mv(z->p,w->p,wn=n>wn?wn:n);
 if(n-=wn)mv(z->p+wn,z->p,n);R z;}
V1(sha){A z=ga(0,1,&w->r);mv(z->p,w->d,w->r);R z;}
V1(id){R w;}V1(size){A z=ga(0,0,0);*z->p=w->r?*w->d:1;R z;}
pi(i){P("%d ",i);}nl(){P("\n");}
pr(w)A w;{I r=w->r,*d=w->d,n=tr(r,d);DO(r,pi(d[i]));nl();
 if(w->t)DO(n,P("< ");pr(w->p[i]))else DO(n,pi(w->p[i]));nl();}

C vt[]="+{~<#,";
A(*vd[])()={0,plus,from,find,0,rsh,cat},
 (*vm[])()={0,id,size,iota,box,sha,0};
I st[26]; qp(a){R  a>='a'&&a<='z';}qv(a){R a<'a';}
A ex(e)I *e;{I a=*e;
 if(qp(a)){if(e[1]=='=')R st[a-'a']=ex(e+2);a= st[ a-'a'];}
 R qv(a)?(*vm[a])(ex(e+1)):e[1]?(*vd[e[1]])(a,ex(e+2)):(A)a;}
noun(c){A z;if(c<'0'||c>'9')R 0;z=ga(0,0,0);*z->p=c-'0';R z;}
verb(c){I i=0;for(;vt[i];)if(vt[i++]==c)R i;R 0;}
I *wd(s)C *s;{I a,n=strlen(s),*e=ma(n+1);C c;
 DO(n,e[i]=(a=noun(c=s[i]))?a:(a=verb(c))?a:c);e[n]=0;R e;}

main(){C s[99];while(gets(s))pr(ex(wd(s)));}
```

The interpreter is limited and probably will crash a lot as you play with it. To help you understand the logic of this highly obfuscated code, here's some hints:

* `A` type is the core data type, it's an array.
* `A.t` is "box" flag (when another array is stored as an element of this one).
* `A.r` is "rank", i.e. how many dimensions this array has. Array data is stored flat, and shape is stored separately. 
* `A.d` is "depth", i.e. how big the array is in each direction.
* `A.p` is array data. Elements can be either numbers (`I`), or other array pointers (`A`). That's why `sizeof(I)` should fit a pointer.
* `ma` is memory allocator, `mv` copies data from one array of numbers to another, `tr` returns number of elements in an array in all its dimensions by multiplying them, `ga` is a constructor for arrays.
* `V1` are monads (functions of one array), `V2` are dyads (working with two arrays). The table of verbs (functions) is stored in `vt`, `vd` and `vm`.
* `pr` prints an array, showing its dimensions (if any), its data and a boxing symbol "<" when needed.
* `st` is variable storage. Variables can be single-letter lowercase symbols. Thus, there can be only 26 variables.
* `ex` recursively applies verbs to nouns (arrays) from right-to-left, like APL does.
* `wd` tokenizes string and builds verbs and nouns (in a weird way, as ints). Numeric literals can only be single-digit.
* There are no error checks and interpreter will crash on invalid input.

## rust port

I decided to rewrite it in Rust to understand better how it works, to make it more stable, and to actually learn Rust. If something is not idiomatic and can be improved (without making it significantly larger) - please open an issue or contact me.

The port differs from the original interpreter:

* Longer variable name and numeric literals are allowed.
* More error checks.
* Element enum type is introduced to avoid array and i64 mixing/casting.
* Token type is introduced to pass data from tokenizer to evaluator in a sane way.

If you think of other verbs that can easily be added without sacrificing the code size - please let me know. I hope this interpreter could become a good entry point to the world of J, K and APL. Have fun!

use criterion::{criterion_group, criterion_main, Criterion};

use adventofcode::{day7, day8};

fn day7_full(c: &mut Criterion) {
    c.bench_function("day7_full", |bencher|bencher.iter(||day7::benchmark_hook(DAY7_INPUT)));
}

fn day8_full(c: &mut Criterion) {
    c.bench_function("day8_full", |bencher| bencher.iter(||day8::benchmark_hook(DAY8_INPUT)));
}

criterion_group!(benches, day7_full, day8_full);
criterion_main!(benches);

const DAY7_INPUT: &'static str = "$ cd /
$ ls
233998 glh.fcb
184686 jzn
dir qcznqph
dir qtbprrq
299692 rbssdzm.ccn
dir vtb
$ cd qcznqph
$ ls
32148 lhsrj.fnr
dir lnj
dir mtr
dir mznnlph
dir pdtpt
24836 rsjcg.lrh
dir vrj
dir wrqcfl
$ cd lnj
$ ls
12592 tlh
$ cd ..
$ cd mtr
$ ls
118870 twdhlmp.gbw
$ cd ..
$ cd mznnlph
$ ls
240977 fmmhnhtf
dir gbhcnts
dir gsbjrrd
dir pmwcs
dir qtbprrq
286007 rhnjndsq.gst
dir twdhlmp
283716 twdhlmp.rpr
$ cd gbhcnts
$ ls
dir fctrnwb
dir gbhcnts
46017 gft.hvm
234925 gjsnzbtw.ncd
dir nvnwh
dir srslsjp
dir swtlfsv
66115 tgpmsb
64086 tqnvb
308270 tqwfpnbn.btp
$ cd fctrnwb
$ ls
112643 qhcdd
$ cd ..
$ cd gbhcnts
$ ls
26196 cmttgsmm.bdn
317410 fthqln
dir lwshph
32809 tdmfc
dir tqcllnv
dir twdhlmp
$ cd lwshph
$ ls
214023 ctqvrzs.jvr
104432 gbch
dir gpqgrw
105909 qshbtd.nml
dir rhhsfbdd
dir svvqh
161439 tqnvb
60152 twdhlmp.qzw
$ cd gpqgrw
$ ls
dir mbsgrlld
dir nhb
dir qtbprrq
$ cd mbsgrlld
$ ls
13247 tsztmlfg
dir twdhlmp
$ cd twdhlmp
$ ls
236804 mcrd
$ cd ..
$ cd ..
$ cd nhb
$ ls
86570 gtvnbsv.zbr
$ cd ..
$ cd qtbprrq
$ ls
111178 npg.qph
110775 tlh
$ cd ..
$ cd ..
$ cd rhhsfbdd
$ ls
37729 fmmhnhtf
263415 ljvwzj.btm
$ cd ..
$ cd svvqh
$ ls
185682 wlcl.fhs
$ cd ..
$ cd ..
$ cd tqcllnv
$ ls
dir cbdj
dir ccsfm
55264 tqnvb
267792 wlcl.fhs
$ cd cbdj
$ ls
128247 fmmhnhtf
dir mtnbs
240520 ngmw.clj
30569 qbqltr.lbw
188801 zwdpp
$ cd mtnbs
$ ls
dir bsfbrmh
dir ftmnrwm
$ cd bsfbrmh
$ ls
dir tltvzp
$ cd tltvzp
$ ls
312469 dnst.sbm
$ cd ..
$ cd ..
$ cd ftmnrwm
$ ls
278974 nlztftc.zhb
$ cd ..
$ cd ..
$ cd ..
$ cd ccsfm
$ ls
4017 wlcl.fhs
$ cd ..
$ cd ..
$ cd twdhlmp
$ ls
dir qtbprrq
$ cd qtbprrq
$ ls
dir tdpz
$ cd tdpz
$ ls
210400 fmmhnhtf
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd nvnwh
$ ls
dir jlpbbds
dir pphv
285452 qtbprrq
$ cd jlpbbds
$ ls
7058 vmrcqz
$ cd ..
$ cd pphv
$ ls
290310 msz.swz
$ cd ..
$ cd ..
$ cd srslsjp
$ ls
dir nnz
192902 twdhlmp.vgp
$ cd nnz
$ ls
215711 tlh
$ cd ..
$ cd ..
$ cd swtlfsv
$ ls
274236 frwncp.gff
$ cd ..
$ cd ..
$ cd gsbjrrd
$ ls
dir dnst
dir gbhcnts
61000 gqdf
175813 jvz
dir ldqjzrtp
$ cd dnst
$ ls
124352 dnst
220618 mzsqzbfz.qfd
134211 qmrvh
dir qqlm
dir qtbprrq
223840 tlh
dir twdhlmp
24794 wfb.rtf
$ cd qqlm
$ ls
113976 wlcl.fhs
$ cd ..
$ cd qtbprrq
$ ls
212775 qtbprrq.ngs
$ cd ..
$ cd twdhlmp
$ ls
308083 fzhd
63311 wlcl.fhs
$ cd ..
$ cd ..
$ cd gbhcnts
$ ls
dir dlvhzdbg
$ cd dlvhzdbg
$ ls
305798 twdhlmp
$ cd ..
$ cd ..
$ cd ldqjzrtp
$ ls
93085 dcvfpz.bjl
264488 zssvm.wdp
$ cd ..
$ cd ..
$ cd pmwcs
$ ls
125444 qtbprrq.tgl
$ cd ..
$ cd qtbprrq
$ ls
dir bjnctfv
133127 fmmhnhtf
dir gztmrrff
dir qtbprrq
$ cd bjnctfv
$ ls
dir cpwrcf
dir fdjzsfc
1223 gbhcnts.qvf
272526 gbhcnts.sgs
dir qnsdl
dir snq
dir tmjnvcbl
dir vdjqsbr
271339 wslnqh.rgr
134589 zzqrbr.fcz
$ cd cpwrcf
$ ls
143124 pdr
$ cd ..
$ cd fdjzsfc
$ ls
dir gbhcnts
dir nqpbzvpq
$ cd gbhcnts
$ ls
151265 jrdvt.fcg
11872 tlh
$ cd ..
$ cd nqpbzvpq
$ ls
dir hpwhslq
27858 ljvwzj.prq
dir nzcnb
$ cd hpwhslq
$ ls
136646 bqgj.wvw
252823 ngmw.clj
137072 tqnvb
$ cd ..
$ cd nzcnb
$ ls
99882 twdhlmp.grg
$ cd ..
$ cd ..
$ cd ..
$ cd qnsdl
$ ls
8925 fmmhnhtf
dir mnzqwfnh
206990 vqgrhqgc
$ cd mnzqwfnh
$ ls
271442 bmztfjlc.lzr
$ cd ..
$ cd ..
$ cd snq
$ ls
25995 tqnvb
$ cd ..
$ cd tmjnvcbl
$ ls
dir gclzbvt
$ cd gclzbvt
$ ls
dir jtfddbs
$ cd jtfddbs
$ ls
10564 pdf.tsj
32415 tlh
$ cd ..
$ cd ..
$ cd ..
$ cd vdjqsbr
$ ls
256668 cwbd
265036 fmmhnhtf
$ cd ..
$ cd ..
$ cd gztmrrff
$ ls
52260 bdqcl.bdw
dir lsss
120102 tlh
$ cd lsss
$ ls
13729 wlcl.fhs
$ cd ..
$ cd ..
$ cd qtbprrq
$ ls
dir bttpq
dir lcvgwpt
$ cd bttpq
$ ls
216247 nnlv.dgl
138688 wlcl.fhs
$ cd ..
$ cd lcvgwpt
$ ls
dir dth
198570 tsqgm.zht
dir zbcstsb
$ cd dth
$ ls
dir cqmbtj
120437 hdqp.vhq
dir vpzn
$ cd cqmbtj
$ ls
11882 sdngnzb
$ cd ..
$ cd vpzn
$ ls
dir jqbz
271714 plcq.bfg
$ cd jqbz
$ ls
dir qqhnfglj
136307 stncbrm
177843 tlh
168253 tqnvb
297085 wcn
$ cd qqhnfglj
$ ls
197471 twdhlmp
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd zbcstsb
$ ls
298115 bvljmpc.gss
308872 ljr.lzl
201657 ngmw.clj
170617 ppln
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd twdhlmp
$ ls
dir dbb
277215 ngmw.clj
310263 twdhlmp.wvs
dir vsfrqsnl
$ cd dbb
$ ls
258300 tqnvb
$ cd ..
$ cd vsfrqsnl
$ ls
dir gbhcnts
12285 tlh
$ cd gbhcnts
$ ls
248251 dnst.bcs
91471 gbhcnts.ntr
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd pdtpt
$ ls
164477 flcgj.zwr
dir ljvwzj
51483 ljvwzj.htl
dir pbtr
dir qtbprrq
dir rrhcsn
$ cd ljvwzj
$ ls
dir nsq
133318 qtbprrq.gqq
166365 rnfbl.ljh
130617 tlh
16112 vbw
$ cd nsq
$ ls
dir fwfcmfbz
$ cd fwfcmfbz
$ ls
71451 zcc.ngn
$ cd ..
$ cd ..
$ cd ..
$ cd pbtr
$ ls
dir qtbprrq
$ cd qtbprrq
$ ls
117780 gjqbnrv.sdl
$ cd ..
$ cd ..
$ cd qtbprrq
$ ls
269746 dld
dir fcmbv
42544 mlzvd.vcw
165396 nbtlfm.vzq
dir sbtl
dir twdhlmp
$ cd fcmbv
$ ls
202047 wdzcrg.mcg
$ cd ..
$ cd sbtl
$ ls
dir dbcdf
dir fbz
dir lvz
dir ncnwbsdh
dir rft
23523 zphlfqf.phv
$ cd dbcdf
$ ls
dir dhdw
dir dvtjfhvm
182513 lclmdwr
63921 ngmw.clj
dir qqmddq
318020 tlh
dir twdwfj
83108 vmwlfdlf
121901 wlcl.fhs
$ cd dhdw
$ ls
dir qtbprrq
dir twdhlmp
dir wbllhmd
$ cd qtbprrq
$ ls
111984 fhc.tzm
$ cd ..
$ cd twdhlmp
$ ls
277414 fwfqbb.dpj
$ cd ..
$ cd wbllhmd
$ ls
dir dnst
dir jqz
dir lbdclnfb
dir ljvwzj
dir mzfdg
96340 ngmw.clj
dir twdhlmp
dir wmcfzznt
147877 zwgvvd
$ cd dnst
$ ls
310179 fmmhnhtf
243908 twdhlmp
$ cd ..
$ cd jqz
$ ls
94739 twdhlmp
$ cd ..
$ cd lbdclnfb
$ ls
112509 ljvwzj
$ cd ..
$ cd ljvwzj
$ ls
28274 bshlmj.lzc
84072 tlh
283462 twdhlmp.ccd
$ cd ..
$ cd mzfdg
$ ls
282099 hbbrjc.jff
63535 tlh
$ cd ..
$ cd twdhlmp
$ ls
283817 jltvl.tgl
$ cd ..
$ cd wmcfzznt
$ ls
294565 fmmhnhtf
$ cd ..
$ cd ..
$ cd ..
$ cd dvtjfhvm
$ ls
292813 qgmvm.fsg
$ cd ..
$ cd qqmddq
$ ls
11670 dnst.btd
241275 fmmhnhtf
196615 fpnmptm
dir nnzscbvw
dir qnrr
$ cd nnzscbvw
$ ls
250962 dflhdfz
$ cd ..
$ cd qnrr
$ ls
dir trzj
$ cd trzj
$ ls
36993 gbhcnts.rdh
273052 tlh
$ cd ..
$ cd ..
$ cd ..
$ cd twdwfj
$ ls
162470 hfdhmbcq.hwz
dir qtbprrq
dir scjzbdsz
2609 wlcl.fhs
$ cd qtbprrq
$ ls
dir cfmglhwj
103703 cscftrsr.jbs
71160 dnst.rbw
dir nrmp
311716 qtbprrq
$ cd cfmglhwj
$ ls
dir fmcmjfg
$ cd fmcmjfg
$ ls
82998 ljvwzj.qbd
8407 nhmmwwzl
dir qtbprrq
261949 tlh
$ cd qtbprrq
$ ls
314421 hwqtl
92593 zcdvf
$ cd ..
$ cd ..
$ cd ..
$ cd nrmp
$ ls
94387 fmmhnhtf
$ cd ..
$ cd ..
$ cd scjzbdsz
$ ls
6861 dgzhldd.dhs
dir gbhcnts
dir qtbprrq
dir sfdl
$ cd gbhcnts
$ ls
dir qdsrs
$ cd qdsrs
$ ls
25165 ngmw.clj
$ cd ..
$ cd ..
$ cd qtbprrq
$ ls
151403 tswd.hpf
$ cd ..
$ cd sfdl
$ ls
308622 jcmsnj
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd fbz
$ ls
dir dgjf
dir qtbprrq
$ cd dgjf
$ ls
254198 rvf.hfq
$ cd ..
$ cd qtbprrq
$ ls
dir frlj
231222 njjfqgt.bph
dir rjsw
dir vjhzc
$ cd frlj
$ ls
dir ljvwzj
$ cd ljvwzj
$ ls
57572 ljvwzj.bvh
$ cd ..
$ cd ..
$ cd rjsw
$ ls
131875 lbcq.rlc
272908 mnfs
$ cd ..
$ cd vjhzc
$ ls
279363 fmmhnhtf
238051 zdzbb.rfj
$ cd ..
$ cd ..
$ cd ..
$ cd lvz
$ ls
289192 tqnvb
dir twdhlmp
$ cd twdhlmp
$ ls
dir wqtgwzdn
$ cd wqtgwzdn
$ ls
283475 ghvpfl
$ cd ..
$ cd ..
$ cd ..
$ cd ncnwbsdh
$ ls
dir dfrdwfgm
dir ljvwzj
dir vgh
$ cd dfrdwfgm
$ ls
279286 mrbwmws.nzd
197337 nqgq.fhf
248096 tqs.jfb
35181 wlcl.fhs
$ cd ..
$ cd ljvwzj
$ ls
250455 gmph.scm
147449 ljvwzj
100189 qfr
$ cd ..
$ cd vgh
$ ls
244540 bzwrldnz.ldt
235508 dzm
dir gbhcnts
dir qtv
dir tvtwlt
262356 wlcl.fhs
$ cd gbhcnts
$ ls
160689 srvpbf.szt
191895 tqnvb
$ cd ..
$ cd qtv
$ ls
9491 dnst.szf
268602 ngmw.clj
dir pbcrfzz
39049 rzgqqvlt.nsm
dir tfpl
79589 wwcrv.ncv
$ cd pbcrfzz
$ ls
dir stt
256685 wlcl.fhs
$ cd stt
$ ls
12650 jbdfwj
$ cd ..
$ cd ..
$ cd tfpl
$ ls
92079 dfhj
$ cd ..
$ cd ..
$ cd tvtwlt
$ ls
dir cqv
$ cd cqv
$ ls
dir vdv
$ cd vdv
$ ls
119483 fmmhnhtf
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd rft
$ ls
24341 bjhzvzp.flg
dir glwdmdt
$ cd glwdmdt
$ ls
288082 jdtlwrzh.wcj
$ cd ..
$ cd ..
$ cd ..
$ cd twdhlmp
$ ls
dir gbhcnts
154240 wlcl.fhs
$ cd gbhcnts
$ ls
217462 ddzp
$ cd ..
$ cd ..
$ cd ..
$ cd rrhcsn
$ ls
308440 dzbfl.vcg
dir jbhcpdh
238941 rnqdz
dir szljjhc
$ cd jbhcpdh
$ ls
dir bmg
dir mdqplln
dir twdhlmp
dir zbt
$ cd bmg
$ ls
dir djwfl
dir gbhcnts
dir ljvwzj
142159 mwl.psh
110681 rzmdgbng
dir zqjbb
$ cd djwfl
$ ls
dir dpfcrjl
dir rqtz
$ cd dpfcrjl
$ ls
206939 tlh
$ cd ..
$ cd rqtz
$ ls
232264 tlh
$ cd ..
$ cd ..
$ cd gbhcnts
$ ls
186364 ngmw.clj
248882 twdhlmp
306411 wjqvlzp
$ cd ..
$ cd ljvwzj
$ ls
dir dgqw
$ cd dgqw
$ ls
dir mpczlcrz
dir qtbprrq
dir twdhlmp
dir zjsltthh
$ cd mpczlcrz
$ ls
142906 gvd.nnz
$ cd ..
$ cd qtbprrq
$ ls
179566 fmmhnhtf
309800 jhwwppc.vcp
$ cd ..
$ cd twdhlmp
$ ls
dir bhqjhjvp
$ cd bhqjhjvp
$ ls
dir lmj
dir qmcqggbl
$ cd lmj
$ ls
275070 twdhlmp
$ cd ..
$ cd qmcqggbl
$ ls
dir mhgnpm
$ cd mhgnpm
$ ls
dir rnzzqr
$ cd rnzzqr
$ ls
126574 pgnlrjs.czj
7567 tqnvb
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd zjsltthh
$ ls
dir twdhlmp
$ cd twdhlmp
$ ls
198813 dnst.cqc
$ cd ..
$ cd ..
$ cd ..
$ cd ..
$ cd zqjbb
$ ls
dir czdvd
94020 dnst
46041 qtbprrq.pzm
dir rcfvq
dir rwj
118305 vbcpcz
48725 wlcl.fhs
$ cd czdvd
$ ls
302317 tlf
$ cd ..
$ cd rcfvq
$ ls
dir cjws
$ cd cjws
$ ls
dir dsgf
dir fvqbhq
203941 hgcbcvb
9562 qqjh.mfh
32161 qtbprrq.tgn
225251 sbmpn
dir sdhvcj
$ cd dsgf
$ ls
dir cbwzg
141466 ctpszzvn.qrq
277153 ngmw.clj
100681 vmdwgrp
$ cd cbwzg
$ ls
dir nblvrbv
$ cd nblvrbv
$ ls
129474 dlcbng.sgf
$ cd ..
$ cd ..
$ cd ..
$ cd fvqbhq
$ ls
75755 fmmhnhtf
229463 tlh
$ cd ..
$ cd sdhvcj
$ ls
306751 tqnvb
$ cd ..
$ cd ..
$ cd ..
$ cd rwj
$ ls
130415 cjbz
283701 rgsdtn
$ cd ..
$ cd ..
$ cd ..
$ cd mdqplln
$ ls
169404 dvss.mvd
105385 fmmhnhtf
222834 jhzpwscp.sqg
164293 jsqlprqn.vnp
57167 pwpjfq.bmb
dir qtbprrq
$ cd qtbprrq
$ ls
62823 ljvwzj.flm
252940 tlh
$ cd ..
$ cd ..
$ cd twdhlmp
$ ls
dir dhvgfhc
dir qrlq
$ cd dhvgfhc
$ ls
dir vpldlp
$ cd vpldlp
$ ls
279067 dnst.jfs
9050 fmmhnhtf
88586 mfbj.fgs
$ cd ..
$ cd ..
$ cd qrlq
$ ls
dir qwwftl
$ cd qwwftl
$ ls
103153 tnczww
$ cd ..
$ cd ..
$ cd ..
$ cd zbt
$ ls
99657 fsq.rzj
158138 gbfjfctj.bgg
260423 tqnvb
161379 trg
$ cd ..
$ cd ..
$ cd szljjhc
$ ls
287080 stnp.lgp
173682 wjzvglm.lfm
$ cd ..
$ cd ..
$ cd ..
$ cd vrj
$ ls
129084 ngmw.clj
250696 pdpzzbs
$ cd ..
$ cd wrqcfl
$ ls
dir bjlwb
105899 gsvm
dir jdnjpg
178665 znnmmhqt.hth
$ cd bjlwb
$ ls
207939 gbhcnts
$ cd ..
$ cd jdnjpg
$ ls
260418 tqnvb
302144 twdhlmp.ghg
$ cd ..
$ cd ..
$ cd ..
$ cd qtbprrq
$ ls
95562 fmmhnhtf
dir plf
dir qtbprrq
306396 rqqmm.wvw
dir wpfj
$ cd plf
$ ls
dir fmftrbn
20347 twb.zjd
$ cd fmftrbn
$ ls
dir rfznrm
$ cd rfznrm
$ ls
283327 rlzjcg
$ cd ..
$ cd ..
$ cd ..
$ cd qtbprrq
$ ls
313931 ztmhrjc
$ cd ..
$ cd wpfj
$ ls
3969 wrbhb.jll
$ cd ..
$ cd ..
$ cd vtb
$ ls
14260 fmmhnhtf
dir gbhcnts
dir lwcznw
dir mhp
dir pqcddzsf
272267 qgh
301727 rsjrn.wjg
101787 vqscjb
dir zvn
$ cd gbhcnts
$ ls
7627 tqnvb
$ cd ..
$ cd lwcznw
$ ls
98498 dnst.tds
dir gfh
dir jdg
dir llnl
161511 mtmrr.hvb
dir ppzwbgnz
210908 qtbprrq
dir tvhz
$ cd gfh
$ ls
169547 mjjvvlqj.jmv
$ cd ..
$ cd jdg
$ ls
dir cthptwcf
dir ljvwzj
dir vnlndl
$ cd cthptwcf
$ ls
98711 qwzwz.qct
$ cd ..
$ cd ljvwzj
$ ls
245473 zhptcmcr.fts
$ cd ..
$ cd vnlndl
$ ls
151466 ljvwzj
285091 twdhlmp.mzv
59067 vcdpbg.nmp
$ cd ..
$ cd ..
$ cd llnl
$ ls
141508 phtmjj.qzl
dir qtbprrq
105151 tlh
$ cd qtbprrq
$ ls
62020 hdzljht.fvq
$ cd ..
$ cd ..
$ cd ppzwbgnz
$ ls
298940 pzdqzrn.zlz
$ cd ..
$ cd tvhz
$ ls
96628 hrzr
$ cd ..
$ cd ..
$ cd mhp
$ ls
226604 mbdn.tbq
dir ndgqtvhg
$ cd ndgqtvhg
$ ls
55244 dnst
dir sljbrmhb
$ cd sljbrmhb
$ ls
32711 dnst
$ cd ..
$ cd ..
$ cd ..
$ cd pqcddzsf
$ ls
dir shwrrq
$ cd shwrrq
$ ls
dir dplcwvhg
dir pvtpf
dir qpsmgfjl
247965 rrw.wwv
dir vmrwpt
$ cd dplcwvhg
$ ls
242534 fmmhnhtf
202367 fzmt.qrw
197586 ljvwzj.qgm
dir stp
dir zpz
$ cd stp
$ ls
12921 mlcqtthb.jtd
$ cd ..
$ cd zpz
$ ls
235965 ngmw.clj
$ cd ..
$ cd ..
$ cd pvtpf
$ ls
319563 rdspj.slv
279577 vqpjzrdl.hhj
$ cd ..
$ cd qpsmgfjl
$ ls
131841 cqhrgc.cqz
105373 fbnp
$ cd ..
$ cd vmrwpt
$ ls
176373 phgsdlnj.ggq
$ cd ..
$ cd ..
$ cd ..
$ cd zvn
$ ls
dir gbhcnts
dir gfh
dir ppqjzln
dir qtbprrq
$ cd gbhcnts
$ ls
156292 wlcl.fhs
$ cd ..
$ cd gfh
$ ls
189836 ljvwzj.wpt
10416 zbnhzjvw.jct
$ cd ..
$ cd ppqjzln
$ ls
95088 sszd
$ cd ..
$ cd qtbprrq
$ ls
295187 hnnl
292421 qtbprrq.ppg
220281 wlcl.fhs
";

const DAY8_INPUT: &'static str = "003112220410413101104044022234320204233341435252223642044225451531421012104343030211442433410302111
301233004003313130222434121135033231250505241131342032404032560542233000343455552123100410402211201
111301041221333142533352050250154136146324550411565615444115604102531135302000320033233340431313123
011312210420442043155233305201305643445224334303310253225205601265233454400214114322131420224022313
130102441231200141254202121022423405224443210463250415204410624313613034320040015223211432442333110
133121132322223054104323242043651144066341346000104210124535555236324451132555525220523220023202433
110011123024113203143145243605143331512223606564503661350336662505131254503242354031400131012444222
221400422202335053520044325014041432662161415523526711633662600635304000112322014001533351130303321
030222020044115331004065013253364503664435753416641653716424535324654054023321025154331103034342414
030331241153233040140314112524504535172167445223426653152774166352145410064425434012002122431142343
224111232453145423354550035141103644127571711431336236226321752314754510214316215104550522141301020
424321022432003551434012134531644165753146143232242275633762323631713541330463531053004424012234010
221233440041435131321565604060121637542135243721227576264551457171313165211546132314103242442012133
301320232222024200651041405631257577625236256225367443317773421262762172454463051224654015452230401
124314451244222611016300517414362722551374735555353871173242751564427674344212032442221554524035432
000210412345203005364660077252742673773341654558568632445755573574757433452636410261362004114032013
333130514223252440150314572132443233363783556666363566733363262631651361523573335361463232143300044
400403124212626240015541437753711632635435857234865883574647337436376563235264524230225211320425534
430312520241033613150127256341412352564548243842255622454762832274857163157242366022041262252034150
125211521340246341461723355443362487532437274863758283488567453538667642367654751321503343220351304
015045453331430626273664314125778234645658386774837783464366334264476243327176571544650404453253132
354034323663213666746174754737374552772744668787244468236435632534753452366177147233020313054042345
305241054630156250416544221136444247327848337737584777837244467756426322644474332262330430251551212
100144030632532041231474166555527386344645548655948476835366757558682728623657547471260405014131510
155501551351164622444476154658383338677369856795967346765977585753438378656352723127554031451225340
213252046404053134327512554784653544697439375473948735593644477348334755453317243676612665133404310
022440140420166534174335264388542568833984445757393864997986883894574577853347367415453352646352541
303425632364032563633467847288735478557676734393879573848466463466672455482853676764477400452414425
421140260266576264621526686828328833799894373434775335763857585383998562862724435337211515426106224
251234063153345361761483764445395779384568938899876568577359785776678952473247274437226445141011143
124165362100637756715727365876646574835535677777467785479666667584536895483528343623556220650044411
523533116461715454347526758528694839737349689796485884675949699354664766465728468324363766204426020
011655232111314171647378426863359767797699886798595874554677869353593448485338287251255363024600501
201454444103773472628888556987787587788889498545597569965587748496753787964648276727241677205552530
022050015673621733475438774338498584975649879876857968775687577968447843939528275674512475556245535
415145246426622536566872225778388854879897849855475687687595454675468637795674534385355242230005213
515410462156777244275234668693874665788796767699675757959578987749863457937753862877526274731341533
532441044352764132482273798897585496755464759898898996955476545867443535986538526852746625533463346
042116466642255478273735578633656564997786577695957789988869999965866866547392358543326526744105124
305463642721567264236826357995536786844968968785757997557699474697669835738584868845834527260231552
066231251534151145564875937345768799895756579656987695996797768779596848379994862646436522463360266
506513057363414584637639439854956668854878865987777598756576697549657897644947484484534264535522136
131666244551736787837659743739478747677995596779697659755568679946768553647438363837764527562663652
424404413335566473356634745356959967996577965786978695878778775749964899347635788855331715645156443
366410447316422733452686338845457787759555776998978979667795556665499755678449775626855353212651324
302136251253517685366593955978666557656668675898896669776695986549696689467499828826265743355612425
546506521534743584874443398869667685788797956777788696997955989646664668879679457843885333126334344
362262154313747445367249673496655464765879658679769897687679766789496785648597346877862642316202103
026226125354532563527338467655476495659657769687899897897899856576869585587683926347427571714351012
324246031677535478765836973778855888768885696676889798687785885666785975868394627754287365751563132
031351623136767474878299634934847976977866799698978866969865695657475657893553946224747277645245120
026056143144572747422538333594694756756655696798979976879958687957995676939686658475475623336500612
662533125413523284737634669357979456995895856799797767679899585888899799675553357648455413641663031
014206153225663545455694774965848955686699676678777689996997596965668976646983688667633463335404141
413012617325174456878669973969459678955956597997687976869576778966464554568469886777721516427616513
502662203642612255238447486939957458967857897679766997696655868558575999589335922456844736112131654
212661502516734223324889447354689969855787756889968698877896757975877697383989627364337421772343024
062105566125733258245733457473775547779958796868868895787767755559647545379345865238624356123242000
402335614754636628236824679555597465548989986755969565969678898559987586856955673832271674166105362
451334645753376162243335579543677879955598768556565759756869877876795947966569855573232735534601241
500262052615557247375888358635954578969966856685689678868986899965887995757987425824514566556143415
025221464647151347477323745438738647477798995699788989796688598687779795846437577837865212462025305
506261641542645445623578634938859695857548567987658895685668887664467397754563764284864222243613255
544001251632514343434242438697698767785468785559797877757559476976744589553445545348132556324315401
312662335054751647576346847944998349986995969646958568675577475645539379875732364426714676752004534
442336642122316123757636234859863735657746598877646757975758669549947438746824526434264347133600103
110222605363451462773826376864836593598856459788567868745977445869448964569655348846536363204056350
000101232124176476234372634789857987399659768658796566947966644948468844584756623732364133065443412
134465433226374477324823357757785535937975665949586889647565456766846389552458234767667646466124353
421406224300535763513347544755878674693949665984559799965894778876384773264625738631365460212615513
452351220036426717772576544574486575445936589965659469684553983864958887588836456352652752044465624
513131310021516677236857346343788436737376979889979498674877368855367664436225742117611411543534241
025341550553445563557374338552585545787433393754685485676457956894674542662544752536711135543323540
535152616216423251726174753447757785566344854848638858756838495369785886445488572266324260520452211
040010242452167571763413223445374499838658479769797468459895353937866635767567612446231533425561512
540233435611063354343623584674548775576735445736954663985773597646587532834332126771664204465150223
115125460640513624264577427672484376565394997956949478368583668555354324253116433334364541633501141
325235035031330065764773672233563447278366899833655673943396323666327273534372213517324021336301134
210455052102126060367721574443448283734742249765698995536842573866568587565571317173455040136014133
130303041444343616551636147765524538863657233824485633222587838375574238775244261440413305250410213
215411311040450233266672334763832288583822422248882386376826865647256855122326112462046245434214110
230231555011111141243461222762213386584878775753548542777758687642766773174237314624160241043301041
323544455001621231632772663352437368846236258422868628887825466644733647626344222306513541033345051
103143140115330535101443674523313247636278477357763543634765673375252223123145511514215131041302240
232425305344242122243634141575125456216884578682284677823883878251534364736772005201664151453532422
102015414143426060302001452111353522415644252842283555326642135161755673127454553246636405351210230
434225550123100466121402111553624464277266354226553673463535217337233242261322143516321221202244233
241043514014314144343662453056766274716234347315115145214653247673432611160552306341450310405413111
242112143335044304622645260260424573275613637141734623226773527441742321265241643455134145215344111
111301110035433343532532505066367716542154757534155161173124736561116131435620565213531350550432142
242323242143224035106205224411500635634124323342513747342475634262646442514656422055112331122023144
002021344233114150010341663531513334522142253265466242366366534525551142525131661102314124401314042
024004104024242151413420354625126365606642452553232242411563750634302214411623615524332515123123121
322320201030004104054100060453523451362415465573527326430616603635233444523531105134335533102202414
013231044323002420215041311645066036204353535300445225661352044506065120312154410321400034312200320
031030413344440552442114452066553136026033443442226333515665300213012326102141034413121232242040400
133131302014414220215344004521556454235620502443013233464014064103264525100404050140223431001414000
221120341111243124154321401255010441026151433422443520262166251632146441120442553301443334201231200
021222222103332103301112520131442023663055232552042262505152050120510455441325453404132240001402111
";
use criterion::{criterion_group, criterion_main, Criterion};

use adventofcode::{day7, day8, day9, day10, day11, day12, day13};

fn day7_full(c: &mut Criterion) {
    c.bench_function("day7_full", |bencher|bencher.iter(||day7::benchmark_full(DAY7_INPUT)));
}

fn day8_full(c: &mut Criterion) {
    c.bench_function("day8_full", |bencher| bencher.iter(||day8::benchmark_full(DAY8_INPUT)));
}

fn day9_parse(c: &mut Criterion) {
    c.bench_function("day9_parse", |bencher| bencher.iter(||day9::benchmark_parse(DAY9_INPUT)));
}
fn day9_full(c: &mut Criterion) {
    c.bench_function("day9_full", |bencher| bencher.iter(||day9::benchmark_full(DAY9_INPUT)));
}

fn day10_full(c: &mut Criterion) {
    c.bench_function("day10_full", |bencher| bencher.iter(||day10::benchmark_full(DAY10_INPUT)));
}

fn day11_parse(c: &mut Criterion) {
    c.bench_function("day11_parse", |bencher| bencher.iter(||day11::benchmark_parse(DAY11_INPUT)));
}
fn day11_full(c: &mut Criterion) {
    c.bench_function("day11_full", |bencher| bencher.iter(||day11::benchmark_full(DAY11_INPUT)));
}
fn day12_full(c: &mut Criterion) {
    c.bench_function("day12_full", |bencher| bencher.iter(||day12::benchmark_full(DAY12_INPUT)));
}
fn day13_full(c: &mut Criterion) {
    c.bench_function("day13_full", |bencher| bencher.iter(||day13::benchmark_full(DAY13_INPUT)));
}

criterion_group!(benches, day7_full, day8_full, day9_parse, day9_full, day10_full, day11_parse, day11_full, day12_full, day13_full);
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

const DAY9_INPUT: &'static str = "L 1
D 2
U 1
L 1
R 2
L 2
U 1
D 2
R 2
L 2
U 1
D 2
R 1
D 2
L 2
D 1
L 2
R 1
U 2
R 2
U 2
L 2
U 1
L 2
R 1
D 1
R 1
L 1
R 1
L 1
R 1
U 1
R 1
L 1
D 2
L 2
D 2
L 1
R 1
D 1
R 1
D 2
L 1
D 2
L 1
R 2
D 2
R 2
U 1
R 1
U 2
R 1
U 2
R 1
D 2
L 1
D 2
U 1
R 2
D 2
L 1
U 2
R 2
L 2
D 2
L 2
D 2
R 1
U 2
L 2
U 1
R 1
D 1
L 1
D 1
U 1
D 2
L 1
R 1
D 1
R 1
L 2
R 1
L 1
U 2
D 2
U 1
L 1
U 2
R 1
D 2
U 1
D 1
L 1
D 2
U 2
D 1
U 2
R 2
L 2
D 1
R 2
D 1
L 2
D 2
L 2
R 1
L 1
R 2
L 2
R 2
U 1
R 1
U 3
D 2
U 3
L 1
R 1
U 3
D 1
L 1
R 3
D 2
L 3
R 3
D 2
R 2
U 3
R 3
U 3
R 2
U 3
D 2
L 3
U 3
R 1
U 1
R 3
U 2
L 1
R 1
U 1
L 3
R 3
U 3
D 2
U 1
L 1
D 3
R 3
U 2
L 1
R 3
L 3
D 3
U 3
R 2
L 1
U 1
L 3
R 3
U 1
R 3
L 2
R 2
D 3
U 1
D 3
U 3
R 3
D 2
L 2
U 2
D 3
L 3
U 3
R 3
D 2
R 3
U 1
D 1
U 2
R 1
D 3
R 1
L 2
U 3
D 1
L 1
D 3
U 2
L 3
R 3
L 1
D 2
L 3
U 3
L 1
D 2
L 1
D 1
L 2
U 2
D 2
U 2
L 2
D 3
L 3
D 3
R 3
U 1
R 3
U 1
R 3
D 3
U 2
D 1
R 2
L 2
R 2
D 2
R 2
D 1
R 3
D 1
U 2
L 4
D 1
L 2
R 1
U 1
D 3
U 4
D 2
L 3
R 2
D 3
R 4
U 1
D 1
U 1
L 3
U 1
D 3
R 3
D 3
R 1
U 4
R 1
U 1
R 2
L 4
U 3
D 2
R 2
D 1
U 2
L 3
D 4
U 4
D 4
U 4
L 3
U 3
D 3
R 3
L 4
R 3
D 2
L 2
R 1
D 1
L 4
R 3
D 2
U 1
L 2
R 1
U 2
L 4
R 4
L 3
U 4
D 2
U 4
D 4
L 1
U 4
L 3
U 2
D 2
R 2
D 3
R 4
D 1
U 4
D 1
U 1
D 3
L 3
D 2
R 4
D 4
L 2
U 2
L 3
R 4
L 2
R 2
D 3
L 2
D 4
U 3
D 4
L 3
R 2
U 3
D 4
R 2
U 3
R 1
D 1
R 1
D 2
R 3
D 4
L 1
U 3
L 3
U 4
R 1
U 3
L 4
D 5
R 2
D 2
U 2
D 1
R 4
D 1
L 4
D 4
L 1
R 1
D 4
U 4
L 1
D 4
L 4
U 5
R 4
D 4
R 5
U 3
L 5
U 1
R 4
L 1
R 3
L 2
D 5
R 1
D 4
R 2
U 5
L 3
R 5
U 4
R 1
D 4
U 3
R 3
L 3
R 3
L 3
U 2
D 4
U 2
L 3
D 2
R 3
U 3
D 2
U 1
R 5
L 4
R 4
L 4
D 3
R 3
U 2
R 1
D 3
U 3
D 2
L 1
D 2
U 1
L 3
U 5
L 1
D 1
U 5
D 3
U 4
D 4
L 5
R 3
D 4
R 2
L 1
D 1
L 3
U 3
R 1
L 2
D 4
L 5
U 1
L 4
R 4
U 2
D 1
R 5
D 3
U 2
D 3
U 4
D 4
U 1
L 2
D 4
L 3
U 4
L 1
R 1
L 2
R 3
L 2
U 1
D 2
U 5
R 2
D 4
U 1
D 4
U 4
D 5
L 6
D 3
U 2
D 1
L 3
U 4
R 4
D 4
R 3
L 4
U 6
D 1
U 4
R 2
L 1
D 2
U 4
D 5
U 3
D 6
L 1
U 3
L 1
U 1
D 6
L 3
U 6
L 3
D 6
L 1
D 6
U 2
D 5
U 2
L 5
D 3
U 2
D 1
U 6
L 1
D 4
L 6
R 3
D 3
L 3
D 2
R 3
D 5
R 3
D 2
L 1
R 4
L 6
D 4
R 4
U 1
R 6
D 3
U 1
R 1
D 1
L 3
D 1
R 3
U 5
L 5
D 2
L 3
D 2
L 5
U 3
L 1
U 5
L 3
D 4
U 5
D 1
L 6
D 1
R 6
U 4
R 1
D 1
L 2
U 5
R 2
U 6
L 5
U 5
L 6
U 6
D 3
L 1
R 3
L 3
U 4
R 5
L 6
D 5
L 4
R 6
L 6
R 5
D 2
R 4
L 2
U 1
L 3
U 6
L 4
D 6
U 6
D 4
L 2
R 7
U 4
R 1
L 5
D 2
R 3
U 4
L 5
U 7
L 2
R 1
U 4
L 2
R 5
L 6
D 5
L 4
U 5
L 7
U 1
L 5
U 1
L 5
D 3
U 3
R 7
U 7
R 7
L 6
D 4
U 7
R 2
D 3
R 3
U 4
D 4
U 5
D 6
L 5
R 6
U 4
R 6
D 1
R 4
U 1
D 1
R 1
L 4
U 5
L 3
D 5
L 5
D 3
U 2
R 5
D 7
U 7
D 1
U 5
R 6
U 4
R 6
L 7
U 5
R 1
L 1
U 1
R 1
D 7
L 2
U 5
R 4
U 2
D 6
U 6
L 7
U 3
D 2
L 3
R 3
U 2
L 1
R 2
L 5
D 6
R 2
U 6
L 1
D 7
R 4
D 3
L 2
U 5
R 4
L 2
R 5
D 1
U 1
R 6
D 5
U 6
R 2
U 2
L 2
U 6
L 5
U 1
L 1
D 7
R 6
U 5
D 3
L 5
R 3
D 3
U 2
D 7
U 7
L 1
D 5
L 5
U 4
L 5
D 1
U 6
D 5
L 7
D 8
U 8
R 6
U 5
D 8
U 1
R 2
U 6
D 6
L 4
D 2
R 3
U 7
R 4
U 4
R 5
U 6
R 1
U 4
R 1
U 4
D 5
U 3
L 7
U 8
R 1
D 1
U 8
L 8
U 3
L 1
D 4
R 2
D 4
U 2
L 4
D 8
R 7
D 5
U 4
L 5
U 5
D 8
R 8
U 3
R 8
L 4
U 5
R 6
U 7
D 3
L 1
U 5
D 2
R 5
U 4
D 8
L 4
R 1
D 6
R 1
L 6
D 6
L 7
D 4
L 4
D 8
R 8
D 7
L 6
R 1
D 4
L 5
D 2
U 3
D 2
U 6
R 1
U 7
D 7
R 2
U 7
D 5
L 6
U 1
L 8
U 1
L 1
R 9
D 1
U 6
D 5
L 1
R 9
D 2
L 2
D 6
U 6
D 7
L 3
R 5
L 6
U 5
R 1
U 1
R 3
L 9
U 6
L 6
U 2
D 6
R 9
U 4
D 3
L 8
U 5
D 7
L 1
U 7
R 1
L 6
D 8
L 5
R 7
L 2
R 6
L 5
D 3
L 8
D 7
U 8
R 6
L 6
D 7
U 8
D 1
U 2
D 5
U 7
D 8
L 1
D 1
L 1
U 1
R 3
L 5
R 9
D 2
L 8
R 6
D 6
U 1
R 7
L 4
U 4
D 1
L 5
U 2
R 9
D 7
U 8
L 8
R 4
D 1
L 1
U 7
R 8
L 3
D 2
U 8
R 1
D 5
L 1
R 7
D 6
R 2
U 9
D 2
L 5
U 4
D 9
L 7
D 9
L 2
U 9
D 5
L 7
D 6
U 5
D 9
R 3
U 8
L 7
U 1
L 3
U 3
D 1
L 6
R 9
U 2
L 7
R 9
U 5
R 2
L 3
R 4
D 9
L 4
R 9
U 4
L 10
R 9
D 9
U 10
L 2
R 4
U 4
R 9
D 2
R 1
L 4
R 7
D 3
L 6
U 1
L 9
D 6
U 10
D 1
R 10
D 9
L 6
D 10
U 3
R 10
L 8
U 9
D 1
L 1
D 10
L 1
D 5
L 9
U 3
L 4
U 4
D 1
U 3
R 7
L 3
R 3
L 3
R 8
D 9
R 4
L 6
D 7
R 10
U 6
R 2
D 2
L 3
R 9
D 9
U 7
D 2
R 6
U 10
R 8
U 5
R 5
U 10
R 6
U 7
L 7
U 7
D 4
L 9
U 2
R 7
L 8
R 4
L 1
U 9
R 8
D 4
R 5
D 4
L 1
D 10
R 8
D 10
U 9
D 3
R 1
U 10
R 1
D 10
R 1
L 6
D 3
U 8
R 4
U 9
D 8
U 3
D 2
R 3
L 5
U 4
L 3
R 4
D 6
L 9
R 1
D 9
R 2
D 11
R 8
D 5
R 8
L 2
D 2
L 7
U 11
D 10
L 6
R 2
U 9
L 10
D 4
L 5
R 7
D 4
L 1
U 3
L 9
D 7
R 4
D 10
L 5
U 9
L 10
D 5
L 11
R 11
U 8
R 11
U 2
R 7
L 10
R 2
D 8
L 2
D 8
R 11
L 6
R 3
L 2
U 2
R 2
U 3
D 3
U 5
L 8
U 9
R 10
D 1
U 4
D 8
R 3
D 11
R 4
D 4
R 7
U 6
L 11
U 11
D 8
L 5
R 2
L 11
U 7
D 5
U 5
D 9
L 2
U 3
R 10
U 1
R 1
L 11
D 3
L 5
R 2
L 10
R 4
L 1
U 1
L 4
R 5
U 2
D 6
R 11
U 8
D 5
R 5
D 3
L 2
U 7
D 10
L 11
D 10
U 11
R 8
L 2
D 9
R 9
L 11
R 10
U 3
R 4
L 5
R 12
U 5
D 5
L 6
D 11
R 8
D 12
U 6
L 10
R 9
U 1
L 12
D 1
R 8
U 9
D 9
R 6
D 3
L 1
R 3
L 8
R 6
U 12
L 1
U 8
D 7
U 5
D 12
L 4
R 11
U 9
D 12
R 8
D 2
R 7
U 8
R 10
U 3
R 4
D 4
U 9
R 7
L 9
R 7
U 8
L 9
U 6
R 4
L 11
D 11
R 6
U 2
D 1
R 2
L 2
D 11
R 5
U 1
R 1
D 3
U 2
L 3
U 12
R 10
U 12
R 9
D 11
U 9
L 2
R 7
U 5
R 7
U 9
R 3
D 4
L 9
D 4
R 12
L 3
U 12
L 9
U 3
L 11
R 7
U 11
D 2
L 11
R 2
L 12
U 10
L 6
R 8
D 6
R 12
L 7
R 4
U 5
D 10
R 8
L 11
D 4
U 1
R 9
U 12
D 3
L 10
R 11
U 1
L 5
U 11
L 7
U 1
D 13
U 10
L 12
U 9
D 8
L 5
R 4
L 1
R 2
L 7
D 1
L 11
U 9
R 5
D 12
L 4
U 7
L 6
R 7
U 1
L 7
U 2
D 4
R 4
D 7
L 3
D 9
U 3
D 3
U 12
R 2
L 12
R 9
L 8
D 10
U 1
R 11
L 13
U 1
D 3
U 12
L 3
R 7
L 7
D 7
U 7
D 7
U 10
D 1
L 7
D 11
R 3
D 6
R 9
D 5
U 12
D 6
R 8
D 3
R 7
L 6
D 10
U 8
R 9
D 4
R 9
D 2
U 4
R 4
U 7
L 12
R 2
D 10
U 13
R 12
L 12
D 1
R 11
L 2
U 13
L 12
D 7
L 2
U 4
L 4
D 8
L 9
R 3
D 4
L 9
U 9
L 9
R 10
L 4
R 4
U 10
L 11
D 8
U 8
L 13
R 1
L 8
U 13
L 9
U 1
R 13
U 7
L 3
R 8
L 1
D 5
R 7
U 8
D 5
L 12
R 11
U 11
D 10
U 5
R 6
D 11
R 1
D 1
R 9
L 8
D 4
L 11
U 9
D 4
U 3
D 13
U 13
L 6
D 6
R 11
L 9
U 3
R 7
D 9
R 5
L 9
D 14
R 8
L 14
R 6
L 6
U 6
R 7
L 11
R 1
L 6
D 10
R 1
D 14
R 12
U 4
R 2
L 9
R 14
L 3
D 9
U 10
R 11
L 5
R 14
U 3
D 8
R 9
L 13
U 4
L 2
R 4
D 6
L 13
D 4
R 2
L 2
D 10
L 10
R 4
D 2
R 2
U 6
D 7
U 12
R 5
U 13
L 6
U 12
L 3
D 11
R 3
L 7
U 11
D 4
L 13
U 14
R 10
D 12
U 2
L 1
U 12
D 6
U 2
R 12
L 3
D 9
U 9
R 14
L 12
R 5
L 11
R 11
L 5
R 2
L 4
D 8
U 3
R 4
D 12
R 10
L 10
D 10
U 10
R 9
L 14
R 3
L 1
D 1
R 15
D 15
R 12
L 15
D 12
L 10
R 3
U 14
D 4
U 7
R 11
L 10
U 4
D 15
L 3
U 1
R 2
L 15
U 6
L 6
D 3
L 14
D 12
U 1
L 15
R 13
U 7
D 6
R 11
U 7
L 2
D 11
L 1
R 7
D 1
L 4
R 11
U 5
L 4
R 13
U 1
D 4
L 6
R 10
L 15
U 7
D 9
L 10
R 8
U 10
D 8
L 14
D 15
R 14
U 10
D 4
U 6
D 2
R 6
L 6
D 4
U 12
L 3
R 6
U 9
R 3
U 11
R 3
U 15
D 1
U 2
D 3
U 11
L 3
U 2
L 14
D 3
R 9
D 5
L 10
U 7
L 6
D 8
L 8
U 8
L 14
R 9
L 14
U 10
L 8
U 5
L 2
U 4
D 5
R 2
L 1
U 14
R 3
L 4
U 9
D 7
U 14
D 11
R 9
U 5
R 8
L 5
U 3
D 8
R 8
L 11
R 7
L 3
D 16
L 11
U 7
D 6
R 5
L 4
U 9
L 12
R 9
L 10
D 13
L 3
D 13
U 6
R 14
U 2
L 5
R 4
L 15
U 9
D 5
L 6
R 7
U 10
L 3
D 5
U 8
D 12
L 11
U 14
D 12
U 2
D 9
U 11
D 5
L 5
R 10
L 14
U 1
R 8
D 5
R 6
U 6
L 3
U 11
L 8
R 3
U 5
L 1
R 16
D 10
U 12
R 11
D 2
U 4
D 15
U 16
D 2
L 9
R 10
L 3
U 8
R 13
U 11
D 12
U 15
D 1
L 14
R 3
U 13
D 7
R 15
L 3
R 1
U 7
R 6
D 12
L 2
R 13
D 3
R 6
D 10
U 2
R 4
D 12
L 7
R 7
L 8
R 14
L 15
D 8
L 6
U 14
L 16
U 10
D 7
R 6
D 12
R 16
U 11
R 2
U 10
L 1
R 6
U 14
D 13
U 7
L 10
D 4
L 7
R 11
U 17
D 6
U 6
R 6
U 12
L 16
U 5
R 15
L 17
D 13
U 12
L 1
D 7
L 13
R 7
U 2
L 5
U 12
R 10
U 7
R 11
U 17
D 15
U 15
L 10
D 3
L 15
U 8
R 8
D 10
L 9
U 6
R 9
L 1
U 5
R 6
L 11
D 17
L 7
U 3
D 17
R 17
U 8
L 15
R 3
D 2
U 17
L 1
R 13
U 13
L 10
R 2
D 12
L 12
R 13
L 14
R 16
D 12
R 4
U 14
L 15
R 10
D 7
R 12
U 6
L 8
U 7
R 1
D 3
R 3
L 11
D 2
R 15
U 17
L 13
U 17
D 4
U 11
L 17
D 12
R 3
L 12
U 8
D 2
L 14
U 16
D 6
U 7
L 12
R 17
L 7
D 1
L 16
D 16
R 17
D 5
U 16
D 2
L 13
U 9
D 13
R 3
U 1
D 12
R 4
D 11
L 8
D 8
R 13
D 16
R 1
D 10
R 18
U 6
D 4
U 17
R 12
L 14
U 5
L 15
U 18
R 4
D 17
L 4
R 18
U 18
D 4
L 1
U 4
L 2
U 17
R 16
L 7
U 10
R 11
U 9
R 11
L 16
U 3
R 3
L 18
U 8
R 1
U 1
L 5
U 5
D 7
L 1
R 2
U 16
R 17
D 4
U 16
L 12
R 3
L 16
D 5
U 5
R 18
L 7
U 1
L 7
U 16
R 14
L 4
R 13
L 3
U 4
L 11
U 3
L 12
U 16
L 14
U 3
L 9
R 18
D 8
L 15
R 10
D 2
L 13
D 11
U 4
L 13
D 7
L 12
U 7
D 17
U 9
D 5
L 10
R 2
D 7
L 14
U 10
D 17
L 3
R 17
U 16
D 13
R 13
U 14
R 9
L 3
U 14
L 7
U 4
D 1
R 18
U 11
D 9
U 8
R 15
L 5
D 9
L 2
D 18
U 12
D 3
U 14
D 9
U 13
R 16
U 8
R 16
U 4
L 8
D 14
R 15
D 6
R 17
U 6
D 1
L 2
U 9
D 15
L 7
D 3
R 1
L 1
R 17
D 12
R 19
U 4
R 19
L 2
D 17
U 11
D 1
L 9
R 18
L 7
D 13
U 6
D 16
L 6
R 13
U 12
R 1
L 2
U 8
L 16
R 10
D 15
R 5
L 1
U 5
R 18
U 1
R 4
U 14
R 3
L 4
U 13
D 14
L 10
D 7
U 3
R 17
L 19
R 1
D 12
L 9
U 19
D 19
L 5
R 10
L 14
U 15
D 19
U 18
D 14
R 11
U 8
R 2
U 16
L 17
D 6
L 1
D 10
R 18
U 5
R 14
U 10
D 6
L 17
D 6
U 7
R 14
L 1
U 3
D 3
R 18
D 1
L 6
D 18
L 2
R 10
L 9
R 10
L 14
R 15
U 10
";

const DAY10_INPUT: &'static str = "noop
addx 33
addx -30
noop
noop
addx 7
addx 1
noop
noop
addx 3
addx 3
addx 3
addx -4
addx 5
addx 2
noop
addx 7
noop
addx 1
addx 4
noop
addx 1
addx -38
noop
addx 16
addx -13
addx 2
addx 7
noop
addx -2
addx -10
addx 17
addx -5
addx 10
noop
addx -15
addx 16
addx 2
noop
noop
addx 7
addx 3
addx -2
addx 2
addx 5
addx -38
addx 7
addx -6
addx 2
noop
addx 7
noop
addx 1
addx 4
noop
noop
noop
noop
noop
addx 3
noop
addx 3
addx 2
noop
addx 7
noop
addx -20
addx 21
addx 3
addx 1
addx -35
addx 1
addx 4
noop
addx 31
noop
addx -26
addx 5
noop
noop
addx -2
addx 25
addx -18
addx -13
addx 14
addx 2
noop
noop
noop
addx 6
addx 1
addx 5
addx 3
addx -2
addx -38
addx 24
addx -17
addx 5
noop
noop
addx -2
addx 31
addx -24
addx 7
addx -10
addx 6
noop
addx 3
addx 2
noop
noop
addx 7
addx -2
addx -26
addx 31
addx 5
addx -40
addx 5
addx 33
addx -31
noop
addx 1
addx 4
addx 1
addx 4
addx 20
noop
noop
addx -14
addx -1
addx 5
noop
noop
addx 1
addx 2
noop
noop
addx 7
noop
noop
noop
noop
noop
noop
";

const DAY11_INPUT: &'static str = "Monkey 0:
  Starting items: 57, 58
  Operation: new = old * 19
  Test: divisible by 7
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 66, 52, 59, 79, 94, 73
  Operation: new = old + 1
  Test: divisible by 19
    If true: throw to monkey 4
    If false: throw to monkey 6

Monkey 2:
  Starting items: 80
  Operation: new = old + 6
  Test: divisible by 5
    If true: throw to monkey 7
    If false: throw to monkey 5

Monkey 3:
  Starting items: 82, 81, 68, 66, 71, 83, 75, 97
  Operation: new = old + 5
  Test: divisible by 11
    If true: throw to monkey 5
    If false: throw to monkey 2

Monkey 4:
  Starting items: 55, 52, 67, 70, 69, 94, 90
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 3

Monkey 5:
  Starting items: 69, 85, 89, 91
  Operation: new = old + 7
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 7

Monkey 6:
  Starting items: 75, 53, 73, 52, 75
  Operation: new = old * 7
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 4

Monkey 7:
  Starting items: 94, 60, 79
  Operation: new = old + 2
  Test: divisible by 3
    If true: throw to monkey 1
    If false: throw to monkey 6
";

const DAY12_INPUT: &str = "abaaaaaccccccccaaaaaaaaaaccccaaaaaaaaaccccaacccccccccccccccccccccaaaaaaaaaaaccaaaaaaaaaccccccccccaaaaaaaacaaaaaaccccccccccccccccccccccccccccccccccccccccccaaaaa
abaaaaaaaccccccaaaaaaaaaacccccaaaaaacccccaaaacccccccccccccaacccccaaaaaaaaaaaacaaaaaaaaacccccccccaaaaaaaaaaaaaaaccccccccccccccccccccccccccccccccccccccccccccaaaa
abacaaaaaccccccccaaaaaacccccccaaaaaacccccaaaacccccccccccccaacccaaaaaaaaaaaaaacaaaaaaaaccccccccccaaaaaaaaaaaaaaaaccccccccccccccccccccccccccaaaccccccccccccccaaaa
abccaacccccccccccaaaaaaccccccaaaaaaacccccaaaacccaacccccaaaaaaaaaaaaaaaaaaaaacaaaaaaaccccccccccccacaaaaaccaaaaaaaccccccccccccccccccccccccccaaccccccccccccccaaaaa
abcaaccccccccaaaaaaaaaaccccccaaacaaaccccccccccccaaaaaccaaaaaaaaaaaaaaaaaaaaaccaccaaacccccccccccccccaaaacccaaaaaaccccccccccccccccccccccccccaaacccccccccccccaaaca
abcccccccccccaaaaaaaaaaaaacccccccccacccccccccccaaaaacccccaaaacaaaaaaaaaaaaccccaaaaaaccccccccaaacccccaaccccaacccccccccccccccccccccccaaaaccaaaccccccccccccccccccc
abccccccccccaaaaaaccccaaaacccccccccccccccccccccaaaaaaccccaaaaaaaaaaaaaacaaacccaaaaaaaacccccaaaaaacccccccccccccccccccccccccccccccccccaaaaaaaaacccccccccccccccccc
abacccccccccaaaaaacccccaaacaaacccccccccacccaaccccaaaacccaaacaaaaaaaaaaacccccccaaaaaaaacccccaaaaaaccccccccccccccccccccccccccccccccjjjjjjjaaaaaaaaaccccaacccccccc
abaccccccccccaaaaaccaaaaaaaaaaccaccccccaacaaacccaaccccccaacccaaaaaaaaaaacccccccaaaaaaacccccaaaaaccccccccccccccccccccccccccccccccijjjjjjjjjhhhhhhhhhcaaaaaaccccc
abaacccccccccaaaccccaaaaaaaaaaaaaccccccaaaaacccccccccccccccccccccaaaaaaacccccccaaaaaccccccccaaaaacccccccccccccccaaaaccccccccccciijjjjjjjjjhhhhhhhhhhcaaaaaccccc
abaaccccccccccccccccccaaaaaacaaaaaacccccaaaaaacccccccccccaaacccccaaaccccccccccaaaaaaccccccccaacaacccccccccccccccaaaaccccccccccciiioooooojjhhhhpphhhhhaaaaaccccc
abacccccccccccccccccccaaaaaaccaaaaacccaaaaaaaacccccccccccaaaaaaccaacccccccccccccccaaccccccccacccccccccccccccccccaaaaccccccccccciiioooooooooopppppphiiaaaaaacccc
abaccccccccccaaaaaccccaaaaaaaaaaaaccccaaaaaaaacccccccccaaaaaaaaccccccccccccaaaaccccccccccccaaacccccccccccccccccccaaccccccccccciiinnoouuooooopppppppiiaaaaaacccc
abcccccccccccaaaaaccccaaacaaaaccaacccccccaaccccccccccccaaaaaaaaccccccccccccaaaaccccccccaaacaaacccccccccccccccccccccccccccccccciiinnotuuuuoopuuuupppiiaaacaccccc
abccccccccccaaaaaaccccaccccccccccccccccccaaccccccccccccaaaaaaacccccccaaacccaaaaccccccccaaaaaaaaaacccaacccccccccccccccccccccccciiinntttuuuuuuuuuuuppiiiaaccccccc
abaaccccccccaaaaaaccccccccccccccccccaaaacccccccccccccccccaaaaaacccccaaaaccccaaccccccccccaaaaaaaaacccaaacaaacaaaccccccccccaaccciiinntttxxuuuuuuuuuppiiiccccccccc
abaacccccccccaaaaaccccccccccccccccccaaaaccccccccaccccccccaaaaaacccccaaaaccccccccccccccccccaaaaacccccaaaaaaacaaaacccccccccaaaaiiiinnttxxxxuuyyyuvvppiiiccccccccc
abaacccccccccaaaccccccccccccccccccccaaaaccaaacaaaacccccccaaccccccccccaaacccccccccccccccccaaaaaaccccccaaaaaacaaaacccccccccaaaaiiinnnttxxxxxxyyyvvppqiiiccccccccc
abaccccccccccccccccccccaaccccccccccccaacccaaaaaaaacccccccccccccccccccccccccccccccccccaacaaaaaaacccaaaaaaaaccaaaccccccccaaaaahhhinnntttxxxxxyyyvvqqqiiiccccccccc
abcccccccccccccccccccaaaaaaccccccccccccccccaaaaaaaaaccccccccccccccccccccccccccccccaaaaaaaaacaaacccaaaaaaaaaccccccccccccaaaaahhhnnnttttxxxxyyyvvvqqqiiiccccccccc
SbcccccccccccccccccccaaaaaaccccccccccccccccaaaaaaaaaccccccccccccccccccccccccccccccaaaaacccccccacccaaaaaaaaaacccccccccccccaahhhnnntttxxxEzzzyyyvvvqqqjjjcccccccc
abcccccccccccccccccccaaaaacccccccccccccaaaaaaaaaaaacccccccccccccccccccccccccccccccaaaaaaaccccccccccccaaacaaacccccccccccccahhhmmmtttxxxxxyyyyyyyvvvqqqjjjccccccc
abccccccccccccccccccccaaaaacccccccccaacaaaaaaaaaaaaccccaccaaaccccccccccccccccccccaaaaaaaaccccccccccccaaacccccccccccccaaccahhhmmmtttxxxyywyyyyyyvvvqqqjjjccccccc
abccccccccccccccccccccaaaaacccccccccaaaaaaaacaaacccccccaaaaaaccccaccaaaccccccccccaaaaaaaaccccccccccccaacccccccccccccaaaccchhhmmmsssxxwwwyyywyyvvvvqqqjjjccccccc
abccaacccccccccccccccccccccccccccccccaaaaaaccaaacccccccaaaaaaccccaacaaacccccccccccacaaacccccccccccccccccccccccccaaaaaaaccchhhmmmssssswwwwyywwvvvvvqqqjjjdcccccc
abccaaaccaaccccccccccccccccccccccccaaaaaaaaccaaccccccccaaaaaaacccaaaaaccccccccccccccaaacccccccccccccccccccccccccaaaaaaaaaahhhmmmmsssssswwywwwrvvqqqqqjjjdddcccc
abccaaaaaaacccccaaaccccccccccccccccaaaaacaacccccccccccaaaaaaaaccccaaaaaaccccccccaaaccccccccccccccccccccccccccccccaaaaaaaaahhhgmmmmmsssswwwwwrrrrrqqqjjjjdddcccc
abcccaaaaaacccccaaacacccccccccccccccccaaaccaacccccccccaaaaaaaaccaaaaaaaacaaccccaaaacccccccccccccccccccccccccccccccaaaaaaaccggggmmmmmmssswwwwrrrrrkjjjjjddddcccc
abaaaaaaaaccccaacaaaaaccccaaacccccccccaaaccaaccccccccccccaaaccccaaaaacaaaaaccccaaaacccccccccccaacccccccccccaaccccaaaaaacccccgggggmmmllssswwrrrkkkkkjjjddddacccc
abaaaaaaaaacccaaaaaaaaccccaaaacccccccccaaaaaaccccccccccccaaacccccccaacccaaacaaacaaaccccccccccaaaacccccccaaaaaacccaaaaaaacccccgggggglllsssrrrrrkkkkkkdddddaacccc
abaaaaaaaaaacccaaaaaccccccaaaaccccccccccaaaaaaaccccccaaccccccccccccaaaaaaaaaaaaccccccccccccccaaaacccccccaaaaaccccaaccaaacccccccggggglllsrrrrrkkkeeedddddaaccccc
abaacaaaaaaaccccaaaaacccccaaaccccccccccccaaaaaaccccaaaaccccccccccccccaaaaaaaaacccccccccccccccaaaacccccccaaaaaaacccccccaacccccccccgggglllrrrrkkkeeeeeddaaaaccccc
abaaaaaacccccccaaacaacccccccccccccccccccaaaaaccccccaaaaaaccccccccaaacccaaaaacccccccccccccccccccccccccccaaaaaaaacccccccccccccccccccggfllllllkkkeeeeeccaaaaaacccc
abaaaaacccccccccaacccccccccccccccccccccaaaaaacccccccaaaacccccacccaaccccaaaaaaccccccccccccccccccccccccccaaaaaaaacccccccccccaacccccccffflllllkkkeeeccccaaaaaacccc
abaaaaacccccccccccccccccccccccccccaacccccccaaccccccaaaaacccccaaaaaaaccaaaaaaaaaaccccccccccccccccccccccccacaaacccccccccaaccaaccccccccfffllllkeeeecccccaacccccccc
abaaaacccccccccccccccccccccccccccaaacccccccccccccccaacaacccccaaaaaaccaaaaacaaaaaccccccccccccccccccccccccccaaacccccccccaaaaaaccccccccffffffffeeeeccccccccccccccc
abaaaccccccccccccccccccccccccccccaaaaacacccccccccccccccccccccaaaaaaaaaaaaccaaaaaaaaccccccaaccccccccccaaaaacccccccccccccaaaaaaacccccccfffffffeeaaccccccccccccaaa
abaacccccaacaacccccccccccccaacaaaaaaaaaacccccccaaccccccccccccaaaaaaaaaaacccaaaaaaaacccccaaacaacccccccaaaaaccccccccccaaccaaaaaaccccccccafffffeaacccccccccccccaaa
abaaaccccaaaaacccccccccccccaacaaaaaaaaaaccccccaaaccccccccccccaaaaaaaaaaaccccaaaaaccccccccaaaaaccccccaaaaaacccccccaacaaaaaaaaccccccccccaaacccccccccccccccccccaaa
abccccccccaaaaacccccccccaaaaaaaaaaaaaacccccaaaaacaaccccccaaaaaaaaaaaaaaaaaaaaaaaaacccccaaaaaacccccccaaaaaacccccccaaaaaaaacaaccccccccccaaaccccccccccccccccaaaaaa
abcccccccaaaaaacccccccccaaaaaaaaaaaaaacccccaaaaaaaaccccccaaaaacaaaaaaaaaaaaaaaaaaacccccaaaaaaaacccccaaaaaaccccccaaaaaaaaccaacccccccccccccccccccccccccccccaaaaaa";

const DAY13_INPUT: &str = "[[],[[[]],9,[],1],[1,0]]
[[[10,8,0,[4,3],[1,4]],4,[7,5,[9,10,7,10],8]],[8,[[1,5,9,1],6,[10,5],5,[7,1]]],[[0,7,3],[[5],[8,3,0],4]],[4,2]]

[[8,6,[[9,0,0],2,[6,3,2,0]],[],6],[10,[10,10,8,2],[[4,3,7],4,8],1,7]]
[[[],0,[[8,8,10,5,7],1,5,0],[]]]

[[[],6]]
[[9,[],3,[9]],[],[[1],[10,9,4,[]],[2]],[]]

[[10,3],[3,1,[3],8,5],[],[6,2,7,[10],[[0,8,9,10],5]]]
[[4],[0,1,[10]],[],[],[0,[[3],[7,7],8,[9,0,0],3],3,8]]

[[9,[],[[7,8,10],1,[7,6,9],[7,5,0],8]],[[3,[],[4,8],[]]],[10,[9,0,7,9],[[0,8,8],[],[6],[4,6],[8,7,0,9]],[10,[],8,[],5]],[[[7,10],[5,10],[4,0,8,3,9],[]],[2]]]
[[[8,8,[]],10,[9,1],[[8,2],[],9,[9,2,7],10]],[[],6,6,[5,3,0,5],3],[[],10,2,[10,[3,1,5]]],[],[10,[],0]]

[[6,2,[0,[5,7],[9],1],6],[0],[[5],4,[[10],[0,5,10],3,[]],[[6,4,9,0,3],[6,4,10]],6],[[1,[0,0,3,1]],[[9,0],3,3,9]],[[[4,5,3,2],[],[3,1,7,2,3],10]]]
[[[7],9,3,0,9],[8]]

[[],[10,6,3,[[2,2],4,2,[7,5,10,5]],[9,3]],[[],[7,7,6,[10,6,2,0],[7,10,5,2,7]]]]
[[],[3,[10,[3,3,8,2,4],6,[10,3,10,10],3]]]

[[],[],[9,10,6],[7,[[4,7,10,6,1],[7,3],7,1,[]],4]]
[[10,0],[10,[6,6,3,[9,5],[7,6]],[],[0,[10,8,10,10],[3,3,1,2,5]]],[[7,[9],8,[3,10,4,5,0],5],[3,2,3],[[9,2]]],[]]

[[0,[[],1,9],[[7,4]],6],[6,[3,6]],[6,[[7,10],[8]],[]],[6,7,9,[7,[4,5,5,9],[],[8,2],7],3],[0,[7,5,6,0],4,[2,10,[4,8],0,1],[]]]
[[0,[],8,[[1,8]],7]]

[[7,[8,5,9,[10,2],[0,1,6]],[10]],[[[2],[1,6],[1,10],[0,0,1]],[[6,2,7,7,0],10,[5,0],[],6]]]
[[1,[[4,7,4,8,6]],2,7,[0,[2],[7,8]]],[[]],[5,[]]]

[[[[],3]],[[7,6,10,[4]],8,3],[4,[6,[2,4,0],[8,10,0,4,0]],0]]
[[[4,7,8],0,[[9,4,5,4]],7]]

[[[[7],2,1],[10,[9],[2],[3,3,4,5],[9,7,10,7,6]],[[8,7],[5,6,8,8,1],[6,1,7,7],0,3],3],[0,[[9],1,[4,0,6],3],9,[[]],3],[[],7,[],[4,[10,2],[0,7],[]]],[[[5,1],9,2],[],[2,9]]]
[[[9,[1,5,6,4,1],[7,9,10],1,0],4,[3,9]],[]]

[[],[[[4,1]],0,4,2],[[[3],[2,7,4]],0],[9,[[0,2],[9,2],10],6,[]],[]]
[[[5],8]]

[[[4,8,[10,2],[4,0],[4,3]],3],[8,6,[[0,6,1,4],[6,5],2,[5,2,8,0,6],[6,1,0,5]],[[7,9],[6,8,10,8],[7,1]],[0,[0,3,5,5,5],[3,2,7],9,4]],[3,[0],2],[7,[[],5,9],4,[],[]],[[2,2,[2,6],[8]],2]]
[[10,[0,[0,5,5],[6,3],10],[2],0],[[[1],9,9,1],[[1,1,2,5,3],0,3,3]],[6],[]]

[[7,7,7,8,2],[4,[]],[[3,[1,5,7,8],[7,4,10,4,0],8],[10,8,[5,4,2],5,[9,7,6,9]],[3],6,[5,[7,6,7]]]]
[[2,9],[]]

[[10,10,[]],[[[7,7,0]],0,[[],7,[5,4,6],[0,10,2,1],[4,10]],8,[[2],[],[],[4,1],8]],[],[[],8,[3,5],[9],[[10,6,4]]]]
[[[[9],7],[],6,[8,[10]],[4,2,7,0,0]]]

[[4,[[8,10,5,7,4],4,[7,6,1],[6,0,2,6],9],[],7,[]],[3,[[7,4,9],[8]],[6,[2,8],[4],8,7]],[[2,[7,1,3],[9,2,1],2,[2,7,6,9]],[[9],2,[10,1,3,10,8],[9,5,4,8,0]],[7,8,5,[]]],[[[2,5,4],[1,10,9],[],10,2],7],[7,[[8,2,7],[9,10,6],0],7]]
[[[[5,9,10,8],[],[],2],1,[[5,3],5],[],6],[[[6,2,8,6],4,2,[4,4]]],[[9],[2,5,[3,10],8,[]],6,[5,6,[9,9,6],10,[4,3,1,8]]],[]]

[[8,1,4,2],[8,10,[9,7,[5,3,4],[4,7,9,1],7]],[2,[7,1,3],7,0]]
[[4,6,[1],[9,0,[8,5,6,4],[10,4,6],2],5]]

[[[],3]]
[[0],[1,[0,2]],[0,[[10,9],10,[6],[6,8,0]],4],[],[[1,[7,6,5,2,7],[1,5,0,3,8]],[0,8,4],[],[0]]]

[[4,10,6],[1,5,[[9,4,10,1,9],[5,10,7,2],[9,0,3,1]]]]
[[[[7,0,4,9],[3,9,2,0,8]],[10,[],3,6,[]],9,3,[[3,10,4],[6]]]]

[[10,[[9,9,9,7],8,6,3,[7,10,9,6]]],[[[9,2,1,9],6,[],4,3],9,[0,1],3],[10,[[],4,[5]],[]]]
[[[1,[3,5,10],[10]],6],[[[4,8,9],6,[]],9,[3,[7,3,8],1],1]]

[[[[7,6],[0,3,7,9,7]],8,4,5,[[4,5,6,7]]],[[8],[10,5,[2,9,10,7],[]]],[[[8,5,7]],6,8],[[1,0,4,4],0,0],[[9,7,1],[9,[2],[9]],[10,[9,8,2,1]]]]
[[3,7],[5,6,[0,[6,0,8,6]]]]

[[],[[6,10,6,[5,5]],[[],[],[1,8,1,6,4],1]],[10,[9,[7],[0,9,0],5],[[6,9,0,3],3,[4,5,0],3,3],4,[[10,7],4,[6,1,0],[6],3]],[]]
[[10,[[3,1,8,8,7],[0],[7,8,9]],5],[2,[[]],2,4,[5,7]],[[[3,4,3,5,6]],4,1],[4,[[9,8,8],[8,5,0]],[]],[[2,5,2]]]

[[4,[1,1,[3,5,5,1]],[5,[4,2,2,6],3,9,5],[[],1,[4,3],0],[[5,8,0]]],[[4,[],1,[0,8],8],3,6],[4,[[3,8,0,1,8],[2,10,7],6,[2,3]],4]]
[[[],2,[[],0],[]],[[6],[10,3,1,[0,6],[8,4,6,4]],[6,[9],[1,8,6],2],[[1],3,5,[]],9],[],[],[[[0,9],[6,4],[4],0,6]]]

[[],[[8,0,[9,3,10]],[4,[6],7],6]]
[[[[10,5,4,3,5],[7,0,1,7]],[[10]],1,10,2],[[7,[1],[7,1,4],[]]],[1,[[8,1],[4,2,4],[1]]]]

[[],[[[0]],7,[[9,9],[7,4]],5,6]]
[[10,[6,[],0],[1],1,[7,[1,4,7,2,9],10,4]],[[3],[[],4,9,[]],[[4,5,1,5],[5],[9,7],[8,3,2,8,8],3]],[4,7,[],10,[]],[0,7],[10,[[10,2],[8,5,3,1,10],[4,0],10,8],[4]]]

[[5,[]],[[],[[7,9],4],[[0,3,9,0],[0,0,6,10],0,2,1],[[5,1],[3,1,4,4],[5,4,3,9],[8,10,7]],7]]
[[[5,[3]],[[10,10,9,3],3,10,10],[[5],10,[3,7,6,8,4]],7],[[],0,8,[5,[8,1,4,6]],[8,1,[9,7,1,1],[2,7,0,2,4],[10]]],[4],[9,10,9,[]]]

[[[0,[9,7],6],[9,[],9]],[5,[[1,7,0],10]],[9,[5,10,8]],[1,[[9,5,0,3,10],1,7,[6],5]]]
[[],[],[],[[[7,4,2,1,6]],2],[[[10],4,[7,5,5,2],5],6,[8,[1],10],[],8]]

[[1],[10,[2,9,5,7],6],[]]
[[2,[4,[1,10,4]],4,5],[[[3],[3]]],[[8,2,[6,3,3,2,3],0,7],[[4,2],[],[3,9,7,4]],0]]

[[4],[7],[[[7]],[9,8,[9,5,2],[4]]]]
[[0],[],[2,1,0,10],[9,[[4,7],6],[1,0,3,4,[0,8]],[[],3,[],[5,7,3,9,2],4]],[7,[],[]]]

[[[6,2,[3,10,0],[4,2]],8,[2,[0]],[[],7]],[7,[[7,10,1]],0],[5,1,3,[[10],2,[5,1]],9],[],[[9,[0],9,0,1],9,[[0,4],4,[6],[],[5]],4]]
[[0,[[8,10],7,5,7,[5]],7],[],[[5,[]],6,1,[4,[8,1,7,10],5,[7,10,6],[7,5,8,8]],[]],[1,10,2,[3,[2],2,0,9],[[2,8,3],[3,2,10,8],[9,2,6,9,8]]]]

[[6,7],[[[8,10,10,7],[2,10,6],[9,2,8,8,5]],[[7],[5,2],[6,9,10],8,4],6,5,[]],[],[[[8,1]],[5,6,[4,9,8],5],[[2,8,7,2,3],[4,4,10,3],[10,2,8]],9,4]]
[[[10,[9,2],2],[]],[10,[[3,4,3],[4,4],[6,4,2,6],2,[6,7,7]]],[],[[[],[4],[9,10,2,1,6],1,[]],[5]]]

[[6],[[9,10,2],[[3,4,6],5,9],[5,4]],[[3,3,8,[],1],[[2,2,6,10,3],6],[6,9,3,[8]],1,[[1,9,9,10],7,6,[5,0]]],[0,4,3,[[2,5,1,10,8],7,6,[],[7]]]]
[[9,10,3],[[],[[1,0,0],[10],[9],[2,5,0]],[],8],[10,[3,[2],10]],[[]]]

[[],[[2,10,[]],[],1,[0,[9,6,1,4],6,[1,9,2,2,0]],[]],[[]]]
[[1],[2,[[],2,[7,3,5]],9,2],[[[5,1,8,0],[7],5,6],[5,1,6],5,[[6],[5],3,1],[3,[6,3],[],10]],[8,[]]]

[[[[1],[0,7,6,3,8],1,[],8],[],3],[],[[]],[[[],[10,10,10]],6,8],[5,5]]
[[[9]],[[[1,9,4,3,3]]],[[[6,10,5,9],9,9,[9,10,5],[10,5,2]],[9,[9,9,3,1],[7,0,8,3],3],9,0,[[0]]],[6,3,[10,10,[5,1,0,2],5,4],9,9],[6,[[3,6,9,9,4],[6,7,7,0,9],[0,2,2,4,9]],[[10]]]]

[[4,[2,[6,10,1,1]],[[7,5,1,1,9],9,0,[8,7,8],10],2],[[[4,10,8],[2]]],[[]]]
[[[[10,10,7]]]]

[8,3,10,5,6]
[8,3,10,5]

[[7,[6]]]
[[4],[[5,1,[8,8,6,9],[9,0,7,8],[2,5,10]],[],[8,[5,6]]],[8,1,10]]

[[],[1,8,[9],[1,2,[9]]],[2,3,3],[5,[]],[[1],8]]
[[[[4,8],[],6],[7,2,[4,3,9,3],[10,5,10,9]],[10,[0],0],1,2],[[9,[5],[0],[2],[1,5,10,9]]],[[[2,9,4]],8,[5,[1,0,0,8]],10]]

[[10,[],1],[[2,[6,1],[6]]],[9,[[10,6],[1,0,6,3],3,9],4,5,[[4,0,3,10],7,[4]]]]
[[[[],10],7,1]]

[[2,10,4,9]]
[[4,5,[]]]

[[7,[[]],4],[[9,1],[8,[10,8,3,4],[1],[0,2,1,5,3]],8,6],[[[1,7,7,5],5],3],[[2,5,3],8,[],2],[[0,9],7]]
[[4,4,[10,4],[[5,1,8,6,9],0]],[6],[[6,0,[8,7,4,0],0],8]]

[[7,[6,[3,9,8],[6,4,8,1,4],[]],2,1],[[],[],2,3],[]]
[[9],[],[[[7,0,8,7,10],1,[4,4,10,9],8],[[6],10],[]]]

[[0],[[9],10,6,0,[8,[5]]],[3,[[5,4],[2]],10,6],[[[3,1],3,[],[0],6],3,[9,0],[10,[]]],[0,10]]
[[6],[3,2,7,[[6,2,8,6],[4,9,10],[8,1,8,1],0],5],[9,[],3],[1,[[4,4],[],9,6,1],[[1],2],10]]

[[[[4],1,7],5,[[7]]],[]]
[[],[2,[7,[0,3,6]],[[],[8,9,6,6,1],[]],[4]],[[3,[2,2,2,3],1,[5,6,8,3],9],2],[[2,6],8,[7,5,5,[1],[5]],4],[[[8,8,10]]]]

[[[9,[],[],[6,3,4,0,6],5],5]]
[[3],[]]

[[[2]],[0,[2,7,8,[6,8]],7,[3,[5,8,7,4,1],[8,2,4,3],4]]]
[[],[[0],[1,0,0,9,0],[[5,3,0,7,8],[8,9,3],4],0,[10,[7,7],9,8,[3,0]]],[7,3,3,[[],5,[7,1,5,4,10],3,1],[7,[10],9,[2,1,0,2,8]]]]

[[[4,7,[2,0,7,7,4],3],3,3],[[],6,[5,[0,6],[7,2,5,0,8],[0,7],5],9,[8,[8,3,1,6,10],[7,8,3,0]]],[],[4,6,[5,7]]]
[[4,[[6,10,10],[3,1]],[[],[8,3,1],[5,1,5,9],[],9]],[[[],3],7,1,[10],[[9,0,5,6],6,[1,0,9,7,7]]],[7],[[[6,5,6],9,[8,5,5,2],7],2,9,[7,[9,7,1,3,9],6,6],0],[1,[[2,9],2],9,[2,3],[2]]]

[[5,[2,[2,3,6,2],3],[10,[5,0,3],[2]],5],[[4,[7,7,0,0],[3,6,7,9,5],[4,0,8],[9]],[[2,10],[7],5,[0,0,8,1,7],2],[1,[9,8,6,4],6,[1,7],9]],[7,[[8],[],[4,1],10,3],5,2]]
[[0,[1,6,8,7],6,4],[[9,3,2,3,[7]]]]

[[],[[7],5,4],[[[9,1,6,9,0]]],[4,[]],[1,[[3,0,3,9],1,[10,1,2,2]],10,[[6,10,0,1,3]]]]
[[],[1,1,[],[7,[3,0,6],4,[5],[3]],[7,10]]]

[[[9,[10,6],5,[1,5,8,7,0],4]],[],[2,[10,8,[1,5,2,0],5],8,3]]
[[[10],9,1,[9,1]],[[],2,[[1,6,1,0],1,[],9,[9,2]]],[],[[10,2],[4,2,0,[8,0,9,9]],[5,10],[10,[5,10],2],[[3],[10,5,10,8,9]]]]

[[8,[[4,4,7,0,10],4,[4,2,9],5,[1,0,3]],2,2,8],[]]
[[[[8,3,6],4,6],5,[0,[5,9,1,0],10],2],[[[],[6,0,4,7]]],[[6,2,[2,6,6,6],2],[],[1],8]]

[[],[6],[4,5,0],[]]
[[[[10,9,6,1,3],[],[5]],2,[[3,2],4,[],[3,6,6,6,10],[0,0]],10]]

[[[6,1,2,[9,0,6,1]]],[[1]],[[[2,9,4],3,10],[6,5],[8,[]],[7],[6,9,8,[]]]]
[[1,[[6],0,10],[8,5,10,10],[[9,10,5,0,8]],[10,[10,7],10,1,[0,7,5]]]]

[[2,[[7,9,4],[8,1],8,[5,5,8,9],1]],[[4,8,[]],[],1,0],[7,1,9,[[1,0,3,0],[4,1,7,6,8],[7,6],[7,8],[1,9,5,5,7]],10],[[[1,3]],8,[[7]],3,[]],[10,9,[[7,3,8],9,0],[]]]
[[],[8,[8,[9],0,[0,7],3],[2,[3,2,5,10],4,4]],[],[[[0,10,3,7,8],0,10,5]]]

[[1,[9],[7],[],[[0,8,10],[4,0,8,9,2],[3,4,7,8,0],[],4]]]
[[[3,[6,3,1]],9,[[9,6,8,10]],[[7,3]],3],[5,[4,[9,1],[5],[5,4,4,6,6],0],[[8,9]]],[8],[[[],2,[0]]]]

[[[0,5,9,2,10],[9,1],[[],[0,3,0,0,4],6]],[9],[6],[10,0]]
[[[2,[6,2,9,6]],6,[[5]],0,[[3,5,4,6,10]]],[10,4],[[],[],[[10,7,3],2,0],[4],4]]

[[],[],[],[[[9,9,6,9],[8,0,2,3,2],3],8],[[5,[2,7,6],3]]]
[[],[],[[4,[],[0,6,7,8,6]],9,1,9]]

[[7,[],5,[7,[0,2,1,6],[7,9,0],10],[]],[9,6,6,9,[2,[6,0,1,6],[10],[10,6,8,3,0]]]]
[[[10,9,[3,10,5,0,0],10],[[0,9],[6,4,10,1,9],1,[5,8],4],8,[[7,10,10,1,3],[0],[7,9,3],9,4]]]

[[7,4,[[],4]],[9,10,[7,7,3,4,[]],7],[3,2,10,9,[[8,6,5,1]]],[]]
[[7,1,[3,3,[2,9,9,4,1],[1,5,1,5,1]]],[[[],[],[8,5,1,3,3],10]],[2,7,[5,[1,10,6,7,2]]],[[[],[7,8,5,2,4],0,6,[4,5]],0,[[2,6,4,3]],[[4,5,0,2,1],2,9]]]

[[[2,2,[2,8]],1,[],[],0],[],[5,[5,[],[9,7],[5,2,3,6,0],2],9,5,5],[[7,2,[9,3,7,4],[7,3,9,1,4],0],[]]]
[[[[7,3,5,2]],2,7,0,2]]

[[[5,10,[5,1,0,10,6],6,[10,9,6]]],[6,[[3,1,7,8],7,4],6,6,9],[[3,0,10,5]],[9,[9,2,6],[3,[],[6,9,10],[9,6,1]],7]]
[[0,[[5]],[],[[0,8,10,5,5]],2]]

[[[10,10,8,[7]],[6,8,[9,5,6,6,9]]],[7],[[3,[10,5,9,4],0,9,1],7,[[],[6,4,10,7,4],[],5,[]],[0,6,7],[0,[7,3,2,4],[5,10]]],[[[6,5,0]],[10,[9,3,5],[8,9,0,4,1],[7,2,7,0]],2,[]]]
[[8,7],[],[7,3,[8,[0]]],[10,6,6,[[],8,[]],9],[4,0,3,10,[]]]

[[[[4,0,3,10]],[6]],[8,2,7,8],[]]
[[5,2,4],[6,[5,[8,8,4,10],5,0,[9,3,6,3]],[],[0,8,[7,7,6,0,0],9],[]],[10],[0,[[2]],[[9,8,10,5,6],[],7,4,1],5]]

[[[],7,2,0],[5],[],[3,9,[[6,1],8],9,[7]]]
[[1,[[4],1],4,[],2],[4,[5]],[[1,[5,8],4,10],[[6,9,2,3],3,0],3],[6,[10,3,[1,9]]],[4,[8,[8,1,3,0,5]],0]]

[[4,[10,[1,6,10,5],[],[4,10]],[[8,10,7],10,2,[10],[5,3,3,6]],[[],[8],10,9,9],10],[[[]],4],[0,[]]]
[]

[[[[0,4,6,7,8]],[[2,4],[9,6,1,1]],[4,[2,3,3],[],6,1],10,[3,[1],7,[9]]],[6,[2,[0,9,10,6]],10,[]],[7,8,9],[[9,[10,2]]],[9,[8,9,1,10],[]]]
[[[5,[0,10,9,5,7],10,[8]],[1]]]

[[0,5,[],[[4,4],[6],4,8],9],[[10,[5],6],10,[[0,7,5,8],[9,9,4]]],[[[9,8,2],8],[9,6,8,[5]],6],[6,[],9,5,4],[[0,7,5],3,[[2,9,3,0],4,[3,0,8,7,0],[2,2]],9]]
[[3],[],[7,[9],9,[[6,3]],[[2,10],[10,6,5,4],[1,10,8,8],3]]]

[[],[[10],8],[[6,[],5],[[],[7,7]],0,[[5,9,6,8,4],1]],[],[[6],3,9,5]]
[[],[],[[[0,2,1,4,5],9,[3,6,9,8,4]]],[[],6,[[5,0],0,6,0],0,7],[[[8,8]],7,10,[]]]

[[[[3],10,[1,0],4]],[[[1,2,9],5],[1,8],5],[[],6]]
[[[7,[5,5,10],6,8,[5,6,6,1,9]],6,[[]],[6]],[1,[]],[],[[10,[1,0,8],[10,6,1,2],[6,10]],[4],7,[[8],7,3,[4,8],6]],[[],9,[0,6,[1,8,7]]]]

[[0,5,[[]],[10,1,[1],[],7]],[6,10,7,10]]
[[7],[4]]

[[1],[[3,[1,0,8,5,9],3,[6,1,5,9],2],6,[4,5],[[7,10,9,9],8,[]],[[9],[9,4,8,0],[1,4,5,6,8],0]]]
[[[0,1],3],[0,10],[[5,4,1,[8,4,9,4,7],[4,9]],7,1,8]]

[[[5],10,6],[[10],9,3],[]]
[[1,3,8,10],[[8,[10,0],[]],[[7,8]]],[[[],[0,4,6,6,6],[7,7,1,4],1],1,[],10,7],[3]]

[[[3,[]],[],[1,2,2,[0,7,10],[9,6,3,3]],[[10,0],[9,4],7,6,8],[]]]
[[8],[5,[[8],7],[[1],[8,7],[8,2,4],[2,3]],5]]

[[[3,10,[4,10,4,2],7,7],[[]],0,10,4],[[[5,4,3,0],2,[8],[5,7,2,1,2]],7,9],[1,[[0,9,9,6,1],[6,5,0,2,2],[2,10]],8,[10,4,[4,4]],8],[2,[10,[4,1,6,10],4,[1]]]]
[[8],[[4,1,0,[0,4,2,1]],[[4,0,10,3],10,1,[]],[[6],[8,1,4],0,[]]],[[4,[4,3],4],6,[1]]]

[[6],[[8]],[1,7,8,[0,1,7,[]]]]
[[[[1,2],8,5,[0,9]],[0,10,10,10],4,6],[6,[4,[1,7]]],[8,8,6,[[8,10,6,2,5],[2,0,7,8,8],10,3],9],[[[],6],6,3,[[8,7],4],5]]

[[[],[[5,8,1,3,5],2]],[4,[1],[[7,5],6,3]]]
[[[[7,2,1,1],[8,9,6,6,9],[2,8]],[[4,5,6],2],[0,[5,10,2,6,10],[3,1,6,7,2]]],[[[],2,[1,0,3],[8,8,6]],[6],6,[[],[4],[2,8],[1,8,9,4]],[[3,8,10],8,[0,8,9,7,6],2,8]]]

[[[0,3,10,[0]],4],[7,[2],[[2],[3]],[0,[],[4,2,6,8],[5,4,3,1,9],6],[[9,8,7,3,4],10]],[]]
[[[[],7,[5]],[4,1,8,[2,2,7,7,5],[4,10]],7,1],[[[],[2,9]],[10,2,[9,4,8,9],[],10],4,[8,[0],[7,8,3,2]],7]]

[[[1],10,[[1,0,4,7],[],[6,9,6,0,6],[4,0,1,9,1],[3]]]]
[[],[[5,[1,9,0]],3,3,1,[7,[9,6,0]]],[[[5],9]]]

[[[8,2,[10,4,2,8,2]],1,[5,3,[4,1,1]]],[[[4]],[]]]
[[[10],3,1,0,8],[7,[[4,9,9,10],[1],[1,7],2]],[[[0,7,4],1,7],7],[2,[[],[2,8,0],[7,2,5,9,4]]],[0,9,[[5,5,9,2],1,[9,2,3,5,1]]]]

[[[3],3,1,4,7],[[10,2,[9,9,3,9,0],3,9],[4],2],[[[],1],[],[]],[],[[0,1,[],10]]]
[[[[2,9],[6,10,0],8,[4,1,0,2]],8,9,3],[[[4],[3,2,5,4,3],4,5],[[0]]],[[7],[[8,1,8,0],4,2,[0,9],8],[[7],[7,0,10,9]]],[[0],8,[],[[1,2,9,7,0],9,[4]],9],[[[],4,3],[[],[4,10]],[8,[7,5],[]]]]

[[[1,4],9,[[0],[],[8,3,1],[10],7]],[],[2,7,0],[[2,4,3,[5,7,7]],[10,[4,10,9],7,8]]]
[[9,9,8,[3,8]],[6,10],[8,[[4,7,8],[8,10,5,2],[10,5],[5,0,4,7],[4,4,10]],10,[]],[[9,7],5,[10],10]]

[[7,9,7,10]]
[[6,[[0,10,5,6,6],[0,7,7],[5],[2,2,10,2,1],[2,3]]],[2,2,2,1,8],[4,5,3,7],[],[[1]]]

[[],[[],8,[[],3,1,[0],[8]]],[4,[[10,2],[4,1,8,4,9]]]]
[[[[1],[6],10,[7,2,1,0]],[[]],4,[[8,2,9],[],0],7]]

[[[]],[7,7,5,[[7,7,9,5,3]]],[5,[4,6]]]
[[[5,9]],[0,[9,5,2],[]]]

[[[6,5,[5,10,10,4],5,8]]]
[[8,7,0],[],[10,[6,[9,1,3,9,8],[4,9,1],6],[[0,1,9,0,10]],[4,[7,6,2,1],[8]]],[],[1,[[8]],2,[[3],5,10,[4,9,1,6,1],4],5]]

[[[7,4],1,[[],[4],[1],5,[4,10,1]],6,8]]
[[7,[[],5,[2,1,4,4],7],[8,[8,5,7,1,7]],[9],6],[4,[3],3,2]]

[[[9],[5,8,2,[]],[[2,5,7,5]]],[7,[[9,10,5,2],[0,5,8,10]],2,10,0],[[1,5,[9,5,3],[3]],0,9],[[10,[8,8],[10],[3,3,5,10,1],0],[[2,9,5,9,3],7,[8,9,10,8,5],1,0],5],[[0,1],3,[[7,1],[3,10]],[]]]
[[7,[10,[],10,9],3],[],[9,4,0],[]]

[[4,[[0],[1,9,4,2,0],0,6,10],6,[6,4,3,[3,10,2,10,4]],1]]
[[4],[],[[[]],[6,1],[],8,[2]]]

[[7,9,[2,[0]]],[[7,0,[1],[]]]]
[[4]]

[[4,6,1,[[9,5,0],4,[2,2],[5,5,5],[2]]],[9],[9,[7,5],[3,[],10,9]]]
[[],[6],[[10,[10,1,1,8,6],[]],0,[[5,10,2,0],4,0]],[8]]

[[9],[[],9,9,[[8,4,4,2,8],8,7],[1,[10,10,1,9],[8],1,7]],[[1,0]],[[],8,2,[[10,4]]]]
[[9,[],[[3,3,6,4,2],[3,3,6,9,2],8]],[8,[[8,10,9,9],3,[],[1,6,7,7],2],5,7,[]],[]]

[[[[0,1],0],4],[[6,6,1,5]],[0,[4]]]
[[9,[2,8,2,[9,2]],[[2],2,[9,0,5,10],[9,6]],[[]],[[]]],[],[10,0,[3,2,9,[8,8,9],[0,1,7]],[[2,9,4,6,2],9,[2],9,10],[[]]]]

[[10],[2,[]]]
[[1,[9],[[1,10,10,7]],[6,[],10]]]

[[],[[],[[2,0],2],8],[5,[[6,9,8,6],[9,6,3,1],[9,3,5,9,7],5,9]],[6]]
[[1,4,7]]

[[5,10,[[8,9],8],[]],[[6]],[5],[6,[9,9,[3,4,9,3,3],[]]],[[[4,4,9]]]]
[[5],[0,[[],[7],[1],3],1],[],[1,[[6,4,7],[4],0],9,4,1],[[10,3,8,[],10],1,[]]]

[[2,[5,[2,5,3,4,6],1,[4,10,7,7,0]]],[5,5,[[2,6,2]]],[],[[8,[6,1],10],8,[[5,10,4,2],[1,10],[5,7,5,6],[8],8]]]
[[3,[[2,6,3]]],[[],[[7,7,8,3],[9,1],9],[[],4,1,[]]],[8,5,[[2,0,3,10],9],5],[8],[7]]

[[6,[[],[10,6,2,3],0,[9,6],[]]],[[[9,9,0,2,3],7,[7,0,2],9],10],[0,[],[3,[10,3]]]]
[[0,[]],[[1,[0,6,6]],9,[3,[9,6,0,4,6]],[[10,0],4,[7],3]]]

[[2,[7,3,[2,6,4,4,7],1,9],[]],[[[3,2]]],[],[9,4,2,[7,[8]],[]]]
[[[[],8,4,[1,6,5]],5]]

[[[[9,8,0,10,10]]],[[6,1,[]],[0,[]],[[10,9],2,[7,5,9,9,10]]],[[[2,7,9,3],[10,6,6,4,8],[],[],3],3],[[[3,7],9,4]],[[]]]
[[4,6,7],[]]

[[],[[2,[],[],9],[[5,8,4,8,1]],[2,[1,4,6],0,3,6]]]
[[4,[[8],8,[6],[]],[[5]],[1,9,[0],10,[4,0]],1],[9,[[8],10,[6,0,5]]],[[1,[8,8,10,9,0]],[[1,6,1,3,6],6,[2,6]],[],[],9]]

[[[],[]],[[],3],[[10,[10,4,6],1],[7,10]],[0,6,3],[]]
[[],[9,1,[[7],[10,4],[]]],[3,[[8,3,2,9],4],[]],[7,[9,[7]]],[[[2,1,9,8,8]]]]

[[[7,4,5],2,[[],0,1,[],1]]]
[[[[2,5],[9],[0,10,8,3,6],[5,6,6,3,0],[4,0,6,9]],1,2],[8,3],[],[10,[[4,5],[2,3,1]],7]]

[[[[5,2,10,2,2]],[[3,2,5,2,8],[10],[],[5]],8]]
[[[[3,4,10,5],[5,8]],9,[[3,2,6],10,2,10]],[[3,4,[1,10,10,6]],[],[[8,7],[6,5]]],[8,10,[[2],[8,8,9,1]]],[0,3,[3,[2,9,6]],[[0,3]]]]

[[3,[[7,3,9,3],[6,3,7,4],8],7],[[],3],[10,[[2]]],[0,[[1,6,5,7],[],7],[[9,6,2,0,7],[6,10,0]]],[0,[10],2]]
[[5],[[[6,7,7],[],[1,10],9,[]]],[7,2,10,10,[9,[8,5,9,5],3,[8,10,4,3,4],9]]]

[[[[6,0,8]],3],[],[],[[[2,0,10,0]],[[7,4,1,3,9],10],[[5,1]],[[],[2,5,4],[3,10,5,10,9],[4]],[3,9,8]],[[[5,7,0,7,6],[],[9,0],[3,10,0,9],5],[[6],[],[9,7,4,10,0],8],1,1,0]]
[[4,[[0,10,5,0]]],[[],[1,[0,3,2,5,5],[5]]],[7,6,8],[]]

[[2,1,1],[7,4],[1,[8,7,[1,4],[2,7],[]],1,[1]],[[],[[5,3,0],[10,0,2,10],5,[5,5,7,4,1]],1],[]]
[[],[[9,7,[9,9],3],[[1,0],5,3],[],8,7],[2],[[4,4,[5,5,7,7],[7,3,6]]]]

[[[[3,1,0],10,0,[2],0],6],[[[],[10],3,4],0],[2,[[7],[7,0],[5,1,6,10],[3]],10],[[9,[],[2,4]],[1,[6,10,8],7]]]
[[[9],[],[[4,1],[1,1,4,2],3,[4,5,1,4],[4,3]],10,4],[],[[],[8,[10],9,[7,2,10,3]],[3,7,9,[0,0,2,1],6]]]

[[],[2],[5,[],[9],7,10],[[10,[0,5,5,0],[9,7,0,10,6],7,[1,7,2,4]],[1,[2],4,[2,2,6],[3,10]],[2,[],7,[5,4,8,6,8]],9],[3,8,[[7]]]]
[[[3]],[],[[[]],[[7,4,7,5],8],2,5,[]],[[2,[2,9,2,6,1],6,[5,8,4,6,1]],10,[7,[6,2,3,7]],[]],[]]

[[[9,[8,2,3,0,5],1,7],5,3],[0,[3,[3,6]]]]
[[5,[[6,2,3,4,2]],[9,[1,5,10,1,10],[8,2]]],[[[],0,[],0,[9]],[[7,2,4],1,1,2]],[[0,[6,9,10,7],8],[10,[0],4],1],[[[8,4],[10,2,1],9],6],[10]]

[[3,9,[[],10,10,[4,3],[]],7],[],[],[[[7,4,2]],[1,7],[],[[6,7,7,6,9]],1],[7,[1,[9,4],[2]]]]
[[],[]]

[[7,0,9],[10,[[9,8],1]],[4,[0,[9,4],[3,1,3,3,0],2],[8,3,[2,4,6,0,4],0],9],[[[1,4,4],10],3]]
[[[],8,[[1],6,9,[5,4,4,8],10]],[],[6,4,[2,[3,3,0,10],[7,0,1,3]],7],[3,[],5,7,10]]

[[2],[],[[0,5,[],[10,3,8,1,1],[]]],[4,[10,3,8,2],6,6,7],[7,0]]
[[[8,5,9],[4]],[1,[[7,8]],7]]

[[1,7,0,7,[[1,1,8,1],9,[]]],[[[7],7,2,[],[4,4,0]],1,[[],[6,5,9],[],10],[[6,1,9,9,10],4,4,5],[[6,0,9,1,3],[2],10]],[3,2,9,[],0],[[[2],5,[],8,[1,8,8,2]],[9,[],1]]]
[[0],[[10,[6,9,1],4],3,[[2,9],[1,2,9],[7,5,4]],[[],2,5,[0,4],3]],[[9,8,[9,8,4,5]],[7]],[3,[4],[[4,4],10,[]],1,[0,[],8]]]

[[8],[[[2],[1,4,10,7,1],[8,5,8,3,9],9],[9,3],[[2],[]]],[10]]
[[[[3,9],5]],[[[2],4,1,0],[8,6,9,[0]],0,[]],[7,[[8]],[[1,5,8,6],[10]]],[3,0],[[],0,[],5]]

[[[[],5]],[],[],[0,2,[]],[[[6,9],4,2]]]
[[3],[5,4,[[8,4,8],[2,10,3,5,10],5,[3,7,5,10]],0],[]]

[[[]],[2,[0,[3]],3,6,10],[],[5,[2,[]],[[5,4,2]]],[]]
[[[[6,5],8,[5,9,6,2],[9,3,0,9,7]],[]],[[[6,2,10,4],6,[4,0,5],10],[[9,2,2],[9,7]],[8,9],5],[0],[6,3,0,0]]

[[7,1,[[10,4,8,5,10],[0,6,1,2,5],[10],3]],[[],8,[8,5,[7],[]],[3,[4,5,10,0,2],[2,10],[],[8,10,6,5]]],[],[[9,2,[2]],[2,[3,8,8,4,7],[8,7,6]],4],[[7,3,6],[[10],[0],[],[10,9]]]]
[[],[8,6,2],[[],3],[],[3,9,5,0,[1]]]

[[5],[5,[6,[3,0,8,2,6],4,2]],[10,[8,[9,10],[8],6]]]
[[[[5,3],5,[],7],9,[[],8,2],7,[[9,2,4]]],[8,[5],[8,4,[10,7,5,1]]],[2,1],[0,0,[4],[[0],0],[7,[9,7]]],[5,[8,[10,1,5,0,1],0,2],[8,[1,8,5,10],3],[[7],[8,4,4]],10]]

[[1,4,7,[[10,2,8,3],7]]]
[[],[],[3,5,1,[8,7,9,5]],[[2],2]]

[[[[6,1],[7],6,[4,1,5,8]],[],0],[[7,[1,1,10,8],[8,7,10,2],[9,4,2,7,2]],[[],[5,1,2,6]],3,9],[8,[[8,7,8,4,6],[2,0,10,8,9],1,[0,3,9],9],0,[[3,7],1,[7]]],[2],[[[9,2]],[],0,[[9,10,1,9]]]]
[[4,0,4,1,2]]

[7,1,9,0]
[7,1,9,0,9]

[[2,9],[9,3,6],[],[2,5,[1,7,[3,2,5,2],10],[[2,10],[4,7,3],0]],[7,[3],[4,0,[10,0],9],5]]
[[6,[[],[9,0,6,3],8],[6,[7,2,7,9,10],[7,5,3,5],7]],[[3,[5,2,3,4]],3],[[4,[2,4,1],4,[1,8],[3,3,3,7]],9,[]],[],[[8,7,4,1],3,[[8],[9],5],5,9]]

[[[],2,8],[[9,[8,10,6,9,8],0,[10,7,9,5],[]],2,3,4,6]]
[[[[4,0,3,3]]],[10],[6,5,[10,[0,0,10,7],[9],0],6]]

[[],[[8,[10,1]],2,7,[[7],7,3],[4,3,9]],[[4,[0]],[6,[4,7],3,[10,2],[6,5]],10,[[5,1,10,1,1],[0,2,9,3,9],7,10,3]],[3,[[],[7,8,3],[7]],[]]]
[[],[[1]],[]]

[[[2],7,[1,8,0,[10,7,8,1]]],[[],[[0,2,8,0],[1,1,9]],6],[[9,0,[0,4,5,9,1],5],7,[0],[7,4,9],7],[]]
[[[2,[8,3,7,2],6,[1,2,10,3]],[2,8,8],[[5],[10,8,2,4,10],[0,1,4],3,0],0,8],[],[6,[],[[1,0,2,3,3],10],7,1]]

[[8,[6,[10,8,8],10,5],2]]
[[5,5,2],[[5,9,4,3]],[[[1,0,4],[2,7,9,0,4]],[],5,[[10,5,5,7,7],[10,7,8,1],[8,8,4],[0,3,0],[1,8,10,8]]]]

[[[[6],[0,10,8,10],[4],[4]],10,10,7,8],[[],[],1,[],[[3,2,5],[9,10,6]]],[],[],[]]
[[[[3],1,3],[[6,7],10,[3],[4]],7]]

[[[[],10,[]],8],[[5,[4,3,9],[7,3,1,10,1],10,5],[[1,7],[8,1],2,[9,4,1,0,3],0]],[9,5,5,9,10],[6,1,0],[10,[10]]]
[[[[3],7,4,0],6,7,[9,2]],[],[[0,1]],[[4,1,3,10,5]]]

[[[6,3,8],[0,[8,10],1],10,3]]
[[]]

[[[[8,9,9,0],4,[3,7,0,1],5,10],4,[2,[4,4,3,4,4],9,3,5],4,[[],7,[9,8,2,0],10,4]],[],[6,6,0,[]]]
[[[6,8,9],1,[6,1,5],[6,5,10]],[2,4,[[10,1],8,[7,6,2],[],0]],[[0,9,[],[1,3]]],[[]],[0]]

[[[],7,[],[[],10,3,2],[[0,1,9,1,8]]],[6,[[2,1,1],1,3]]]
[[[3,9,2,8],[],0,[[10],[10,6,9,3],7,2,[10,2,10,10,0]],8]]

[[5]]
[[10,[]]]

[[[[4,4,7,2],0,2],4,[[4,6,7],5,8,5,[7,7,8,2,0]]]]
[[],[1,[[1,3,4,2],[6,10]]],[3,[]]]

[[4,3,1],[8,[],[[8,2,9,9,4],[6,5,9,6]],2],[],[[[3,7],2,[10],9,5],4]]
[[1]]

[[10,[[4,6,5,6],[8,7,8,7,2],1],0,[[4,6,7,9],[]],0],[[2,1,[6,1,3,6,10]],[[3,8],0,[1,10],0,[2]],8],[5,[10,[],[5,10],6,4]],[[],9,[[2,10,2,3,5],[0,10],[1,10],1,[5,4]],8,0]]
[[[1,[5,6,6],9,[10,4,9,6]]],[],[[8,3,[6,10],1],[]],[[10,[6,4,2,7],5],7,4,[6,4,4,[6]],4],[0,[4,[8,7,6],[],[]],[[4,3,8,10],8]]]

[[3,2,[[],6,1],0,[[9,3,2]]],[[0]],[[3,2,9,[9]]],[10,[2,6,2,[5,7,8]],[5,4,[5,6,7,6]],6],[]]
[[3,4,[[1,8,6,1,8],9,3]],[[4,0,7,[6,1,2,6]]]]

[[10,5,1,2],[[],9,[9,[8,4],7],[5,1,10,[9]],[[3,0],2,[6,9],6]],[[[],4,[5,6,7]],[[6,10],2]],[],[[2,10,[],4],[[],2,5],1,[3,[2,1,6]],3]]
[[[[3],[6,3,1,8],[8,10,4,10,4]]],[9,[8,[10,3]],10,[[],10]],[[[3,8]],4,2,5,10]]

[[[4,8],2,[3],[0],[[3,4],[9,4,5,9,0],[10,6,3,3],[8,10,6,4]]]]
[[[[],2,3]],[[0,[8,7,10,9]],[10,[8],[10,5,1,1],[],[]],[[1,10,3,10],7,[],[1,8,4],3],[[3,1,8,2]]]]

[[[1,2,1,6,[8,8]],[7],[[],9,[2,0],[7,7]],[1,[8]],[]],[[1,[9],2,[0,4],[4,6,10,4,6]],[],5],[[10,7,[7,5,6,4,9],[4,6],[]]],[8,3]]
[[4,[],5,[[],[5,1],[1,8,6],[],4]],[6,[4]],[]]

[[[]],[],[10,1,0,4]]
[[[9,2,[5,9,7,3,1]],1],[0]]

[[9,[],[0,2,[3,10,5]]],[10,[[0,9,7,7,5],2,[10,4,8,4],5],[6,6]],[[10,[],9,[0,5,6,7,0],[9,6,3,2]],6,[],7],[[[3],2,1],9,[8]],[9,6,[8,[5],10]]]
[[[0],[6,[7,9,5,0],0],[]],[[[9,0,1,3,8]],8]]

[[[[1,2,9],[6,7],9,3,[]]],[[5,0],[[1,4,5,1,0],1,[0,2,8,8],[7,10,2,2]],0,10],[[7,9,[0,9,5,2],[7,3,0,6,4],10],7,[],0,1],[0]]
[[],[[3,8,[],5,8]]]

[[[4,3,8],[9]]]
[[2,[[7,0],[1,3]],10],[]]

[[[[4,8,9,7,1],3],[10],8],[3,5,[]],[5,2,3],[1,5,8],[[[],9,[6,0,3,6],1,[8]],[1],4]]
[[[[2]],[[3],7],[[2,7],7,[1,6,8,0],7,1],10,3]]

[[2,9,[3,[7,1,5,10],[9]],[8],5],[[[8,10,8],[],[0,9,7,0],3,[9,6,5]]],[8,[],10,6],[[]],[[5,[10,5,6,10,2]],5,0,5,[[4,10,1,8,3],2,6,[8,5,8],[9,10,2]]]]
[[],[4,1,[]],[[6,8,[3,5,4],[1,1,4],[7]],[[6,3],1,5]],[2,3,[5,[8,2,0],[],[0,0],[1,6]],[4,[0,2,7],1,[1]],3]]

[[],[8,2,[2]],[],[[7,10]]]
[[[[6,10,10,5],4,10,9,[8,7,2,9,1]]],[]]

[[7,[[1,7,4,9],1,[7,4,10,3]],[[],[2],[2,4,7],2,6],[]]]
[[[7,4,[9],[2,10,8]],[7,[3,2,6,8,9],8,[2]]],[[2,[10],7,[7,0,2,9]],[[],[4,2,4,4,10],[0,2,3],[]],0,[[9,6,0,8,9],5],[9,[],[2,2,3,10],[8,0,3,6]]],[2]]

[[],[[],10],[3,1,[[]],[[9],[],2,[3]],[0,[4,10,9,0,6],[7,7,10,2,2]]]]
[[7,6,4,8,[]],[[],[[10,0,10,1,9],7,7],[[0,1,5,6]]],[2],[6,[5,5],1,9,[]]]

[[[]],[[10,10],5,[],10],[7],[1,[[],4,[],2],9],[3,0]]
[[[7],2],[5,[5,[10,7,6,1,10],[10,0,3]],[[5,4],4,[10,8,8,4,9],3],[[4,5,5],[6,6,6]]],[4,[],5]]
";
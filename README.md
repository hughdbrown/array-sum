# Purpose
Compare two algorithms for finding the range of a vector of integers that adds to a target value.

Naive algorithm is `O(n * n)`.

Smart algorithm is `O(n)`.

# Benchmark
A representative run in release:
```
---------------- 0
target is 150 in range (438, 471)
Naive: (468, 42) 12.420293ms
Smart: (6, 37) 5.141µs
---------------- 1
target is 214 in range (264, 313)
Naive: (476, 57) 12.333955ms
Smart: (5, 50) 4.663µs
---------------- 2
target is 2222 in range (489, 982)
Naive: (79, 509) 5.861248ms
Smart: (0, 505) 3.203µs
---------------- 3
target is 3323 in range (151, 899)
Naive: (106, 758) 1.8291ms
Smart: (9, 748) 4.518µs
---------------- 4
target is 940 in range (469, 684)
Naive: (394, 222) 11.003541ms
Smart: (3, 215) 5.31µs
---------------- 5
target is 852 in range (339, 539)
Naive: (80, 202) 10.567667ms
Smart: (1, 190) 2.749µs
---------------- 6
target is 801 in range (256, 431)
Naive: (359, 193) 10.433801ms
Smart: (3, 181) 2.46µs
---------------- 7
target is 2270 in range (345, 859)
Naive: (80, 521) 5.292728ms
Smart: (4, 520) 2.701µs
---------------- 8
target is 1807 in range (464, 874)
Naive: (125, 420) 6.948982ms
Smart: (3, 407) 2.531µs
---------------- 9
target is 3071 in range (173, 867)
Naive: (109, 702) 2.386461ms
Smart: (2, 692) 2.855µs
---------------- 10
target is 1184 in range (271, 544)
Naive: (262, 277) 9.527111ms
Smart: (1, 273) 4.162µs
---------------- 11
target is 2098 in range (291, 761)
Naive: (63, 483) 6.157141ms
Smart: (3, 473) 2.811µs
---------------- 12
target is 1087 in range (464, 712)
Naive: (357, 256) 9.627851ms
Smart: (9, 247) 28.666µs
---------------- 13
target is 1210 in range (114, 385)
Naive: (263, 283) 9.184016ms
Smart: (1, 278) 4.937µs
---------------- 14
target is 2263 in range (282, 789)
Naive: (4, 519) 5.331062ms
Smart: (3, 517) 3.431µs
---------------- 15
target is 1273 in range (218, 508)
Naive: (354, 296) 9.012919ms
Smart: (9, 287) 2.868µs
---------------- 16
target is 2215 in range (160, 665)
Naive: (103, 511) 5.810976ms
Smart: (0, 503) 2.992µs
---------------- 17
target is 727 in range (368, 541)
Naive: (357, 177) 12.999056ms
Smart: (9, 161) 31.31µs
---------------- 18
target is 1315 in range (261, 561)
Naive: (237, 307) 8.720413ms
Smart: (11, 297) 4.905µs
---------------- 19
target is 2542 in range (212, 784)
Naive: (4, 581) 4.218442ms
Smart: (0, 579) 3.151µs
---------------- 20
target is 2157 in range (287, 769)
Naive: (50, 496) 5.63547ms
Smart: (16, 492) 2.734µs
---------------- 21
target is 973 in range (495, 711)
Naive: (387, 230) 9.811472ms
Smart: (8, 220) 2.305µs
---------------- 22
target is 1170 in range (481, 746)
Naive: (385, 273) 10.772427ms
Smart: (3, 268) 2.56µs
---------------- 23
target is 2793 in range (347, 972)
Naive: (170, 638) 3.442429ms
Smart: (4, 634) 3.243µs
---------------- 24
target is 1311 in range (405, 700)
Naive: (354, 306) 8.767429ms
Smart: (5, 300) 2.409µs
---------------- 25
target is 1599 in range (459, 823)
Naive: (164, 372) 7.725113ms
Smart: (2, 358) 2.608µs
---------------- 26
target is 2005 in range (97, 555)
Naive: (79, 466) 6.105002ms
Smart: (8, 450) 2.606µs
---------------- 27
target is 1134 in range (115, 372)
Naive: (349, 263) 9.429748ms
Smart: (5, 257) 2.405µs
---------------- 28
target is 1950 in range (217, 660)
Naive: (88, 451) 6.502815ms
Smart: (0, 438) 2.505µs
---------------- 29
target is 332 in range (408, 484)
Naive: (457, 87) 13.766109ms
Smart: (8, 71) 2.49µs
---------------- 30
target is 3262 in range (10, 746)
Naive: (104, 744) 1.959779ms
Smart: (1, 738) 3.653µs
---------------- 31
target is 123 in range (453, 481)
Naive: (474, 36) 13.169893ms
Smart: (0, 32) 35.829µs
---------------- 32
target is 2172 in range (172, 666)
Naive: (104, 500) 5.554014ms
Smart: (0, 491) 5.971µs
---------------- 33
target is 150 in range (470, 510)
Naive: (468, 42) 11.914962ms
Smart: (6, 37) 3.255µs
---------------- 34
target is 1505 in range (322, 665)
Naive: (196, 351) 10.304095ms
Smart: (3, 340) 3.949µs
---------------- 35
target is 805 in range (251, 425)
Naive: (387, 190) 11.082502ms
Smart: (1, 181) 5.685µs
---------------- 36
target is 1915 in range (139, 575)
Naive: (102, 445) 6.717632ms
Smart: (1, 431) 2.901µs
---------------- 37
target is 2200 in range (122, 623)
Naive: (107, 509) 5.444474ms
Smart: (4, 504) 3.229µs
---------------- 38
target is 1264 in range (262, 554)
Naive: (358, 294) 9.518443ms
Smart: (11, 284) 7.877µs
---------------- 39
target is 3431 in range (51, 826)
Naive: (78, 781) 1.406343ms
Smart: (1, 775) 4.195µs
---------------- 40
target is 386 in range (12, 96)
Naive: (437, 99) 11.54165ms
Smart: (3, 87) 4.85µs
---------------- 41
target is 3218 in range (221, 944)
Naive: (106, 734) 2.047915ms
Smart: (1, 728) 3.183µs
---------------- 42
target is 1244 in range (346, 628)
Naive: (258, 289) 10.582981ms
Smart: (6, 284) 6.626µs
---------------- 43
target is 3556 in range (170, 969)
Naive: (107, 810) 1.047634ms
Smart: (0, 808) 3.6µs
---------------- 44
target is 516 in range (394, 518)
Naive: (413, 127) 12.181994ms
Smart: (3, 116) 8.052µs
---------------- 45
target is 296 in range (413, 480)
Naive: (464, 80) 11.405443ms
Smart: (2, 67) 2.873µs
---------------- 46
target is 2259 in range (197, 707)
Naive: (99, 518) 5.267818ms
Smart: (0, 515) 3.018µs
---------------- 47
target is 1078 in range (164, 406)
Naive: (358, 253) 9.633374ms
Smart: (9, 244) 2.442µs
---------------- 48
target is 1116 in range (202, 450)
Naive: (262, 259) 9.410102ms
Smart: (0, 254) 2.829µs
---------------- 49
target is 2313 in range (86, 616)
Naive: (84, 532) 5.002331ms
Smart: (0, 528) 2.691µs
---------------- 50
target is 244 in range (456, 518)
Naive: (481, 65) 11.649442ms
Smart: (6, 56) 2.917µs
---------------- 51
target is 1199 in range (293, 567)
Naive: (262, 282) 9.109442ms
Smart: (1, 276) 2.44µs
---------------- 52
target is 2008 in range (352, 811)
Naive: (78, 467) 6.124466ms
Smart: (11, 450) 2.633µs
---------------- 53
target is 990 in range (123, 348)
Naive: (354, 232) 10.173403ms
Smart: (3, 225) 7.076µs
---------------- 54
target is 912 in range (425, 634)
Naive: (321, 215) 11.676674ms
Smart: (3, 210) 31.169µs
---------------- 55
target is 1592 in range (96, 454)
Naive: (168, 371) 12.489318ms
Smart: (3, 356) 8.532µs
---------------- 56
target is 3104 in range (171, 872)
Naive: (102, 708) 2.815959ms
Smart: (10, 700) 4.375µs
---------------- 57
target is 3463 in range (44, 824)
Naive: (132, 789) 1.593897ms
Smart: (3, 785) 6.302µs
---------------- 58
target is 1094 in range (411, 662)
Naive: (357, 259) 11.830356ms
Smart: (20, 246) 40.764µs
---------------- 59
target is 2932 in range (307, 962)
Naive: (134, 670) 3.488936ms
Smart: (0, 665) 6.726µs
---------------- 60
target is 1885 in range (43, 464)
Naive: (107, 440) 7.998705ms
Smart: (0, 424) 3.545µs
---------------- 61
target is 654 in range (206, 350)
Naive: (377, 159) 12.675368ms
Smart: (1, 146) 2.484µs
---------------- 62
target is 3039 in range (312, 987)
Naive: (109, 696) 2.566553ms
Smart: (0, 687) 2.76µs
---------------- 63
target is 1225 in range (107, 384)
Naive: (258, 286) 9.842877ms
Smart: (2, 282) 2.645µs
---------------- 64
target is 3219 in range (68, 795)
Naive: (76, 735) 2.022983ms
Smart: (0, 728) 2.694µs
---------------- 65
target is 241 in range (479, 542)
Naive: (476, 65) 11.77001ms
Smart: (9, 53) 3.265µs
---------------- 66
target is 3613 in range (182, 994)
Naive: (100, 822) 943.708µs
Smart: (1, 819) 2.845µs
---------------- 67
target is 2530 in range (264, 835)
Naive: (75, 579) 4.366104ms
Smart: (1, 576) 2.761µs
---------------- 68
target is 2568 in range (294, 871)
Naive: (73, 588) 4.494174ms
Smart: (0, 585) 22.796µs
---------------- 69
target is 1488 in range (289, 625)
Naive: (191, 348) 9.008405ms
Smart: (0, 337) 30.851µs
---------------- 70
target is 618 in range (421, 565)
Naive: (386, 153) 12.467934ms
Smart: (0, 139) 10.375µs
---------------- 71
target is 2882 in range (38, 685)
Naive: (2, 658) 3.035908ms
Smart: (2, 657) 3.239µs
---------------- 72
target is 2632 in range (228, 822)
Naive: (190, 601) 4.676024ms
Smart: (9, 597) 4.597µs
---------------- 73
target is 476 in range (418, 533)
Naive: (424, 118) 12.497267ms
Smart: (5, 105) 4.813µs
---------------- 74
target is 1325 in range (263, 567)
Naive: (233, 309) 9.194018ms
Smart: (1, 302) 2.96µs
---------------- 75
target is 2896 in range (192, 849)
Naive: (144, 661) 3.062007ms
Smart: (0, 659) 2.769µs
---------------- 76
target is 2411 in range (241, 785)
Naive: (106, 555) 4.785494ms
Smart: (0, 552) 2.842µs
---------------- 77
target is 1447 in range (21, 345)
Naive: (187, 335) 8.380163ms
Smart: (3, 328) 2.48µs
---------------- 78
target is 2984 in range (272, 940)
Naive: (123, 683) 2.700018ms
Smart: (0, 677) 2.988µs
---------------- 79
target is 1093 in range (268, 517)
Naive: (358, 258) 9.985409ms
Smart: (2, 250) 2.33µs
---------------- 80
target is 131 in range (48, 72)
Naive: (472, 37) 12.21085ms
Smart: (2, 35) 4.723µs
---------------- 81
target is 2630 in range (254, 847)
Naive: (4, 600) 4.0794ms
Smart: (4, 600) 2.762µs
---------------- 82
target is 304 in range (345, 413)
Naive: (464, 81) 11.81145ms
Smart: (1, 69) 2.675µs
---------------- 83
target is 4076 in range (25, 939)
Naive: (3, 922) 195.703µs
Smart: (3, 922) 2.926µs
---------------- 84
target is 2618 in range (410, 998)
Naive: (78, 598) 4.166708ms
Smart: (0, 594) 3.077µs
---------------- 85
target is 668 in range (370, 526)
Naive: (384, 163) 14.783209ms
Smart: (1, 151) 7.019µs
---------------- 86
target is 126 in range (159, 186)
Naive: (472, 36) 13.917314ms
Smart: (4, 35) 54.304µs
---------------- 87
target is 2343 in range (75, 612)
Naive: (6, 540) 6.86481ms
Smart: (0, 537) 8.846µs
---------------- 88
target is 871 in range (270, 461)
Naive: (334, 206) 11.419849ms
Smart: (1, 197) 3.754µs
---------------- 89
target is 2823 in range (123, 762)
Naive: (160, 645) 3.472164ms
Smart: (8, 642) 3.078µs
---------------- 90
target is 1278 in range (438, 729)
Naive: (358, 297) 9.008355ms
Smart: (2, 291) 2.618µs
---------------- 91
target is 3173 in range (63, 778)
Naive: (79, 724) 2.136316ms
Smart: (4, 718) 2.958µs
---------------- 92
target is 2816 in range (20, 658)
Naive: (103, 642) 3.692981ms
Smart: (2, 638) 31.747µs
---------------- 93
target is 396 in range (283, 364)
Naive: (439, 100) 12.56666ms
Smart: (2, 89) 5.954µs
---------------- 94
target is 2435 in range (361, 915)
Naive: (102, 559) 5.423979ms
Smart: (2, 556) 3.525µs
---------------- 95
target is 2538 in range (26, 599)
Naive: (76, 582) 4.701314ms
Smart: (12, 575) 3.14µs
---------------- 96
target is 226 in range (420, 470)
Naive: (478, 60) 11.876755ms
Smart: (0, 51) 2.476µs
---------------- 97
target is 153 in range (301, 334)
Naive: (469, 42) 11.927936ms
Smart: (6, 38) 4.029µs
---------------- 98
target is 1304 in range (242, 543)
Naive: (239, 305) 8.922852ms
Smart: (4, 298) 2.634µs
---------------- 99
target is 1637 in range (205, 578)
Naive: (164, 381) 7.775867ms
Smart: (0, 369) 5.362µs
```

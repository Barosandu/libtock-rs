use core::{convert::TryFrom, fmt, mem::transmute};

// TODO: Add a ufmt debug implementation for process binaries to use.
/// An error code that libtock-rs APIs may return, as specified in
/// [TRD 104][error-codes]. Note that while `BADRVAL` can never be produced by
/// the kernel, it can be produced by userspace APIs.
/// 
/// [error-codes]: https://github.com/tock/tock/blob/master/doc/reference/trd104-syscalls.md#33-error-codes
#[derive(Clone, Copy, PartialEq, Eq)]
// Explicit repr to use `transmute`. A word-sized error code results in
// significant code size savings in typical use when compared to `#[repr(u16)]`.
#[repr(u32)]
#[rustfmt::skip]
pub enum ErrorCode {
    Fail = 1,
    Busy = 2,
    Already = 3,
    Off = 4,
    Reserve = 5,
    Invalid = 6,
    Size = 7,
    Cancel = 8,
    NoMem = 9,
    NoSupport = 10,
    NoDevice = 11,
    Uninstalled = 12,
    NoAck = 13,
    BadRVal = 1024,

    // Error codes reserved for future use. We have to include these for future
    // compatibility -- this allows process binaries compiled with this version
    // of libtock-rs to run on future kernel versions that may return a larger
    // variety of error codes.
                                                    N00014 =    14, N00015 =    15,
    N00016 =    16, N00017 =    17, N00018 =    18, N00019 =    19, N00020 =    20,
    N00021 =    21, N00022 =    22, N00023 =    23, N00024 =    24, N00025 =    25,
    N00026 =    26, N00027 =    27, N00028 =    28, N00029 =    29, N00030 =    30,
    N00031 =    31, N00032 =    32, N00033 =    33, N00034 =    34, N00035 =    35,
    N00036 =    36, N00037 =    37, N00038 =    38, N00039 =    39, N00040 =    40,
    N00041 =    41, N00042 =    42, N00043 =    43, N00044 =    44, N00045 =    45,
    N00046 =    46, N00047 =    47, N00048 =    48, N00049 =    49, N00050 =    50,
    N00051 =    51, N00052 =    52, N00053 =    53, N00054 =    54, N00055 =    55,
    N00056 =    56, N00057 =    57, N00058 =    58, N00059 =    59, N00060 =    60,
    N00061 =    61, N00062 =    62, N00063 =    63, N00064 =    64, N00065 =    65,
    N00066 =    66, N00067 =    67, N00068 =    68, N00069 =    69, N00070 =    70,
    N00071 =    71, N00072 =    72, N00073 =    73, N00074 =    74, N00075 =    75,
    N00076 =    76, N00077 =    77, N00078 =    78, N00079 =    79, N00080 =    80,
    N00081 =    81, N00082 =    82, N00083 =    83, N00084 =    84, N00085 =    85,
    N00086 =    86, N00087 =    87, N00088 =    88, N00089 =    89, N00090 =    90,
    N00091 =    91, N00092 =    92, N00093 =    93, N00094 =    94, N00095 =    95,
    N00096 =    96, N00097 =    97, N00098 =    98, N00099 =    99, N00100 =   100,
    N00101 =   101, N00102 =   102, N00103 =   103, N00104 =   104, N00105 =   105,
    N00106 =   106, N00107 =   107, N00108 =   108, N00109 =   109, N00110 =   110,
    N00111 =   111, N00112 =   112, N00113 =   113, N00114 =   114, N00115 =   115,
    N00116 =   116, N00117 =   117, N00118 =   118, N00119 =   119, N00120 =   120,
    N00121 =   121, N00122 =   122, N00123 =   123, N00124 =   124, N00125 =   125,
    N00126 =   126, N00127 =   127, N00128 =   128, N00129 =   129, N00130 =   130,
    N00131 =   131, N00132 =   132, N00133 =   133, N00134 =   134, N00135 =   135,
    N00136 =   136, N00137 =   137, N00138 =   138, N00139 =   139, N00140 =   140,
    N00141 =   141, N00142 =   142, N00143 =   143, N00144 =   144, N00145 =   145,
    N00146 =   146, N00147 =   147, N00148 =   148, N00149 =   149, N00150 =   150,
    N00151 =   151, N00152 =   152, N00153 =   153, N00154 =   154, N00155 =   155,
    N00156 =   156, N00157 =   157, N00158 =   158, N00159 =   159, N00160 =   160,
    N00161 =   161, N00162 =   162, N00163 =   163, N00164 =   164, N00165 =   165,
    N00166 =   166, N00167 =   167, N00168 =   168, N00169 =   169, N00170 =   170,
    N00171 =   171, N00172 =   172, N00173 =   173, N00174 =   174, N00175 =   175,
    N00176 =   176, N00177 =   177, N00178 =   178, N00179 =   179, N00180 =   180,
    N00181 =   181, N00182 =   182, N00183 =   183, N00184 =   184, N00185 =   185,
    N00186 =   186, N00187 =   187, N00188 =   188, N00189 =   189, N00190 =   190,
    N00191 =   191, N00192 =   192, N00193 =   193, N00194 =   194, N00195 =   195,
    N00196 =   196, N00197 =   197, N00198 =   198, N00199 =   199, N00200 =   200,
    N00201 =   201, N00202 =   202, N00203 =   203, N00204 =   204, N00205 =   205,
    N00206 =   206, N00207 =   207, N00208 =   208, N00209 =   209, N00210 =   210,
    N00211 =   211, N00212 =   212, N00213 =   213, N00214 =   214, N00215 =   215,
    N00216 =   216, N00217 =   217, N00218 =   218, N00219 =   219, N00220 =   220,
    N00221 =   221, N00222 =   222, N00223 =   223, N00224 =   224, N00225 =   225,
    N00226 =   226, N00227 =   227, N00228 =   228, N00229 =   229, N00230 =   230,
    N00231 =   231, N00232 =   232, N00233 =   233, N00234 =   234, N00235 =   235,
    N00236 =   236, N00237 =   237, N00238 =   238, N00239 =   239, N00240 =   240,
    N00241 =   241, N00242 =   242, N00243 =   243, N00244 =   244, N00245 =   245,
    N00246 =   246, N00247 =   247, N00248 =   248, N00249 =   249, N00250 =   250,
    N00251 =   251, N00252 =   252, N00253 =   253, N00254 =   254, N00255 =   255,
    N00256 =   256, N00257 =   257, N00258 =   258, N00259 =   259, N00260 =   260,
    N00261 =   261, N00262 =   262, N00263 =   263, N00264 =   264, N00265 =   265,
    N00266 =   266, N00267 =   267, N00268 =   268, N00269 =   269, N00270 =   270,
    N00271 =   271, N00272 =   272, N00273 =   273, N00274 =   274, N00275 =   275,
    N00276 =   276, N00277 =   277, N00278 =   278, N00279 =   279, N00280 =   280,
    N00281 =   281, N00282 =   282, N00283 =   283, N00284 =   284, N00285 =   285,
    N00286 =   286, N00287 =   287, N00288 =   288, N00289 =   289, N00290 =   290,
    N00291 =   291, N00292 =   292, N00293 =   293, N00294 =   294, N00295 =   295,
    N00296 =   296, N00297 =   297, N00298 =   298, N00299 =   299, N00300 =   300,
    N00301 =   301, N00302 =   302, N00303 =   303, N00304 =   304, N00305 =   305,
    N00306 =   306, N00307 =   307, N00308 =   308, N00309 =   309, N00310 =   310,
    N00311 =   311, N00312 =   312, N00313 =   313, N00314 =   314, N00315 =   315,
    N00316 =   316, N00317 =   317, N00318 =   318, N00319 =   319, N00320 =   320,
    N00321 =   321, N00322 =   322, N00323 =   323, N00324 =   324, N00325 =   325,
    N00326 =   326, N00327 =   327, N00328 =   328, N00329 =   329, N00330 =   330,
    N00331 =   331, N00332 =   332, N00333 =   333, N00334 =   334, N00335 =   335,
    N00336 =   336, N00337 =   337, N00338 =   338, N00339 =   339, N00340 =   340,
    N00341 =   341, N00342 =   342, N00343 =   343, N00344 =   344, N00345 =   345,
    N00346 =   346, N00347 =   347, N00348 =   348, N00349 =   349, N00350 =   350,
    N00351 =   351, N00352 =   352, N00353 =   353, N00354 =   354, N00355 =   355,
    N00356 =   356, N00357 =   357, N00358 =   358, N00359 =   359, N00360 =   360,
    N00361 =   361, N00362 =   362, N00363 =   363, N00364 =   364, N00365 =   365,
    N00366 =   366, N00367 =   367, N00368 =   368, N00369 =   369, N00370 =   370,
    N00371 =   371, N00372 =   372, N00373 =   373, N00374 =   374, N00375 =   375,
    N00376 =   376, N00377 =   377, N00378 =   378, N00379 =   379, N00380 =   380,
    N00381 =   381, N00382 =   382, N00383 =   383, N00384 =   384, N00385 =   385,
    N00386 =   386, N00387 =   387, N00388 =   388, N00389 =   389, N00390 =   390,
    N00391 =   391, N00392 =   392, N00393 =   393, N00394 =   394, N00395 =   395,
    N00396 =   396, N00397 =   397, N00398 =   398, N00399 =   399, N00400 =   400,
    N00401 =   401, N00402 =   402, N00403 =   403, N00404 =   404, N00405 =   405,
    N00406 =   406, N00407 =   407, N00408 =   408, N00409 =   409, N00410 =   410,
    N00411 =   411, N00412 =   412, N00413 =   413, N00414 =   414, N00415 =   415,
    N00416 =   416, N00417 =   417, N00418 =   418, N00419 =   419, N00420 =   420,
    N00421 =   421, N00422 =   422, N00423 =   423, N00424 =   424, N00425 =   425,
    N00426 =   426, N00427 =   427, N00428 =   428, N00429 =   429, N00430 =   430,
    N00431 =   431, N00432 =   432, N00433 =   433, N00434 =   434, N00435 =   435,
    N00436 =   436, N00437 =   437, N00438 =   438, N00439 =   439, N00440 =   440,
    N00441 =   441, N00442 =   442, N00443 =   443, N00444 =   444, N00445 =   445,
    N00446 =   446, N00447 =   447, N00448 =   448, N00449 =   449, N00450 =   450,
    N00451 =   451, N00452 =   452, N00453 =   453, N00454 =   454, N00455 =   455,
    N00456 =   456, N00457 =   457, N00458 =   458, N00459 =   459, N00460 =   460,
    N00461 =   461, N00462 =   462, N00463 =   463, N00464 =   464, N00465 =   465,
    N00466 =   466, N00467 =   467, N00468 =   468, N00469 =   469, N00470 =   470,
    N00471 =   471, N00472 =   472, N00473 =   473, N00474 =   474, N00475 =   475,
    N00476 =   476, N00477 =   477, N00478 =   478, N00479 =   479, N00480 =   480,
    N00481 =   481, N00482 =   482, N00483 =   483, N00484 =   484, N00485 =   485,
    N00486 =   486, N00487 =   487, N00488 =   488, N00489 =   489, N00490 =   490,
    N00491 =   491, N00492 =   492, N00493 =   493, N00494 =   494, N00495 =   495,
    N00496 =   496, N00497 =   497, N00498 =   498, N00499 =   499, N00500 =   500,
    N00501 =   501, N00502 =   502, N00503 =   503, N00504 =   504, N00505 =   505,
    N00506 =   506, N00507 =   507, N00508 =   508, N00509 =   509, N00510 =   510,
    N00511 =   511, N00512 =   512, N00513 =   513, N00514 =   514, N00515 =   515,
    N00516 =   516, N00517 =   517, N00518 =   518, N00519 =   519, N00520 =   520,
    N00521 =   521, N00522 =   522, N00523 =   523, N00524 =   524, N00525 =   525,
    N00526 =   526, N00527 =   527, N00528 =   528, N00529 =   529, N00530 =   530,
    N00531 =   531, N00532 =   532, N00533 =   533, N00534 =   534, N00535 =   535,
    N00536 =   536, N00537 =   537, N00538 =   538, N00539 =   539, N00540 =   540,
    N00541 =   541, N00542 =   542, N00543 =   543, N00544 =   544, N00545 =   545,
    N00546 =   546, N00547 =   547, N00548 =   548, N00549 =   549, N00550 =   550,
    N00551 =   551, N00552 =   552, N00553 =   553, N00554 =   554, N00555 =   555,
    N00556 =   556, N00557 =   557, N00558 =   558, N00559 =   559, N00560 =   560,
    N00561 =   561, N00562 =   562, N00563 =   563, N00564 =   564, N00565 =   565,
    N00566 =   566, N00567 =   567, N00568 =   568, N00569 =   569, N00570 =   570,
    N00571 =   571, N00572 =   572, N00573 =   573, N00574 =   574, N00575 =   575,
    N00576 =   576, N00577 =   577, N00578 =   578, N00579 =   579, N00580 =   580,
    N00581 =   581, N00582 =   582, N00583 =   583, N00584 =   584, N00585 =   585,
    N00586 =   586, N00587 =   587, N00588 =   588, N00589 =   589, N00590 =   590,
    N00591 =   591, N00592 =   592, N00593 =   593, N00594 =   594, N00595 =   595,
    N00596 =   596, N00597 =   597, N00598 =   598, N00599 =   599, N00600 =   600,
    N00601 =   601, N00602 =   602, N00603 =   603, N00604 =   604, N00605 =   605,
    N00606 =   606, N00607 =   607, N00608 =   608, N00609 =   609, N00610 =   610,
    N00611 =   611, N00612 =   612, N00613 =   613, N00614 =   614, N00615 =   615,
    N00616 =   616, N00617 =   617, N00618 =   618, N00619 =   619, N00620 =   620,
    N00621 =   621, N00622 =   622, N00623 =   623, N00624 =   624, N00625 =   625,
    N00626 =   626, N00627 =   627, N00628 =   628, N00629 =   629, N00630 =   630,
    N00631 =   631, N00632 =   632, N00633 =   633, N00634 =   634, N00635 =   635,
    N00636 =   636, N00637 =   637, N00638 =   638, N00639 =   639, N00640 =   640,
    N00641 =   641, N00642 =   642, N00643 =   643, N00644 =   644, N00645 =   645,
    N00646 =   646, N00647 =   647, N00648 =   648, N00649 =   649, N00650 =   650,
    N00651 =   651, N00652 =   652, N00653 =   653, N00654 =   654, N00655 =   655,
    N00656 =   656, N00657 =   657, N00658 =   658, N00659 =   659, N00660 =   660,
    N00661 =   661, N00662 =   662, N00663 =   663, N00664 =   664, N00665 =   665,
    N00666 =   666, N00667 =   667, N00668 =   668, N00669 =   669, N00670 =   670,
    N00671 =   671, N00672 =   672, N00673 =   673, N00674 =   674, N00675 =   675,
    N00676 =   676, N00677 =   677, N00678 =   678, N00679 =   679, N00680 =   680,
    N00681 =   681, N00682 =   682, N00683 =   683, N00684 =   684, N00685 =   685,
    N00686 =   686, N00687 =   687, N00688 =   688, N00689 =   689, N00690 =   690,
    N00691 =   691, N00692 =   692, N00693 =   693, N00694 =   694, N00695 =   695,
    N00696 =   696, N00697 =   697, N00698 =   698, N00699 =   699, N00700 =   700,
    N00701 =   701, N00702 =   702, N00703 =   703, N00704 =   704, N00705 =   705,
    N00706 =   706, N00707 =   707, N00708 =   708, N00709 =   709, N00710 =   710,
    N00711 =   711, N00712 =   712, N00713 =   713, N00714 =   714, N00715 =   715,
    N00716 =   716, N00717 =   717, N00718 =   718, N00719 =   719, N00720 =   720,
    N00721 =   721, N00722 =   722, N00723 =   723, N00724 =   724, N00725 =   725,
    N00726 =   726, N00727 =   727, N00728 =   728, N00729 =   729, N00730 =   730,
    N00731 =   731, N00732 =   732, N00733 =   733, N00734 =   734, N00735 =   735,
    N00736 =   736, N00737 =   737, N00738 =   738, N00739 =   739, N00740 =   740,
    N00741 =   741, N00742 =   742, N00743 =   743, N00744 =   744, N00745 =   745,
    N00746 =   746, N00747 =   747, N00748 =   748, N00749 =   749, N00750 =   750,
    N00751 =   751, N00752 =   752, N00753 =   753, N00754 =   754, N00755 =   755,
    N00756 =   756, N00757 =   757, N00758 =   758, N00759 =   759, N00760 =   760,
    N00761 =   761, N00762 =   762, N00763 =   763, N00764 =   764, N00765 =   765,
    N00766 =   766, N00767 =   767, N00768 =   768, N00769 =   769, N00770 =   770,
    N00771 =   771, N00772 =   772, N00773 =   773, N00774 =   774, N00775 =   775,
    N00776 =   776, N00777 =   777, N00778 =   778, N00779 =   779, N00780 =   780,
    N00781 =   781, N00782 =   782, N00783 =   783, N00784 =   784, N00785 =   785,
    N00786 =   786, N00787 =   787, N00788 =   788, N00789 =   789, N00790 =   790,
    N00791 =   791, N00792 =   792, N00793 =   793, N00794 =   794, N00795 =   795,
    N00796 =   796, N00797 =   797, N00798 =   798, N00799 =   799, N00800 =   800,
    N00801 =   801, N00802 =   802, N00803 =   803, N00804 =   804, N00805 =   805,
    N00806 =   806, N00807 =   807, N00808 =   808, N00809 =   809, N00810 =   810,
    N00811 =   811, N00812 =   812, N00813 =   813, N00814 =   814, N00815 =   815,
    N00816 =   816, N00817 =   817, N00818 =   818, N00819 =   819, N00820 =   820,
    N00821 =   821, N00822 =   822, N00823 =   823, N00824 =   824, N00825 =   825,
    N00826 =   826, N00827 =   827, N00828 =   828, N00829 =   829, N00830 =   830,
    N00831 =   831, N00832 =   832, N00833 =   833, N00834 =   834, N00835 =   835,
    N00836 =   836, N00837 =   837, N00838 =   838, N00839 =   839, N00840 =   840,
    N00841 =   841, N00842 =   842, N00843 =   843, N00844 =   844, N00845 =   845,
    N00846 =   846, N00847 =   847, N00848 =   848, N00849 =   849, N00850 =   850,
    N00851 =   851, N00852 =   852, N00853 =   853, N00854 =   854, N00855 =   855,
    N00856 =   856, N00857 =   857, N00858 =   858, N00859 =   859, N00860 =   860,
    N00861 =   861, N00862 =   862, N00863 =   863, N00864 =   864, N00865 =   865,
    N00866 =   866, N00867 =   867, N00868 =   868, N00869 =   869, N00870 =   870,
    N00871 =   871, N00872 =   872, N00873 =   873, N00874 =   874, N00875 =   875,
    N00876 =   876, N00877 =   877, N00878 =   878, N00879 =   879, N00880 =   880,
    N00881 =   881, N00882 =   882, N00883 =   883, N00884 =   884, N00885 =   885,
    N00886 =   886, N00887 =   887, N00888 =   888, N00889 =   889, N00890 =   890,
    N00891 =   891, N00892 =   892, N00893 =   893, N00894 =   894, N00895 =   895,
    N00896 =   896, N00897 =   897, N00898 =   898, N00899 =   899, N00900 =   900,
    N00901 =   901, N00902 =   902, N00903 =   903, N00904 =   904, N00905 =   905,
    N00906 =   906, N00907 =   907, N00908 =   908, N00909 =   909, N00910 =   910,
    N00911 =   911, N00912 =   912, N00913 =   913, N00914 =   914, N00915 =   915,
    N00916 =   916, N00917 =   917, N00918 =   918, N00919 =   919, N00920 =   920,
    N00921 =   921, N00922 =   922, N00923 =   923, N00924 =   924, N00925 =   925,
    N00926 =   926, N00927 =   927, N00928 =   928, N00929 =   929, N00930 =   930,
    N00931 =   931, N00932 =   932, N00933 =   933, N00934 =   934, N00935 =   935,
    N00936 =   936, N00937 =   937, N00938 =   938, N00939 =   939, N00940 =   940,
    N00941 =   941, N00942 =   942, N00943 =   943, N00944 =   944, N00945 =   945,
    N00946 =   946, N00947 =   947, N00948 =   948, N00949 =   949, N00950 =   950,
    N00951 =   951, N00952 =   952, N00953 =   953, N00954 =   954, N00955 =   955,
    N00956 =   956, N00957 =   957, N00958 =   958, N00959 =   959, N00960 =   960,
    N00961 =   961, N00962 =   962, N00963 =   963, N00964 =   964, N00965 =   965,
    N00966 =   966, N00967 =   967, N00968 =   968, N00969 =   969, N00970 =   970,
    N00971 =   971, N00972 =   972, N00973 =   973, N00974 =   974, N00975 =   975,
    N00976 =   976, N00977 =   977, N00978 =   978, N00979 =   979, N00980 =   980,
    N00981 =   981, N00982 =   982, N00983 =   983, N00984 =   984, N00985 =   985,
    N00986 =   986, N00987 =   987, N00988 =   988, N00989 =   989, N00990 =   990,
    N00991 =   991, N00992 =   992, N00993 =   993, N00994 =   994, N00995 =   995,
    N00996 =   996, N00997 =   997, N00998 =   998, N00999 =   999, N01000 =  1000,
    N01001 =  1001, N01002 =  1002, N01003 =  1003, N01004 =  1004, N01005 =  1005,
    N01006 =  1006, N01007 =  1007, N01008 =  1008, N01009 =  1009, N01010 =  1010,
    N01011 =  1011, N01012 =  1012, N01013 =  1013, N01014 =  1014, N01015 =  1015,
    N01016 =  1016, N01017 =  1017, N01018 =  1018, N01019 =  1019, N01020 =  1020,
    N01021 =  1021, N01022 =  1022, N01023 =  1023,
}

/// The provided value is not a recognized TRD 104 error code.
#[derive(PartialEq, Eq, Debug)]
pub struct NotAnErrorCode;

impl ErrorCode {
    /// Represent this error code as a string, if defined.
    fn as_str(self) -> Option<&'static str> {
        match self {
            Self::Fail => Some("FAIL"),
            Self::Busy => Some("BUSY"),
            Self::Already => Some("ALREADY"),
            Self::Off => Some("OFF"),
            Self::Reserve => Some("RESERVE"),
            Self::Invalid => Some("INVALID"),
            Self::Size => Some("SIZE"),
            Self::Cancel => Some("CANCEL"),
            Self::NoMem => Some("NOMEM"),
            Self::NoSupport => Some("NOSUPPORT"),
            Self::NoDevice => Some("NODEVICE"),
            Self::Uninstalled => Some("UNINSTALLED"),
            Self::NoAck => Some("NOACK"),
            Self::BadRVal => Some("BADRVAL"),
            _ => None,
        }
    }
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Some(s) => write!(f, "{}", s),
            None => write!(f, "code {}", *self as u16),
        }
    }
}

impl TryFrom<u32> for ErrorCode {
    type Error = NotAnErrorCode;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (1..=1024).contains(&value) {
            Ok(unsafe { transmute(value) })
        } else {
            Err(NotAnErrorCode)
        }
    }
}

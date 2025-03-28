struct BLOSUM62 {
    matrix: [i8; 210],
    // C S T A G P D E Q N H R K M I L V W Y F
    // indexing for i,j : i + sum(j)
}
fn build_BLOSUM62() -> BLOSUM62 {
    BLOSUM62 {
        matrix: [
    //  C   S   T   A   G   P   D   E   Q   N   H   R   K   M   I   L   V   W   Y   F
/*C*/   9,
/*S*/  -1,  4,
/*T*/  -1,  1,  5,
/*A*/   0,  1,  0,  4,
/*G*/  -3,  0, -2,  0,  6,
/*P*/  -3, -1, -1, -1, -2,  7,
/*D*/  -3,  0, -1, -2, -1, -1,  6,
/*E*/  -4,  0, -1, -1, -2, -1,  2,  5,
/*Q*/  -3,  0, -1, -1, -2, -1,  0,  2,  5,
/*N*/  -3,  1,  0, -2,  0, -2,  1,  0,  0,  6,
/*H*/  -3, -1, -2, -2, -2, -2, -1,  0,  0,  1,  8,
/*R*/  -3, -1, -1, -1, -2, -2, -2,  0,  1,  0,  0,  5,
/*K*/  -3,  0, -1, -1, -2, -1, -1,  1,  1,  0, -1,  2,  5,
/*M*/  -1, -1, -1, -1, -3, -2, -3, -2,  0, -2, -2, -1, -1,  5,
/*I*/  -1, -2, -1, -1, -4, -3, -3, -3, -3, -3, -3, -3, -3,  1,  4,
/*L*/  -1, -2, -1, -1, -4, -3, -4, -3, -2, -3, -3, -2, -2,  2,  2,  4,
/*V*/  -1, -2,  0,  0, -3, -2, -3, -2, -2, -3, -3, -3, -2,  1,  3,  1,  4,
/*W*/  -2, -3, -2, -3, -2, -4, -4, -3, -2, -4, -2, -3, -3, -1, -3, -2, -3, 11,
/*Y*/  -2, -2, -2, -2, -3, -3, -3, -2, -1, -2,  2, -2, -2, -1, -1, -1, -1,  2,  7,
/*F*/  -2, -2, -2, -2, -3, -4, -3, -3, -3, -3, -1, -3, -3,  0,  0,  0, -1,  1,  3,  6
        ]
    }
}

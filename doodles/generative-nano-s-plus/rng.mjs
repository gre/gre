export function RNG(seed) {
  if (seed == undefined) {
    seed = new Date().getTime();
  }

  let N = 624;
  let M = 397;
  let MATRIX_A = 0x9908b0df; /* constant vector a */
  let UPPER_MASK = 0x80000000; /* most significant w-r bits */
  let LOWER_MASK = 0x7fffffff; /* least significant r bits */

  let mt = new Array(N); /* the array for the state vector */
  let mti = N + 1; /* mti==N+1 means mt[N] is not initialized */

  if (typeof seed === "object") {
    init_by_array(seed, seed.length);
  } else {
    init_seed(seed);
  }

  function init_seed(s) {
    mt[0] = s >>> 0;
    for (mti = 1; mti < N; mti++) {
      let s = mt[mti - 1] ^ (mt[mti - 1] >>> 30);
      mt[mti] =
        ((((s & 0xffff0000) >>> 16) * 1812433253) << 16) +
        (s & 0x0000ffff) * 1812433253 +
        mti;
      /* See Knuth TAOCP Vol2. 3rd Ed. P.106 for multiplier. */
      /* In the previous versions, MSBs of the seed affect   */
      /* only MSBs of the array mt[].                        */
      /* 2002/01/09 modified by Makoto Matsumoto             */
      mt[mti] >>>= 0;
      /* for >32 bit machines */
    }
  }

  /* initialize by an array with array-length */
  /* init_key is the array for initializing keys */
  /* key_length is its length */
  /* slight change for C++, 2004/2/26 */
  function init_by_array(init_key, key_length) {
    var i, j, k;
    init_seed(19650218);
    i = 1;
    j = 0;
    k = N > key_length ? N : key_length;
    for (; k; k--) {
      var s = mt[i - 1] ^ (mt[i - 1] >>> 30);
      mt[i] =
        (mt[i] ^
          (((((s & 0xffff0000) >>> 16) * 1664525) << 16) +
            (s & 0x0000ffff) * 1664525)) +
        init_key[j] +
        j; /* non linear */
      mt[i] >>>= 0; /* for WORDSIZE > 32 machines */
      i++;
      j++;
      if (i >= N) {
        mt[0] = mt[N - 1];
        i = 1;
      }
      if (j >= key_length) j = 0;
    }
    for (k = N - 1; k; k--) {
      var s = mt[i - 1] ^ (mt[i - 1] >>> 30);
      mt[i] =
        (mt[i] ^
          (((((s & 0xffff0000) >>> 16) * 1566083941) << 16) +
            (s & 0x0000ffff) * 1566083941)) -
        i; /* non linear */
      mt[i] >>>= 0; /* for WORDSIZE > 32 machines */
      i++;
      if (i >= N) {
        mt[0] = mt[N - 1];
        i = 1;
      }
    }

    mt[0] = 0x80000000; /* MSB is 1; assuring non-zero initial array */
  }

  /* generates a random number on [0,0xffffffff]-interval */
  /* origin name genrand_int32 */
  function random_int() {
    var y;
    var mag01 = new Array(0x0, MATRIX_A);
    /* mag01[x] = x * MATRIX_A  for x=0,1 */

    if (mti >= N) {
      /* generate N words at one time */
      var kk;

      if (mti == N + 1)
        /* if init_seed() has not been called, */
        init_seed(5489); /* a default initial seed is used */

      for (kk = 0; kk < N - M; kk++) {
        y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
        mt[kk] = mt[kk + M] ^ (y >>> 1) ^ mag01[y & 0x1];
      }
      for (; kk < N - 1; kk++) {
        y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
        mt[kk] = mt[kk + (M - N)] ^ (y >>> 1) ^ mag01[y & 0x1];
      }
      y = (mt[N - 1] & UPPER_MASK) | (mt[0] & LOWER_MASK);
      mt[N - 1] = mt[M - 1] ^ (y >>> 1) ^ mag01[y & 0x1];

      mti = 0;
    }

    y = mt[mti++];

    /* Tempering */
    y ^= y >>> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= y >>> 18;

    return y >>> 0;
  }

  /* generates a random number on [0,1)-real-interval */
  return () => random_int() * (1.0 / 4294967296.0);
}

p=tr(p,0.489,0.001,4.683);
s=U(s,box(p,vec2(0.079,0.113)),k);
p=pres;
p=tr(p,0.001,0.090,1.487);
s=U(s,box(p,vec2(0.053,0.099)),k);
p=pres;
p=tr(p,0.205,0.387,2.825);
s=U(s,box(p,vec2(0.026,0.126)),k);
p=pres;
p=tr(p,0.210,0.003,5.135+t2);
s=U(s,box(p,vec2(0.023,0.031)),k);
p=pres;
p=tr(p,0.088,0.001,0.376);
s=U(s,box(p,vec2(0.082,0.119)),k);
p=pres;
p=tr(p,0.272,0.149+-0.064*cos(t2),5.790);
s=U(s,box(p,vec2(0.051,0.110)),k);
p=pres;
p=tr(p,0.312,0.034,4.100);
s=U(s,box(p,vec2(0.046,0.182)),k);
p=pres;
p=tr(p,0.175+0.100*sin(t1),0.108,4.741);
s=U(s,box(p,vec2(0.023,0.077)),k);
p=pres;
p=tr(p,0.241,0.170,1.550+t1);
s=U(s,box(p,vec2(0.047,0.052)),k);
p=tr(p,0.264,0.032,2.008);
s=U(s,box(p,vec2(0.027,0.197)),k);
pmm1(p.y,0.228);
pmp(p,16.0);
p=tr(p,0.160,0.116,4.057);
s=U(s,box(p,vec2(0.100,0.106)),k);
p=pres;
p=tr(p,0.242,0.085,0.433);
s=U(s,box(p,vec2(0.051,0.101)),k);
pmp(p,14.0);
p=tr(p,0.208,0.001,2.024);
s=U(s,box(p,vec2(0.028,0.118)),k);
p=pres;
p=tr(p,0.001,0.028,3.789+t3);
s=U(s,box(p,vec2(0.043,0.061)),k);
p=tr(p,0.148,0.224,2.946+t2);
s=U(s,box(p,vec2(0.045,0.067)),k);
p=pres;
p=tr(p,0.309,0.074,0.998);
s=U(s,box(p,vec2(0.140,0.185)),k);
p=pres;
p=tr(p,0.063,0.144,2.217);
s=U(s,box(p,vec2(0.054,0.147)),k);
p=tr(p,0.008,0.077,5.270);
s=U(s,box(p,vec2(0.026,0.110)),k);
p=pres;
p=tr(p,0.010,0.351,4.003);
s=U(s,box(p,vec2(0.028,0.079)),k);
p=tr(p,0.001,0.095,3.422);
s=U(s,box(p,vec2(0.030,0.030)),k);
p=pres;
p=tr(p,0.038,0.048,5.776);
s=U(s,box(p,vec2(0.024,0.052)),k);
p=pres;
p=tr(p,0.070,0.023,0.276);
s=U(s,box(p,vec2(0.118,0.134)),k);
pmm1(p.x,1.108);
p=tr(p,0.026,0.001,0.449+t1);
s=U(s,box(p,vec2(0.029,0.110)),k);
p=tr(p,0.023,0.161,5.384);
s=U(s,box(p,vec2(0.051,0.046)),k);
p=pres;
p=tr(p,0.133,0.053,5.169+t3);
s=U(s,box(p,vec2(0.065,0.166)),k);
p=tr(p,0.032,0.000,5.514);
s=U(s,box(p,vec2(0.105,0.051)),k);
p=o;
p=tr(p,0.130,0.007,1.439);
s=U(s,sf(p,68.,0.005,0.814,1.451),k);
pmm1(p.y,0.334);
p=pres;
p=tr(p,0.355+0.047*sin(t2),0.058,0.616+t2);
s=U(s,box(p,vec2(0.025,0.074)),k);
p=pres;
p=tr(p,0.083,0.042,4.028);
s=U(s,box(p,vec2(0.024,0.059)),k);
pmp(p,3.0);
p=pres;
p=tr(p,0.001+0.035*sin(t1),0.050,5.768);
s=U(s,box(p,vec2(0.119,0.064)),k);
pmm1(p.x,0.801);
s=max(s,-1.135-s);
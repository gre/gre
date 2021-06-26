import * as Day001 from "./001";
import * as Day002 from "./002";
import * as Day003 from "./003";
import * as Day004 from "./004";
import * as Day005 from "./005";
import * as Day006 from "./006";
import * as Day007 from "./007";
import * as Day008 from "./008";
import * as Day009 from "./009";
import * as Day010 from "./010";
import * as Day011 from "./011";
import * as Day012 from "./012";
import * as Day013 from "./013";
import * as Day014 from "./014";
import * as Day015 from "./015";
import * as Day016 from "./016";
import * as Day017 from "./017";
import * as Day018 from "./018";
import * as Day019 from "./019";
import * as Day020 from "./020";
import * as Day021 from "./021";
import * as Day022 from "./022";
import * as Day023 from "./023";
import * as Day024 from "./024";
import * as Day025 from "./025";
import * as Day026 from "./026";
import * as Day027 from "./027";
import * as Day028 from "./028";
import * as Day029 from "./029";
import * as Day030 from "./030";
import * as Day031 from "./031";
import * as Day032 from "./032";
import * as Day033 from "./033";
import * as Day034 from "./034";
import * as Day035 from "./035";
import * as Day036 from "./036";
import * as Day037 from "./037";
import * as Day038 from "./038";
import * as Day039 from "./039";
import * as Day040 from "./040";
import * as Day041 from "./041";
import * as Day042 from "./042";
import * as Day043 from "./043";
import * as Day044 from "./044";
import * as Day045 from "./045";
import * as Day046 from "./046";
import * as Day047 from "./047";
import * as Day048 from "./048";
import * as Day049 from "./049";
import * as Day050 from "./050";
import * as Day051 from "./051";
import * as Day052 from "./052";
import * as Day053 from "./053";
import * as Day054 from "./054";
import * as Day055 from "./055";
import * as Day056 from "./056";
import * as Day057 from "./057";
import * as Day058 from "./058";
import * as Day059 from "./059";
import * as Day060 from "./060";
import * as Day061 from "./061";
import * as Day062 from "./062";
import * as Day063 from "./063";
import * as Day064 from "./064";
import * as Day065 from "./065";
import * as Day066 from "./066";
import * as Day067 from "./067";
import * as Day068 from "./068";
import * as Day069 from "./069";
import * as Day070 from "./070";
import * as Day071 from "./071";
import * as Day072 from "./072";
import * as Day073 from "./073";
import * as Day074 from "./074";
import * as Day075 from "./075";
import * as Day076 from "./076";
import * as Day077 from "./077";
import * as Day078 from "./078";
import * as Day079 from "./079";
import * as Day080 from "./080";
import * as Day081 from "./081";
import * as Day082 from "./082";
import * as Day083 from "./083";
import * as Day084 from "./084";
import * as Day085 from "./085";
import * as Day086 from "./086";
import * as Day087 from "./087";
import * as Day088 from "./088";
import * as Day089 from "./089";
import * as Day090 from "./090";
import * as Day091 from "./091";
import * as Day092 from "./092";
import * as Day093 from "./093";
import * as Day094 from "./094";
import * as Day095 from "./095";
import * as Day096 from "./096";
import * as Day097 from "./097";
import * as Day098 from "./098";
import * as Day099 from "./099";
import * as Day100 from "./100";
import * as Day101 from "./101";
import * as Day102 from "./102";

const days = [
  Day001,
  Day002,
  Day003,
  Day004,
  Day005,
  Day006,
  Day007,
  Day008,
  Day009,
  Day010,
  Day011,
  Day012,
  Day013,
  Day014,
  Day015,
  Day016,
  Day017,
  Day018,
  Day019,
  Day020,
  Day021,
  Day022,
  Day023,
  Day024,
  Day025,
  Day026,
  Day027,
  Day028,
  Day029,
  Day030,
  Day031,
  Day032,
  Day033,
  Day034,
  Day035,
  Day036,
  Day037,
  Day038,
  Day039,
  Day040,
  Day041,
  Day042,
  Day043,
  Day044,
  Day045,
  Day046,
  Day047,
  Day048,
  Day049,
  Day050,
  Day051,
  Day052,
  Day053,
  Day054,
  Day055,
  Day056,
  Day057,
  Day058,
  Day059,
  Day060,
  Day061,
  Day062,
  Day063,
  Day064,
  Day065,
  Day066,
  Day067,
  Day068,
  Day069,
  Day070,
  Day071,
  Day072,
  Day073,
  Day074,
  Day075,
  Day076,
  Day077,
  Day078,
  Day079,
  Day080,
  Day081,
  Day082,
  Day083,
  Day084,
  Day085,
  Day086,
  Day087,
  Day088,
  Day089,
  Day090,
  Day091,
  Day092,
  Day093,
  Day094,
  Day095,
  Day096,
  Day097,
  Day098,
  Day099,
  Day100,
  Day101,
  Day102,
];

export function getDays() {
  return days;
}

export function getToday() {
  let days = getDays();
  return days[days.length - 1];
}

export function findDay(n) {
  let days = getDays();
  return days.find((d) => d.n === n);
}
